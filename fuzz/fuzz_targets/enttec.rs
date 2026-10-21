// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! ENTTEC API messages and streams: a message that decodes re-encodes byte
//! for byte, and a framer fed anything never panics, never outgrows one
//! message, and only hands out messages that decode.

#![no_main]

use libfuzzer_sys::fuzz_target;
use regalium_dmx::serial::enttec::{Framer, Message, Params, serial};

fuzz_target!(|data: &[u8]| {
    if let Ok(message) = Message::decode(data) {
        let mut again = Vec::new();
        message.encode(&mut again);
        assert_eq!(again, data, "ENTTEC messages are lossless");
        let _ = (Params::decode(message.data), serial(message.data));
    }
    let mut framer = Framer::new();
    for chunk in data.chunks(7) {
        framer.feed(chunk, |m| {
            let mut out = Vec::new();
            m.encode(&mut out);
            assert!(Message::decode(&out).is_ok());
        });
    }
});
