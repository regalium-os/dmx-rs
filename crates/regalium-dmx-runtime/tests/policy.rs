// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The output policy, as the wire sees it: blackout, hold and master act at
//! the output, go out at once, and never change the levels behind them.

mod common;

use common::{ms, rig};
use regalium_dmx_core::address::Address;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_runtime::{Output, Policy};
use std::sync::Arc;

/// An output sending `[200, 100]` on change, keep-alive 1 s, with a policy.
fn lit() -> (
    Output<common::Recording, Arc<regalium_dmx_runtime::ManualClock>>,
    Arc<regalium_dmx_runtime::ManualClock>,
    common::Wire,
    regalium_dmx_runtime::Levels,
) {
    let (transport, clock, wire) = rig(Refresh::on_change(ms(25), ms(1000), 0));
    let (output, _events) = Output::builder(transport, clock.clone()).build();
    let levels = output.patch(1);
    levels.set_len(2).unwrap();
    levels.write(Address::FIRST, &[200, 100]).unwrap();
    output.step();
    (output, clock, wire, levels)
}

fn last(wire: &common::Wire) -> Vec<u8> {
    wire.sent().last().unwrap().octets.clone()
}

#[test]
fn blackout_goes_out_at_the_next_interval_and_ends_on_the_levels_as_they_now_are() {
    let (output, clock, wire, levels) = lit();
    assert_eq!(last(&wire), [0x00, 200, 100]);
    assert!(output.policy().set_blackout(true));
    clock.set(ms(25));
    output.step();
    assert_eq!(last(&wire), [0x00, 0, 0], "zeros, at the universe's length");
    levels.set(Address::FIRST, 50);
    clock.set(ms(50));
    output.step();
    assert_eq!(
        last(&wire),
        [0x00, 0, 0],
        "a write during blackout stays dark"
    );
    assert_eq!(levels.snapshot().level(Address::FIRST), 50, "but is kept");
    output.policy().set_blackout(false);
    clock.set(ms(75));
    output.step();
    assert_eq!(last(&wire), [0x00, 50, 100]);
}

#[test]
fn hold_keeps_sending_the_last_frame_and_releases_the_writes_it_held() {
    let (output, clock, wire, levels) = lit();
    output.policy().set_hold(true);
    levels.set(Address::FIRST, 10);
    for t in [25, 50, 1100] {
        clock.set(ms(t));
        output.step();
    }
    assert!(
        wire.sent().iter().all(|s| s.octets == [0x00, 200, 100]),
        "{:?}",
        wire.sent()
    );
    assert!(
        wire.sent().iter().any(|s| s.at == ms(1100)),
        "kept alive while held"
    );
    output.policy().set_hold(false);
    clock.set(ms(1125));
    output.step();
    assert_eq!(last(&wire), [0x00, 10, 100]);
}

#[test]
fn blackout_overrides_hold_and_hold_resumes_the_frame_from_before_it() {
    let (output, clock, wire, _levels) = lit();
    output.policy().set_hold(true);
    output.policy().set_blackout(true);
    clock.set(ms(25));
    output.step();
    assert_eq!(last(&wire), [0x00, 0, 0]);
    output.policy().set_blackout(false);
    clock.set(ms(50));
    output.step();
    assert_eq!(
        last(&wire),
        [0x00, 200, 100],
        "the held frame, not the zeros"
    );
}

#[test]
fn master_scales_levels_on_the_way_out_and_one_policy_governs_every_output() {
    let policy = Arc::new(Policy::new());
    let (a_transport, clock, a_wire) = rig(Refresh::continuous(ms(25)));
    let (a, _) = Output::builder(a_transport, clock.clone())
        .policy(Arc::clone(&policy))
        .build();
    let a_levels = a.patch(1);
    a_levels.set_len(3).unwrap();
    a_levels.write(Address::FIRST, &[255, 100, 0]).unwrap();
    let b_wire = common::Wire::default();
    let b_transport = common::Recording {
        clock: clock.clone(),
        wire: b_wire.clone(),
        refresh: Refresh::continuous(ms(25)),
    };
    let (b, _) = Output::builder(b_transport, clock.clone())
        .policy(Arc::clone(&policy))
        .build();
    let b_levels = b.patch(2);
    b_levels.set_len(1).unwrap();
    b_levels.write(Address::FIRST, &[255]).unwrap();
    policy.set_master(128);
    a.step();
    b.step();
    assert_eq!(last(&a_wire), [0x00, 128, 50, 0]);
    assert_eq!(last(&b_wire), [0x00, 128]);
    assert_eq!(
        a_levels.snapshot().level(Address::FIRST),
        255,
        "the levels themselves are untouched"
    );
}

#[test]
fn an_unchanged_setting_is_no_change() {
    let policy = Policy::new();
    assert!(!policy.set_blackout(false));
    assert!(!policy.set_master(255));
    assert_eq!(policy.rules().generation, 0);
    assert!(policy.set_master(0));
    assert_eq!(policy.rules().generation, 1);
}
