// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An ENTTEC DMX USB Pro, or a widget speaking its API: the core's
//! transport trait, satisfied.
//!
//! Opening probes the port first and refuses one that does not answer as
//! a widget, so nothing but the probe reaches a port that is something
//! else. The widget generates the line and repeats the last packet on its
//! own, so the host sends on change and keeps alive, as for Art-Net.
//!
//! # Concurrency
//!
//! `Send` and `Sync`. One thread may send while another receives; each
//! message is written whole, under a lock.

use crate::Error;
use crate::discovery::Identity;
use crate::enttec::{Framer, Label, MIN_SLOTS, Message};
use crate::port::{Framing, Port};
use crate::scan::identify;
use regalium_dmx_core::address::SLOTS;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_core::transport::{Received, Transport};
use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;

/// The universe a single-port line carries.
pub const UNIVERSE: u32 = 1;

/// How long a write may wait for the widget to take it.
pub const WRITE: Duration = Duration::from_millis(100);

/// How often unchanged levels are resent; the widget repeats them anyway.
pub const KEEP_ALIVE: Duration = Duration::from_secs(1);

/// Frames received but not yet handed out, before the oldest are dropped.
pub(crate) const PENDING: usize = 8;

/// An open, identified widget.
#[derive(Debug)]
pub struct Enttec {
    pub(crate) port: Port,
    identity: Identity,
    out: Mutex<Vec<u8>>,
    /// The stream's framer and the DMX received but not yet handed out.
    /// Whoever reads the port holds it: a receive, or an RDM exchange.
    pub(crate) framer: Mutex<(Framer, VecDeque<Vec<u8>>)>,
}

pub(crate) fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Enttec {
    /// Opens and probes `path`.
    ///
    /// # Errors
    ///
    /// [`Error::NotAWidget`] when it does not answer as one within the
    /// probe's timeout; [`Error::Io`] when it cannot be opened.
    pub fn open(path: &str) -> Result<Self, Error> {
        let port = Port::open(
            path,
            Framing {
                baud: 57_600,
                two_stop: false,
            },
        )?;
        let identity = identify(&port)?.ok_or_else(|| Error::NotAWidget {
            path: path.to_owned(),
        })?;
        Ok(Self {
            port,
            identity,
            out: Mutex::new(Vec::with_capacity(520)),
            framer: Mutex::new((Framer::new(), VecDeque::new())),
        })
    }

    /// What the widget said about itself when it was opened.
    #[must_use]
    pub const fn identity(&self) -> Identity {
        self.identity
    }

    pub(crate) fn write(&self, label: Label, data: &[u8]) -> Result<(), Error> {
        let mut out = lock(&self.out);
        out.clear();
        Message { label, data }.encode(&mut out);
        self.port.write_all(&out, WRITE)?;
        Ok(())
    }
}

impl Transport for Enttec {
    type Error = Error;

    fn protocol(&self) -> &'static str {
        "serial"
    }

    fn refresh(&self) -> Refresh {
        Refresh::on_change(Refresh::FULL_RATE, KEEP_ALIVE, 0)
    }

    /// Asks the widget to report every packet it receives.
    fn join(&self, universe: u32) -> Result<(), Error> {
        if universe != UNIVERSE {
            return Err(Error::Universe(universe));
        }
        self.write(Label::RECEIVE_ON_CHANGE, &[0])
    }

    /// The API takes at least 24 slots, so a shorter universe is padded
    /// with dark slots: the line carries more slots than the universe.
    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Error> {
        if universe != UNIVERSE {
            return Err(Error::Universe(universe));
        }
        let mut data = [0u8; 1 + SLOTS];
        let slots = frame.slots();
        for (to, from) in data
            .iter_mut()
            .zip(core::iter::once(&frame.start_code().0).chain(slots))
        {
            *to = *from;
        }
        let len = 1 + slots.len().max(MIN_SLOTS);
        self.write(Label::SEND_DMX, data.get(..len).unwrap_or_default())
    }

    fn receive(&self, buf: &mut [u8], timeout: Duration) -> Result<Option<Received>, Error> {
        let mut chunk = [0u8; 256];
        let mut state = lock(&self.framer);
        if state.1.is_empty() {
            let n = self.port.read(&mut chunk, timeout)?;
            let (framer, pending) = &mut *state;
            framer.feed(chunk.get(..n).unwrap_or_default(), |m| {
                // A status other than 0 means the widget lost data: the
                // packet may be torn, so it is not delivered.
                if let (Label::RECEIVED_DMX, Some((0, dmx))) = (m.label, m.data.split_first()) {
                    if pending.len() == PENDING {
                        pending.pop_front();
                    }
                    pending.push_back(dmx.to_vec());
                }
            });
        }
        let Some(dmx) = state.1.pop_front() else {
            return Ok(None);
        };
        let mut len = 0;
        for (to, from) in buf.iter_mut().zip(&dmx) {
            *to = *from;
            len += 1;
        }
        let source = SourceId(self.identity.serial.map_or(0, u64::from));
        Ok(Some(Received::new(UNIVERSE, source, len)))
    }
}
