// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What a transport hands the runtime for each frame it receives.

use crate::merge::{DEFAULT_PRIORITY, SourceId};

/// A frame that arrived: for which universe, from whom, at what priority,
/// and how many octets of the caller's buffer it fills — start code and
/// slots, header removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Received {
    /// The universe, as this transport's protocol numbers it.
    pub universe: u32,
    /// The source, as the merge will know it.
    pub source: SourceId,
    /// Octets written to the buffer, ready for [`Frame::decode`].
    pub len: usize,
    /// The source's priority for this universe; [`DEFAULT_PRIORITY`] from
    /// a protocol without one.
    pub priority: u8,
    /// The source said it is stopping (sACN's `Stream_Terminated`): the
    /// runtime drops it from the merge now, not after the source timeout,
    /// and does not merge this frame.
    pub ended: bool,
}

impl Received {
    /// `len` octets for `universe` from `source`, at the default priority.
    #[must_use]
    pub const fn new(universe: u32, source: SourceId, len: usize) -> Self {
        Self {
            universe,
            source,
            len,
            priority: DEFAULT_PRIORITY,
            ended: false,
        }
    }

    /// At `priority` instead.
    #[must_use]
    pub const fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Marked as the source's last.
    #[must_use]
    pub const fn ended(mut self) -> Self {
        self.ended = true;
        self
    }
}
