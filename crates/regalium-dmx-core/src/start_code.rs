// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The start code: the first octet of every DMX512 packet, saying what the
//! slots after it mean.
//!
//! Only the null start code carries levels. Every other value is an
//! *alternate* start code (ANSI E1.11), and a receiver that does not know
//! one must ignore the packet rather than drive lights from it — which is
//! why the core keeps the start code beside the slots instead of discarding
//! it, and why [`Universe`](crate::universe::Universe) and
//! [`Merge`](crate::merge::Merge) refuse anything but null.

use core::fmt;

/// A packet's start code. Any octet is representable: an unknown alternate
/// start code is decoded and re-encoded as it came, not refused.
///
/// `Copy`, `Send` and `Sync`: plain data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StartCode(pub u8);

impl StartCode {
    /// The null start code: the slots are levels, 0 to 255, for dimmers,
    /// moving lights and everything else patched to the universe.
    pub const NULL: Self = Self(0x00);

    /// An ASCII text packet, an alternate start code ANSI E1.11 defines, for
    /// diagnostics carried on the line.
    pub const TEXT: Self = Self(0x17);

    /// Remote Device Management (ANSI E1.20): the slots are an RDM message,
    /// decoded by `rdm` from Phase 6b.
    pub const RDM: Self = Self(0xCC);

    /// A System Information Packet, an alternate start code ANSI E1.11
    /// defines, carrying a checksum of the preceding null packet.
    pub const SIP: Self = Self(0xCF);

    /// Whether the slots are levels.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0 == Self::NULL.0
    }

    /// The name this crate gives a start code it knows: `null`, `text`,
    /// `rdm` or `sip`. `None` for any other alternate start code.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        match self.0 {
            0x00 => Some("null"),
            0x17 => Some("text"),
            0xCC => Some("rdm"),
            0xCF => Some("sip"),
            _ => None,
        }
    }
}

impl fmt::Display for StartCode {
    /// The name when known, else `alternate 0xNN`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "alternate 0x{:02x}", self.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StartCode;

    #[test]
    fn only_the_null_start_code_carries_levels() {
        assert!(StartCode::NULL.is_null());
        assert!(!StartCode::RDM.is_null());
        assert!(!StartCode(0x55).is_null());
    }

    #[test]
    fn an_unknown_start_code_displays_its_value() {
        assert_eq!(StartCode::RDM.to_string(), "rdm");
        assert_eq!(StartCode(0x91).to_string(), "alternate 0x91");
    }
}
