// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The header every Art-Net packet opens with: the eight-octet ID and a
//! little-endian opcode; and the protocol version most packets carry next.

use super::error::{Error, ErrorKind};
use core::fmt;

/// `Art-Net` and a NUL: the first eight octets of every packet.
pub const ID: [u8; 8] = *b"Art-Net\0";

/// The UDP port Art-Net uses, sending and receiving: 0x1936.
pub const PORT: u16 = 6454;

/// The protocol version this crate sends: 14, Art-Net 4's. Received
/// packets keep whatever version they carried.
pub const VERSION: u16 = 14;

/// The ID and the opcode: where every packet's body begins.
pub(crate) const HEADER: usize = 10;

/// An Art-Net opcode. A newtype over every value, so an opcode this crate
/// does not know still decodes and re-encodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OpCode(pub u16);

impl OpCode {
    /// `OpPoll`: who is out there.
    pub const POLL: Self = Self(0x2000);
    /// `OpPollReply`: a node describing itself.
    pub const POLL_REPLY: Self = Self(0x2100);
    /// `OpDmx` (also `OpOutput`): one universe's levels.
    pub const DMX: Self = Self(0x5000);
    /// `OpSync`: output what you hold, now.
    pub const SYNC: Self = Self(0x5200);
    /// `OpAddress`: reconfigure a node.
    pub const ADDRESS: Self = Self(0x6000);
    /// `OpTodRequest`: send your table of RDM devices.
    pub const TOD_REQUEST: Self = Self(0x8000);
    /// `OpTodData`: a table of RDM devices.
    pub const TOD_DATA: Self = Self(0x8100);
    /// `OpTodControl`: rebuild your table.
    pub const TOD_CONTROL: Self = Self(0x8200);
    /// `OpRdm`: one RDM message.
    pub const RDM: Self = Self(0x8300);

    /// The packet's name in the Art-Net 4 specification, or `None`.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::POLL => "ArtPoll",
            Self::POLL_REPLY => "ArtPollReply",
            Self::DMX => "ArtDmx",
            Self::SYNC => "ArtSync",
            Self::ADDRESS => "ArtAddress",
            Self::TOD_REQUEST => "ArtTodRequest",
            Self::TOD_DATA => "ArtTodData",
            Self::TOD_CONTROL => "ArtTodControl",
            Self::RDM => "ArtRdm",
            _ => return None,
        })
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "opcode {:#06x}", self.0),
        }
    }
}

/// Reads the header: the opcode, if `datagram` is Art-Net.
pub(crate) fn read(datagram: &[u8]) -> Result<OpCode, Error<'_>> {
    let Some(id) = datagram.get(..ID.len()) else {
        return Err(Error::at(ErrorKind::NotArtNet, datagram, 0));
    };
    if id != ID {
        return Err(Error::at(ErrorKind::NotArtNet, datagram, 0));
    }
    let Some(&[lo, hi]) = datagram.get(ID.len()..HEADER) else {
        return Err(Error::truncated(datagram, HEADER));
    };
    Ok(OpCode(u16::from_le_bytes([lo, hi])))
}

/// Writes the header for `opcode`.
pub(crate) fn write(out: &mut Vec<u8>, opcode: OpCode) {
    out.extend_from_slice(&ID);
    out.extend_from_slice(&opcode.0.to_le_bytes());
}

/// Reads a big-endian `u16` at `at`, as Art-Net sends everything but the
/// opcode and the poll reply's port.
pub(crate) fn be16(datagram: &[u8], at: usize) -> Option<u16> {
    let &[hi, lo] = datagram.get(at..at.checked_add(2)?)? else {
        return None;
    };
    Some(u16::from_be_bytes([hi, lo]))
}
