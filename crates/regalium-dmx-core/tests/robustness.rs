// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Every length a packet could arrive at, every start code: decoding never
//! panics, succeeds exactly up to 513 octets, and is always lossless. The
//! fuzz target (`just fuzz`) explores further; this runs on every commit.

use regalium_dmx_core::error::ErrorKind;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::{Merge, Mode, SourceId};
use regalium_dmx_core::universe::Universe;

/// A deterministic, uneven byte pattern, so neighbouring slots differ.
fn pattern(len: usize, seed: u8) -> Vec<u8> {
    (0..len)
        .map(|i| {
            u8::try_from(i % 256)
                .unwrap()
                .wrapping_mul(31)
                .wrapping_add(seed)
        })
        .collect()
}

#[test]
fn every_length_up_to_513_octets_decodes_losslessly() {
    let mut checked = 0;
    for seed in [0x00, 0xCC, 0x17, 0xFF] {
        for len in 1..=513 {
            let input = pattern(len, seed);
            let frame = Frame::decode(&input).unwrap();
            let mut again = Vec::new();
            frame.encode(&mut again);
            assert_eq!(again, input, "length {len}, seed {seed:#04x}");
            checked += 1;
        }
    }
    assert_eq!(checked, 4 * 513);
}

#[test]
fn every_length_past_513_octets_is_too_long_and_says_so() {
    for len in 514..=1500 {
        let input = pattern(len, 0);
        let e = Frame::decode(&input).unwrap_err();
        assert_eq!(e.kind, ErrorKind::TooLong { slots: len - 1 });
        assert_eq!(e.offset, 513);
        assert_eq!(e.bytes.len(), len - 513);
    }
}

#[test]
fn a_merge_fed_every_length_never_panics_and_never_exceeds_512() {
    for mode in [Mode::Htp, Mode::Ltp] {
        let mut m = Merge::new(mode);
        let mut u = Universe::new();
        for len in 0..=512 {
            let input = pattern(len + 1, 0);
            let frame = Frame::decode(&input).unwrap();
            let source = SourceId(u64::try_from(len % 3).unwrap());
            m.update(source, &frame).unwrap();
            m.render(&mut u);
            assert!(u.levels().len() <= 512);
        }
        assert_eq!(m.sources(), 3);
    }
}
