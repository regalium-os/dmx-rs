// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// Shared by several test binaries, each of which uses a subset.
#![allow(dead_code)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

//! A transport that remembers what it was asked to send, and when, by the
//! same manual clock the output runs on; and a switch to make it fail.

pub(crate) mod bench;
pub(crate) mod script;

use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_core::transport::{Received, Transport};
use regalium_dmx_runtime::{Clock, ManualClock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// One send: when, which universe, and the octets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Sent {
    pub(crate) at: Duration,
    pub(crate) universe: u32,
    pub(crate) octets: Vec<u8>,
}

/// The test's view of the transport: its log and its failure switch.
#[derive(Clone, Default)]
pub(crate) struct Wire {
    pub(crate) log: Arc<Mutex<Vec<Sent>>>,
    pub(crate) failing: Arc<AtomicBool>,
    pub(crate) ended: Arc<Mutex<Vec<u32>>>,
}

impl Wire {
    pub(crate) fn sent(&self) -> Vec<Sent> {
        self.log.lock().unwrap().clone()
    }

    pub(crate) fn fail(&self, on: bool) {
        self.failing.store(on, Ordering::SeqCst);
    }
}

/// The transport the output owns.
pub(crate) struct Recording {
    pub(crate) clock: Arc<ManualClock>,
    pub(crate) wire: Wire,
    pub(crate) refresh: Refresh,
}

#[derive(Debug)]
pub(crate) struct Unplugged;

impl std::fmt::Display for Unplugged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("widget unplugged")
    }
}

impl std::error::Error for Unplugged {}

impl Transport for Recording {
    type Error = Unplugged;

    fn protocol(&self) -> &'static str {
        "serial"
    }

    fn refresh(&self) -> Refresh {
        self.refresh
    }

    fn end(&self, universe: u32) -> Result<(), Unplugged> {
        self.wire.ended.lock().unwrap().push(universe);
        Ok(())
    }

    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Unplugged> {
        if self.wire.failing.load(Ordering::SeqCst) {
            return Err(Unplugged);
        }
        let mut octets = Vec::new();
        frame.encode(&mut octets);
        self.wire.log.lock().unwrap().push(Sent {
            at: self.clock.now(),
            universe,
            octets,
        });
        Ok(())
    }

    fn receive(&self, _: &mut [u8], _: Duration) -> Result<Option<Received>, Unplugged> {
        Ok(None)
    }
}

/// A recording transport on a fresh manual clock, refreshing by `refresh`.
pub(crate) fn rig(refresh: Refresh) -> (Recording, Arc<ManualClock>, Wire) {
    let clock = Arc::new(ManualClock::new());
    let wire = Wire::default();
    let transport = Recording {
        clock: Arc::clone(&clock),
        wire: wire.clone(),
        refresh,
    };
    (transport, clock, wire)
}

pub(crate) fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}
