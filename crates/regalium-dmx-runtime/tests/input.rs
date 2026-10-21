// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The input loop stepped by hand over a scripted transport: what merges,
//! what is dropped, and when a silent source leaves.

mod common;

use common::script::Script;
use regalium_dmx_core::address::Address;
use regalium_dmx_core::merge::{Merge, Mode, SourceId};
use regalium_dmx_runtime::{Event, Events, Input, ManualClock};
use std::sync::Arc;
use std::time::Duration;

type Rig = (
    Input<Arc<Script>, Arc<ManualClock>>,
    Events,
    Arc<Script>,
    Arc<ManualClock>,
);

fn rig() -> Rig {
    let script = Arc::new(Script::default());
    let clock = Arc::new(ManualClock::new());
    let (input, events) = Input::builder(Arc::clone(&script), Arc::clone(&clock)).build();
    (input, events, script, clock)
}

fn drain(events: &Events) -> Vec<Event> {
    std::iter::from_fn(|| events.try_next()).collect()
}

fn level(n: u16) -> Address {
    Address::new(n).unwrap()
}

#[test]
fn two_sources_merge_highest_first_and_each_is_found_once() {
    let (input, events, script, _clock) = rig();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    script.frame(1, 10, &[0x00, 255, 0, 40]);
    script.frame(1, 20, &[0x00, 100, 200]);
    script.frame(1, 10, &[0x00, 255, 0, 40]);
    for _ in 0..3 {
        assert!(input.step(Duration::ZERO));
    }
    let universe = merged.snapshot();
    assert_eq!(universe.levels(), [255, 200, 40]);
    assert_eq!(merged.sources(), 2);
    let found: Vec<_> = drain(&events);
    assert!(
        matches!(
            found.as_slice(),
            [
                Event::SourceFound {
                    universe: 1,
                    source: SourceId(10),
                    ..
                },
                Event::SourceFound {
                    universe: 1,
                    source: SourceId(20),
                    ..
                },
            ]
        ),
        "{found:?}"
    );
}

#[test]
fn a_universe_nobody_listens_to_and_an_rdm_frame_are_dropped() {
    let (input, events, script, _clock) = rig();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    script.frame(2, 10, &[0x00, 255]);
    script.frame(1, 10, &[0xCC, 1, 2]);
    input.step(Duration::ZERO);
    input.step(Duration::ZERO);
    assert_eq!(merged.sources(), 0);
    assert_eq!(drain(&events), []);
}

#[test]
fn a_silent_source_leaves_after_the_timeout_and_the_last_takes_the_light() {
    let (input, events, script, clock) = rig();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    script.frame(1, 10, &[0x00, 50]);
    input.step(Duration::ZERO);
    clock.set(Duration::from_secs(5));
    script.frame(1, 20, &[0x00, 90]);
    input.step(Duration::ZERO);
    clock.set(Duration::from_secs(10));
    input.step(Duration::ZERO);
    assert_eq!(merged.sources(), 2, "exactly ten seconds is not yet silent");
    clock.set(Duration::from_millis(10_001));
    input.step(Duration::ZERO);
    assert_eq!(merged.sources(), 1);
    assert_eq!(merged.snapshot().level(level(1)), 90);
    clock.set(Duration::from_millis(15_001));
    input.step(Duration::ZERO);
    assert_eq!(merged.sources(), 0);
    assert_eq!(merged.snapshot().level(level(1)), 0, "no source: dark");
    let lost: Vec<_> = drain(&events)
        .into_iter()
        .filter_map(|e| match e {
            Event::SourceLost { source, .. } => Some(source.0),
            _ => None,
        })
        .collect();
    assert_eq!(lost, [10, 20]);
}

#[test]
fn a_failing_receive_is_reported_once_and_its_recovery_once() {
    let (input, events, script, _clock) = rig();
    for _ in 0..5 {
        script.fail();
    }
    let worked: Vec<bool> = (0..6).map(|_| input.step(Duration::ZERO)).collect();
    assert_eq!(worked, [false, false, false, false, false, true]);
    let all = drain(&events);
    assert!(
        matches!(all.as_slice(), [
        Event::ReceiveFailed { error, .. },
        Event::ReceiveRecovered { .. },
    ] if error == "network down"),
        "{all:?}"
    );
}

#[test]
fn listening_twice_keeps_the_first_merge() {
    let (input, _events, script, _clock) = rig();
    let a = input
        .listen(1, Merge::new(Mode::Htp).max_sources(1))
        .unwrap();
    let b = input.listen(1, Merge::new(Mode::Ltp)).unwrap();
    script.frame(1, 10, &[0x00, 1]);
    script.frame(1, 20, &[0x00, 2]);
    input.step(Duration::ZERO);
    input.step(Duration::ZERO);
    assert_eq!(
        b.sources(),
        1,
        "the second source is past the first merge's limit"
    );
    assert_eq!(a.snapshot().level(level(1)), 1);
    assert!(input.ignore(1).unwrap());
    assert!(!input.ignore(1).unwrap());
}
