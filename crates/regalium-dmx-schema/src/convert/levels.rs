// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A universe's levels, and its merge mode.

use crate::universe::{Levels, MergeMode};
use regalium_dmx_core::merge::Mode;
use regalium_dmx_core::universe::Universe;

/// `universe`'s levels as the message for `name` (`universes/x`): its
/// slots, as many as are sent, and its generation. The update time is the
/// caller's, who knows the clock.
#[must_use]
pub fn levels(name: &str, universe: &Universe) -> Levels {
    Levels {
        name: format!("{name}/levels"),
        slots: universe.levels().to_vec(),
        generation: i64::try_from(universe.generation()).unwrap_or(i64::MAX),
        update_time: None,
    }
}

/// The core's mode for a message's, or `None` for one unset.
#[must_use]
pub const fn mode(merge: MergeMode) -> Option<Mode> {
    match merge {
        MergeMode::Htp => Some(Mode::Htp),
        MergeMode::Ltp => Some(Mode::Ltp),
        MergeMode::Unspecified => None,
    }
}

/// The message's mode for the core's.
#[must_use]
pub const fn merge_mode(mode: Mode) -> MergeMode {
    match mode {
        Mode::Htp => MergeMode::Htp,
        Mode::Ltp => MergeMode::Ltp,
    }
}
