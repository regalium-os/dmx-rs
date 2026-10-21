// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The 15-bit Port-Address: Art-Net 4's universe number.
//!
//! Art-Net splits it three ways: a 7-bit Net, a 4-bit Sub-Net and a 4-bit
//! Universe, so 32 768 universes in all. `ArtDmx` carries it as two octets,
//! `SubUni` (Sub-Net and Universe) then `Net`.

use core::fmt;

/// A Port-Address, 0 to 32 767. Universe 0 is valid in Art-Net, unlike sACN.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortAddress(u16);

impl PortAddress {
    /// The highest Port-Address: Net 127, Sub-Net 15, Universe 15.
    pub const MAX: Self = Self(0x7FFF);

    /// The Port-Address `n`, or `None` above 32 767.
    #[must_use]
    pub const fn new(n: u32) -> Option<Self> {
        if n > Self::MAX.0 as u32 {
            return None;
        }
        #[allow(clippy::cast_possible_truncation, reason = "checked above")]
        Some(Self(n as u16))
    }

    /// From `ArtDmx`'s two octets. Net's top bit is not part of the
    /// address, and is dropped.
    #[must_use]
    pub const fn from_octets(sub_uni: u8, net: u8) -> Self {
        Self(((net as u16 & 0x7F) << 8) | sub_uni as u16)
    }

    /// `ArtDmx`'s two octets: `SubUni`, then `Net`.
    #[must_use]
    pub const fn octets(self) -> [u8; 2] {
        let [net, sub_uni] = self.0.to_be_bytes();
        [sub_uni, net]
    }

    /// The whole address, as the transport trait numbers a universe.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }

    /// The Net: bits 14 to 8.
    #[must_use]
    pub const fn net(self) -> u8 {
        self.octets()[1]
    }

    /// The Sub-Net: bits 7 to 4.
    #[must_use]
    pub const fn sub_net(self) -> u8 {
        self.octets()[0] >> 4
    }

    /// The Universe within the Sub-Net: bits 3 to 0.
    #[must_use]
    pub const fn universe(self) -> u8 {
        self.octets()[0] & 0x0F
    }
}

/// `net:sub-net:universe`, as consoles display it.
impl fmt::Display for PortAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.net(), self.sub_net(), self.universe())
    }
}

#[cfg(test)]
mod tests {
    use super::PortAddress;

    #[test]
    fn a_port_address_splits_into_net_sub_net_and_universe() {
        let p = PortAddress::new(0x1234).unwrap_or(PortAddress::MAX);
        assert_eq!((p.net(), p.sub_net(), p.universe()), (0x12, 3, 4));
        assert_eq!(p.octets(), [0x34, 0x12]);
        assert_eq!(PortAddress::from_octets(0x34, 0x12), p);
        assert_eq!(p.to_string(), "18:3:4");
    }

    #[test]
    fn above_15_bits_is_refused_and_nets_top_bit_ignored() {
        assert_eq!(PortAddress::new(0x8000), None);
        assert_eq!(PortAddress::from_octets(0xFF, 0xFF), PortAddress::MAX);
    }
}
