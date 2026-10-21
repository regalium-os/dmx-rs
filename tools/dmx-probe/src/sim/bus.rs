// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A network in memory: ports on a bus satisfy the driver's transport
//! trait, so the real runtime — refresh, merge, priority, source loss —
//! runs over it, with no socket.
//!
//! A frame sent on one port reaches every other port of the same protocol,
//! as a multicast group would, carrying the sender's identity and priority.
//! Each send is also recorded on the capture, encoded as the protocol would
//! put it on the wire.

use crate::engine::Capture;
use crate::engine::model::Protocol;
use regalium_dmx::frame::Frame;
use regalium_dmx::merge::SourceId;
use regalium_dmx::refresh::Refresh;
use regalium_dmx::transport::{Received, Transport};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::time::Duration;

/// Frames a port holds for its receiver before the oldest are dropped.
const QUEUE: usize = 256;

type Queued = (u32, SourceId, u8, Vec<u8>);

#[derive(Default)]
struct Inbox {
    protocol: &'static str,
    queue: VecDeque<Queued>,
}

/// The network.
#[derive(Default)]
pub struct Bus {
    inboxes: Mutex<Vec<Inbox>>,
    arrived: Condvar,
}

/// Why a port could not send: it was made to fail.
#[derive(Debug)]
pub struct Unplugged;

impl std::fmt::Display for Unplugged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("widget unplugged")
    }
}

impl std::error::Error for Unplugged {}

/// One port on the bus.
pub struct Port {
    bus: Arc<Bus>,
    at: usize,
    protocol: Protocol,
    source: SourceId,
    priority: u8,
    refresh: Refresh,
    /// While set, every send fails.
    pub failing: Arc<AtomicBool>,
    capture: Arc<Capture>,
    peer: String,
}

impl Bus {
    /// A port for `protocol`, sending as `source` at `priority`, recording on
    /// `capture` as going to `peer`.
    pub fn port(self: &Arc<Self>, protocol: Protocol, source: u64, priority: u8, capture: &Arc<Capture>, peer: &str) -> Port {
        let mut inboxes = self.inboxes.lock().unwrap_or_else(PoisonError::into_inner);
        inboxes.push(Inbox { protocol: protocol.name(), queue: VecDeque::new() });
        // Art-Net streamed at full rate, as many consoles send it; sACN on
        // change and kept alive, as the driver's own transport does; the
        // line continuously, as a line must be.
        let refresh = match protocol {
            Protocol::Serial | Protocol::ArtNet => Refresh::default(),
            Protocol::Sacn => Refresh::on_change(Refresh::FULL_RATE, Duration::from_secs(1), 0),
        };
        Port {
            bus: Arc::clone(self),
            at: inboxes.len() - 1,
            protocol,
            source: SourceId(source),
            priority,
            refresh,
            failing: Arc::new(AtomicBool::new(false)),
            capture: Arc::clone(capture),
            peer: peer.to_owned(),
        }
    }
}

impl Transport for Port {
    type Error = Unplugged;

    fn protocol(&self) -> &'static str {
        self.protocol.name()
    }

    fn refresh(&self) -> Refresh {
        self.refresh
    }

    fn source_timeout(&self) -> Duration {
        match self.protocol {
            Protocol::Sacn => Duration::from_millis(2500),
            Protocol::ArtNet | Protocol::Serial => Duration::from_secs(10),
        }
    }

    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Unplugged> {
        if self.failing.load(Ordering::Relaxed) {
            return Err(Unplugged);
        }
        let mut octets = Vec::with_capacity(513);
        frame.encode(&mut octets);
        self.capture.record(true, self.protocol, &self.peer, &octets, true);
        let mut inboxes = self.bus.inboxes.lock().unwrap_or_else(PoisonError::into_inner);
        let name = self.protocol.name();
        for (at, inbox) in inboxes.iter_mut().enumerate() {
            if at != self.at && inbox.protocol == name {
                if inbox.queue.len() == QUEUE {
                    inbox.queue.pop_front();
                }
                inbox.queue.push_back((universe, self.source, self.priority, octets.clone()));
            }
        }
        self.bus.arrived.notify_all();
        Ok(())
    }

    fn receive(&self, buf: &mut [u8], timeout: Duration) -> Result<Option<Received>, Unplugged> {
        let inboxes = self.bus.inboxes.lock().unwrap_or_else(PoisonError::into_inner);
        let (mut inboxes, _) = self
            .bus
            .arrived
            .wait_timeout_while(inboxes, timeout, |i| i.get(self.at).is_none_or(|b| b.queue.is_empty()))
            .unwrap_or_else(PoisonError::into_inner);
        let Some((universe, source, priority, octets)) = inboxes.get_mut(self.at).and_then(|b| b.queue.pop_front()) else {
            return Ok(None);
        };
        let mut len = 0;
        for (to, from) in buf.iter_mut().zip(&octets) {
            *to = *from;
            len += 1;
        }
        Ok(Some(Received::new(universe, source, len).priority(priority)))
    }
}
