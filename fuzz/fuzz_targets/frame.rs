// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Frames, universes and merges: anything that decodes re-encodes byte for
//! byte, and applying or merging it never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use regalium_dmx::frame::Frame;
use regalium_dmx::merge::{Merge, Mode, SourceId};
use regalium_dmx::universe::Universe;

fuzz_target!(|data: &[u8]| {
    if let Ok(frame) = Frame::decode(data) {
        let mut again = Vec::new();
        frame.encode(&mut again);
        assert_eq!(again, data, "frames are lossless");
        let mut universe = Universe::new();
        let _ = universe.apply(&frame);
        for mode in [Mode::Htp, Mode::Ltp] {
            let mut merge = Merge::new(mode);
            let _ = merge.update(SourceId(1), &frame);
            let _ = merge.update(SourceId(2), &frame);
            merge.render(&mut universe);
            assert!(universe.levels().len() <= 512);
        }
    }
});
