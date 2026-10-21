// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net over a UDP socket: the core's transport trait, satisfied.
//!
//! [`ArtNet::send`](regalium_dmx_core::transport::Transport::send) wraps a
//! frame in `ArtDmx` and sends it to the target, numbering each universe's
//! packets. [`receive`](regalium_dmx_core::transport::Transport::receive)
//! hands back `ArtDmx` levels and handles the rest inside the wait: it
//! answers an `ArtPoll`, records an `ArtPollReply`, and drops late
//! packets and datagrams that are not Art-Net.
//!
//! # Concurrency
//!
//! `Send` and `Sync`. One thread may send while another receives, so one
//! socket, in an `Arc`, serves a runtime output and input together. Two
//! threads receiving at once would steal each other's packets and timeouts;
//! do not.
//!
//! # Not yet
//!
//! Every universe goes to the one target, broadcast or unicast. Art-Net 4
//! prefers unicast to each node subscribed to the universe, routed by the
//! discovered nodes' output ports; that arrives with the simulator's rig.

mod builder;
mod error;
mod nodes;
mod receive;

pub use builder::Builder;
pub use error::Error;
pub use nodes::{MAX_NODES, Node};

use crate::codec::{ArtDmx, ArtPoll, PortAddress};
use crate::sequence;
use nodes::Nodes;
use regalium_dmx_core::address::SLOTS;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_core::transport::{Received, Transport};
use regalium_dmx_telemetry::Meter;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

/// How often unchanged levels are resent: well inside the time after which
/// an Art-Net node treats its source as gone.
pub const KEEP_ALIVE: Duration = Duration::from_secs(1);

/// What sending and receiving remember.
#[derive(Debug, Default)]
struct State {
    /// The last sequence number sent, per Port-Address.
    sent: HashMap<u16, u8>,
    /// The last sequence number received, per source and Port-Address.
    heard: HashMap<(SourceId, u16), u8>,
    nodes: Nodes,
}

/// An Art-Net 4 socket. See the module documentation for what it does
/// inside a receive, and for concurrency.
pub struct ArtNet {
    socket: UdpSocket,
    target: SocketAddr,
    short_name: String,
    long_name: String,
    refresh: Refresh,
    meter: Arc<dyn Meter>,
    state: Mutex<State>,
}

impl ArtNet {
    /// Starts configuring a socket bound to `bind` that sends to `target`:
    /// usually `0.0.0.0:6454` and a directed broadcast such as
    /// `2.255.255.255:6454`.
    #[must_use]
    pub fn builder(bind: SocketAddr, target: SocketAddr) -> Builder {
        Builder::new(bind, target)
    }

    /// The address the socket is bound to.
    ///
    /// # Errors
    ///
    /// When the operating system cannot say.
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    /// Sends an `ArtPoll` to the target. Replies are recorded by later
    /// receives, and read with [`ArtNet::nodes`].
    ///
    /// # Errors
    ///
    /// When the socket failed.
    pub fn poll(&self) -> Result<(), Error> {
        let mut out = Vec::with_capacity(14);
        ArtPoll::new().encode(&mut out);
        self.socket.send_to(&out, self.target)?;
        Ok(())
    }

    /// Every node that has replied to a poll, by its latest reply.
    #[must_use]
    pub fn nodes(&self) -> Vec<Node> {
        self.state().nodes.all()
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Transport for ArtNet {
    type Error = Error;

    fn protocol(&self) -> &'static str {
        "artnet"
    }

    fn refresh(&self) -> Refresh {
        self.refresh
    }

    /// Art-Net 4 requires an even length of at least two, so an odd or
    /// empty frame is padded with one or two dark slots: the receiver sees
    /// one slot more than the universe holds, at level 0.
    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Error> {
        let port = PortAddress::new(universe).ok_or(Error::PortAddress(universe))?;
        if !frame.start_code().is_null() {
            return Err(Error::StartCode(frame.start_code()));
        }
        let mut padded = [0u8; SLOTS];
        let slots = frame.slots();
        let len = (slots.len() + slots.len() % 2).max(2);
        for (to, from) in padded.iter_mut().zip(slots) {
            *to = *from;
        }
        let sequence = {
            let mut state = self.state();
            let n = state.sent.entry(port.get()).or_insert(0);
            *n = sequence::next(*n).max(1);
            *n
        };
        let mut out = Vec::with_capacity(18 + len);
        ArtDmx::new(port, sequence, padded.get(..len).unwrap_or_default()).encode(&mut out);
        self.socket.send_to(&out, self.target)?;
        Ok(())
    }

    fn receive(&self, buf: &mut [u8], timeout: Duration) -> Result<Option<Received>, Error> {
        self.receive_within(buf, timeout)
    }
}
