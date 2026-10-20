// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The RDM series: requests counted per responder and timed per parameter,
//! whether a responder answers, who it is, and what a discovery found.
//!
//! The count keeps the UID because it is what tells a silent fixture from a
//! busy one. The duration drops it: kept per UID and parameter, a rig of 500
//! fixtures polled for ten parameters is fifty thousand histograms, for a
//! latency that differs by parameter and gateway, not by which fixture
//! answered. Which fixture was slow is in its trace.

use super::{UniverseId, select, universe_labels};
use crate::instruments::{
    RDM_DEVICE_INFO, RDM_DEVICE_UP, RDM_DEVICES_DISCOVERED, RDM_REQUEST_DURATION, RDM_REQUESTS,
};
use crate::{Attribute, Key, Meter, keys};

/// The keys `dmx.rdm.device.info` may carry besides the UID, in the order
/// they are emitted: who the responder is (`DEVICE_INFO` and the label
/// PIDs), then how it is patched, then the universe it was found on.
///
/// Anything else handed to [`record_rdm_device_info`] is dropped.
pub const RDM_DEVICE_INFO_KEYS: [Key; 11] = [
    keys::RDM_MANUFACTURER,
    keys::RDM_MANUFACTURER_LABEL,
    keys::RDM_MODEL,
    keys::RDM_MODEL_DESCRIPTION,
    keys::RDM_LABEL,
    keys::RDM_SOFTWARE,
    keys::RDM_FOOTPRINT,
    keys::RDM_START_ADDRESS,
    keys::RDM_PERSONALITY,
    keys::PROTOCOL,
    keys::UNIVERSE,
];

/// Records one RDM request to responder `uid`: its `command` class (`get`,
/// `set`, `discovery`), `pid` by standard name, its `outcome` (`ack`,
/// `nack`, `timeout`, …) and how long it took, retries included, in ms.
///
/// `dmx.rdm.requests` carries command, outcome and UID;
/// `dmx.rdm.request.duration` carries command, parameter and outcome only.
pub fn record_rdm_request(
    meter: &dyn Meter,
    command: &str,
    pid: &str,
    outcome: &str,
    uid: &str,
    elapsed_ms: f64,
) {
    let command = Attribute::str(keys::RDM_COMMAND, command);
    let outcome = Attribute::str(keys::OUTCOME, outcome);
    meter.record(
        &RDM_REQUESTS,
        1.0,
        &[command, outcome, Attribute::str(keys::RDM_UID, uid)],
    );
    meter.record(
        &RDM_REQUEST_DURATION,
        elapsed_ms,
        &[command, Attribute::str(keys::RDM_PID, pid), outcome],
    );
}

/// Records on `dmx.rdm.device.up` whether responder `uid` answers: 1 or 0.
pub fn record_rdm_device_up(meter: &dyn Meter, uid: &str, up: bool) {
    meter.record(
        &RDM_DEVICE_UP,
        f64::from(u8::from(up)),
        &[Attribute::str(keys::RDM_UID, uid)],
    );
}

/// Records `dmx.rdm.device.info`, valued 1: `dmx.rdm.uid`, then whichever of
/// [`RDM_DEVICE_INFO_KEYS`] `identity` carries, in that order, each once.
///
/// Call it whenever the responder is read: a fixture re-addressed or
/// relabelled from the stage is seen to change rather than going stale.
pub fn record_rdm_device_info(meter: &dyn Meter, uid: &str, identity: &[Attribute<'_>]) {
    let (labels, used) = select::<{ 1 + RDM_DEVICE_INFO_KEYS.len() }>(
        &[Attribute::str(keys::RDM_UID, uid)],
        &RDM_DEVICE_INFO_KEYS,
        identity,
    );
    meter.record(&RDM_DEVICE_INFO, 1.0, labels.get(..used).unwrap_or(&[]));
}

/// Records on `dmx.rdm.discovery.devices` how many responders the last RDM
/// discovery of universe `id` found.
pub fn record_rdm_discovered(meter: &dyn Meter, id: &UniverseId<'_>, found: u32) {
    meter.record(
        &RDM_DEVICES_DISCOVERED,
        f64::from(found),
        &universe_labels(id),
    );
}
