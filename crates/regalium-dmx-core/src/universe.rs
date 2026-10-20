// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A universe: 512 levels, how many of them are sent, and a generation that
//! changes whenever either does.
//!
//! The generation is the change tracking. A refresh loop remembers the
//! generation it last sent; when the universe's differs, there is something
//! new to send now, and when it does not, the loop waits for its keep-alive
//! deadline instead. Comparing one integer is the whole cost, which is what
//! lets a loop check at every tick without copying 512 octets.
//!
//! The universe never reads a clock. When to send is the runtime's decision,
//! from Phase 3; this type only says whether anything changed.

use crate::address::{Address, SLOTS};
use crate::error::Rejected;
use crate::frame::Frame;
use crate::start_code::StartCode;

/// One universe's state. `Default` is dark: every level 0, all 512 slots
/// sent, generation 0.
///
/// `Send` and `Sync`, and plain data: reading from several threads is
/// correct, and every change takes `&mut self`, so the compiler serialises
/// writers. The runtime owns each universe from one task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Universe {
    levels: [u8; SLOTS],
    len: u16,
    generation: u64,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            levels: [0; SLOTS],
            len: 512,
            generation: 0,
        }
    }
}

impl Universe {
    /// A dark universe of 512 slots; the same as `Default`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The level at `address`, whether or not it is within the sent length.
    #[must_use]
    pub fn level(&self, address: Address) -> u8 {
        self.levels.get(address.index()).copied().unwrap_or(0)
    }

    /// The levels sent: the first [`Universe::len`] of them.
    #[must_use]
    pub fn levels(&self) -> &[u8] {
        self.levels.get(..usize::from(self.len)).unwrap_or(&[])
    }

    /// How many slots are sent after the start code, 0 to 512.
    #[must_use]
    pub const fn len(&self) -> u16 {
        self.len
    }

    /// Whether no slots are sent at all: a start code alone.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The generation: it changes whenever a level or the length does, and
    /// never otherwise. Wraps after 2⁶⁴ changes, which at 44 changes a
    /// second is longer than the universe has existed.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Sets one level. Returns whether it changed.
    pub fn set(&mut self, address: Address, level: u8) -> bool {
        let changed = self
            .levels
            .get_mut(address.index())
            .is_some_and(|slot| core::mem::replace(slot, level) != level);
        self.bump(changed)
    }

    /// Writes `levels` from `start` onward. All or nothing: a write that
    /// would run past slot 512 changes no slot. Returns whether any changed.
    ///
    /// # Errors
    ///
    /// [`Rejected::OutOfRange`] when `levels` runs past slot 512.
    pub fn write(&mut self, start: Address, levels: &[u8]) -> Result<bool, Rejected> {
        let range = start.index()..start.index().saturating_add(levels.len());
        let Some(target) = self.levels.get_mut(range) else {
            return Err(Rejected::OutOfRange {
                start: start.get(),
                len: levels.len(),
            });
        };
        let changed = target != levels;
        target.copy_from_slice(levels);
        Ok(self.bump(changed))
    }

    /// Sets how many slots are sent, 0 to 512. Levels past the length are
    /// kept, not cleared, so shortening and lengthening again loses nothing.
    ///
    /// # Errors
    ///
    /// [`Rejected::OutOfRange`] for a length over 512.
    pub fn set_len(&mut self, len: u16) -> Result<bool, Rejected> {
        if usize::from(len) > SLOTS {
            return Err(Rejected::OutOfRange {
                start: 0,
                len: usize::from(len),
            });
        }
        let changed = core::mem::replace(&mut self.len, len) != len;
        Ok(self.bump(changed))
    }

    /// Every level to 0, the length kept. Returns whether anything changed.
    pub fn blackout(&mut self) -> bool {
        let changed = self.levels.iter().any(|&l| l != 0);
        self.levels = [0; SLOTS];
        self.bump(changed)
    }

    /// Takes a received null start code packet as this universe's state:
    /// its levels, and its length. Returns whether anything changed.
    ///
    /// # Errors
    ///
    /// [`Rejected::NotLevels`] for an alternate start code; nothing changes.
    pub fn apply(&mut self, frame: &Frame<'_>) -> Result<bool, Rejected> {
        if !frame.start_code().is_null() {
            return Err(Rejected::NotLevels(frame.start_code()));
        }
        let slots = frame.slots();
        let mut levels = self.levels;
        if let Some(target) = levels.get_mut(..slots.len()) {
            target.copy_from_slice(slots);
        }
        // A frame holds at most 512 slots, so this always fits.
        let len = u16::try_from(slots.len()).unwrap_or(512);
        Ok(self.replace(&levels, len))
    }

    /// This universe as a null start code packet, borrowing its levels.
    #[must_use]
    pub fn frame(&self) -> Frame<'_> {
        Frame::within_limit(StartCode::NULL, self.levels())
    }

    /// Replaces every level and the length at once, as a merge renders.
    pub(crate) fn replace(&mut self, levels: &[u8; SLOTS], len: u16) -> bool {
        let changed = self.levels != *levels || self.len != len;
        self.levels = *levels;
        self.len = len.min(512);
        self.bump(changed)
    }

    fn bump(&mut self, changed: bool) -> bool {
        if changed {
            self.generation = self.generation.wrapping_add(1);
        }
        changed
    }
}
