// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Names round-trip, devices fill from what RDM read, and the messages
//! speak protobuf's canonical JSON.

use regalium_dmx_core::address::Address;
use regalium_dmx_core::merge::Mode;
use regalium_dmx_core::rdm::{DeviceInfo, Pid, Uid};
use regalium_dmx_core::universe::Universe;
use regalium_dmx_schema::convert::{device, levels, names};
use regalium_dmx_schema::node::Protocol;
use regalium_dmx_schema::universe::{Input, MergeMode, universe::Role};

#[test]
fn a_uid_is_a_device_id_with_a_hyphen_and_back() {
    let uid = Uid::new(0x4c55, 0x1234_abcd);
    assert_eq!(names::device_id(uid), "4c55-1234abcd");
    assert_eq!(names::uid("4c55-1234abcd"), Some(uid));
    for bad in [
        "4C55-1234ABCD",
        "4c55:1234abcd",
        "4c5-1234abcd",
        "4c55-1234abc",
        "",
        "zz55-12345678",
    ] {
        assert_eq!(names::uid(bad), None, "{bad}");
    }
}

#[test]
fn every_standard_parameter_and_any_other_round_trips() {
    for &(pid, name) in names::PARAMETERS {
        assert_eq!(names::parameter_id(pid), name);
        assert_eq!(names::pid(name), Some(pid));
    }
    assert_eq!(names::parameter_id(Pid(0x8123)), "pid-8123");
    assert_eq!(names::pid("pid-8123"), Some(Pid(0x8123)));
    assert_eq!(names::pid("pid-8G23"), None);
    assert_eq!(names::pid("pid-81234"), None);
    assert_eq!(names::pid("start-address"), None);
}

#[test]
fn a_device_fills_from_device_info_and_no_footprint_is_no_address() {
    let uid = Uid::new(0x4c55, 7);
    let info = DeviceInfo {
        model: 3,
        footprint: 16,
        personality: 2,
        personalities: 4,
        start_address: 101,
        sensors: 1,
        ..DeviceInfo::default()
    };
    let d = device::with_info("universes/stage-left", uid, &info);
    assert_eq!(d.name, "universes/stage-left/devices/4c55-00000007");
    assert_eq!(d.rdm_uid, "4c55:00000007");
    assert_eq!(
        (
            d.manufacturer_id,
            d.footprint,
            d.start_address,
            d.personality_count
        ),
        (0x4c55, 16, 101, 4)
    );
    let none = DeviceInfo {
        start_address: 0xFFFF,
        ..info
    };
    assert_eq!(
        device::with_info("universes/x", uid, &none).start_address,
        0
    );
}

#[test]
fn levels_carry_the_slots_sent_and_the_generation() {
    let mut u = Universe::new();
    u.set_len(3).unwrap();
    u.set(Address::FIRST, 255);
    let l = levels::levels("universes/stage-left", &u);
    assert_eq!(l.name, "universes/stage-left/levels");
    assert_eq!(l.slots, [255, 0, 0]);
    assert_eq!(l.generation, i64::try_from(u.generation()).unwrap());
    assert_eq!(levels::mode(levels::merge_mode(Mode::Ltp)), Some(Mode::Ltp));
    assert_eq!(levels::mode(MergeMode::Unspecified), None);
}

#[test]
fn a_universe_is_canonical_protobuf_json() {
    let universe = regalium_dmx_schema::universe::Universe {
        name: "universes/foh-in".to_owned(),
        protocol: Protocol::Sacn.into(),
        number: 7,
        role: Some(Role::Input(Input {
            merge_mode: MergeMode::Htp.into(),
            max_sources: 4,
            sources: vec![],
        })),
        ..Default::default()
    };
    let json = serde_json::to_value(&universe).unwrap();
    assert_eq!(json["protocol"], "PROTOCOL_SACN", "enums by name");
    assert_eq!(
        json["input"]["mergeMode"], "MERGE_MODE_HTP",
        "fields in lowerCamelCase"
    );
    assert_eq!(json["input"]["maxSources"], 4);
    let back: regalium_dmx_schema::universe::Universe = serde_json::from_value(json).unwrap();
    assert_eq!(back, universe);
}
