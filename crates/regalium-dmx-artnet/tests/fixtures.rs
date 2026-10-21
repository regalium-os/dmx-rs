// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! The phase gate: every record of every Art-Net corpus decodes to what the
//! record says, and re-encodes to exactly its octets.
//!
//! A record is `octets => meaning`, as in the core's corpus. `just
//! fixtures` fails when a `.hex` file here is not named in `CORPORA`.

use regalium_dmx_artnet::codec::{Error, ErrorKind, Packet, text};
use std::fmt::Write as _;

#[path = "fixtures/rdm.rs"]
mod rdm;

const CORPORA: &[(&str, &str)] = &[
    ("dmx.hex", include_str!("../fixtures/dmx.hex")),
    ("poll.hex", include_str!("../fixtures/poll.hex")),
    ("poll_reply.hex", include_str!("../fixtures/poll_reply.hex")),
    ("sync.hex", include_str!("../fixtures/sync.hex")),
    ("address.hex", include_str!("../fixtures/address.hex")),
    ("other.hex", include_str!("../fixtures/other.hex")),
    ("rdm.hex", include_str!("../fixtures/rdm.hex")),
    ("malformed.hex", include_str!("../fixtures/malformed.hex")),
];

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).expect("hex octet"))
        .collect()
}

pub(crate) fn octets(b: &[u8]) -> String {
    b.iter().fold(String::new(), |mut out, x| {
        let _ = write!(out, "{x:02x}");
        out
    })
}

fn quoted(field: &[u8]) -> String {
    format!("{:?}", String::from_utf8_lossy(text(field)))
}

fn meaning(packet: &Packet<'_>) -> String {
    match packet {
        Packet::Dmx(d) => {
            let lit: Vec<String> = d
                .slots()
                .iter()
                .enumerate()
                .filter(|(_, l)| **l != 0)
                .map(|(i, l)| format!("{}={l}", i + 1))
                .collect();
            let levels = if lit.is_empty() {
                "dark".to_owned()
            } else {
                lit.join(" ")
            };
            let mut out = format!(
                "dmx v{} seq {} phys {} universe {} {} slots; {levels}",
                d.version(),
                d.sequence(),
                d.physical(),
                d.port_address(),
                d.slots().len(),
            );
            if !d.trailing().is_empty() {
                let _ = write!(out, "; trailing {}", octets(d.trailing()));
            }
            out
        }
        Packet::Poll(p) => {
            let target = p.target().map_or_else(
                || "everyone".to_owned(),
                |r| format!("{}..={}", r.start(), r.end()),
            );
            format!(
                "poll v{} flags {:#04x} diag {:#04x} for {target}",
                p.version, p.flags, p.diagnostics
            )
        }
        Packet::PollReply(r) => {
            let f = r.fields();
            format!(
                "reply {} octets from {}:{} {} {} style {} ports {} out {:?} net {} sub {} mac {} status2 {:#04x}",
                r.len(),
                f.ip,
                f.port,
                quoted(f.short_name),
                quoted(f.long_name),
                f.style,
                f.ports,
                f.sw_out,
                f.net_switch,
                f.sub_switch,
                octets(&f.mac),
                f.status2,
            )
        }
        Packet::Sync(s) => format!("sync v{} aux {}", s.version, octets(&s.aux)),
        Packet::Address(a) => format!(
            "address v{} net {:#04x} sub {:#04x} bind {} {} {} out {:?} command {:#04x}",
            a.version,
            a.net_switch,
            a.sub_switch,
            a.bind_index,
            quoted(a.short_name),
            quoted(a.long_name),
            a.sw_out,
            a.command,
        ),
        Packet::Rdm(_) | Packet::TodRequest(_) | Packet::TodControl(_) | Packet::TodData(_) => {
            rdm::meaning(packet)
        }
        Packet::Other { opcode, body: [] } => format!("{opcode}; no body"),
        Packet::Other { opcode, body } => format!("{opcode}; body {}", octets(body)),
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
        ErrorKind::NotArtNet => "not-artnet".to_owned(),
        ErrorKind::Truncated { need } => format!("truncated need {need}"),
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
fn fixture_every_artnet_record_decodes_to_its_meaning_and_back() {
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
            let got = decode(&hex(input));
            assert_eq!(got, want.trim(), "{name}:{}", n + 1);
            in_file += 1;
        }
        assert!(in_file > 0, "{name} has no records");
        records += in_file;
    }
    assert!(records >= CORPORA.len(), "walked {records} records");
}
