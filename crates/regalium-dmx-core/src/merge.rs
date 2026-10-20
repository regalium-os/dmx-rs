// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Combining several sources into one universe: highest takes precedence
//! (HTP) or latest takes precedence (LTP), slot by slot.
//!
//! Which sources take part is decided before they get here. sACN's priority
//! (ANSI E1.31) chooses among sources and hands only the highest-priority
//! ones to a merge; Art-Net merges up to two sources per port. Either way the
//! combining is this module's, so Art-Net and sACN merge identically.
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
use crate::universe::Universe;

/// Sources a merge holds unless built with [`Merge::max_sources`]: enough
/// for a primary console, a backup and a few remotes. Bounded, because
/// sources arrive from the network, and memory per source is ~4.5 KiB.
pub const DEFAULT_SOURCES: usize = 8;

/// How slots from several sources combine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Highest takes precedence: each slot is the highest level any source
    /// sends. Right for intensities; wrong for a pan, where it picks a side.
    Htp,
    /// Latest takes precedence: each slot follows the source that last
    /// changed it.
    Ltp,
}

impl Mode {
    /// The name `dmx.merge.mode` carries: `htp` or `ltp`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Htp => "htp",
            Self::Ltp => "ltp",
        }
    }
}

/// Who a source is, as the transport that received it numbers it: an
/// Art-Net sender's address, an sACN CID mapped to an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(pub u64);

/// One source's last levels, and when it last changed each slot.
#[derive(Clone, Debug)]
struct Input {
    source: SourceId,
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

    /// Takes `frame` as `source`'s latest levels, adding the source if new.
    ///
    /// # Errors
    ///
    /// [`Rejected::NotLevels`] for an alternate start code, and
    /// [`Rejected::TooManySources`] for a new source past the limit; in
    /// either case the merge is unchanged.
    pub fn update(&mut self, source: SourceId, frame: &Frame<'_>) -> Result<(), Rejected> {
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

    /// Writes the merged levels into `universe`, its length the longest
    /// source's. With no sources, the universe goes dark at its current
    /// length. Returns whether the universe changed.
    pub fn render(&self, universe: &mut Universe) -> bool {
        if self.inputs.is_empty() {
            return universe.blackout();
        }
        let len = self.inputs.iter().map(|i| i.len).max().unwrap_or(0);
        let mut out = [0u8; SLOTS];
        for (slot, level) in out.iter_mut().enumerate().take(len) {
            let carrying = self.inputs.iter().filter(|i| slot < i.len);
            *level = match self.mode {
                Mode::Htp => carrying
                    .filter_map(|i| i.levels.get(slot))
                    .max()
                    .copied()
                    .unwrap_or(0),
                Mode::Ltp => carrying
                    .filter_map(|i| Some((i.changed.get(slot)?, i.levels.get(slot)?)))
                    .max_by_key(|(changed, _)| **changed)
                    .map_or(0, |(_, l)| *l),
            };
        }
        universe.replace(&out, u16::try_from(len).unwrap_or(512))
    }
}
