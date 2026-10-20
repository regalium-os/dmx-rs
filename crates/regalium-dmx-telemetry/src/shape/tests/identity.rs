// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// A test asserts by panicking; the never-panic lints are not the rule here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The node and RDM series: identity on one info series, filtered to the
//! declared keys, and RDM counted per responder but timed per parameter.

use super::Recorder;
use crate::shape::*;
use crate::{Attribute, keys};

#[test]
fn node_info_carries_the_identity_in_declared_order() {
    let meter = Recorder::default();
    record_node_info(
        &meter,
        "artnet",
        "10.0.0.5/1",
        &[
            Attribute::str(keys::SERVER_ADDRESS, "10.0.0.5:6454"),
            Attribute::str(keys::NODE_SHORT_NAME, "FOH node"),
            Attribute::int(keys::NODE_PORTS, 4),
        ],
    );
    let want = vec![
        ("dmx.protocol", "artnet".to_owned()),
        ("dmx.node", "10.0.0.5/1".to_owned()),
        ("dmx.node.short_name", "FOH node".to_owned()),
        ("dmx.node.ports", "4".to_owned()),
        ("server.address", "10.0.0.5:6454".to_owned()),
    ];
    assert_eq!(meter.all(), [("dmx.node.info", want, 1.0)]);
}

#[test]
fn node_info_drops_a_universe_label_a_repeat_and_a_second_node() {
    let meter = Recorder::default();
    record_node_info(
        &meter,
        "artnet",
        "10.0.0.5/1",
        &[
            Attribute::str(keys::NODE, "10.0.0.9/1"),
            Attribute::int(keys::UNIVERSE, 7),
            Attribute::str(keys::NODE_SHORT_NAME, "first"),
            Attribute::str(keys::NODE_SHORT_NAME, "second"),
        ],
    );
    let want = vec![
        ("dmx.protocol", "artnet".to_owned()),
        ("dmx.node", "10.0.0.5/1".to_owned()),
        ("dmx.node.short_name", "first".to_owned()),
    ];
    assert_eq!(meter.all(), [("dmx.node.info", want, 1.0)]);
}

#[test]
fn an_rdm_request_is_counted_per_responder_and_timed_per_parameter_only() {
    let meter = Recorder::default();
    record_rdm_request(&meter, "get", "DEVICE_INFO", "ack", "7a70:00000001", 2.4);
    let all = meter.all();
    let labels = |name: &str| all.iter().find(|r| r.0 == name).map(|r| r.1.clone());
    let uid = ("dmx.rdm.uid", "7a70:00000001".to_owned());
    let count = labels("dmx.rdm.requests").unwrap_or_default();
    let time = labels("dmx.rdm.request.duration").unwrap_or_default();
    assert!(count.contains(&uid), "{count:?}");
    assert_eq!(
        time,
        vec![
            ("dmx.rdm.command", "get".to_owned()),
            ("dmx.rdm.pid", "DEVICE_INFO".to_owned()),
            ("dmx.outcome", "ack".to_owned()),
        ]
    );
}

#[test]
fn rdm_device_info_keeps_the_patch_and_drops_what_is_not_identity() {
    let meter = Recorder::default();
    record_rdm_device_info(
        &meter,
        "7a70:00000001",
        &[
            Attribute::int(keys::RDM_START_ADDRESS, 101),
            Attribute::str(keys::RDM_LABEL, "SL wash 3"),
            Attribute::str(keys::RDM_UID, "ffff:ffffffff"),
            Attribute::str(keys::NODE, "10.0.0.5/1"),
        ],
    );
    let want = vec![
        ("dmx.rdm.uid", "7a70:00000001".to_owned()),
        ("dmx.rdm.device.label", "SL wash 3".to_owned()),
        ("dmx.rdm.start_address", "101".to_owned()),
    ];
    assert_eq!(meter.all(), [("dmx.rdm.device.info", want, 1.0)]);
}
