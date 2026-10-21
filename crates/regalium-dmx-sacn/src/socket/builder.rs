// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Configuring a [`Sacn`] socket before it is bound.

use super::{KEEP_ALIVE, Sacn, State};
use crate::codec::Cid;
use regalium_dmx_core::merge::DEFAULT_PRIORITY;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_telemetry::{Meter, NoopMeter};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

/// Options for a [`Sacn`]. The address and CID are positional on
/// [`Sacn::builder`]; everything here is optional.
pub struct Builder {
    bind: SocketAddr,
    cid: Cid,
    name: String,
    priority: u8,
    unicast: Option<SocketAddr>,
    interface: Ipv4Addr,
    refresh: Refresh,
    meter: Arc<dyn Meter>,
}

impl Builder {
    pub(super) fn new(bind: SocketAddr, cid: Cid) -> Self {
        Self {
            bind,
            cid,
            name: "dmx-rs".to_owned(),
            priority: DEFAULT_PRIORITY,
            unicast: None,
            interface: Ipv4Addr::UNSPECIFIED,
            refresh: Refresh::on_change(Refresh::FULL_RATE, KEEP_ALIVE, 0),
            meter: Arc::new(NoopMeter),
        }
    }

    /// The source name sent with every packet; cut to 63 octets.
    #[must_use]
    pub fn name(mut self, name: &str) -> Self {
        name.clone_into(&mut self.name);
        self
    }

    /// The priority sent with every data packet, 0 to 200; above 200 is
    /// sent as 200.
    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(200);
        self
    }

    /// Sends every universe, and discovery, to `to` instead of to the
    /// multicast groups: for a receiver that asks for unicast, and for
    /// tests on loopback.
    #[must_use]
    pub const fn unicast(mut self, to: SocketAddr) -> Self {
        self.unicast = Some(to);
        self
    }

    /// Joins multicast groups on the interface with this address, rather
    /// than the one the operating system picks.
    #[must_use]
    pub const fn interface(mut self, address: Ipv4Addr) -> Self {
        self.interface = address;
        self
    }

    /// Refreshes universes by `refresh` instead of on change and every
    /// [`KEEP_ALIVE`].
    #[must_use]
    pub const fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = refresh;
        self
    }

    /// Counts late packets dropped on `dmx.universe.sequence_errors`.
    #[must_use]
    pub fn meter(mut self, meter: Arc<dyn Meter>) -> Self {
        self.meter = meter;
        self
    }

    /// Binds the socket.
    ///
    /// # Errors
    ///
    /// When the address is in use or cannot be bound. Without
    /// `SO_REUSEADDR`, which `std` cannot set before binding, only one
    /// process on a host can hold port 5568.
    pub fn open(self) -> std::io::Result<Sacn> {
        let socket = UdpSocket::bind(self.bind)?;
        Ok(Sacn {
            socket,
            cid: self.cid,
            name: self.name,
            priority: self.priority,
            unicast: self.unicast,
            interface: self.interface,
            refresh: self.refresh,
            meter: self.meter,
            state: Mutex::new(State::default()),
        })
    }
}
