// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! What a transport says about its sources reaches the merge: priority,
//! a source ending, and which universes it is asked to receive.

mod common;

use common::script::Script;
use regalium_dmx_core::merge::{Merge, Mode, SourceId};
use regalium_dmx_core::transport::Received;
use regalium_dmx_runtime::{Event, Input, ManualClock};
use std::sync::Arc;
use std::time::Duration;

fn rig() -> (
    Input<Arc<Script>, Arc<ManualClock>>,
    regalium_dmx_runtime::Events,
    Arc<Script>,
) {
    let script = Arc::new(Script::default());
    let clock = Arc::new(ManualClock::new());
    let (input, events) = Input::builder(Arc::clone(&script), clock).build();
    (input, events, script)
}

#[test]
fn a_higher_priority_source_holds_the_universe_until_it_ends() {
    let (input, events, script) = rig();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    script.push(Received::new(1, SourceId(1), 2).priority(150), &[0x00, 10]);
    script.push(Received::new(1, SourceId(2), 2), &[0x00, 200]);
    input.step(Duration::ZERO);
    input.step(Duration::ZERO);
    assert_eq!(
        merged.snapshot().levels(),
        [10],
        "150 beats 100, however bright"
    );
    script.push(
        Received::new(1, SourceId(1), 2).priority(150).ended(),
        &[0x00, 10],
    );
    input.step(Duration::ZERO);
    assert_eq!(
        merged.snapshot().levels(),
        [200],
        "dropped at once, not after a timeout"
    );
    assert_eq!(merged.sources(), 1);
    let lost = std::iter::from_fn(|| events.try_next())
        .filter(|e| {
            matches!(
                e,
                Event::SourceLost {
                    source: SourceId(1),
                    ..
                }
            )
        })
        .count();
    assert_eq!(lost, 1);
}

#[test]
fn an_ended_source_never_merged_is_not_reported_lost() {
    let (input, events, script) = rig();
    let _merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    script.push(Received::new(1, SourceId(9), 2).ended(), &[0x00, 10]);
    input.step(Duration::ZERO);
    assert!(events.try_next().is_none());
}

#[test]
fn listening_joins_once_and_ignoring_leaves() {
    let (input, _events, script) = rig();
    let _a = input.listen(7, Merge::new(Mode::Htp)).unwrap();
    let _b = input.listen(7, Merge::new(Mode::Htp)).unwrap();
    assert!(input.ignore(7).unwrap());
    assert!(!input.ignore(7).unwrap());
    assert_eq!(*script.membership.lock().unwrap(), [(true, 7), (false, 7)]);
}
