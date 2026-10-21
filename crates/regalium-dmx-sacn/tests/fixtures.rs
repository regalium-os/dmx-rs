// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! The phase gate: every record of every E1.31 corpus decodes to what the
//! record says, and re-encodes to exactly its octets.
//!
//! A record is `octets => meaning`, as in the core's corpus. `just
//! fixtures` fails when a `.hex` file here is not named in `CORPORA`.

use regalium_dmx_sacn::codec::{Error, ErrorKind, Packet, text};
use std::fmt::Write as _;

const CORPORA: &[(&str, &str)] = &[
    ("data.hex", include_str!("../fixtures/data.hex")),
    ("sync.hex", include_str!("../fixtures/sync.hex")),
    ("discovery.hex", include_str!("../fixtures/discovery.hex")),
    ("other.hex", include_str!("../fixtures/other.hex")),
    ("malformed.hex", include_str!("../fixtures/malformed.hex")),
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

fn quoted(field: &[u8]) -> String {
    format!("{:?}", String::from_utf8_lossy(text(field)))
}

fn levels(slots: &[u8]) -> String {
    let lit: Vec<String> = slots
        .iter()
        .enumerate()
        .filter(|(_, l)| **l != 0)
        .map(|(i, l)| format!("{}={l}", i + 1))
        .collect();
    if lit.is_empty() {
        "dark".to_owned()
    } else {
        lit.join(" ")
    }
}

fn meaning(packet: &Packet<'_>) -> String {
    match packet {
        Packet::Data(d) => {
            let body = if d.start_code.0 == 0 {
                levels(d.slots)
            } else {
                format!("start {:#04x} data {}", d.start_code.0, octets(d.slots))
            };
            format!(
                "data {} {} priority {} sync {} seq {} options {:#04x} universe {} {} slots; {body}",
                d.cid,
                quoted(d.source_name),
                d.priority,
                d.sync_address,
                d.sequence,
                d.options,
                d.universe,
                d.slots.len(),
            )
        }
        Packet::Sync(s) => format!(
            "sync {} seq {} address {} reserved {}",
            s.cid,
            s.sequence,
            s.sync_address,
            octets(&s.reserved)
        ),
        Packet::Discovery(d) => {
            let universes: Vec<String> = d.universes().map(|u| u.to_string()).collect();
            format!(
                "discovery {} {} page {} of {} universes [{}]",
                d.cid,
                quoted(d.source_name),
                d.page,
                d.last_page,
                universes.join(" ")
            )
        }
        Packet::Other {
            root,
            framing,
            datagram,
        } => {
            format!(
                "other root {root:#010x} framing {framing:#010x} {} octets",
                datagram.len()
            )
        }
        other => format!("{other:?}"),
    }
}

fn failure(input: &[u8], e: &Error<'_>) -> String {
    assert_eq!(
        input.get(e.offset..),
        Some(e.bytes),
        "an error's bytes are the input from its offset"
    );
    let kind = match e.kind {
        ErrorKind::NotAcn => "not-acn".to_owned(),
        ErrorKind::Truncated { need } => format!("truncated need {need}"),
        ErrorKind::Flags { flags } => format!("flags {flags:#x}"),
        ErrorKind::Length { said, actual } => format!("length {said} not {actual}"),
        ErrorKind::Dmp => "dmp".to_owned(),
        ErrorKind::TooLong { slots } => format!("too-long {slots} slots"),
        other => format!("{other:?}"),
    };
    format!("error {kind} at {}", e.offset)
}

fn decode(input: &[u8]) -> String {
    match Packet::decode(input) {
        Ok(packet) => {
            let mut again = Vec::new();
            packet.encode(&mut again);
            assert_eq!(again, input, "re-encoding is not lossless");
            meaning(&packet)
        }
        Err(e) => failure(input, &e),
    }
}

#[test]
fn fixture_every_sacn_record_decodes_to_its_meaning_and_back() {
    let mut records = 0;
    for (name, corpus) in CORPORA {
        let mut in_file = 0;
        for (n, line) in corpus.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (input, want) = line
                .split_once("=>")
                .unwrap_or_else(|| panic!("{name}:{}: no `=>`", n + 1));
            assert_eq!(decode(&hex(input)), want.trim(), "{name}:{}", n + 1);
            in_file += 1;
        }
        assert!(in_file > 0, "{name} has no records");
        records += in_file;
    }
    assert!(records >= CORPORA.len(), "walked {records} records");
}
