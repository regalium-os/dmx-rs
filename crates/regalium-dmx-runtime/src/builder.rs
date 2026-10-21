// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Configuring an [`Output`] before it exists: the optional parts, with
//! no-op defaults, so adding one never breaks a caller.

use crate::clock::{Clock, Wake};
use crate::event::Events;
use crate::event::Sink;
use crate::output::{Inner, Output};
use crate::policy::Policy;
use regalium_dmx_core::transport::Transport;
use regalium_dmx_telemetry::{Logger, Meter};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// Options for an [`Output`]. Required arguments are positional on
/// [`Output::builder`]; everything here is optional.
pub struct OutputBuilder<T, C> {
    pub(crate) transport: T,
    pub(crate) clock: C,
    pub(crate) meter: Arc<dyn Meter>,
    pub(crate) logger: Arc<dyn Logger>,
    pub(crate) policy: Arc<Policy>,
}

impl<T: Transport, C: Clock> OutputBuilder<T, C> {
    /// Records frames, intervals and packets into `meter`.
    #[must_use]
    pub fn meter(mut self, meter: Arc<dyn Meter>) -> Self {
        self.meter = meter;
        self
    }

    /// Logs failures and recoveries to `logger`.
    #[must_use]
    pub fn logger(mut self, logger: Arc<dyn Logger>) -> Self {
        self.logger = logger;
        self
    }

    /// Passes every frame through `policy`, shared with the rig's other
    /// outputs, rather than a policy of its own.
    #[must_use]
    pub fn policy(mut self, policy: Arc<Policy>) -> Self {
        self.policy = policy;
        self
    }

    /// The output, and the events it will report. Starts nothing.
    pub fn build(self) -> (Output<T, C>, Events) {
        let (events, rx) = Sink::new();
        let wake = Arc::new(Wake::new());
        self.policy.watch(&wake);
        let inner = Inner {
            transport: self.transport,
            clock: self.clock,
            wake,
            closed: AtomicBool::new(false),
            slots: Mutex::new(Vec::new()),
            events,
            meter: self.meter,
            logger: self.logger,
            policy: self.policy,
        };
        (
            Output {
                inner: Arc::new(inner),
            },
            rx,
        )
    }
}
