// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The output loop stepped by hand on a manual clock: what goes on the wire,
//! when, and what the caller is told.

mod common;

use common::{ms, rig};
use regalium_dmx_core::address::Address;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_runtime::{Event, Output};

#[test]
fn building_and_patching_sends_nothing_until_stepped() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, _events) = Output::builder(transport, clock).build();
    let _levels = output.patch(1);
    assert!(wire.sent().is_empty(), "construction starts nothing");
}

#[test]
fn a_patched_universe_goes_out_at_once_then_every_interval() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, _events) = Output::builder(transport, clock.clone()).build();
    let _levels = output.patch(1);
    assert_eq!(output.step(), Some(ms(25)));
    clock.set(ms(10));
    assert_eq!(output.step(), Some(ms(25)), "nothing due at 10 ms");
    clock.set(ms(25));
    assert_eq!(output.step(), Some(ms(50)));
    let times: Vec<_> = wire.sent().iter().map(|s| s.at).collect();
    assert_eq!(times, [ms(0), ms(25)]);
}

#[test]
fn a_change_goes_out_at_the_next_interval_not_the_keep_alive() {
    let (transport, clock, wire) = rig(Refresh::on_change(ms(25), ms(1000), 0));
    let (output, _events) = Output::builder(transport, clock.clone()).build();
    let levels = output.patch(7);
    levels.set_len(3).unwrap();
    output.step();
    clock.set(ms(40));
    assert_eq!(output.step(), Some(ms(1000)), "unchanged: keep-alive");
    levels.write(Address::FIRST, &[255, 128, 1]).unwrap();
    assert_eq!(
        output.step(),
        Some(ms(1040)),
        "sent at 40; with nothing more to say, the keep-alive is next"
    );
    // Another change 10 ms later must wait out the interval, not go at once.
    clock.set(ms(50));
    levels.set(Address::FIRST, 0);
    assert_eq!(output.step(), Some(ms(65)), "no sooner than 40 + 25");
    clock.set(ms(65));
    output.step();
    let sent = wire.sent();
    assert_eq!(sent.len(), 3);
    assert_eq!(sent[1].at, ms(40));
    assert_eq!(sent[1].universe, 7);
    assert_eq!(sent[1].octets, [0x00, 255, 128, 1]);
    assert_eq!(sent[2].at, ms(65));
    assert_eq!(sent[2].octets, [0x00, 0, 128, 1]);
}

#[test]
fn each_universe_keeps_its_own_schedule() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, _events) = Output::builder(transport, clock.clone()).build();
    let _fast = output.patch(1);
    let _slow = output.patch_with(2, Refresh::continuous(ms(100)));
    for t in (0..=100).step_by(25) {
        clock.set(ms(t));
        output.step();
    }
    let count = |u| wire.sent().iter().filter(|s| s.universe == u).count();
    assert_eq!(count(1), 5, "0, 25, 50, 75, 100");
    assert_eq!(count(2), 2, "0, 100");
}

#[test]
fn a_failing_transport_is_reported_once_and_its_recovery_once() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, events) = Output::builder(transport, clock.clone()).build();
    let _levels = output.patch(1);
    wire.fail(true);
    for t in (0..250).step_by(25) {
        clock.set(ms(t));
        output.step();
    }
    wire.fail(false);
    clock.set(ms(250));
    output.step();
    let all: Vec<Event> = std::iter::from_fn(|| events.try_next()).collect();
    assert_eq!(all.len(), 2, "{all:?}");
    assert!(
        matches!(&all[0], Event::SendFailed { universe: 1, error, .. } if error == "widget unplugged")
    );
    assert!(matches!(all[1], Event::SendRecovered { universe: 1, .. }));
    assert_eq!(wire.sent().len(), 1, "only the send after recovery");
}

#[test]
fn unpatching_stops_the_universe_and_an_empty_output_has_no_deadline() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, _events) = Output::builder(transport, clock).build();
    let _levels = output.patch(1);
    output.step();
    assert!(output.unpatch(1));
    assert!(!output.unpatch(1));
    assert_eq!(output.step(), None);
    assert_eq!(wire.sent().len(), 1);
}

#[test]
fn patching_twice_returns_the_same_universe() {
    let (transport, clock, _wire) = rig(Refresh::default());
    let (output, _events) = Output::builder(transport, clock).build();
    let a = output.patch(3);
    let b = output.patch(3);
    a.set(Address::FIRST, 99);
    assert_eq!(b.snapshot().level(Address::FIRST), 99);
}

#[test]
fn a_zero_interval_is_raised_rather_than_spinning() {
    let (transport, clock, _wire) = rig(Refresh::continuous(ms(0)));
    let (output, _events) = Output::builder(transport, clock).build();
    let _levels = output.patch(1);
    assert_eq!(output.step(), Some(regalium_dmx_runtime::MIN_INTERVAL));
}
