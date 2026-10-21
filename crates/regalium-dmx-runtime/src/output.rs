// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An output: one transport, the universes patched to it, and the loop that
//! refreshes them.
//!
//! Lifecycle is explicit. [`Output::builder`] constructs and starts nothing.
//! [`Output::run`] blocks the calling thread, refreshing, until
//! [`Output::close`]; [`Output::start`] does the same on one thread of its
//! own and returns a [`Running`](crate::Running) that joins it on close or
//! drop. Close is
//! idempotent and safe from any thread.

use crate::builder::OutputBuilder;
use crate::clock::{Clock, Wake, lock};
use crate::event::Sink;
use crate::levels::{Levels, Slot};
use crate::policy::Policy;
use regalium_dmx_core::refresh::{Refresh, Refresher};
use regalium_dmx_core::transport::Transport;
use regalium_dmx_telemetry::{Logger, Meter, NoopLogger, NoopMeter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// The shortest interval a patch may ask for. A DMX512 packet cannot be
/// sent faster than about a millisecond, and a zero interval would spin.
pub const MIN_INTERVAL: Duration = Duration::from_millis(1);

/// How long the loop waits when nothing is patched, before looking again.
const IDLE: Duration = Duration::from_secs(1);

pub(crate) struct Inner<T, C> {
    pub(crate) transport: T,
    pub(crate) clock: C,
    pub(crate) wake: Arc<Wake>,
    pub(crate) closed: AtomicBool,
    pub(crate) slots: Mutex<Vec<Arc<Slot>>>,
    pub(crate) events: Sink,
    pub(crate) meter: Arc<dyn Meter>,
    pub(crate) logger: Arc<dyn Logger>,
    pub(crate) policy: Arc<Policy>,
}

/// One transport's output. `Clone`: every clone is the same output, so one
/// thread may [`run`](Output::run) it while another patches and closes.
///
/// `Send` and `Sync` when the transport and clock are.
pub struct Output<T, C> {
    pub(crate) inner: Arc<Inner<T, C>>,
}

impl<T, C> Clone for Output<T, C> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Transport, C: Clock> Output<T, C> {
    /// Starts configuring an output over `transport`, timed by `clock`.
    pub fn builder(transport: T, clock: C) -> OutputBuilder<T, C> {
        OutputBuilder {
            transport,
            clock,
            meter: Arc::new(NoopMeter),
            logger: Arc::new(NoopLogger),
            policy: Arc::new(Policy::new()),
        }
    }

    /// Patches `universe` with the transport's own refresh policy, or
    /// returns the handle to it if it is already patched.
    #[must_use]
    pub fn patch(&self, universe: u32) -> Levels {
        self.patch_with(universe, self.inner.transport.refresh())
    }

    /// Patches `universe` with `policy`, its interval raised to at least
    /// [`MIN_INTERVAL`]. An already patched universe keeps its policy.
    #[must_use]
    pub fn patch_with(&self, universe: u32, policy: Refresh) -> Levels {
        let slot = {
            let mut slots = lock(&self.inner.slots);
            if let Some(slot) = slots.iter().find(|s| s.number == universe) {
                Arc::clone(slot)
            } else {
                let interval = policy.interval.max(MIN_INTERVAL);
                let policy = Refresh::on_change(interval, policy.keep_alive, policy.repeats);
                let slot = Arc::new(Slot::new(universe, Refresher::new(policy)));
                slots.push(Arc::clone(&slot));
                slot
            }
        };
        // A new universe goes out at once, not at the next deadline.
        self.inner.wake.notify();
        Levels::new(slot, Arc::clone(&self.inner.wake))
    }

    /// Stops refreshing `universe`, and tells receivers through the
    /// transport's [`end`](Transport::end). Returns whether it was patched.
    /// Handles to it keep working, but nothing sends what they set.
    #[allow(
        clippy::must_use_candidate,
        reason = "unpatching is done for its effect"
    )]
    pub fn unpatch(&self, universe: u32) -> bool {
        let removed = {
            let mut slots = lock(&self.inner.slots);
            let before = slots.len();
            slots.retain(|s| s.number != universe);
            slots.len() != before
        };
        if removed {
            // Receivers time the source out if this is lost; nothing to retry.
            let _ = self.inner.transport.end(universe);
        }
        removed
    }

    /// Sends every universe that is due now, and returns the earliest next
    /// deadline, or `None` when nothing is patched. What [`Output::run`] does
    /// once per wake-up; public so a caller with its own loop can drive it.
    #[allow(
        clippy::must_use_candidate,
        reason = "a step is taken for its sends; the deadline is advice"
    )]
    pub fn step(&self) -> Option<Duration> {
        self.inner.step()
    }

    /// Refreshes until [`Output::close`], on the calling thread.
    pub fn run(&self) {
        let inner = &self.inner;
        while !inner.closed.load(Ordering::Acquire) {
            let next = self.step();
            if inner.closed.load(Ordering::Acquire) {
                break;
            }
            let deadline = next.unwrap_or_else(|| inner.clock.now().saturating_add(IDLE));
            inner.clock.wait(&inner.wake, deadline);
        }
        // Closing ends every universe, as unpatching one does.
        for slot in lock(&inner.slots).iter() {
            let _ = inner.transport.end(slot.number);
        }
    }

    /// Stops [`Output::run`] after its current step. Idempotent.
    pub fn close(&self) {
        self.inner.closed.store(true, Ordering::Release);
        self.inner.wake.notify();
    }

    /// Whether [`Output::close`] has been called.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.closed.load(Ordering::Acquire)
    }

    /// The policy every frame of this output passes through.
    #[must_use]
    pub fn policy(&self) -> &Arc<Policy> {
        &self.inner.policy
    }

    /// Events dropped because the reader fell behind.
    #[must_use]
    pub fn dropped_events(&self) -> u64 {
        self.inner.events.dropped()
    }
}
