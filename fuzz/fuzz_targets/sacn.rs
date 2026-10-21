// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! E1.31 datagrams: anything that decodes re-encodes byte for byte, and
//! reading any packet's frame or universes never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use regalium_dmx::sacn::codec::Packet;

fuzz_target!(|data: &[u8]| {
    if let Ok(packet) = Packet::decode(data) {
        let mut again = Vec::new();
        packet.encode(&mut again);
        assert_eq!(again, data, "E1.31 packets are lossless");
        match packet {
            Packet::Data(d) => assert!(d.frame().slots().len() <= 512),
            Packet::Discovery(d) => assert!(d.universes().count() <= 512),
            _ => {}
        }
    }
});
