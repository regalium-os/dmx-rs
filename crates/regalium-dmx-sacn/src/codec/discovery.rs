// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! E1.31 universe discovery: a source listing the universes it sends, in
//! pages of up to 512, on the discovery universe every ten seconds.

use super::cid::Cid;
use super::error::{Error, ErrorKind};
use super::root::{self, FRAMING, VECTOR_EXTENDED, be16};
use super::text::{self, NAME};

/// `VECTOR_E131_EXTENDED_DISCOVERY`.
pub const VECTOR: u32 = 0x0000_0002;
/// `VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST`.
pub const VECTOR_LIST: u32 = 0x0000_0001;
/// Universes one page may list.
pub const PAGE: usize = 512;

const NAME_AT: usize = FRAMING + 6;
const LIST: usize = NAME_AT + NAME + 4;
const UNIVERSES: usize = LIST + 8;

/// A universe discovery packet. Borrows the name and the list.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiscoveryPacket<'a> {
    /// The source.
    pub cid: Cid,
    /// The source's name, as in its data packets.
    pub source_name: &'a [u8],
    /// Four reserved octets; kept.
    pub reserved: [u8; 4],
    /// This page, from 0.
    pub page: u8,
    /// The last page of this source's list.
    pub last_page: u8,
    /// The universes, two octets each, big-endian, ascending; read them
    /// with [`DiscoveryPacket::universes`].
    pub list: &'a [u8],
}

impl<'a> DiscoveryPacket<'a> {
    pub(crate) fn decode(d: &'a [u8], cid: Cid) -> Result<Self, Error<'a>> {
        if d.len() < UNIVERSES {
            return Err(Error::truncated(d, UNIVERSES));
        }
        root::pdu(d, FRAMING)?;
        root::pdu(d, LIST)?;
        if root::be32(d, LIST + 2) != Some(VECTOR_LIST) {
            return Err(Error::at(ErrorKind::Dmp, d, LIST + 2));
        }
        let list = d.get(UNIVERSES..).unwrap_or_default();
        if list.len() % 2 != 0 || list.len() > 2 * PAGE {
            let kind = ErrorKind::Length {
                said: d.len() - LIST,
                actual: list.len(),
            };
            return Err(Error::at(kind, d, UNIVERSES));
        }
        let octet = |at: usize| d.get(at).copied().unwrap_or_default();
        let r = NAME_AT + NAME;
        Ok(Self {
            cid,
            source_name: d.get(NAME_AT..r).unwrap_or_default(),
            reserved: [octet(r), octet(r + 1), octet(r + 2), octet(r + 3)],
            page: octet(LIST + 6),
            last_page: octet(LIST + 7),
            list,
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let list = self.list.get(..2 * PAGE).unwrap_or(self.list);
        let total = UNIVERSES + list.len();
        root::write(out, VECTOR_EXTENDED, self.cid, total);
        root::flags_length(out, total - FRAMING);
        out.extend_from_slice(&VECTOR.to_be_bytes());
        text::write(out, self.source_name);
        out.extend_from_slice(&self.reserved);
        root::flags_length(out, total - LIST);
        out.extend_from_slice(&VECTOR_LIST.to_be_bytes());
        out.extend_from_slice(&[self.page, self.last_page]);
        out.extend_from_slice(list);
    }

    /// The universes listed.
    pub fn universes(&self) -> impl Iterator<Item = u16> + 'a {
        let list = self.list;
        (0..list.len() / 2).filter_map(move |i| be16(list, 2 * i))
    }
}

/// `universes` as a discovery list: two octets each, big-endian. The caller
/// sorts them and pages them by [`PAGE`].
pub fn list(universes: &[u16], out: &mut Vec<u8>) {
    for universe in universes {
        out.extend_from_slice(&universe.to_be_bytes());
    }
}
