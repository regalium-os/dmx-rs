// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The RDM unique id: a 16-bit ESTA manufacturer id and a 32-bit device
//! id, 48 bits in all, sent most significant octet first (ANSI E1.20).

use core::fmt;

/// A UID. Ordered as the 48-bit number it is, which discovery's binary
/// search relies on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Uid(u64);

impl Uid {
    /// Every device: the broadcast UID, `FFFF:FFFFFFFF`.
    pub const ALL: Self = Self(0xFFFF_FFFF_FFFF);
    /// The lowest UID discovery searches.
    pub const MIN: Self = Self(0);
    /// The highest UID a device may have: one below broadcast.
    pub const MAX: Self = Self(0xFFFF_FFFF_FFFE);

    /// The UID of `manufacturer`'s device `device`.
    #[must_use]
    pub const fn new(manufacturer: u16, device: u32) -> Self {
        Self(((manufacturer as u64) << 32) | device as u64)
    }

    /// Every device of `manufacturer`: `mmmm:FFFFFFFF`.
    #[must_use]
    pub const fn all_of(manufacturer: u16) -> Self {
        Self::new(manufacturer, u32::MAX)
    }

    /// The UID whose 48 bits are the low 48 of `n`.
    #[must_use]
    pub const fn from_u64(n: u64) -> Self {
        Self(n & 0xFFFF_FFFF_FFFF)
    }

    /// The 48 bits.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The ESTA manufacturer id.
    #[must_use]
    pub const fn manufacturer(self) -> u16 {
        #[allow(clippy::cast_possible_truncation, reason = "the top 16 of 48 bits")]
        let m = (self.0 >> 32) as u16;
        m
    }

    /// The device id within the manufacturer.
    #[must_use]
    pub const fn device(self) -> u32 {
        #[allow(clippy::cast_possible_truncation, reason = "the low 32 bits")]
        let d = self.0 as u32;
        d
    }

    /// Whether this is a broadcast UID: all devices, or all of one
    /// manufacturer. A broadcast request gets no response.
    #[must_use]
    pub const fn is_broadcast(self) -> bool {
        self.device() == u32::MAX
    }

    /// The six octets, most significant first.
    #[must_use]
    pub const fn octets(self) -> [u8; 6] {
        let [_, _, octets @ ..] = self.0.to_be_bytes();
        octets
    }

    /// From six octets, most significant first.
    #[must_use]
    pub const fn from_octets(octets: [u8; 6]) -> Self {
        let [m0, m1, d0, d1, d2, d3] = octets;
        Self(u64::from_be_bytes([0, 0, m0, m1, d0, d1, d2, d3]))
    }

    /// From the first six octets of `bytes`, if there are six.
    #[must_use]
    pub fn read(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_octets(bytes.get(..6)?.try_into().ok()?))
    }
}

/// `mmmm:dddddddd`, in hexadecimal, as RDM tools print it.
impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}:{:08x}", self.manufacturer(), self.device())
    }
}
