// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM: a message that decodes re-encodes byte for byte; a discovery answer
//! decodes or is a collision, never a panic; and a responder handed any
//! octets as a request answers or stays silent, never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use regalium_dmx::rdm::{DeviceInfo, Discovery, Message, Responder, Uid, disc};

fuzz_target!(|data: &[u8]| {
    if let Ok(message) = Message::decode(data) {
        let mut again = Vec::new();
        message.encode(&mut again);
        assert_eq!(again, data, "RDM messages are lossless");
    }
    let _ = disc::decode(data);
    let _ = DeviceInfo::decode(data);
    let mut device = Responder::new(Uid::new(0x4c55, 1), DeviceInfo::default());
    let _ = device.answer(data);
    let mut search = Discovery::new();
    search.sent();
    search.branched(Some(data));
    search.muted(Some(data));
    let _ = search.next();
});
