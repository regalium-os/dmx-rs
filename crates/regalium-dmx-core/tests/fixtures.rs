// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! The phase gate: every record of every corpus under `fixtures/` decodes to
//! what the record says, and re-encodes to exactly its octets.
//!
//! A record is `octets => meaning`. The meaning is rendered from what the
//! codec decoded, so a corpus states the codec's behaviour in a form a
//! person can check against the standard, octet by octet.
//!
//! The corpora are compiled in rather than read: this is the pure core, and
//! its purity gate refuses file access even in a test. `just fixtures` fails
//! when a `.hex` file on disk is not named here, so a new corpus cannot sit
//! beside the walker unread.

use regalium_dmx_core::address::Address;
use regalium_dmx_core::error::{Error, ErrorKind};
use regalium_dmx_core::frame::Frame;
use std::fmt::Write as _;

/// Every corpus, by file name. `just fixtures` checks this list is complete.
const CORPORA: &[(&str, &str)] = &[
    ("null.hex", include_str!("../fixtures/frame/null.hex")),
    (
        "alternate.hex",
        include_str!("../fixtures/frame/alternate.hex"),
    ),
    (
        "malformed.hex",
        include_str!("../fixtures/frame/malformed.hex"),
    ),
];

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).expect("hex octet"))
        .collect()
}

fn octets(b: &[u8]) -> String {
    b.iter().fold(String::new(), |mut out, x| {
        let _ = write!(out, "{x:02x}");
        out
    })
}

/// What a decoded frame means, as the corpus spells it.
fn meaning(frame: &Frame<'_>) -> String {
    let code = frame.start_code();
    let head = format!("{code} {} slots", frame.slots().len());
    if !code.is_null() {
        return match frame.slots() {
            [] => format!("{head}; no data"),
            data => format!("{head}; data {}", octets(data)),
        };
    }
    // Read through `Frame::level`, by address, so the corpus checks the
    // one-based conversion and not only the raw slots.
    let lit: Vec<String> = (1..=512)
        .filter_map(Address::new)
        .filter_map(|a| Some((a, frame.level(a)?)))
        .filter(|(_, level)| *level != 0)
        .map(|(a, level)| format!("{a}={level}"))
        .collect();
    if lit.is_empty() {
        format!("{head}; dark")
    } else {
        format!("{head}; {}", lit.join(" "))
    }
}

/// What a decode error means, after checking it points at the octets it
/// names.
fn failure(input: &[u8], e: &Error<'_>) -> String {
    assert_eq!(
        input.get(e.offset..),
        Some(e.bytes),
        "an error's bytes are the input from its offset"
    );
    let kind = match e.kind {
        ErrorKind::Empty => "empty".to_owned(),
        ErrorKind::TooLong { slots } => format!("too-long {slots} slots"),
        other => format!("{other:?}"),
    };
    format!("error {kind} at {}", e.offset)
}

fn decode(input: &[u8]) -> String {
    match Frame::decode(input) {
        Ok(frame) => {
            let mut again = Vec::new();
            frame.encode(&mut again);
            assert_eq!(again, input, "re-encoding is not lossless");
            assert_eq!(frame.octets(), input.len());
            meaning(&frame)
        }
        Err(e) => failure(input, &e),
    }
}

#[test]
fn fixture_every_frame_record_decodes_to_its_meaning_and_back() {
    let mut records = 0;
    for (name, corpus) in CORPORA {
        assert!(!corpus.trim().is_empty(), "{name} is empty");
        let mut in_file = 0;
        for (n, line) in corpus.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (input, want) = line
                .split_once("=>")
                .unwrap_or_else(|| panic!("{name}:{}: no `=>`", n + 1));
            let got = decode(&hex(input));
            assert_eq!(got, want.trim(), "{name}:{}", n + 1);
            in_file += 1;
        }
        // A corpus of comments alone asserts nothing; that is a failure.
        assert!(in_file > 0, "{name} has no records");
        records += in_file;
    }
    assert!(records >= CORPORA.len(), "walked {records} records");
}
