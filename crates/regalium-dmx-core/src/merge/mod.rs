// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Combining several sources into one universe: highest takes precedence
//! (HTP) or latest takes precedence (LTP), slot by slot.
//!
//! # Priority first
//!
//! Each source has a priority, 0 to 200 as ANSI E1.31 numbers them. Only
//! the sources at the highest priority present take part; the rest are
//! held, not merged, and take over when the higher ones leave. Art-Net has
//! no priority, so every Art-Net source is at [`DEFAULT_PRIORITY`] and all
//! of them merge. Either way the combining is this module's, so Art-Net
//! and sACN merge identically.
//!
//! # LTP is per slot, and by change
//!
//! A source takes a slot in LTP when it *changes* that slot's level, not
//! when it sends it. Consoles resend every slot at the refresh rate, so
//! "latest packet wins" would hand the whole universe to whichever console
//! happened to send last; "latest change wins" hands each slot to the
//! operator who last moved it. A source's first packet counts as changing
//! every slot it carries.
//!
//! # No span
//!
//! A merge runs once per received frame. Its effect is on the runtime's
//! source and frame metrics; see the frame module on why there is no span.

use crate::address::SLOTS;
use crate::error::Rejected;
use crate::frame::Frame;

/// Sources a merge holds unless built with [`Merge::max_sources`]: enough
/// for a primary console, a backup and a few remotes. Bounded, because
/// sources arrive from the network, and memory per source is ~4.5 KiB.
pub const DEFAULT_SOURCES: usize = 8;

/// The priority of a source that states none: ANSI E1.31's default, 100.
pub const DEFAULT_PRIORITY: u8 = 100;

mod mode;
mod render;

pub use mode::{Mode, SourceId};

/// One source's last levels, and when it last changed each slot.
#[derive(Clone, Debug)]
struct Input {
    source: SourceId,
    priority: u8,
    levels: [u8; SLOTS],
    len: usize,
    changed: [u64; SLOTS],
}

/// A merge of up to a fixed number of sources into one universe.
///
/// `Send` and `Sync`, and plain data; every change takes `&mut self`. The
/// runtime owns one per merged universe, beside the universe it renders to.
#[derive(Clone, Debug)]
pub struct Merge {
    mode: Mode,
    limit: usize,
    inputs: Vec<Input>,
    sequence: u64,
}

impl Merge {
    /// An empty merge in `mode`, holding up to [`DEFAULT_SOURCES`] sources.
    #[must_use]
    pub const fn new(mode: Mode) -> Self {
        Self {
            mode,
            limit: DEFAULT_SOURCES,
            inputs: Vec::new(),
            sequence: 0,
        }
    }

    /// Holds up to `limit` sources instead.
    #[must_use]
    pub const fn max_sources(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// The mode.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// How many sources are merged now.
    #[must_use]
    pub fn sources(&self) -> usize {
        self.inputs.len()
    }

    /// Takes `frame` as `source`'s latest levels at [`DEFAULT_PRIORITY`],
    /// adding the source if new.
    ///
    /// # Errors
    ///
    /// As [`Merge::update_at`].
    pub fn update(&mut self, source: SourceId, frame: &Frame<'_>) -> Result<(), Rejected> {
        self.update_at(source, DEFAULT_PRIORITY, frame)
    }

    /// Takes `frame` as `source`'s latest levels at `priority`, adding the
    /// source if new. A source may change its priority between frames.
    ///
    /// # Errors
    ///
    /// [`Rejected::NotLevels`] for an alternate start code, and
    /// [`Rejected::TooManySources`] for a new source past the limit; in
    /// either case the merge is unchanged.
    pub fn update_at(
        &mut self,
        source: SourceId,
        priority: u8,
        frame: &Frame<'_>,
    ) -> Result<(), Rejected> {
        if !frame.start_code().is_null() {
            return Err(Rejected::NotLevels(frame.start_code()));
        }
        let at = match self.inputs.iter().position(|i| i.source == source) {
            Some(at) => at,
            None if self.inputs.len() >= self.limit => {
                return Err(Rejected::TooManySources { limit: self.limit });
            }
            None => {
                self.inputs.push(Input {
                    source,
                    priority,
                    levels: [0; SLOTS],
                    len: 0,
                    changed: [0; SLOTS],
                });
                self.inputs.len() - 1
            }
        };
        self.sequence += 1;
        let sequence = self.sequence;
        let Some(input) = self.inputs.get_mut(at) else {
            return Ok(());
        };
        input.priority = priority;
        let slots = frame.slots();
        let fresh = input.len;
        for (i, ((level, changed), &new)) in input
            .levels
            .iter_mut()
            .zip(input.changed.iter_mut())
            .zip(slots)
            .enumerate()
        {
            if i >= fresh || *level != new {
                *changed = sequence;
            }
            *level = new;
        }
        input.len = slots.len();
        Ok(())
    }

    /// Drops `source`, as when an sACN source times out. Returns whether it
    /// was there.
    pub fn remove(&mut self, source: SourceId) -> bool {
        let before = self.inputs.len();
        self.inputs.retain(|i| i.source != source);
        self.inputs.len() != before
    }
}
