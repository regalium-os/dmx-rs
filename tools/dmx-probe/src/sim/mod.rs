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
mod layout;
mod show;

use crate::engine::model::Protocol;
use crate::engine::{Capture, RdmPort, Rig, Stats};
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

    let mut output = |protocol, source, priority, peer: &str| {
        let port = bus.port(protocol, source, priority, &capture, peer);
        let failing = Arc::clone(&port.failing);
        let (output, e) = Output::builder(port, SystemClock::new())
            .meter(Arc::clone(&meter))
            .policy(Arc::clone(&policy))
            .build();
        events.push(e);
        (output, failing)
    };
    let (art_out, _) = output(Protocol::ArtNet, 1, 100, "10.0.0.5");
    let (sacn_out, _) = output(Protocol::Sacn, 2, 150, "239.255.0.7");
    let (serial_out, unplugged) = output(Protocol::Serial, 3, 100, "/dev/ttyUSB0");
    let stage_left = art_out.patch(1);
    let stage_right = art_out.patch(2);
    let cyc = sacn_out.patch(7);
    let _ = cyc.set_len(96);
    let floor = serial_out.patch(1);
    let _ = floor.set_len(48);
    for o in [&art_out, &sacn_out, &serial_out] {
        running.push(o.start()?);
    }

    let sacn_in = bus.port(Protocol::Sacn, 4, 100, &capture, "0.0.0.0");
    let (input, e) = Input::builder(sacn_in, SystemClock::new())
        .meter(Arc::clone(&meter))
        .build();
    events.push(e);
    let foh = input
        .listen(1, Merge::new(Mode::Htp))
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    running.push(input.start()?);

    let art_in = bus.port(Protocol::ArtNet, 5, 100, &capture, "0.0.0.0");
    let (input, e) = Input::builder(art_in, SystemClock::new())
        .meter(Arc::clone(&meter))
        .build();
    events.push(e);
    let console_in = input
        .listen(16, Merge::new(Mode::Ltp))
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    running.push(input.start()?);

    let line = Arc::new(fixtures::Line::new(fixtures::stage(), &capture));
    let controller: Arc<dyn RdmPort> =
        Arc::new(Controller::new(line, SystemClock::new(), CONTROLLER));
    let rdm = HashMap::from([("stage-left".to_owned(), controller)]);

    let background = show::start(
        &bus,
        &capture,
        &stop,
        &unplugged,
        &stage_left,
        &stage_right,
        &cyc,
        &floor,
    )?;

    let entries = layout::entries([stage_left, stage_right, cyc, floor], foh, console_in);
    Ok(Rig {
        entries,
        policy,
        stats,
        capture,
        events,
        rdm,
        nodes: layout::nodes(),
        running,
        background,
        stop,
        attached: "demo rig".to_owned(),
    })
}
