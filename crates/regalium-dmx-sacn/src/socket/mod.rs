// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! sACN over a UDP socket: the core's transport trait, satisfied.
//!
//! [`send`](Transport::send) wraps a frame in a data packet and sends it to
//! the universe's multicast group, numbering each universe's packets, and
//! announces the universes sent every ten seconds on the discovery
//! universe. [`receive`](Transport::receive) hands back data from the
//! groups [`join`](Transport::join)ed, with the source's priority, and
//! marks a stream-terminated packet as its source's last. Preview data,
//! late packets, this socket's own packets looped back, and datagrams that
//! are not E1.31 are dropped inside the wait.
//!
//! # Concurrency
//!
//! `Send` and `Sync`. One thread may send while another receives, so one
//! socket, in an `Arc`, serves a runtime output and input together. Two
//! threads receiving at once would steal each other's packets; do not.

mod announce;
mod builder;
mod error;
mod receive;

pub use announce::{ANNOUNCE, MAX_SOURCES, Source};
pub use builder::Builder;
pub use error::Error;

use crate::codec::{Cid, DataPacket, MAX_UNIVERSE, PORT, TERMINATED, multicast, name};
use announce::Sources;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::Transport;
use regalium_dmx_telemetry::Meter;
use std::collections::{BTreeSet, HashMap};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

/// How often unchanged levels are resent.
pub const KEEP_ALIVE: Duration = Duration::from_secs(1);

/// How long a source may be silent before receivers drop it: E1.31's
/// network data loss timeout, 2.5 s.
pub const DATA_LOSS: Duration = Duration::from_millis(2500);

/// Stream-terminated packets sent when a universe ends: E1.31's three, so
/// one lost packet does not leave receivers waiting out the timeout.
pub const TERMINATIONS: usize = 3;

#[derive(Debug, Default)]
struct State {
    /// The last sequence number sent, per universe.
    sent: HashMap<u16, u8>,
    /// The universes being sent, for discovery.
    sending: BTreeSet<u16>,
    /// The last sequence number received, per source and universe.
    heard: HashMap<(u64, u16), u8>,
    announced: Option<Instant>,
    sources: Sources,
}

/// An E1.31 socket. See the module documentation for what it does.
pub struct Sacn {
    socket: UdpSocket,
    cid: Cid,
    name: String,
    priority: u8,
    unicast: Option<SocketAddr>,
    interface: Ipv4Addr,
    refresh: Refresh,
    meter: Arc<dyn Meter>,
    state: Mutex<State>,
}

impl Sacn {
    /// Starts configuring a socket bound to `bind` that sends as `cid`:
    /// usually `0.0.0.0:5568` and a CID the caller stores across restarts.
    #[must_use]
    pub fn builder(bind: SocketAddr, cid: Cid) -> Builder {
        Builder::new(bind, cid)
    }

    /// The address the socket is bound to.
    ///
    /// # Errors
    ///
    /// When the operating system cannot say.
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Where `universe` goes: the unicast target, or its multicast group.
    fn destination(&self, universe: u16) -> SocketAddr {
        self.unicast.unwrap_or_else(|| {
            let group = multicast(universe).unwrap_or(Ipv4Addr::BROADCAST);
            SocketAddr::V4(SocketAddrV4::new(group, PORT))
        })
    }

    fn data(&self, universe: u16, options: u8, frame: &Frame<'_>) -> Result<(), Error> {
        let sequence = {
            let mut state = self.state();
            let n = state.sent.entry(universe).or_insert(u8::MAX);
            *n = n.wrapping_add(1);
            *n
        };
        let packet = DataPacket {
            cid: self.cid,
            source_name: name(&self.name),
            priority: self.priority,
            sync_address: 0,
            sequence,
            options,
            universe,
            start_code: frame.start_code(),
            slots: frame.slots(),
        };
        let mut out = Vec::with_capacity(638);
        packet.encode(&mut out);
        self.socket.send_to(&out, self.destination(universe))?;
        Ok(())
    }
}

/// `universe` as a data universe, 1 to 63 999.
fn data_universe(universe: u32) -> Result<u16, Error> {
    u16::try_from(universe)
        .ok()
        .filter(|u| (1..=MAX_UNIVERSE).contains(u))
        .ok_or(Error::Universe(universe))
}

impl Transport for Sacn {
    type Error = Error;

    fn protocol(&self) -> &'static str {
        "sacn"
    }

    fn refresh(&self) -> Refresh {
        self.refresh
    }

    fn source_timeout(&self) -> Duration {
        DATA_LOSS
    }

    fn join(&self, universe: u32) -> Result<(), Error> {
        let group = multicast(data_universe(universe)?).unwrap_or(Ipv4Addr::BROADCAST);
        self.socket.join_multicast_v4(&group, &self.interface)?;
        Ok(())
    }

    fn leave(&self, universe: u32) -> Result<(), Error> {
        let group = multicast(data_universe(universe)?).unwrap_or(Ipv4Addr::BROADCAST);
        self.socket.leave_multicast_v4(&group, &self.interface)?;
        Ok(())
    }

    fn end(&self, universe: u32) -> Result<(), Error> {
        let universe = data_universe(universe)?;
        self.state().sending.remove(&universe);
        let last = Frame::within_limit(StartCode::NULL, &[]);
        for _ in 0..TERMINATIONS {
            self.data(universe, TERMINATED, &last)?;
        }
        Ok(())
    }

    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Error> {
        let universe = data_universe(universe)?;
        self.data(universe, 0, frame)?;
        if self.state().sending.insert(universe) {
            // A new universe is announced at once, not ten seconds later.
            self.state().announced = None;
        }
        self.announce_if_due()
    }

    fn receive(
        &self,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<Option<regalium_dmx_core::transport::Received>, Error> {
        self.receive_within(buf, timeout)
    }
}
