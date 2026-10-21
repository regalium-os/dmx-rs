// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtPoll` (`OpPoll`, 0x2000): who is out there.
//!
//! After the header: the protocol version, the `Flags` octet and a
//! diagnostics priority. Art-Net 4 may append a targeted range of
//! Port-Addresses, an ESTA manufacturer and an OEM code; a receiver must
//! accept a poll with or without them, so they are kept as octets and read
//! on request.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, VERSION, be16};
use super::port_address::PortAddress;
use core::ops::RangeInclusive;

const LEN: usize = HEADER + 4;

/// Flag: reply again, unasked, whenever the node's state changes.
pub const REPLY_ON_CHANGE: u8 = 0x02;
/// Flag: send diagnostics messages.
pub const DIAGNOSTICS: u8 = 0x04;
/// Flag: send diagnostics unicast rather than broadcast.
pub const DIAGNOSTICS_UNICAST: u8 = 0x08;
/// Flag: only nodes with a port in the targeted range should reply.
pub const TARGETED: u8 = 0x20;

/// An `ArtPoll` packet. Borrows its Art-Net 4 extension.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtPoll<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// `Flags`: [`REPLY_ON_CHANGE`], [`DIAGNOSTICS`], [`TARGETED`] and
    /// others, kept whole.
    pub flags: u8,
    /// The lowest priority of diagnostics to send.
    pub diagnostics: u8,
    /// Everything after: the targeted range, ESTA and OEM codes, and
    /// anything a later revision adds.
    pub extension: &'a [u8],
}

impl ArtPoll<'_> {
    /// An Art-Net 4 poll asking every node to reply once.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            version: VERSION,
            flags: 0,
            diagnostics: 0,
            extension: &[],
        }
    }
}

impl Default for ArtPoll<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ArtPoll<'a> {
    /// Reads the body of a datagram whose opcode was `OpPoll`.
    pub(crate) fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(&[_, _, flags, diagnostics]) = datagram.get(HEADER..LEN) else {
            return Err(Error::truncated(datagram, LEN));
        };
        Ok(Self {
            version: be16(datagram, HEADER).unwrap_or_default(),
            flags,
            diagnostics,
            extension: datagram.get(LEN..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram, header included.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::POLL);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&[self.flags, self.diagnostics]);
        out.extend_from_slice(self.extension);
    }

    /// The Port-Addresses a targeted poll is for: `None` for an untargeted
    /// one, or one too short to say. Art-Net 4 sends the top first.
    #[must_use]
    pub fn target(&self) -> Option<RangeInclusive<PortAddress>> {
        if self.flags & TARGETED == 0 {
            return None;
        }
        let top = PortAddress::new(be16(self.extension, 0)?.into())?;
        let bottom = PortAddress::new(be16(self.extension, 2)?.into())?;
        Some(bottom..=top)
    }

    /// Whether a node with a port at `port` should answer.
    #[must_use]
    pub fn reaches(&self, port: PortAddress) -> bool {
        self.target().is_none_or(|range| range.contains(&port))
    }
}
