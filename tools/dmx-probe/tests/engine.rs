// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The engine over the demo rig, end to end on the real runtime: what the
//! TUI will draw, and what its commands do.

use dmx_probe::engine::model::{Model, State};
use dmx_probe::engine::{Command, Engine};
use dmx_probe::sim;
use std::time::{Duration, Instant};

/// Waits up to `secs` for the model to satisfy `done`, returning the last.
fn until(engine: &mut Engine, secs: u64, mut done: impl FnMut(&Model) -> bool) -> Model {
    let start = Instant::now();
    loop {
        if let Some(model) = engine.model() {
            if done(model) || start.elapsed() > Duration::from_secs(secs) {
                return model.clone();
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn universe<'a>(model: &'a Model, name: &str) -> &'a dmx_probe::engine::model::Universe {
    model.universes.iter().find(|u| u.name == name).expect(name)
}

#[test]
fn the_demo_rig_runs_its_outputs_merges_its_consoles_and_answers_rdm() {
    let mut engine = Engine::start(sim::demo().unwrap()).unwrap();

    let model = until(&mut engine, 5, |m| {
        m.universes.len() == 6
            && universe(m, "stage-left").fps > 30.0
            && universe(m, "foh-in").sources.len() == 2
    });
    let names: Vec<&str> = model.universes.iter().map(|u| u.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "stage-left",
            "stage-right",
            "cyc",
            "floor",
            "foh-in",
            "console-in"
        ]
    );
    let left = universe(&model, "stage-left");
    // Streaming, not keeping alive; the exact rate is the clock's, measured
    // by `cargo run --example fps`, not asserted under a loaded test runner.
    assert!(
        left.fps > 30.0 && left.fps < 48.0,
        "full rate: {}",
        left.fps
    );
    assert_eq!(left.levels[100..103], [255, 255, 128]);
    let foh = universe(&model, "foh-in");
    assert_eq!(foh.state, State::Live);
    let priorities: Vec<(u8, bool)> = foh.sources.iter().map(|s| (s.priority, s.merged)).collect();
    assert!(
        priorities.contains(&(150, true)) && priorities.contains(&(100, false)),
        "{priorities:?}"
    );
    assert_eq!(universe(&model, "console-in").state, State::Idle);

    engine.send(Command::Master(128));
    engine.send(Command::Blackout(true));
    engine.send(Command::Write {
        universe: "stage-left".into(),
        start: 1,
        values: vec![7, 8, 9],
    });
    let model = until(&mut engine, 2, |m| {
        m.policy.blackout && universe(m, "stage-left").levels[..3] == [7, 8, 9]
    });
    assert_eq!(model.policy.master, 128);
    assert!(model.events.iter().any(|e| e.text == "blackout on"));

    engine.send(Command::Discover("stage-left".into()));
    let model = until(&mut engine, 20, |m| m.devices.len() == 24);
    assert_eq!(model.devices.len(), 24, "{:?}", model.discovery);
    let wash = model
        .devices
        .iter()
        .find(|d| d.uid == "4c55:00000203")
        .unwrap();
    assert_eq!((wash.address, wash.footprint), (97, 4));

    engine.send(Command::SetAddress {
        universe: "stage-left".into(),
        uid: "4c55:00000203".into(),
        address: 321,
    });
    let model = until(&mut engine, 5, |m| {
        m.devices
            .iter()
            .any(|d| d.uid == "4c55:00000203" && d.address == 321)
    });
    assert!(
        model
            .devices
            .iter()
            .any(|d| d.uid == "4c55:00000203" && d.address == 321),
        "read back after the SET"
    );
    assert!(
        model
            .packets
            .iter()
            .any(|p| p.octets.first() == Some(&0xCC)),
        "the RDM traffic was captured"
    );
}

#[test]
fn every_captured_packet_dissects_and_the_rdm_answer_shows_its_fields() {
    let mut engine = Engine::start(sim::demo().unwrap()).unwrap();
    engine.send(Command::Discover("stage-left".into()));
    let model = until(&mut engine, 20, |m| m.devices.len() == 24);
    for packet in &model.packets {
        let d = dmx_probe::dissect::dissect(packet);
        assert!(!d.summary.is_empty() && !d.tree.is_empty(), "{packet:?}");
        for field in &d.tree {
            if let Some((from, to)) = field.span {
                assert!(
                    from <= to && to < packet.octets.len().max(1) + 23,
                    "{} {:?} in {}",
                    field.text,
                    field.span,
                    packet.octets.len()
                );
            }
        }
    }
    let replies: Vec<String> = model
        .packets
        .iter()
        .map(|p| dmx_probe::dissect::dissect(p).summary)
        .collect();
    assert!(
        replies
            .iter()
            .any(|s| s.starts_with("ArtPollReply · Node A")),
        "{replies:?}"
    );
    assert!(
        replies.iter().any(|s| s.starts_with("ACK 0x0060")),
        "a DEVICE_INFO answer: {replies:?}"
    );
}
