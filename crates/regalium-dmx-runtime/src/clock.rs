// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The injected clock, and the wake-up a level change sends the output loop.
//!
//! The loop never reads time or sleeps on its own: it asks a [`Clock`]. The
//! [`SystemClock`] waits for real; a [`ManualClock`] jumps straight to each
//! deadline, so a test runs an hour of refresh in milliseconds and every
//! frame lands at exactly the time the policy says.

use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// Time, and waiting for it.
///
/// # Concurrency
///
/// `Send + Sync`: the output thread waits while callers' threads read the time.
pub trait Clock: Send + Sync {
    /// Time since this clock's epoch. Only differences between readings
    /// matter, and readings never go backwards.
    fn now(&self) -> Duration;

    /// Blocks until `deadline`, or until `wake` is notified, whichever comes
    /// first, and consumes the notification.
    fn wait(&self, wake: &Wake, deadline: Duration);
}

impl<C: Clock + ?Sized> Clock for Arc<C> {
    fn now(&self) -> Duration {
        (**self).now()
    }

    fn wait(&self, wake: &Wake, deadline: Duration) {
        (**self).wait(wake, deadline);
    }
}

/// A pending wake-up: set when a level changes or the output closes, so the
/// loop steps at once instead of sleeping until its next deadline.
///
/// `Send + Sync`; any thread may notify, and one thread waits.
#[derive(Debug, Default)]
pub struct Wake {
    pending: Mutex<bool>,
    signal: Condvar,
}

impl Wake {
    /// A wake-up with nothing pending.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wakes the waiting thread, or the next one to wait.
    pub fn notify(&self) {
        *lock(&self.pending) = true;
        self.signal.notify_all();
    }

    /// Consumes a pending notification. Returns whether there was one.
    pub fn take(&self) -> bool {
        std::mem::replace(&mut *lock(&self.pending), false)
    }

    /// Waits at most `timeout` for a notification and consumes it. Returns
    /// whether one came.
    pub fn wait_for(&self, timeout: Duration) -> bool {
        let guard = lock(&self.pending);
        let (mut pending, _) = self
            .signal
            .wait_timeout_while(guard, timeout, |pending| !*pending)
            .unwrap_or_else(PoisonError::into_inner);
        std::mem::replace(&mut *pending, false)
    }
}

/// Real time, from a monotonic clock, waited with a condition variable.
///
/// A wait wakes within the operating system's timer resolution, typically
/// well under a millisecond; what it achieves is on `dmx.frame.interval`.
#[derive(Clone, Copy, Debug)]
pub struct SystemClock {
    epoch: Instant,
}

impl SystemClock {
    /// A clock whose epoch is now.
    #[must_use]
    pub fn new() -> Self {
        Self {
            epoch: Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Duration {
        self.epoch.elapsed()
    }

    fn wait(&self, wake: &Wake, deadline: Duration) {
        let left = deadline.saturating_sub(self.now());
        if left.is_zero() {
            wake.take();
        } else {
            wake.wait_for(left);
        }
    }
}

/// Time that moves only when told, or when waited on: a wait jumps straight
/// to its deadline. For tests and simulations; never for a real rig.
#[derive(Debug, Default)]
pub struct ManualClock {
    now: Mutex<Duration>,
}

impl ManualClock {
    /// A clock at its epoch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Moves the time forward by `by`.
    pub fn advance(&self, by: Duration) {
        let mut now = lock(&self.now);
        *now = now.saturating_add(by);
    }

    /// Moves the time to `to`, if that is later. Time never goes backwards.
    pub fn set(&self, to: Duration) {
        let mut now = lock(&self.now);
        *now = (*now).max(to);
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Duration {
        *lock(&self.now)
    }

    fn wait(&self, wake: &Wake, deadline: Duration) {
        if !wake.take() {
            self.set(deadline);
        }
        // Let a test's thread observe each step rather than racing past it.
        std::thread::yield_now();
    }
}

/// Locks `mutex`, recovering it if another thread panicked while holding it:
/// the data is plain levels and times, still valid, and a lighting output
/// that stops because of one bad thread takes the rig dark with it.
pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
