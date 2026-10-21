// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The output policy: the one place every write is checked on its way to
//! the lights, whatever surface made it — the probe, the server, MCP.
//!
//! It acts at the output, after the levels, never on them: a level written
//! during blackout is kept, and goes out when blackout ends. In order:
//!
//! 1. **Blackout**: every output sends zeros, at its current length.
//!    Overrides hold, because stopping the light is what it is for.
//! 2. **Hold**: every output keeps sending what it last sent; writes wait.
//! 3. **Master**: every level of a null start code packet is scaled by
//!    `master / 255`; alternate start codes are not levels and pass as they
//!    are.
//!
//! A change wakes every output sharing the policy, so blackout goes out at
//! the next interval, not the next keep-alive.
//!
//! # Concurrency
//!
//! `Send` and `Sync`; share one in an `Arc` across every output of a rig.

use crate::clock::{Wake, lock};
use std::sync::{Arc, Mutex, Weak};

/// The policy's state at one moment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    /// Every output sends zeros.
    pub blackout: bool,
    /// Every output keeps sending what it last sent.
    pub hold: bool,
    /// Every level is scaled by this over 255.
    pub master: u8,
    /// Moves on every change, so an output knows to send again.
    pub generation: u64,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            blackout: false,
            hold: false,
            master: u8::MAX,
            generation: 0,
        }
    }
}

/// The policy, shared by the outputs it governs.
#[derive(Debug, Default)]
pub struct Policy {
    rules: Mutex<Rules>,
    wakes: Mutex<Vec<Weak<Wake>>>,
}

impl Policy {
    /// Live: no blackout, no hold, master full.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The rules as they stand.
    #[must_use]
    pub fn rules(&self) -> Rules {
        *lock(&self.rules)
    }

    /// Blacks out every output, or ends it. Returns whether it changed.
    #[allow(clippy::must_use_candidate, reason = "set for its effect")]
    pub fn set_blackout(&self, on: bool) -> bool {
        self.change(|r| r.blackout = on)
    }

    /// Holds every output, or releases it. Returns whether it changed.
    #[allow(clippy::must_use_candidate, reason = "set for its effect")]
    pub fn set_hold(&self, on: bool) -> bool {
        self.change(|r| r.hold = on)
    }

    /// Sets the master, 0 to 255. Returns whether it changed.
    #[allow(clippy::must_use_candidate, reason = "set for its effect")]
    pub fn set_master(&self, master: u8) -> bool {
        self.change(|r| r.master = master)
    }

    /// Wakes `wake`'s output whenever the rules change.
    pub(crate) fn watch(&self, wake: &Arc<Wake>) {
        let mut wakes = lock(&self.wakes);
        wakes.retain(|w| w.strong_count() > 0);
        wakes.push(Arc::downgrade(wake));
    }

    fn change(&self, f: impl FnOnce(&mut Rules)) -> bool {
        let changed = {
            let mut rules = lock(&self.rules);
            let before = *rules;
            f(&mut rules);
            let changed = *rules != before;
            if changed {
                rules.generation = rules.generation.wrapping_add(1);
            }
            changed
        };
        if changed {
            for wake in lock(&self.wakes).iter().filter_map(Weak::upgrade) {
                wake.notify();
            }
        }
        changed
    }
}

/// A level scaled by `master`: `level × master / 255`, rounded, so full
/// stays full and zero stays zero.
#[must_use]
pub const fn scale(level: u8, master: u8) -> u8 {
    let scaled = (level as u16 * master as u16 + 127) / 255;
    #[allow(clippy::cast_possible_truncation, reason = "at most 255")]
    let scaled = scaled as u8;
    scaled
}

#[cfg(test)]
mod tests {
    use super::scale;

    #[test]
    fn master_scaling_keeps_the_ends_and_rounds_between() {
        assert_eq!(scale(255, 255), 255);
        assert_eq!(scale(0, 255), 0);
        assert_eq!(scale(255, 0), 0);
        assert_eq!(scale(255, 128), 128);
        assert_eq!(scale(100, 128), 50);
        assert_eq!(scale(1, 128), 1, "0.502 rounds up");
    }
}
