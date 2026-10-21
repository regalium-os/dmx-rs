// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Rendering a merge: the highest priority present, then HTP or LTP among
//! the sources at it, slot by slot.

use super::{Input, Merge, Mode};
use crate::address::SLOTS;
use crate::universe::Universe;

impl Merge {
    /// The highest priority among the sources, if there are any.
    #[must_use]
    pub fn priority(&self) -> Option<u8> {
        self.inputs.iter().map(|i| i.priority).max()
    }

    /// Every source, with its priority and whether it is merged (at the
    /// highest priority present) or held.
    pub fn list(&self) -> impl Iterator<Item = (super::SourceId, u8, bool)> + '_ {
        let top = self.priority().unwrap_or(0);
        self.inputs.iter().map(move |i| (i.source, i.priority, i.priority == top))
    }

    /// How many sources are at the highest priority, and so merged; the
    /// rest are held.
    #[must_use]
    pub fn merging(&self) -> usize {
        self.leading().count()
    }

    /// Writes the merged levels into `universe`, its length the longest
    /// merged source's. With no sources, the universe goes dark at its
    /// current length. Returns whether the universe changed.
    pub fn render(&self, universe: &mut Universe) -> bool {
        if self.inputs.is_empty() {
            return universe.blackout();
        }
        let len = self.leading().map(|i| i.len).max().unwrap_or(0);
        let mut out = [0u8; SLOTS];
        for (slot, level) in out.iter_mut().enumerate().take(len) {
            let carrying = self.leading().filter(|i| slot < i.len);
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

    /// The sources at the highest priority present.
    fn leading(&self) -> impl Iterator<Item = &Input> {
        let top = self.priority().unwrap_or(0);
        self.inputs.iter().filter(move |i| i.priority == top)
    }
}
