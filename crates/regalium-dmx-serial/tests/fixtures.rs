// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! The phase gate: every record of every ENTTEC API corpus decodes to what
//! the record says, and re-encodes to exactly its octets.
//!
//! A record is `octets => meaning`, as in the core's corpus. `just
//! fixtures` fails when a `.hex` file here is not named in `CORPORA`.

use regalium_dmx_serial::enttec::{Error, ErrorKind, Label, Message, Params, serial};
use std::fmt::Write as _;

const CORPORA: &[(&str, &str)] = &[
    ("requests.hex", include_str!("../fixtures/requests.hex")),
    ("replies.hex", include_str!("../fixtures/replies.hex")),
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

fn meaning(m: &Message<'_>) -> String {
    let what = match (m.label, m.data) {
        (Label::SEND_DMX, [code, slots @ ..]) => {
            format!("start {code:#04x} {} slots; {}", slots.len(), levels(slots))
        }
        (Label::RECEIVED_DMX, [status, code, slots @ ..]) => format!(
            "status {status:#04x} start {code:#04x} {} slots; {}",
            slots.len(),
            levels(slots)
        ),
        (Label::GET_SERIAL, data @ [_, _, _, _]) => match serial(data) {
            Some(n) => format!("serial {n}"),
            None => format!("serial not bcd {}", octets(data)),
        },
        (Label::GET_PARAMETERS, data) if data.len() >= 5 => {
            let p = Params::decode(data).unwrap();
            format!(
                "firmware {:#06x} break {} ticks mab {} ticks rate {}",
                p.firmware, p.break_ticks, p.mab_ticks, p.rate
            )
        }
        (_, []) => "no data".to_owned(),
        (_, data) => format!("data {}", octets(data)),
    };
    format!("{} {} octets; {what}", m.label, m.data.len())
}

fn failure(input: &[u8], e: &Error<'_>) -> String {
    assert_eq!(
        input.get(e.offset..),
        Some(e.bytes),
        "an error's bytes are the input from its offset"
    );
    let kind = match e.kind {
        ErrorKind::Start => "start".to_owned(),
        ErrorKind::End => "end".to_owned(),
        ErrorKind::TooLong { len } => format!("too-long {len}"),
        ErrorKind::Truncated { need } => format!("truncated need {need}"),
        ErrorKind::Trailing => "trailing".to_owned(),
        other => format!("{other:?}"),
    };
    format!("error {kind} at {}", e.offset)
}

fn decode(input: &[u8]) -> String {
    match Message::decode(input) {
        Ok(m) => {
            let mut again = Vec::new();
            m.encode(&mut again);
            assert_eq!(again, input, "re-encoding is not lossless");
            meaning(&m)
        }
        Err(e) => failure(input, &e),
    }
}

#[test]
fn fixture_every_enttec_record_decodes_to_its_meaning_and_back() {
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
