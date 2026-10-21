// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Prints the demo's frame rate and interval histogram for stage-left: a
//! measurement of the system clock's waiting on this machine.

#![allow(
    clippy::expect_used,
    clippy::cast_precision_loss,
    reason = "a measuring tool, run by hand"
)]

use dmx_probe::engine::Engine;
use dmx_probe::sim;
use std::time::Duration;

fn main() {
    let mut engine = Engine::start(sim::demo().expect("demo")).expect("engine");
    std::thread::sleep(Duration::from_secs(6));
    if let Some(m) = engine.model() {
        for u in m.universes.iter().take(2) {
            let mean = u.intervals.iter().sum::<f32>() / u.intervals.len().max(1) as f32;
            println!(
                "{}: {:.1} fps, mean {:.2} ms, histogram {:?}",
                u.name, u.fps, mean, u.histogram
            );
        }
    }
}
