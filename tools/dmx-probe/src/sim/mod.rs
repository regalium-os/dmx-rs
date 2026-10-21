// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A rig in memory, for `dmx-probe --demo`, for tests, and for showing the
//! probe without a lighting network: the real runtime, over a simulated
//! bus, with simulated consoles and RDM fixtures.
//!
//! Outputs: `stage-left` and `stage-right` on Art-Net, `cyc` on sACN at
//! priority 150, `floor` on a serial line that is unplugged now and then.
//! Inputs: `foh-in`, merging two sACN consoles at 150 and 100 — the backup
//! goes silent after twenty seconds — and `console-in`, idle. Behind
//! `stage-left`, nineteen spots and five washes answer RDM, and Wash 3 is
//! patched over Spot 7.

pub mod bus;
pub mod fixtures;
mod show;

use crate::engine::model::{Node, Protocol};
use crate::engine::{Capture, Entry, RdmPort, Rig, Stats};
use bus::Bus;
use regalium_dmx::merge::{Merge, Mode};
use regalium_dmx::rdm::Uid;
use regalium_dmx::runtime::rdm::Controller;
use regalium_dmx::runtime::{Input, Output, Policy, SystemClock};
use regalium_dmx_telemetry::Meter;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// The probe's own UID when it is the RDM controller.
pub const CONTROLLER: Uid = Uid::new(0x7a70, 0x0000_0001);

/// Builds and starts the demo rig.
///
/// # Errors
///
/// When the operating system cannot start a thread.
pub fn demo() -> std::io::Result<Rig> {
    let capture = Arc::new(Capture::new());
    let stats = Arc::new(Stats::new());
    let meter: Arc<dyn Meter> = Arc::clone(&stats) as Arc<dyn Meter>;
    let policy = Arc::new(Policy::new());
    let bus = Arc::new(Bus::default());
    let stop = Arc::new(AtomicBool::new(false));
    let mut running = Vec::new();
    let mut events = Vec::new();

    let artnet = bus.port(Protocol::ArtNet, 1, 100, &capture, "10.0.0.5");
    let (art_out, e) = Output::builder(artnet, SystemClock::new()).meter(Arc::clone(&meter)).policy(Arc::clone(&policy)).build();
    events.push(e);
    let stage_left = art_out.patch(1);
    let stage_right = art_out.patch(2);
    running.push(art_out.start()?);

    let sacn = bus.port(Protocol::Sacn, 2, 150, &capture, "239.255.0.7");
    let (sacn_out, e) = Output::builder(sacn, SystemClock::new()).meter(Arc::clone(&meter)).policy(Arc::clone(&policy)).build();
    events.push(e);
    let cyc = sacn_out.patch(7);
    let _ = cyc.set_len(96);
    running.push(sacn_out.start()?);

    let serial = bus.port(Protocol::Serial, 3, 100, &capture, "/dev/ttyUSB0");
    let unplugged = Arc::clone(&serial.failing);
    let (serial_out, e) = Output::builder(serial, SystemClock::new()).meter(Arc::clone(&meter)).policy(Arc::clone(&policy)).build();
    events.push(e);
    let floor = serial_out.patch(1);
    let _ = floor.set_len(48);
    running.push(serial_out.start()?);

    let sacn_in = bus.port(Protocol::Sacn, 4, 100, &capture, "0.0.0.0");
    let (input, e) = Input::builder(sacn_in, SystemClock::new()).meter(Arc::clone(&meter)).build();
    events.push(e);
    let foh = input.listen(1, Merge::new(Mode::Htp)).map_err(|e| std::io::Error::other(e.to_string()))?;
    running.push(input.start()?);

    let art_in = bus.port(Protocol::ArtNet, 5, 100, &capture, "0.0.0.0");
    let (input, e) = Input::builder(art_in, SystemClock::new()).meter(Arc::clone(&meter)).build();
    events.push(e);
    let console_in = input.listen(16, Merge::new(Mode::Ltp)).map_err(|e| std::io::Error::other(e.to_string()))?;
    running.push(input.start()?);

    let line = Arc::new(fixtures::Line::new(fixtures::stage(), &capture));
    let controller: Arc<dyn RdmPort> = Arc::new(Controller::new(line, SystemClock::new(), CONTROLLER));
    let rdm = HashMap::from([("stage-left".to_owned(), controller)]);

    let background = show::start(&bus, &capture, &stop, &unplugged, &stage_left, &stage_right, &cyc, &floor)?;

    let entry = |name: &str, protocol, number: u32, label: &str, target: &str| Entry {
        name: name.to_owned(),
        protocol,
        number,
        label: label.to_owned(),
        levels: None,
        merged: None,
        target: target.to_owned(),
    };
    let entries = vec![
        Entry { levels: Some(stage_left), ..entry("stage-left", Protocol::ArtNet, 1, "0:0:1", "10.0.0.5") },
        Entry { levels: Some(stage_right), ..entry("stage-right", Protocol::ArtNet, 2, "0:0:2", "10.0.0.5") },
        Entry { levels: Some(cyc), ..entry("cyc", Protocol::Sacn, 7, "7", "239.255.0.7") },
        Entry { levels: Some(floor), ..entry("floor", Protocol::Serial, 1, "1", "/dev/ttyUSB0") },
        Entry { merged: Some((foh, "htp")), ..entry("foh-in", Protocol::Sacn, 1, "1", "") },
        Entry { merged: Some((console_in, "ltp")), ..entry("console-in", Protocol::ArtNet, 16, "0:1:0", "") },
    ];
    Ok(Rig { entries, policy, stats, capture, events, rdm, nodes: nodes(), running, background, stop, attached: "demo rig".to_owned() })
}

fn nodes() -> Vec<Node> {
    let node = |protocol, name: &str, address: &str, ports: &str, firmware: &str, reachable| Node {
        protocol,
        name: name.to_owned(),
        address: address.to_owned(),
        ports: ports.to_owned(),
        firmware: firmware.to_owned(),
        reachable,
    };
    vec![
        node(Protocol::ArtNet, "Node A", "10.0.0.5:6454", "out 0:0:1 0:0:2", "0x0102", true),
        node(Protocol::ArtNet, "Truss 2", "10.0.0.6:6454", "out 0:0:3", "0x0300", true),
        node(Protocol::ArtNet, "Desk", "10.0.0.9:6454", "controller", "—", true),
        node(Protocol::Sacn, "FOH console", "10.0.0.20", "univ 1 7", "—", true),
        node(Protocol::Sacn, "Backup console", "10.0.0.21", "univ 1", "—", true),
        node(Protocol::Serial, "DMX USB PRO", "/dev/ttyUSB0", "1 line", "1.44", true),
    ]
}
