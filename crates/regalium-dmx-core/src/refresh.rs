// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! When to send a universe: now, or idle until a deadline.
//!
//! The refresh policy is the difference between a rig that follows the
//! console and one that flickers or floods the network. Three numbers say it:
//!
//! - **interval**, the fastest a universe is sent: about 44 Hz for a full
//!   512-slot universe, the most a DMX512 line carries;
//! - **repeats**, how many extra copies of changed levels follow, so a lost
//!   datagram does not leave a fixture at the wrong level until the next
//!   change;
//! - **keep-alive**, how often unchanged levels are resent, so a receiver
//!   knows its source is still there and does not fall into its loss mode.
//!
//! A serial line refreshes continuously (keep-alive equal to the interval):
//! receivers on the wire expect it. Art-Net and sACN send on change and keep
//! alive slowly, as each protocol asks; their transports supply the policy
//! through [`Transport::refresh`](crate::transport::Transport::refresh).
//!
//! # Time is injected
//!
//! [`Refresher::next`] is handed the time and the universe's generation, and
//! never reads a clock. Times are a `Duration` since an epoch the runtime
//! chooses; only differences between them matter.

use core::time::Duration;

/// A refresh policy. `Default` is [`Refresh::continuous`] at
/// [`Refresh::FULL_RATE`]: what a receiver on a DMX512 line expects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Refresh {
    /// The least time between two sends of the universe.
    pub interval: Duration,
    /// The most time between two sends while nothing changes.
    pub keep_alive: Duration,
    /// Extra copies of changed levels, each an `interval` apart.
    pub repeats: u32,
}

impl Refresh {
    /// One full universe on a DMX512 line, about 44 Hz: a 512-slot packet
    /// with its break and mark-after-break takes about this long to send.
    pub const FULL_RATE: Duration = Duration::from_micros(22_727);

    /// Send every `interval`, changed or not.
    #[must_use]
    pub const fn continuous(interval: Duration) -> Self {
        Self {
            interval,
            keep_alive: interval,
            repeats: 0,
        }
    }

    /// Send on change, at most every `interval`, followed by `repeats`
    /// copies; resend unchanged levels every `keep_alive`. A keep-alive
    /// shorter than the interval is raised to it.
    #[must_use]
    pub const fn on_change(interval: Duration, keep_alive: Duration, repeats: u32) -> Self {
        let keep_alive = if keep_alive.as_nanos() < interval.as_nanos() {
            interval
        } else {
            keep_alive
        };
        Self {
            interval,
            keep_alive,
            repeats,
        }
    }
}

impl Default for Refresh {
    fn default() -> Self {
        Self::continuous(Self::FULL_RATE)
    }
}

/// What to do now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Step {
    /// Send the universe now. The refresher has recorded the send.
    Send,
    /// Nothing is due before `until`, unless the universe changes first.
    Idle {
        /// The next deadline.
        until: Duration,
    },
}

/// One universe's refresh state. `Send` and `Sync`; plain data, changed only
/// through `&mut self` by the loop that owns the universe's output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refresher {
    policy: Refresh,
    last: Option<(Duration, u64)>,
    repeats_left: u32,
}

impl Refresher {
    /// A refresher that has sent nothing yet, so its first step is a send.
    #[must_use]
    pub const fn new(policy: Refresh) -> Self {
        Self {
            policy,
            last: None,
            repeats_left: 0,
        }
    }

    /// The policy it follows.
    #[must_use]
    pub const fn policy(&self) -> Refresh {
        self.policy
    }

    /// When the last send was, if there has been one.
    #[must_use]
    pub fn last_sent(&self) -> Option<Duration> {
        self.last.map(|(at, _)| at)
    }

    /// The step at `now` for a universe at `generation`. A [`Step::Send`] is
    /// recorded as sent at `now`; a [`Step::Idle`] changes nothing, so asking
    /// again — after a change woke the loop — is always safe.
    pub fn next(&mut self, now: Duration, generation: u64) -> Step {
        let Some((at, sent)) = self.last else {
            return self.send(now, generation, self.policy.repeats);
        };
        let since = now.saturating_sub(at);
        let (wait, repeats) = if generation != sent {
            (self.policy.interval, self.policy.repeats)
        } else if self.repeats_left > 0 {
            (self.policy.interval, self.repeats_left - 1)
        } else {
            (self.policy.keep_alive, 0)
        };
        if since >= wait {
            self.send(now, generation, repeats)
        } else {
            Step::Idle {
                until: at.saturating_add(wait),
            }
        }
    }

    fn send(&mut self, now: Duration, generation: u64, repeats: u32) -> Step {
        self.last = Some((now, generation));
        self.repeats_left = repeats;
        Step::Send
    }
}
