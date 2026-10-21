// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An input: one transport, the universes listened to on it, and the loop
//! that receives, merges and drops silent sources.
//!
//! The mirror of [`Output`](crate::Output), with the same lifecycle:
//! [`Input::builder`] starts nothing, [`Input::run`] blocks until
//! [`Input::close`], [`Input::start`] runs it on one thread of its own.
//!
//! # Merging
//!
//! Each listened universe has its own [`Merge`], chosen by the caller: HTP
//! or LTP, and how many sources. Frames for a universe nobody listens to
//! are dropped. A source silent for the transport's
//! [`source_timeout`](regalium_dmx_core::transport::Transport::source_timeout)
//! leaves the merge; when the last leaves, the universe goes dark. Whether
//! a rig instead holds its last look is the output policy's decision, made
//! above this loop, not here.

mod builder;
mod ingest;
mod merged;

pub use builder::InputBuilder;
pub use merged::Merged;

use crate::clock::{Clock, Wake, lock};
use crate::event::Sink;
use merged::Listening;
use regalium_dmx_core::merge::Merge;
use regalium_dmx_core::transport::Transport;
use regalium_dmx_telemetry::{Logger, Meter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// The longest one receive blocks, and so the longest [`Input::close`]
/// waits for the loop: a socket cannot be woken the way a clock can.
pub const MAX_WAIT: Duration = Duration::from_millis(100);

/// How long the loop waits after a receive fails before trying again, so
/// a dead socket is not spun on.
pub const RETRY: Duration = Duration::from_secs(1);

pub(crate) struct Inner<T, C> {
    pub(crate) transport: T,
    pub(crate) clock: C,
    pub(crate) wake: Arc<Wake>,
    pub(crate) closed: AtomicBool,
    pub(crate) listening: Mutex<Vec<Arc<Listening>>>,
    pub(crate) failing: AtomicBool,
    pub(crate) events: Sink,
    pub(crate) meter: Arc<dyn Meter>,
    pub(crate) logger: Arc<dyn Logger>,
}

/// One transport's input. `Clone`: every clone is the same input.
///
/// `Send` and `Sync` when the transport and clock are.
pub struct Input<T, C> {
    inner: Arc<Inner<T, C>>,
}

impl<T, C> Clone for Input<T, C> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Transport, C: Clock> Input<T, C> {
    /// Starts configuring an input over `transport`, timed by `clock`.
    pub fn builder(transport: T, clock: C) -> InputBuilder<T, C> {
        InputBuilder::new(transport, clock)
    }

    /// Merges `universe` by `merge`, joining it on the transport, or
    /// returns the handle to it if it is already listened to; it then keeps
    /// its merge.
    ///
    /// # Errors
    ///
    /// When the transport cannot receive the universe, as when a multicast
    /// group cannot be joined. Nothing is listened to then.
    pub fn listen(&self, universe: u32, merge: Merge) -> Result<Merged, T::Error> {
        let mut listening = lock(&self.inner.listening);
        if let Some(known) = listening.iter().find(|l| l.number == universe) {
            return Ok(Merged {
                listening: Arc::clone(known),
            });
        }
        self.inner.transport.join(universe)?;
        let new = Arc::new(Listening::new(universe, merge));
        listening.push(Arc::clone(&new));
        Ok(Merged { listening: new })
    }

    /// Stops merging `universe` and leaves it on the transport. Returns
    /// whether it was listened to.
    ///
    /// # Errors
    ///
    /// When the transport failed to leave it; it is no longer merged.
    pub fn ignore(&self, universe: u32) -> Result<bool, T::Error> {
        let mut listening = lock(&self.inner.listening);
        let before = listening.len();
        listening.retain(|l| l.number != universe);
        if listening.len() == before {
            return Ok(false);
        }
        self.inner.transport.leave(universe)?;
        Ok(true)
    }

    /// Receives at most one frame, waiting at most `timeout`, merges it,
    /// then drops every source silent too long. Returns whether the
    /// transport worked. What [`Input::run`] does in a loop.
    #[allow(clippy::must_use_candidate, reason = "run retries on its own")]
    pub fn step(&self, timeout: Duration) -> bool {
        self.inner.step(timeout)
    }

    /// Receives until [`Input::close`], on the calling thread.
    pub fn run(&self) {
        let inner = &self.inner;
        while !inner.closed.load(Ordering::Acquire) {
            if !self.step(inner.next_wait()) && !inner.closed.load(Ordering::Acquire) {
                let deadline = inner.clock.now().saturating_add(RETRY);
                inner.clock.wait(&inner.wake, deadline);
            }
        }
    }

    /// Stops [`Input::run`] within [`MAX_WAIT`]. Idempotent.
    pub fn close(&self) {
        self.inner.closed.store(true, Ordering::Release);
        self.inner.wake.notify();
    }

    /// Whether [`Input::close`] has been called.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.closed.load(Ordering::Acquire)
    }

    /// Events dropped because the reader fell behind.
    #[must_use]
    pub fn dropped_events(&self) -> u64 {
        self.inner.events.dropped()
    }
}

impl<T, C> Input<T, C>
where
    T: Transport + Send + Sync + 'static,
    C: Clock + 'static,
{
    /// Runs the input on a thread of its own, named for its protocol.
    ///
    /// # Errors
    ///
    /// When the operating system cannot start a thread.
    pub fn start(&self) -> std::io::Result<crate::Running> {
        let input = self.clone();
        let thread = std::thread::Builder::new()
            .name(format!("dmx-input-{}", self.inner.transport.protocol()))
            .spawn(move || input.run())?;
        let input = self.clone();
        Ok(crate::Running::new(Box::new(move || input.close()), thread))
    }
}
