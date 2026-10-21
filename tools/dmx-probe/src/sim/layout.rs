// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The demo's universes, in the order the probe shows them, and the nodes
//! it pretends to have found.

use crate::engine::Entry;
use crate::engine::model::{Node, Protocol};
use regalium_dmx::runtime::{Levels, Merged};

/// The universes, in the order the probe shows them.
pub(super) fn entries(outputs: [Levels; 4], foh: Merged, console_in: Merged) -> Vec<Entry> {
    let entry = |name: &str, protocol, number: u32, label: &str, target: &str| Entry {
        name: name.to_owned(),
        protocol,
        number,
        label: label.to_owned(),
        levels: None,
        merged: None,
        target: target.to_owned(),
    };
    let [stage_left, stage_right, cyc, floor] = outputs;
    vec![
        Entry {
            levels: Some(stage_left),
            ..entry("stage-left", Protocol::ArtNet, 1, "0:0:1", "10.0.0.5")
        },
        Entry {
            levels: Some(stage_right),
            ..entry("stage-right", Protocol::ArtNet, 2, "0:0:2", "10.0.0.5")
        },
        Entry {
            levels: Some(cyc),
            ..entry("cyc", Protocol::Sacn, 7, "7", "239.255.0.7")
        },
        Entry {
            levels: Some(floor),
            ..entry("floor", Protocol::Serial, 1, "1", "/dev/ttyUSB0")
        },
        Entry {
            merged: Some((foh, "htp")),
            ..entry("foh-in", Protocol::Sacn, 1, "1", "")
        },
        Entry {
            merged: Some((console_in, "ltp")),
            ..entry("console-in", Protocol::ArtNet, 16, "0:1:0", "")
        },
    ]
}

pub(super) fn nodes() -> Vec<Node> {
    let node = |protocol, name: &str, address: &str, ports: &str, firmware: &str, reachable| Node {
        protocol,
        name: name.to_owned(),
        address: address.to_owned(),
        ports: ports.to_owned(),
        firmware: firmware.to_owned(),
        reachable,
    };
    vec![
        node(
            Protocol::ArtNet,
            "Node A",
            "10.0.0.5:6454",
            "out 0:0:1 0:0:2",
            "0x0102",
            true,
        ),
        node(
            Protocol::ArtNet,
            "Truss 2",
            "10.0.0.6:6454",
            "out 0:0:3",
            "0x0300",
            true,
        ),
        node(
            Protocol::ArtNet,
            "Desk",
            "10.0.0.9:6454",
            "controller",
            "—",
            true,
        ),
        node(
            Protocol::Sacn,
            "FOH console",
            "10.0.0.20",
            "univ 1 7",
            "—",
            true,
        ),
        node(
            Protocol::Sacn,
            "Backup console",
            "10.0.0.21",
            "univ 1",
            "—",
            true,
        ),
        node(
            Protocol::Serial,
            "DMX USB PRO",
            "/dev/ttyUSB0",
            "1 line",
            "1.44",
            true,
        ),
    ]
}
