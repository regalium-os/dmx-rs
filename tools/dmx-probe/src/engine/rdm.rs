// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM as the engine uses it: one object-safe face over the runtime's
//! controller, whatever line and clock it was built with, and the views it
//! fills.

use super::model::Device;
use regalium_dmx::rdm::{DeviceInfo, Pid, Uid};
use regalium_dmx::runtime::Clock;
use regalium_dmx::runtime::rdm::Controller;
use regalium_dmx::transport::RdmLine;

/// What the engine asks of an RDM controller. Errors are the controller's,
/// described.
pub trait RdmPort: Send + Sync {
    /// The UIDs on `universe`, in order.
    ///
    /// # Errors
    ///
    /// When the line failed.
    fn discover(&self, universe: u32) -> Result<Vec<Uid>, String>;

    /// `uid`'s `DEVICE_INFO`.
    ///
    /// # Errors
    ///
    /// When it did not answer, or refused.
    fn info(&self, universe: u32, uid: Uid) -> Result<DeviceInfo, String>;

    /// `uid`'s `DEVICE_LABEL`, or an empty string where it has none.
    fn label(&self, universe: u32, uid: Uid) -> String;

    /// SETs `pid` on `uid` to `data`.
    ///
    /// # Errors
    ///
    /// When it did not answer, or refused.
    fn set(&self, universe: u32, uid: Uid, pid: Pid, data: &[u8]) -> Result<(), String>;
}

impl<L, C> RdmPort for Controller<L, C>
where
    L: RdmLine + Send + Sync,
    C: Clock,
{
    fn discover(&self, universe: u32) -> Result<Vec<Uid>, String> {
        Controller::discover(self, universe).map(|d| d.devices).map_err(|e| e.to_string())
    }

    fn info(&self, universe: u32, uid: Uid) -> Result<DeviceInfo, String> {
        self.device_info(universe, uid).map_err(|e| e.to_string())
    }

    fn label(&self, universe: u32, uid: Uid) -> String {
        self.get(universe, uid, Pid::DEVICE_LABEL).map(|d| String::from_utf8_lossy(&d).into_owned()).unwrap_or_default()
    }

    fn set(&self, universe: u32, uid: Uid, pid: Pid, data: &[u8]) -> Result<(), String> {
        Controller::set(self, universe, uid, pid, data).map(|_| ()).map_err(|e| e.to_string())
    }
}

/// A device's row, from what RDM read.
#[must_use]
pub(crate) fn device(universe: &str, uid: Uid, info: &DeviceInfo, label: String) -> Device {
    Device {
        universe: universe.to_owned(),
        uid: uid.to_string(),
        label,
        model: format!("{:#06x}", info.model),
        address: if (1..=512).contains(&info.start_address) { info.start_address } else { 0 },
        footprint: info.footprint,
        personality: format!("{}/{}", info.personality, info.personalities),
        responding: true,
    }
}
