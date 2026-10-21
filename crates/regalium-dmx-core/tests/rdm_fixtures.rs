// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! The RDM phase gate: every message record decodes to its meaning and
//! re-encodes to exactly its octets; every discovery-response record
//! decodes to its UID, or to the collision it is.
//!
//! Compiled in, as the frame corpus is: this is the pure core.

use regalium_dmx_core::rdm::{
    DeviceInfo, Error, ErrorKind, Message, NackReason, Pid, ResponseType, disc,
};
use std::fmt::Write as _;

const MESSAGES: &[(&str, &str)] = &[
    ("requests.hex", include_str!("../fixtures/rdm/requests.hex")),
    (
        "responses.hex",
        include_str!("../fixtures/rdm/responses.hex"),
    ),
    (
        "malformed.hex",
        include_str!("../fixtures/rdm/malformed.hex"),
    ),
];

const DISCOVERY: &[(&str, &str)] = &[(
    "discovery.hex",
    include_str!("../fixtures/rdm/discovery.hex"),
)];

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

fn data(m: &Message<'_>) -> String {
    let response = m.command_class.0 & 1 == 1;
    match (m.pid, ResponseType(m.port_or_response), m.data) {
        (_, ResponseType::NACK_REASON, &[hi, lo]) if response => {
            let reason = NackReason(u16::from_be_bytes([hi, lo]));
            format!("nack {}", reason.name().unwrap_or("unknown"))
        }
        (_, ResponseType::ACK_TIMER, &[hi, lo]) if response => {
            format!(
                "ack-timer {} ms",
                100 * u32::from(u16::from_be_bytes([hi, lo]))
            )
        }
        (Pid::DEVICE_INFO, ResponseType::ACK, d) if response && d.len() == 19 => {
            let i = DeviceInfo::decode(d).unwrap();
            format!(
                "ack device-info model {:#06x} footprint {} personality {}/{} address {} sensors {}",
                i.model, i.footprint, i.personality, i.personalities, i.start_address, i.sensors
            )
        }
        (_, ResponseType::ACK, []) if response => "ack, no data".to_owned(),
        (_, kind, d) if response => format!(
            "ack{} {}",
            if kind == ResponseType::ACK_OVERFLOW {
                "-overflow"
            } else {
                ""
            },
            octets(d)
        ),
        (_, _, []) => "no data".to_owned(),
        (_, _, d) => format!("data {}", octets(d)),
    }
}

fn meaning(m: &Message<'_>) -> String {
    format!(
        "{} {} -> {} tn {} {} {} count {} sub {} pid {}; {}",
        m.command_class.name().unwrap_or("class?"),
        m.source,
        m.destination,
        m.transaction,
        if m.command_class.0 & 1 == 1 {
            "type"
        } else {
            "port"
        },
        m.port_or_response,
        m.message_count,
        m.sub_device,
        m.pid,
        data(m),
    )
}

fn failure(input: &[u8], e: &Error<'_>) -> String {
    assert_eq!(
        input.get(e.offset..),
        Some(e.bytes),
        "an error's bytes are the input from its offset"
    );
    let kind = match e.kind {
        ErrorKind::NotRdm => "not-rdm".to_owned(),
        ErrorKind::Truncated { need } => format!("truncated need {need}"),
        ErrorKind::Length { said, implied } => format!("length {said} not {implied}"),
        ErrorKind::Checksum { said, computed } => {
            format!("checksum {said:#06x} not {computed:#06x}")
        }
        ErrorKind::Trailing => "trailing".to_owned(),
        ErrorKind::Encoding => "collision".to_owned(),
        other => format!("{other:?}"),
    };
    format!("error {kind} at {}", e.offset)
}

fn message(input: &[u8]) -> String {
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

fn discovery(input: &[u8]) -> String {
    match disc::decode(input) {
        Ok(uid) => format!("uid {uid}"),
        Err(e) => failure(input, &e),
    }
}

fn walk(corpora: &[(&str, &str)], decode: fn(&[u8]) -> String) -> usize {
    let mut records = 0;
    for (name, corpus) in corpora {
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
    records
}

#[test]
fn fixture_every_rdm_message_record_decodes_to_its_meaning_and_back() {
    assert!(walk(MESSAGES, message) >= MESSAGES.len());
}

#[test]
fn fixture_every_discovery_response_record_decodes_to_its_uid_or_collision() {
    assert!(walk(DISCOVERY, discovery) >= DISCOVERY.len());
}
