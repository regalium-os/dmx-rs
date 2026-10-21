// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Priority chooses who merges; the mode says how.

use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::{DEFAULT_PRIORITY, Merge, Mode, SourceId};
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::universe::Universe;

fn levels(slots: &[u8]) -> Frame<'_> {
    Frame::new(StartCode::NULL, slots).unwrap()
}

fn render(merge: &Merge) -> Vec<u8> {
    let mut universe = Universe::new();
    merge.render(&mut universe);
    universe.levels().to_vec()
}

#[test]
fn only_the_highest_priority_merges_however_bright_the_rest() {
    let mut merge = Merge::new(Mode::Htp);
    merge
        .update_at(SourceId(1), 150, &levels(&[10, 0]))
        .unwrap();
    merge
        .update_at(SourceId(2), 100, &levels(&[255, 255, 255]))
        .unwrap();
    assert_eq!(
        render(&merge),
        [10, 0],
        "the 100 source is held, its length too"
    );
    assert_eq!(
        (merge.priority(), merge.merging(), merge.sources()),
        (Some(150), 1, 2)
    );
}

#[test]
fn sources_at_equal_priority_merge_by_the_mode() {
    let mut merge = Merge::new(Mode::Htp);
    merge
        .update_at(SourceId(1), 150, &levels(&[10, 90]))
        .unwrap();
    merge
        .update_at(SourceId(2), 150, &levels(&[80, 20]))
        .unwrap();
    merge
        .update_at(SourceId(3), 50, &levels(&[255, 255]))
        .unwrap();
    assert_eq!(render(&merge), [80, 90]);
    assert_eq!(merge.merging(), 2);
}

#[test]
fn when_the_highest_leaves_the_held_source_takes_over() {
    let mut merge = Merge::new(Mode::Htp);
    merge.update_at(SourceId(1), 200, &levels(&[10])).unwrap();
    merge.update_at(SourceId(2), 100, &levels(&[60])).unwrap();
    assert!(merge.remove(SourceId(1)));
    assert_eq!(render(&merge), [60]);
}

#[test]
fn a_source_may_change_its_priority_between_frames() {
    let mut merge = Merge::new(Mode::Htp);
    merge.update(SourceId(1), &levels(&[10])).unwrap();
    merge.update(SourceId(2), &levels(&[60])).unwrap();
    assert_eq!(merge.priority(), Some(DEFAULT_PRIORITY));
    merge.update_at(SourceId(1), 101, &levels(&[10])).unwrap();
    assert_eq!(render(&merge), [10]);
}
