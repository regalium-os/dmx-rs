// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What moves in the demo: two consoles feeding `foh-in`, the floor's
//! widget unplugged now and then, a look on the stage, and the discovery
//! traffic a real network would show at start.

use super::bus::Bus;
use crate::engine::Capture;
use crate::engine::model::Protocol;
use regalium_dmx::address::Address;
use regalium_dmx::artnet::codec::{ArtPoll, Reply, ST_NODE, name};
use regalium_dmx::runtime::{Levels, Output, SystemClock};
use regalium_dmx::sacn::codec::{Cid, DiscoveryPacket, list};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// How long the backup console sends before it goes silent.
const BACKUP_FOR: Duration = Duration::from_secs(20);

/// Starts the demo's threads; they stop when `stop` is set.
///
/// # Errors
///
/// When the operating system cannot start a thread.
#[allow(clippy::too_many_arguments, reason = "the demo's parts, named once")]
pub(super) fn start(
    bus: &Arc<Bus>,
    capture: &Arc<Capture>,
    stop: &Arc<AtomicBool>,
    unplugged: &Arc<AtomicBool>,
    stage_left: &Levels,
    stage_right: &Levels,
    cyc: &Levels,
    floor: &Levels,
) -> std::io::Result<Vec<JoinHandle<()>>> {
    look(stage_left, stage_right, cyc, floor);
    announce(capture);

    let foh = bus.port(Protocol::Sacn, 0xF0C, 150, capture, "239.255.0.1");
    let (foh, _) = Output::builder(foh, SystemClock::new()).build();
    let foh_levels = foh.patch(1);
    let mut foh_running = foh.start()?;
    let backup = bus.port(Protocol::Sacn, 0xBAC, 100, capture, "239.255.0.1");
    let (backup, _) = Output::builder(backup, SystemClock::new()).build();
    let backup_levels = backup.patch(1);
    let _ = backup_levels.write(Address::FIRST, &[90; 24]);
    let mut backup_running = Some(backup.start()?);

    let (stop, unplugged) = (Arc::clone(stop), Arc::clone(unplugged));
    let thread = std::thread::Builder::new()
        .name("dmx-demo".into())
        .spawn(move || {
            let started = Instant::now();
            let mut beat = 0usize;
            while !stop.load(Ordering::Acquire) {
                std::thread::sleep(Duration::from_millis(100));
                let t = started.elapsed();
                // A chase on the FOH console's first 24 slots.
                beat = beat.wrapping_add(1);
                let chase: Vec<u8> = (0..24)
                    .map(|i| {
                        if (i + beat / 2).is_multiple_of(8) {
                            255
                        } else {
                            40
                        }
                    })
                    .collect();
                let _ = foh_levels.write(Address::FIRST, &chase);
                if t >= BACKUP_FOR
                    && let Some(mut running) = backup_running.take()
                {
                    running.close();
                }
                // The widget is out for ten seconds in every forty, after thirty.
                let cycle = t.as_secs() % 40;
                unplugged.store(t.as_secs() >= 30 && cycle >= 30, Ordering::Relaxed);
            }
            foh_running.close();
        })?;
    Ok(vec![thread])
}

/// The look the mock shows: sixteen-slot fixtures from 1 with their
/// intensities up, and a run at 101–108.
fn look(stage_left: &Levels, stage_right: &Levels, cyc: &Levels, floor: &Levels) {
    let mut levels = [0u8; 512];
    for (i, level) in levels.iter_mut().enumerate().take(304) {
        let a = i + 1;
        *level = match (a - 1) % 16 {
            0 => u8::try_from(255 - (a * 7) % 90).unwrap_or(255),
            1 => u8::try_from(180 + (a * 3) % 60).unwrap_or(255),
            2 | 3 => 128,
            6 => u8::try_from((a * 13) % 256).unwrap_or(0),
            _ => 0,
        };
    }
    if let Some(run) = levels.get_mut(100..108) {
        run.copy_from_slice(&[255, 255, 128, 0, 64, 0, 0, 0]);
    }
    let _ = stage_left.write(Address::FIRST, &levels);
    let _ = stage_right.write(Address::FIRST, &[200; 64]);
    let _ = cyc.write(Address::FIRST, &[0, 120, 255]);
    let _ = floor.write(Address::FIRST, &[255; 12]);
}

/// The discovery a network shows at start, as real datagrams.
fn announce(capture: &Capture) {
    let mut out = Vec::new();
    ArtPoll::new().encode(&mut out);
    capture.record(true, Protocol::ArtNet, "10.0.0.255", &out, false);
    for (ip, short, long) in [
        ([10, 0, 0, 5], "Node A", "Stage left dimmers"),
        ([10, 0, 0, 6], "Truss 2", "Truss 2 movers"),
        ([10, 0, 0, 9], "Desk", "Lighting desk"),
    ] {
        let mut reply = Reply::controller(Ipv4Addr::from(ip));
        reply.style = ST_NODE;
        reply.short_name = name(short, 18);
        reply.long_name = name(long, 64);
        out.clear();
        reply.encode(&mut out);
        capture.record(
            false,
            Protocol::ArtNet,
            &format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]),
            &out,
            false,
        );
    }
    let mut universes = Vec::new();
    list(&[1, 7], &mut universes);
    let mut padded = [0u8; 64];
    for (to, from) in padded.iter_mut().zip(b"FOH console") {
        *to = *from;
    }
    out.clear();
    DiscoveryPacket {
        cid: Cid([0x5a; 16]),
        source_name: &padded,
        reserved: [0; 4],
        page: 0,
        last_page: 0,
        list: &universes,
    }
    .encode(&mut out);
    capture.record(false, Protocol::Sacn, "10.0.0.20", &out, false);
}
