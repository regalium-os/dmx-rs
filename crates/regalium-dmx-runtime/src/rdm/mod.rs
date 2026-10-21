// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The RDM controller: discovery, GET and SET over any RDM line.
//!
//! The core says what the messages are and what an answer means; this
//! numbers the requests, retries what went unanswered or garbled, joins
//! `ACK_OVERFLOW` parts, follows `ACK_TIMER` through the queue on the
//! injected clock, and records every request on `dmx.rdm.requests` and
//! `dmx.rdm.request.duration`, each in a `dmx.rdm.request` span —
//! discovery in a `dmx.rdm.discovery` span around them.
//!
//! # Writes
//!
//! A SET changes a fixture: its address, its personality, whether it
//! identifies. Every RDM write in this driver goes through
//! [`Controller::request`], which is where the output policy will gate it.
//!
//! # Concurrency
//!
//! `Send` and `Sync` when the line and clock are. Requests from several
//! threads are serialised by the line.

mod discover;
mod error;
mod exchange;

pub use discover::Discovered;
pub use error::RdmError;
pub use exchange::{MAX_PARTS, QUEUE_POLLS};

use crate::clock::{Clock, Wake};
use regalium_dmx_core::rdm::{DeviceInfo, Pid, Request, Uid};
use regalium_dmx_core::transport::RdmLine;
use regalium_dmx_telemetry::{Meter, NoopMeter, NoopTracer, Tracer};
use std::sync::Arc;
use std::sync::atomic::AtomicU8;

/// Tries after the first when a request goes unanswered or garbled.
pub const RETRIES: u8 = 2;

/// A controller on one line.
pub struct Controller<L, C> {
    line: L,
    clock: C,
    wake: Wake,
    uid: Uid,
    port: u8,
    transaction: AtomicU8,
    meter: Arc<dyn Meter>,
    tracer: Arc<dyn Tracer>,
}

impl<L: RdmLine, C: Clock> Controller<L, C> {
    /// A controller sending as `uid` over `line`, timed by `clock`, on
    /// port 1.
    pub fn new(line: L, clock: C, uid: Uid) -> Self {
        Self {
            line,
            clock,
            wake: Wake::new(),
            uid,
            port: 1,
            transaction: AtomicU8::new(0),
            meter: Arc::new(NoopMeter),
            tracer: Arc::new(NoopTracer),
        }
    }

    /// Sends as port `port` instead: 1 to 255.
    #[must_use]
    pub fn port(mut self, port: u8) -> Self {
        self.port = port.max(1);
        self
    }

    /// Records requests and discoveries into `meter`.
    #[must_use]
    pub fn meter(mut self, meter: Arc<dyn Meter>) -> Self {
        self.meter = meter;
        self
    }

    /// Opens request and discovery spans on `tracer`.
    #[must_use]
    pub fn tracer(mut self, tracer: Arc<dyn Tracer>) -> Self {
        self.tracer = tracer;
        self
    }

    /// The UID this controller sends as.
    #[must_use]
    pub const fn uid(&self) -> Uid {
        self.uid
    }

    /// GETs `pid` from `uid`'s root device on `universe`.
    ///
    /// # Errors
    ///
    /// As [`Controller::request`].
    pub fn get(&self, universe: u32, uid: Uid, pid: Pid) -> Result<Vec<u8>, RdmError<L::Error>> {
        self.request(universe, &Request::get(uid, pid))
    }

    /// SETs `pid` on `uid`'s root device to `data`.
    ///
    /// # Errors
    ///
    /// As [`Controller::request`].
    pub fn set(
        &self,
        universe: u32,
        uid: Uid,
        pid: Pid,
        data: &[u8],
    ) -> Result<Vec<u8>, RdmError<L::Error>> {
        self.request(universe, &Request::set(uid, pid, data))
    }

    /// `uid`'s `DEVICE_INFO`.
    ///
    /// # Errors
    ///
    /// As [`Controller::request`], and [`RdmError::Malformed`] when the
    /// answer is not 19 octets.
    pub fn device_info(&self, universe: u32, uid: Uid) -> Result<DeviceInfo, RdmError<L::Error>> {
        let data = self.get(universe, uid, Pid::DEVICE_INFO)?;
        DeviceInfo::decode(&data).ok_or(RdmError::Malformed)
    }
}
