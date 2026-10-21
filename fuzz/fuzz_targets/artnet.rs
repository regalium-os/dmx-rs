// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net datagrams: anything that decodes re-encodes byte for byte, and
//! reading any packet's fields or levels never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use regalium_dmx::artnet::codec::Packet;

fuzz_target!(|data: &[u8]| {
    if let Ok(packet) = Packet::decode(data) {
        let mut again = Vec::new();
        packet.encode(&mut again);
        assert_eq!(again, data, "Art-Net packets are lossless");
        match packet {
            Packet::Dmx(dmx) => assert!(dmx.frame().slots().len() <= 512),
            Packet::Poll(poll) => {
                let _ = poll.target();
            }
            Packet::PollReply(reply) => {
                let _ = reply.fields();
            }
            _ => {}
        }
    }
});
