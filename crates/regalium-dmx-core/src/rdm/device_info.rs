// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `DEVICE_INFO`'s 19 octets: what a fixture is and where it is patched.

/// A `DEVICE_INFO` response.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeviceInfo {
    /// The RDM version the device implements: 0x0100 for E1.20.
    pub protocol: u16,
    /// The manufacturer's model id.
    pub model: u16,
    /// What kind of product: fixture, dimmer, and so on.
    pub category: u16,
    /// The firmware's id.
    pub software: u32,
    /// Slots the device uses in its current personality.
    pub footprint: u16,
    /// The current personality, from 1.
    pub personality: u8,
    /// How many personalities it has.
    pub personalities: u8,
    /// Its DMX512 start address, 1 to 512, or 0xFFFF with no footprint.
    pub start_address: u16,
    /// How many sub-devices it has.
    pub sub_devices: u16,
    /// How many sensors it has.
    pub sensors: u8,
}

/// Octets in a `DEVICE_INFO` response.
pub const LEN: usize = 19;

impl DeviceInfo {
    /// Reads a response's data. `None` unless it is exactly 19 octets.
    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let &[
            p0,
            p1,
            m0,
            m1,
            c0,
            c1,
            s0,
            s1,
            s2,
            s3,
            f0,
            f1,
            personality,
            personalities,
            a0,
            a1,
            d0,
            d1,
            sensors,
        ] = data
        else {
            return None;
        };
        Some(Self {
            protocol: u16::from_be_bytes([p0, p1]),
            model: u16::from_be_bytes([m0, m1]),
            category: u16::from_be_bytes([c0, c1]),
            software: u32::from_be_bytes([s0, s1, s2, s3]),
            footprint: u16::from_be_bytes([f0, f1]),
            personality,
            personalities,
            start_address: u16::from_be_bytes([a0, a1]),
            sub_devices: u16::from_be_bytes([d0, d1]),
            sensors,
        })
    }

    /// The response's data.
    #[must_use]
    pub fn encode(&self) -> [u8; LEN] {
        let mut out = [0u8; LEN];
        let fields = [
            &self.protocol.to_be_bytes()[..],
            &self.model.to_be_bytes(),
            &self.category.to_be_bytes(),
            &self.software.to_be_bytes(),
            &self.footprint.to_be_bytes(),
            &[self.personality, self.personalities],
            &self.start_address.to_be_bytes(),
            &self.sub_devices.to_be_bytes(),
            &[self.sensors],
        ];
        for (to, from) in out.iter_mut().zip(fields.into_iter().flatten()) {
            *to = *from;
        }
        out
    }
}
