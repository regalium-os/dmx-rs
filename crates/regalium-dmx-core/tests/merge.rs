// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! HTP and LTP, as an operator would describe them: the brighter console
//! wins an intensity, the console that last moved a fader wins that slot.

use regalium_dmx_core::error::Rejected;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::{Merge, Mode, SourceId};
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::universe::Universe;

const MAIN: SourceId = SourceId(1);
const BACKUP: SourceId = SourceId(2);

/// Feeds `levels` from `source` as a null packet.
fn feed(merge: &mut Merge, source: SourceId, levels: &[u8]) {
    merge
        .update(source, &Frame::new(StartCode::NULL, levels).unwrap())
        .unwrap();
}

fn render(merge: &Merge) -> Vec<u8> {
    let mut u = Universe::new();
    merge.render(&mut u);
    u.levels().to_vec()
}

#[test]
fn htp_takes_the_highest_level_of_each_slot() {
    let mut m = Merge::new(Mode::Htp);
    feed(&mut m, MAIN, &[100, 0, 50]);
    feed(&mut m, BACKUP, &[20, 200, 50]);
    assert_eq!(render(&m), [100, 200, 50]);
}

#[test]
fn htp_counts_a_shorter_source_as_zero_past_its_end() {
    let mut m = Merge::new(Mode::Htp);
    feed(&mut m, MAIN, &[10, 10, 10, 10]);
    feed(&mut m, BACKUP, &[90]);
    assert_eq!(
        render(&m),
        [90, 10, 10, 10],
        "the longest source sets the length"
    );
}

#[test]
fn ltp_gives_each_slot_to_the_source_that_last_changed_it() {
    let mut m = Merge::new(Mode::Ltp);
    feed(&mut m, MAIN, &[100, 100, 100]);
    // The backup's first packet changes every slot it carries: it takes all.
    feed(&mut m, BACKUP, &[0, 0, 0]);
    assert_eq!(render(&m), [0, 0, 0]);
    // Main moves slot 2 only; slots 1 and 3 stay the backup's.
    feed(&mut m, MAIN, &[100, 255, 100]);
    assert_eq!(render(&m), [0, 255, 0]);
}

#[test]
fn ltp_is_not_stolen_by_a_source_resending_the_same_levels() {
    let mut m = Merge::new(Mode::Ltp);
    feed(&mut m, MAIN, &[100]);
    feed(&mut m, BACKUP, &[40]);
    // Main resends 100 at the refresh rate; nothing changed, so it does not
    // take the slot back from the backup that moved it last.
    feed(&mut m, MAIN, &[100]);
    feed(&mut m, MAIN, &[100]);
    assert_eq!(render(&m), [40]);
}

#[test]
fn removing_a_source_gives_its_slots_back() {
    let mut m = Merge::new(Mode::Htp);
    feed(&mut m, MAIN, &[10]);
    feed(&mut m, BACKUP, &[90]);
    assert!(m.remove(BACKUP));
    assert!(!m.remove(BACKUP), "already gone");
    assert_eq!(m.sources(), 1);
    assert_eq!(render(&m), [10]);
}

#[test]
fn a_merge_with_no_sources_renders_dark() {
    let m = Merge::new(Mode::Htp);
    let mut u = Universe::new();
    u.set_len(4).unwrap();
    u.write(regalium_dmx_core::address::Address::FIRST, &[1, 2, 3, 4])
        .unwrap();
    assert!(m.render(&mut u));
    assert_eq!(u.levels(), [0, 0, 0, 0]);
}

#[test]
fn a_source_past_the_limit_is_refused_and_changes_nothing() {
    let mut m = Merge::new(Mode::Htp).max_sources(1);
    feed(&mut m, MAIN, &[10]);
    let refused = m.update(BACKUP, &Frame::new(StartCode::NULL, &[99]).unwrap());
    assert_eq!(refused, Err(Rejected::TooManySources { limit: 1 }));
    assert_eq!(render(&m), [10]);
}

#[test]
fn an_rdm_packet_is_never_merged_as_levels() {
    let mut m = Merge::new(Mode::Htp);
    let rdm = Frame::new(StartCode::RDM, &[0x01, 0x18]).unwrap();
    assert_eq!(
        m.update(MAIN, &rdm),
        Err(Rejected::NotLevels(StartCode::RDM))
    );
    assert_eq!(m.sources(), 0, "not even added as a source");
}

#[test]
fn rendering_the_same_merge_twice_is_no_change() {
    let mut m = Merge::new(Mode::Htp);
    feed(&mut m, MAIN, &[5, 6]);
    let mut u = Universe::new();
    assert!(m.render(&mut u));
    let generation = u.generation();
    assert!(!m.render(&mut u));
    assert_eq!(u.generation(), generation);
}
