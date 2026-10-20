// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The RDM keys (ANSI E1.20): which responder, which parameter, and who the
//! responder says it is.
//!
//! The identity keys are the fields of `DEVICE_INFO` (PID 0x0060, §10.5.1)
//! and the label PIDs beside it. They are labels on one info series per
//! device and nowhere else; see [`crate::shape`].

use crate::Key;

/// A responder's UID (§5.1), as `MMMM:DDDDDDDD`: the ESTA manufacturer id
/// and device id in hex. Unique in the world, and the join key from every RDM
/// series to `dmx_rdm_device_info`.
pub const RDM_UID: Key = Key::declare("dmx.rdm.uid");

/// A request's command class by name: `get`, `set` or `discovery`.
pub const RDM_COMMAND: Key = Key::declare("dmx.rdm.command");

/// A parameter by its standard name, e.g. `DEVICE_INFO`, `DMX_START_ADDRESS`;
/// a manufacturer-specific one as `0x8000` and up, in hex.
pub const RDM_PID: Key = Key::declare("dmx.rdm.pid");

/// Why a responder refused a request, by its ANSI E1.20 name, e.g.
/// `NR_UNKNOWN_PID`, `NR_DATA_OUT_OF_RANGE`.
pub const RDM_NACK_REASON: Key = Key::declare("dmx.rdm.nack_reason");

/// A request's transaction number (§6.2.6): pairs it with its response.
pub const RDM_TRANSACTION: Key = Key::declare("dmx.rdm.transaction");

/// The sub-device a request addressed, 0 for the root device.
pub const RDM_SUB_DEVICE: Key = Key::declare("dmx.rdm.sub_device");

/// Branches a discovery probed with `DISC_UNIQUE_BRANCH`.
pub const RDM_BRANCHES: Key = Key::declare("dmx.rdm.discovery.branches");

/// Branches that answered with a collision, more than one responder at once.
pub const RDM_COLLISIONS: Key = Key::declare("dmx.rdm.discovery.collisions");

/// The ESTA manufacturer id from the responder's UID, as an integer.
pub const RDM_MANUFACTURER: Key = Key::declare("dmx.rdm.manufacturer.id");

/// `MANUFACTURER_LABEL`: the maker's name, as the responder spells it.
pub const RDM_MANUFACTURER_LABEL: Key = Key::declare("dmx.rdm.manufacturer.label");

/// `DEVICE_INFO`'s device model id.
pub const RDM_MODEL: Key = Key::declare("dmx.rdm.model.id");

/// `DEVICE_MODEL_DESCRIPTION`: the model's name.
pub const RDM_MODEL_DESCRIPTION: Key = Key::declare("dmx.rdm.model.description");

/// `DEVICE_LABEL`: the name an installer gave this fixture.
pub const RDM_LABEL: Key = Key::declare("dmx.rdm.device.label");

/// `SOFTWARE_VERSION_LABEL`: the firmware, as text.
pub const RDM_SOFTWARE: Key = Key::declare("dmx.rdm.software");

/// `DEVICE_INFO`'s DMX512 footprint: how many slots the fixture uses.
pub const RDM_FOOTPRINT: Key = Key::declare("dmx.rdm.footprint");

/// `DMX_START_ADDRESS`: the first slot the fixture listens to, 1 to 512.
pub const RDM_START_ADDRESS: Key = Key::declare("dmx.rdm.start_address");

/// `DEVICE_INFO`'s current personality: which of its modes it is in.
pub const RDM_PERSONALITY: Key = Key::declare("dmx.rdm.personality");
