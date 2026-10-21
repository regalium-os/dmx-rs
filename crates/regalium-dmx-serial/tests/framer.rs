// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Messages out of a stream: split anywhere, led by noise, or cut short.

use regalium_dmx_serial::enttec::{CAPACITY, Framer, Label, Message};

fn message(label: u8, data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    Message {
        label: Label(label),
        data,
    }
    .encode(&mut out);
    out
}

/// Feeds `chunks` one after another; returns each message's label and data.
fn frame(chunks: &[&[u8]]) -> (Vec<(u8, Vec<u8>)>, u64) {
    let mut framer = Framer::new();
    let mut got = Vec::new();
    for chunk in chunks {
        framer.feed(chunk, |m| got.push((m.label.0, m.data.to_vec())));
    }
    (got, framer.skipped())
}

#[test]
fn a_message_split_at_every_octet_still_arrives_whole() {
    let bytes = message(5, &[0, 0, 7]);
    let chunks: Vec<&[u8]> = bytes.chunks(1).collect();
    assert_eq!(frame(&chunks), (vec![(5, vec![0, 0, 7])], 0));
}

#[test]
fn two_messages_in_one_read_both_arrive_in_order() {
    let mut bytes = message(10, &[1, 2, 3, 4]);
    bytes.extend(message(3, &[0, 1, 9, 1, 40]));
    let (got, _) = frame(&[&bytes]);
    assert_eq!(got.iter().map(|m| m.0).collect::<Vec<_>>(), [10, 3]);
}

#[test]
fn noise_before_a_message_is_skipped_and_counted() {
    let mut bytes = vec![0x00, 0xFF, 0x12];
    bytes.extend(message(10, &[]));
    assert_eq!(frame(&[&bytes]), (vec![(10, vec![])], 3));
}

#[test]
fn a_message_missing_its_end_costs_only_itself() {
    let mut cut = message(5, &[0, 0, 1]);
    *cut.last_mut().unwrap() = 0x00;
    cut.extend(message(10, &[]));
    let (got, skipped) = frame(&[&cut]);
    assert_eq!(got, [(10, vec![])]);
    assert_eq!(
        skipped, 8,
        "the broken message, octet by octet, to the next start"
    );
}

#[test]
fn a_length_over_600_is_noise_not_a_wait_for_more() {
    let mut bytes = vec![0x7E, 6, 0x59, 0x02];
    bytes.extend(message(10, &[]));
    assert_eq!(frame(&[&bytes]).0, [(10, vec![])]);
}

#[test]
fn the_largest_message_fits_and_the_framer_stays_bounded() {
    let big = message(6, &[1; 600]);
    assert_eq!(big.len(), CAPACITY);
    assert_eq!(frame(&[&big]).0.len(), 1);
}
