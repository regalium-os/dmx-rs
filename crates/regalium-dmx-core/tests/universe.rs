// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! A universe's levels, length and generation: the generation moves exactly
//! when something a receiver would see changes, and a refused change changes
//! nothing.

use regalium_dmx_core::address::Address;
use regalium_dmx_core::error::Rejected;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::universe::Universe;

fn at(n: u16) -> Address {
    Address::new(n).unwrap()
}

#[test]
fn a_new_universe_is_dark_full_length_and_generation_zero() {
    let u = Universe::new();
    assert_eq!(u.len(), 512);
    assert!(u.levels().iter().all(|&l| l == 0));
    assert_eq!(u.generation(), 0);
}

#[test]
fn setting_a_level_bumps_the_generation_only_when_it_changes() {
    let mut u = Universe::new();
    assert!(u.set(at(1), 255));
    assert_eq!(u.generation(), 1);
    assert!(!u.set(at(1), 255), "the same level again is no change");
    assert_eq!(u.generation(), 1);
    assert_eq!(u.level(at(1)), 255);
}

#[test]
fn a_write_past_slot_512_changes_nothing() {
    let mut u = Universe::new();
    let refused = u.write(at(510), &[1, 2, 3, 4]);
    assert_eq!(refused, Err(Rejected::OutOfRange { start: 510, len: 4 }));
    assert_eq!(u.generation(), 0);
    assert_eq!(u.level(at(510)), 0, "all or nothing");
    assert_eq!(u.write(at(510), &[1, 2, 3]), Ok(true));
    assert_eq!(u.level(at(512)), 3);
}

#[test]
fn shortening_keeps_the_levels_past_the_length() {
    let mut u = Universe::new();
    u.set(at(100), 50);
    assert_eq!(u.set_len(24), Ok(true));
    assert_eq!(u.levels().len(), 24);
    assert_eq!(u.frame().slots().len(), 24);
    assert_eq!(u.set_len(512), Ok(true));
    assert_eq!(u.level(at(100)), 50, "lengthening again loses nothing");
    assert!(u.set_len(513).is_err());
}

#[test]
fn applying_a_frame_takes_its_levels_and_its_length() {
    let mut u = Universe::new();
    let frame = Frame::decode(&[0x00, 9, 8, 7]).unwrap();
    assert_eq!(u.apply(&frame), Ok(true));
    assert_eq!(u.levels(), [9, 8, 7]);
    assert_eq!(
        u.apply(&frame),
        Ok(false),
        "the same frame twice is no change"
    );
}

#[test]
fn an_rdm_packet_is_never_applied_as_levels() {
    let mut u = Universe::new();
    let rdm = Frame::decode(&[0xCC, 0x01, 0x18]).unwrap();
    assert_eq!(u.apply(&rdm), Err(Rejected::NotLevels(StartCode::RDM)));
    assert_eq!(u.generation(), 0);
    assert!(u.levels().iter().all(|&l| l == 0));
}

#[test]
fn the_universe_encodes_as_a_null_packet_of_its_length() {
    let mut u = Universe::new();
    u.set_len(3).unwrap();
    u.write(Address::FIRST, &[1, 2, 3]).unwrap();
    let mut out = Vec::new();
    u.frame().encode(&mut out);
    assert_eq!(out, [0x00, 1, 2, 3]);
}

#[test]
fn blackout_darkens_every_level_and_keeps_the_length() {
    let mut u = Universe::new();
    u.set_len(24).unwrap();
    u.set(at(300), 9);
    assert!(u.blackout());
    assert_eq!(u.len(), 24);
    assert_eq!(u.level(at(300)), 0, "beyond the length too");
    assert!(!u.blackout(), "already dark is no change");
}
