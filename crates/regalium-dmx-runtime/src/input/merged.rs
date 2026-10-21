// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A universe an input listens to: its merge, the universe the merge
//! renders, and when each source was last heard.

use crate::clock::lock;
use regalium_dmx_core::merge::{Merge, SourceId};
use regalium_dmx_core::universe::Universe;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// What the input loop keeps for one universe.
#[derive(Debug)]
pub(crate) struct Listening {
    pub(crate) number: u32,
    pub(crate) state: Mutex<MergeState>,
}

#[derive(Debug)]
pub(crate) struct MergeState {
    pub(crate) merge: Merge,
    pub(crate) universe: Universe,
    /// Each merged source and when it was last heard; as long as the
    /// merge's own list, which is bounded.
    pub(crate) heard: Vec<(SourceId, Duration)>,
    /// When the last frame arrived, from any source, for the interval.
    pub(crate) last_frame: Option<Duration>,
}

impl Listening {
    pub(crate) fn new(number: u32, merge: Merge) -> Self {
        Self {
            number,
            state: Mutex::new(MergeState {
                merge,
                universe: Universe::new(),
                heard: Vec::new(),
                last_frame: None,
            }),
        }
    }
}

/// The handle to one universe an input merges.
///
/// `Clone`, `Send` and `Sync`. Reading locks the merge briefly; the input
/// loop holds the same lock only while it merges one frame.
#[derive(Clone, Debug)]
pub struct Merged {
    pub(crate) listening: Arc<Listening>,
}

impl Merged {
    /// The universe, as the transport's protocol numbers it.
    #[must_use]
    pub fn universe(&self) -> u32 {
        self.listening.number
    }

    /// A copy of the merged levels. Its generation moves when they do, so
    /// a caller polling it can tell a change from a repeat.
    #[must_use]
    pub fn snapshot(&self) -> Universe {
        lock(&self.listening.state).universe.clone()
    }

    /// How many sources are merged now.
    #[must_use]
    pub fn sources(&self) -> usize {
        lock(&self.listening.state).merge.sources()
    }
}
