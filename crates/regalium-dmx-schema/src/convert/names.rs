// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Resource IDs for what RDM numbers: a device's UID, a parameter's PID.
//!
//! AIP-122 IDs are lower case, digits and hyphens, so a UID's colon becomes
//! a hyphen, `4c55-12345678`, and a PID is named by the standard's name,
//! `dmx-start-address`, or `pid-` and four hexadecimal digits.

use regalium_dmx_core::rdm::{Pid, Uid};

/// Standard parameters by the ID this API names them with: E1.20's name,
/// in lower case with hyphens.
pub const PARAMETERS: &[(Pid, &str)] = &[
    (Pid::QUEUED_MESSAGE, "queued-message"),
    (Pid::STATUS_MESSAGES, "status-messages"),
    (Pid::SUPPORTED_PARAMETERS, "supported-parameters"),
    (Pid::PARAMETER_DESCRIPTION, "parameter-description"),
    (Pid::DEVICE_INFO, "device-info"),
    (Pid::DEVICE_MODEL_DESCRIPTION, "device-model-description"),
    (Pid::MANUFACTURER_LABEL, "manufacturer-label"),
    (Pid::DEVICE_LABEL, "device-label"),
    (Pid::SOFTWARE_VERSION_LABEL, "software-version-label"),
    (Pid::DMX_PERSONALITY, "dmx-personality"),
    (
        Pid::DMX_PERSONALITY_DESCRIPTION,
        "dmx-personality-description",
    ),
    (Pid::DMX_START_ADDRESS, "dmx-start-address"),
    (Pid::SENSOR_DEFINITION, "sensor-definition"),
    (Pid::SENSOR_VALUE, "sensor-value"),
    (Pid::IDENTIFY_DEVICE, "identify-device"),
    (Pid::RESET_DEVICE, "reset-device"),
];

/// A device ID: `4c55-12345678`.
#[must_use]
pub fn device_id(uid: Uid) -> String {
    format!("{:04x}-{:08x}", uid.manufacturer(), uid.device())
}

/// The UID a device ID names, or `None` if it is not one.
#[must_use]
pub fn uid(device_id: &str) -> Option<Uid> {
    let (manufacturer, device) = device_id.split_once('-')?;
    let lower = |s: &str| {
        s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    };
    if manufacturer.len() != 4 || device.len() != 8 || !lower(manufacturer) || !lower(device) {
        return None;
    }
    let manufacturer = u16::from_str_radix(manufacturer, 16).ok()?;
    let device = u32::from_str_radix(device, 16).ok()?;
    Some(Uid::new(manufacturer, device))
}

/// A parameter ID: the standard name, or `pid-8123`.
#[must_use]
pub fn parameter_id(pid: Pid) -> String {
    PARAMETERS.iter().find(|(p, _)| *p == pid).map_or_else(
        || format!("pid-{:04x}", pid.0),
        |(_, name)| (*name).to_owned(),
    )
}

/// The PID a parameter ID names, or `None` if it is neither a standard
/// name nor `pid-` and four lower-case hexadecimal digits.
#[must_use]
pub fn pid(parameter_id: &str) -> Option<Pid> {
    if let Some(&(pid, _)) = PARAMETERS.iter().find(|(_, name)| *name == parameter_id) {
        return Some(pid);
    }
    let hex = parameter_id.strip_prefix("pid-")?;
    let lower = hex.len() == 4
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    lower
        .then(|| u16::from_str_radix(hex, 16).ok().map(Pid))
        .flatten()
}
