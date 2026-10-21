// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An RDM device as the API shows it, from what the controller read.

use super::names::device_id;
use crate::rdm::{Device, DeviceState};
use regalium_dmx_core::rdm::{DeviceInfo, Uid};

/// The device `uid` on `universe` (`universes/stage-left`), as discovery
/// found it: named, responding, nothing else read yet.
#[must_use]
pub fn discovered(universe: &str, uid: Uid) -> Device {
    Device {
        name: format!("{universe}/devices/{}", device_id(uid)),
        rdm_uid: uid.to_string(),
        manufacturer_id: i32::from(uid.manufacturer()),
        state: DeviceState::Responding.into(),
        ..Device::default()
    }
}

/// The device `uid` on `universe`, with what its `DEVICE_INFO` said. Labels
/// are read separately and set by the caller.
#[must_use]
pub fn with_info(universe: &str, uid: Uid, info: &DeviceInfo) -> Device {
    Device {
        model_id: i32::from(info.model),
        product_category: i32::from(info.category),
        software_version_id: i64::from(info.software),
        footprint: i32::from(info.footprint),
        personality: i32::from(info.personality),
        personality_count: i32::from(info.personalities),
        // 0xFFFF, "no footprint", is no address: left unset.
        start_address: if (1..=512).contains(&info.start_address) {
            i32::from(info.start_address)
        } else {
            0
        },
        sub_device_count: i32::from(info.sub_devices),
        sensor_count: i32::from(info.sensors),
        ..discovered(universe, uid)
    }
}
