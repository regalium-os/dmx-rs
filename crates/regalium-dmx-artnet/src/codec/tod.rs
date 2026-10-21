// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtTodData` (0x8100): a node's table of the RDM devices on one port,
//! which it discovered itself, in blocks of up to 200 UIDs.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, VERSION, be16};
use super::port_address::PortAddress;
use super::rdm::RDM_VERSION;
use regalium_dmx_core::rdm::Uid;

/// `CommandResponse`: this is the full table.
pub const TOD_FULL: u8 = 0x00;
/// `CommandResponse`: the node could not provide one.
pub const TOD_NAK: u8 = 0xFF;
/// UIDs one block carries at most.
pub const BLOCK: usize = 200;

const UIDS: usize = HEADER + 18;

/// An `ArtTodData` packet. Borrows its UIDs.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtTodData<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// `RdmVer`, 0x01 for E1.20.
    pub rdm_version: u8,
    /// The physical port, 1 to 4.
    pub port: u8,
    /// Six spare octets; kept.
    pub spare: [u8; 6],
    /// Which of the node's replies this port is in, from 1.
    pub bind_index: u8,
    /// [`TOD_FULL`] or [`TOD_NAK`].
    pub response: u8,
    /// The port.
    pub port_address: PortAddress,
    /// UIDs in the whole table, across blocks.
    pub total: u16,
    /// This block, from 0.
    pub block: u8,
    /// The UIDs, six octets each; read them with [`ArtTodData::uids`].
    pub list: &'a [u8],
    /// Octets after the UIDs; kept.
    pub trailing: &'a [u8],
}

impl<'a> ArtTodData<'a> {
    /// A full table for `port_address`, in one block: at most [`BLOCK`]
    /// UIDs are listed.
    #[must_use]
    pub fn new(port_address: PortAddress, uids: &[Uid], list: &'a mut Vec<u8>) -> Self {
        list.clear();
        for uid in uids.iter().take(BLOCK) {
            list.extend_from_slice(&uid.octets());
        }
        Self {
            version: VERSION,
            rdm_version: RDM_VERSION,
            port: 1,
            spare: [0; 6],
            bind_index: 1,
            response: TOD_FULL,
            port_address,
            total: u16::try_from(uids.len()).unwrap_or(u16::MAX),
            block: 0,
            list,
            trailing: &[],
        }
    }

    pub(crate) fn decode(d: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(
            &[
                _,
                _,
                rdm_version,
                port,
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                bind_index,
                net,
                response,
                address,
                _,
                _,
                block,
                count,
            ],
        ) = d.get(HEADER..UIDS)
        else {
            return Err(Error::truncated(d, UIDS));
        };
        let end = UIDS + 6 * usize::from(count);
        let Some(list) = d.get(UIDS..end) else {
            return Err(Error::truncated(d, end));
        };
        Ok(Self {
            version: be16(d, HEADER).unwrap_or_default(),
            rdm_version,
            port,
            spare: [s0, s1, s2, s3, s4, s5],
            bind_index,
            response,
            port_address: PortAddress::from_octets(address, net),
            total: be16(d, HEADER + 14).unwrap_or_default(),
            block,
            list,
            trailing: d.get(end..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::TOD_DATA);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&[self.rdm_version, self.port]);
        out.extend_from_slice(&self.spare);
        out.push(self.bind_index);
        let [address, net] = self.port_address.octets();
        out.extend_from_slice(&[net, self.response, address]);
        out.extend_from_slice(&self.total.to_be_bytes());
        out.push(self.block);
        out.push(u8::try_from(self.list.len() / 6).unwrap_or(0));
        out.extend_from_slice(self.list);
        out.extend_from_slice(self.trailing);
    }

    /// The UIDs in this block.
    pub fn uids(&self) -> impl Iterator<Item = Uid> + 'a {
        self.list.chunks(6).filter_map(Uid::read)
    }
}
