// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The E1.31 synchronisation packet: release the data held for a sync
//! address, now.

use super::cid::Cid;
use super::error::Error;
use super::root::{self, FRAMING, VECTOR_EXTENDED, be16};

/// `VECTOR_E131_EXTENDED_SYNCHRONIZATION`.
pub const VECTOR: u32 = 0x0000_0001;

const LEN: usize = FRAMING + 11;

/// An E1.31 synchronisation packet.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SyncPacket {
    /// The source.
    pub cid: Cid,
    /// Numbered per sync address, as data is per universe.
    pub sequence: u8,
    /// The universe number data packets name as their sync address.
    pub sync_address: u16,
    /// Two reserved octets, zero from a conforming source; kept.
    pub reserved: [u8; 2],
}

impl SyncPacket {
    pub(crate) fn decode(d: &[u8], cid: Cid) -> Result<Self, Error<'_>> {
        if d.len() < LEN {
            return Err(Error::truncated(d, LEN));
        }
        root::pdu(d, FRAMING)?;
        let octet = |at: usize| d.get(at).copied().unwrap_or_default();
        Ok(Self {
            cid,
            sequence: octet(FRAMING + 6),
            sync_address: be16(d, FRAMING + 7).unwrap_or_default(),
            reserved: [octet(FRAMING + 9), octet(FRAMING + 10)],
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        root::write(out, VECTOR_EXTENDED, self.cid, LEN);
        root::flags_length(out, LEN - FRAMING);
        out.extend_from_slice(&VECTOR.to_be_bytes());
        out.push(self.sequence);
        out.extend_from_slice(&self.sync_address.to_be_bytes());
        out.extend_from_slice(&self.reserved);
    }
}
