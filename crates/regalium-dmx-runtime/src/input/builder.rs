// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Configuring an [`Input`] before it exists.

use super::{Inner, Input};
use crate::clock::{Clock, Wake};
use crate::event::{Events, Sink};
use regalium_dmx_core::transport::Transport;
use regalium_dmx_telemetry::{Logger, Meter, NoopLogger, NoopMeter};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// Options for an [`Input`]. Required arguments are positional on
/// [`Input::builder`]; everything here is optional.
pub struct InputBuilder<T, C> {
    transport: T,
    clock: C,
    meter: Arc<dyn Meter>,
    logger: Arc<dyn Logger>,
}

impl<T: Transport, C: Clock> InputBuilder<T, C> {
    pub(super) fn new(transport: T, clock: C) -> Self {
        Self {
            transport,
            clock,
            meter: Arc::new(NoopMeter),
            logger: Arc::new(NoopLogger),
        }
    }

    /// Records frames received, sources and sources lost into `meter`.
    #[must_use]
    pub fn meter(mut self, meter: Arc<dyn Meter>) -> Self {
        self.meter = meter;
        self
    }

    /// Logs receive failures and lost sources to `logger`.
    #[must_use]
    pub fn logger(mut self, logger: Arc<dyn Logger>) -> Self {
        self.logger = logger;
        self
    }

    /// The input, and the events it will report. Starts nothing.
    pub fn build(self) -> (Input<T, C>, Events) {
        let (events, rx) = Sink::new();
        let inner = Inner {
            transport: self.transport,
            clock: self.clock,
            wake: Arc::new(Wake::new()),
            closed: AtomicBool::new(false),
            listening: Mutex::new(Vec::new()),
            failing: AtomicBool::new(false),
            events,
            meter: self.meter,
            logger: self.logger,
        };
        let input = Input {
            inner: Arc::new(inner),
        };
        (input, rx)
    }
}
