// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! Packets this crate composes decode to what it put in.

use regalium_dmx_artnet::codec::{
    ArtDmx, ArtPoll, ArtSync, Packet, PortAddress, REPLY_LEN, Reply, name, text,
};
use std::net::Ipv4Addr;

fn decode(out: &[u8]) -> Packet<'_> {
    Packet::decode(out).unwrap()
}

#[test]
fn a_composed_reply_is_239_octets_and_reads_back_field_for_field() {
    let mut reply = Reply::controller(Ipv4Addr::new(10, 0, 0, 7));
    reply.short_name = name("a short name that is far too long", 18);
    reply.long_name = name("Long", 64);
    reply.mac = [1, 2, 3, 4, 5, 6];
    reply.sw_out = [1, 2, 3, 4];
    let mut out = Vec::new();
    reply.encode(&mut out);
    assert_eq!(out.len(), REPLY_LEN);
    let Packet::PollReply(back) = decode(&out) else {
        panic!("not a reply")
    };
    let fields = back.fields();
    // The text fields come back at their full width; compare them as text.
    let texts = Reply {
        short_name: fields.short_name,
        long_name: fields.long_name,
        report: fields.report,
        ..reply
    };
    assert_eq!(fields, texts);
    assert_eq!(text(fields.report), b"");
    assert_eq!(
        text(back.fields().short_name),
        b"a short name that",
        "17 octets and a NUL"
    );
    assert_eq!(text(back.fields().long_name), b"Long");
}

#[test]
fn a_name_is_cut_at_a_character_boundary() {
    assert_eq!(
        name("ééééééééé", 18),
        "éééééééé".as_bytes(),
        "16 octets: a 17th would split an é"
    );
    assert_eq!(name("", 18), b"");
}

#[test]
fn composed_dmx_poll_and_sync_decode_to_themselves() {
    let port = PortAddress::new(300).unwrap();
    for packet in [
        Packet::Dmx(ArtDmx::new(port, 5, &[1, 2])),
        Packet::Poll(ArtPoll::new()),
        Packet::Sync(ArtSync::new()),
    ] {
        let mut out = Vec::new();
        packet.encode(&mut out);
        assert_eq!(decode(&out), packet);
    }
}
