// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! Packets this crate composes decode to what it put in.

use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_sacn::codec::{
    Cid, DataPacket, DiscoveryPacket, Packet, SyncPacket, list, name, text,
};

#[test]
fn composed_packets_decode_to_themselves() {
    let cid = Cid([7; 16]);
    let mut universes = Vec::new();
    list(&[1, 2, 63_999], &mut universes);
    let mut padded = [0u8; 64];
    padded[..4].copy_from_slice(b"Desk");
    for packet in [
        Packet::Data(DataPacket {
            cid,
            source_name: &padded,
            priority: 200,
            sync_address: 7,
            sequence: 3,
            options: 0,
            universe: 9,
            start_code: StartCode::NULL,
            slots: &[1, 2, 3],
        }),
        Packet::Sync(SyncPacket {
            cid,
            sequence: 1,
            sync_address: 7,
            reserved: [0; 2],
        }),
        Packet::Discovery(DiscoveryPacket {
            cid,
            source_name: &padded,
            reserved: [0; 4],
            page: 0,
            last_page: 0,
            list: &universes,
        }),
    ] {
        let mut out = Vec::new();
        packet.encode(&mut out);
        assert_eq!(Packet::decode(&out).unwrap(), packet);
    }
}

#[test]
fn a_cid_prints_as_a_uuid_and_a_name_is_cut_with_room_for_its_nul() {
    let cid = Cid([
        0x5a, 0x7c, 0x1e, 0x2f, 0x0b, 0x9d, 0x4c, 0x3e, 0x8f, 0x6a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f,
        0x60,
    ]);
    assert_eq!(cid.to_string(), "5a7c1e2f-0b9d-4c3e-8f6a-1b2c3d4e5f60");
    assert_ne!(
        Cid([2; 16]).folded(),
        Cid([3; 16]).folded(),
        "halves alike must not fold alike"
    );
    let long = "x".repeat(100);
    assert_eq!(name(&long).len(), 63);
    assert_eq!(text(b"Desk\0junk"), b"Desk");
}
