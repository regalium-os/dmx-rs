// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Configuring an [`ArtNet`] socket before it is bound.

use super::{ArtNet, KEEP_ALIVE, State};
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_telemetry::{Meter, NoopMeter};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

/// Options for an [`ArtNet`]. The addresses are positional on
/// [`ArtNet::builder`]; everything here is optional.
pub struct Builder {
    pub(super) bind: SocketAddr,
    pub(super) target: SocketAddr,
    pub(super) short_name: String,
    pub(super) long_name: String,
    pub(super) refresh: Refresh,
    pub(super) meter: Arc<dyn Meter>,
}

impl Builder {
    pub(super) fn new(bind: SocketAddr, target: SocketAddr) -> Self {
        Self {
            bind,
            target,
            short_name: "dmx-rs".to_owned(),
            long_name: "Regalium DMX driver".to_owned(),
            refresh: Refresh::on_change(Refresh::FULL_RATE, KEEP_ALIVE, 0),
            meter: Arc::new(NoopMeter),
        }
    }

    /// The names this controller answers polls with; cut to Art-Net's 17
    /// and 63 octets.
    #[must_use]
    pub fn name(mut self, short: &str, long: &str) -> Self {
        short.clone_into(&mut self.short_name);
        long.clone_into(&mut self.long_name);
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

    /// Binds the socket, with broadcast allowed so the target may be a
    /// broadcast address.
    ///
    /// # Errors
    ///
    /// When the address is in use or cannot be bound.
    pub fn open(self) -> std::io::Result<ArtNet> {
        let socket = UdpSocket::bind(self.bind)?;
        socket.set_broadcast(true)?;
        Ok(ArtNet {
            socket,
            target: self.target,
            short_name: self.short_name,
            long_name: self.long_name,
            refresh: self.refresh,
            meter: self.meter,
            state: Mutex::new(State::default()),
        })
    }
}
