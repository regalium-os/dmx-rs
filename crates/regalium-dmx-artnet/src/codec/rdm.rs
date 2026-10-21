// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtRdm` (0x8300): one RDM message to or from a port.
//!
//! It names its port the way `ArtDmx` does, split: the `Net`, then the low
//! octet of the Port-Address. It carries the RDM message without its start
//! code, from the sub-start code to the checksum.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, VERSION, be16};
use super::port_address::PortAddress;

/// `RdmVer`: E1.20.
pub const RDM_VERSION: u8 = 0x01;

const RDM_DATA: usize = HEADER + 14;

/// An `ArtRdm` packet. Borrows the message.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtRdm<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// `RdmVer`, 0x01 for E1.20.
    pub rdm_version: u8,
    /// `Filler2` and seven spare octets, zero from a conforming sender.
    pub spare: [u8; 8],
    /// The port.
    pub port_address: PortAddress,
    /// `Command`: 0 to process the message.
    pub command: u8,
    /// The RDM message from its sub-start code to its checksum.
    pub message: &'a [u8],
}

impl<'a> ArtRdm<'a> {
    /// The message `rdm` (start code included, as the core encodes it) for
    /// `port_address`.
    #[must_use]
    pub fn new(port_address: PortAddress, rdm: &'a [u8]) -> Self {
        let message = rdm.strip_prefix(&[0xCC]).unwrap_or(rdm);
        Self {
            version: VERSION,
            rdm_version: RDM_VERSION,
            spare: [0; 8],
            port_address,
            command: 0,
            message,
        }
    }

    pub(crate) fn decode(d: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(
            &[
                _,
                _,
                rdm_version,
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                s6,
                s7,
                net,
                command,
                address,
            ],
        ) = d.get(HEADER..RDM_DATA)
        else {
            return Err(Error::truncated(d, RDM_DATA));
        };
        Ok(Self {
            version: be16(d, HEADER).unwrap_or_default(),
            rdm_version,
            spare: [s0, s1, s2, s3, s4, s5, s6, s7],
            port_address: PortAddress::from_octets(address, net),
            command,
            message: d.get(RDM_DATA..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::RDM);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.push(self.rdm_version);
        out.extend_from_slice(&self.spare);
        let [address, net] = self.port_address.octets();
        out.extend_from_slice(&[net, self.command, address]);
        out.extend_from_slice(self.message);
    }

    /// The message as the core decodes it: start code first.
    #[must_use]
    pub fn rdm(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.message.len());
        out.push(0xCC);
        out.extend_from_slice(self.message);
        out
    }
}
