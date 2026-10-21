// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtSync` (`OpSync`, 0x5200): output what you hold, now.
//!
//! A controller sending several universes sends one after the batch, so a
//! node in synchronous mode switches them on the same frame. After the
//! header: the protocol version and two auxiliary octets, sent as zero.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, VERSION, be16};

const LEN: usize = HEADER + 4;

/// An `ArtSync` packet. Borrows any trailing octets.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtSync<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// `Aux1` and `Aux2`, zero from a conforming sender.
    pub aux: [u8; 2],
    /// Octets after the packet, which a receiver ignores.
    pub trailing: &'a [u8],
}

impl ArtSync<'_> {
    /// An Art-Net 4 `ArtSync`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            version: VERSION,
            aux: [0, 0],
            trailing: &[],
        }
    }
}

impl Default for ArtSync<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ArtSync<'a> {
    /// Reads the body of a datagram whose opcode was `OpSync`.
    pub(crate) fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(&[_, _, aux1, aux2]) = datagram.get(HEADER..LEN) else {
            return Err(Error::truncated(datagram, LEN));
        };
        Ok(Self {
            version: be16(datagram, HEADER).unwrap_or_default(),
            aux: [aux1, aux2],
            trailing: datagram.get(LEN..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram, header included.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::SYNC);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&self.aux);
        out.extend_from_slice(self.trailing);
    }
}
