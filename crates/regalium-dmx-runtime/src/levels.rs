// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A patched universe's levels, as its owner changes them, and the slot the
//! output loop keeps for it.
//!
//! [`Levels`] is the caller's handle. Every change goes through the
//! universe's own methods, and a change that moves the generation wakes the
//! output loop, so new levels go out at the next interval rather than at the
//! next keep-alive.

use crate::clock::{Wake, lock};
use regalium_dmx_core::address::Address;
use regalium_dmx_core::error::Rejected;
use regalium_dmx_core::refresh::Refresher;
use regalium_dmx_core::universe::Universe;
use std::sync::{Arc, Mutex};

/// One patched universe, shared between its [`Levels`] handles and the
/// output loop.
#[derive(Debug)]
pub(crate) struct Slot {
    pub(crate) number: u32,
    pub(crate) universe: Mutex<Universe>,
    pub(crate) state: Mutex<SlotState>,
}

/// What only the output loop touches: the refresh state, the encode buffer
/// it reuses, and whether the transport is failing for this universe.
#[derive(Debug)]
pub(crate) struct SlotState {
    pub(crate) refresher: Refresher,
    pub(crate) buf: Vec<u8>,
    pub(crate) failing: bool,
    /// Moves when what a receiver would see moves: the levels (unless
    /// held) or the policy. The refresher's generation.
    pub(crate) counter: u64,
    /// The universe and policy generations the counter last counted.
    pub(crate) universe_seen: u64,
    pub(crate) policy_seen: u64,
    /// The last frame sent outside blackout: what hold keeps sending.
    pub(crate) held: Vec<u8>,
}

impl Slot {
    pub(crate) fn new(number: u32, refresher: Refresher) -> Self {
        Self {
            number,
            universe: Mutex::new(Universe::new()),
            state: Mutex::new(SlotState {
                refresher,
                buf: Vec::with_capacity(513),
                failing: false,
                counter: 0,
                universe_seen: 0,
                policy_seen: 0,
                held: Vec::with_capacity(513),
            }),
        }
    }
}

/// The handle to one patched universe's levels.
///
/// `Clone`, `Send` and `Sync`: hand one to each thread that drives the
/// universe. Each change locks the universe briefly; the output loop holds
/// the lock only to copy the levels out, never while sending.
#[derive(Clone, Debug)]
pub struct Levels {
    slot: Arc<Slot>,
    wake: Arc<Wake>,
}

impl Levels {
    pub(crate) fn new(slot: Arc<Slot>, wake: Arc<Wake>) -> Self {
        Self { slot, wake }
    }

    /// The universe, as the transport's protocol numbers it.
    #[must_use]
    pub fn universe(&self) -> u32 {
        self.slot.number
    }

    /// Sets one level. Returns whether it changed.
    #[allow(
        clippy::must_use_candidate,
        reason = "a caller setting a level rarely cares whether it moved"
    )]
    pub fn set(&self, address: Address, level: u8) -> bool {
        self.change(|u| u.set(address, level))
    }

    /// Writes `levels` from `start`, all or nothing.
    ///
    /// # Errors
    ///
    /// [`Rejected::OutOfRange`] when they run past slot 512.
    pub fn write(&self, start: Address, levels: &[u8]) -> Result<bool, Rejected> {
        self.change(|u| u.write(start, levels))
    }

    /// Sets how many slots are sent, 0 to 512.
    ///
    /// # Errors
    ///
    /// [`Rejected::OutOfRange`] for a length over 512.
    pub fn set_len(&self, len: u16) -> Result<bool, Rejected> {
        self.change(|u| u.set_len(len))
    }

    /// Every level to 0. Returns whether anything changed.
    pub fn blackout(&self) -> bool {
        self.change(Universe::blackout)
    }

    /// A copy of the universe as it stands.
    #[must_use]
    pub fn snapshot(&self) -> Universe {
        lock(&self.slot.universe).clone()
    }

    /// Applies `f`, and wakes the output loop if the universe changed.
    fn change<R>(&self, f: impl FnOnce(&mut Universe) -> R) -> R {
        let (result, moved) = {
            let mut universe = lock(&self.slot.universe);
            let before = universe.generation();
            let result = f(&mut universe);
            (result, universe.generation() != before)
        };
        if moved {
            self.wake.notify();
        }
        result
    }
}
