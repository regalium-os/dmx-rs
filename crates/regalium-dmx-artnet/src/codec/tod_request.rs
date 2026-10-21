// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtTodRequest` (0x8000) and `ArtTodControl` (0x8200): ask a node for
//! the table of RDM devices it discovered itself, or to rebuild it.
//!
//! Each names its ports as `ArtDmx` does: the `Net`, then the low octet of
//! each Port-Address.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, VERSION, be16};
use super::port_address::PortAddress;

/// `ArtTodControl` command: flush the table and discover again.
pub const ATC_FLUSH: u8 = 0x01;

const TOD_REQUEST: usize = HEADER + 14;
const TOD_CONTROL: usize = HEADER + 14;

/// An `ArtTodRequest` or `ArtTodControl`: which ports, and what to do.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtTod<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// Two filler and seven spare octets, zero from a conforming sender.
    pub spare: [u8; 9],
    /// The ports' Net.
    pub net: u8,
    /// The command: 0 for the full table, or [`ATC_FLUSH`].
    pub command: u8,
    /// Each port's low Port-Address octet: up to 32 for a request, one
    /// for control.
    pub addresses: &'a [u8],
    /// Octets after the addresses: a request padded to the full 32.
    pub trailing: &'a [u8],
}

impl<'a> ArtTod<'a> {
    /// A request for the table of the port at `port_address`.
    #[must_use]
    pub const fn request(port_address: &'a PortAddress, low: &'a [u8; 1]) -> Self {
        Self {
            version: VERSION,
            spare: [0; 9],
            net: port_address.net(),
            command: 0,
            addresses: low,
            trailing: &[],
        }
    }

    /// Reads an `ArtTodRequest`: the count, then that many addresses.
    pub(crate) fn decode_request(d: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(&[net, command, count]) = d.get(TOD_REQUEST - 3..TOD_REQUEST) else {
            return Err(Error::truncated(d, TOD_REQUEST));
        };
        let end = TOD_REQUEST + usize::from(count);
        let Some(addresses) = d.get(TOD_REQUEST..end) else {
            return Err(Error::truncated(d, end));
        };
        Ok(Self {
            version: be16(d, HEADER).unwrap_or_default(),
            spare: spare(d),
            net,
            command,
            addresses,
            trailing: d.get(end..).unwrap_or_default(),
        })
    }

    /// Reads an `ArtTodControl`: one address.
    pub(crate) fn decode_control(d: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(&[net, command, _]) = d.get(TOD_CONTROL - 3..TOD_CONTROL) else {
            return Err(Error::truncated(d, TOD_CONTROL));
        };
        Ok(Self {
            version: be16(d, HEADER).unwrap_or_default(),
            spare: spare(d),
            net,
            command,
            addresses: d.get(TOD_CONTROL - 1..TOD_CONTROL).unwrap_or_default(),
            trailing: d.get(TOD_CONTROL..).unwrap_or_default(),
        })
    }

    /// Appends an `ArtTodRequest`, or with `control` an `ArtTodControl`.
    pub fn encode(&self, out: &mut Vec<u8>, control: bool) {
        opcode::write(
            out,
            if control {
                OpCode::TOD_CONTROL
            } else {
                OpCode::TOD_REQUEST
            },
        );
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&self.spare);
        out.extend_from_slice(&[self.net, self.command]);
        if !control {
            out.push(u8::try_from(self.addresses.len()).unwrap_or(0));
        }
        out.extend_from_slice(self.addresses);
        out.extend_from_slice(self.trailing);
    }
}

fn spare(d: &[u8]) -> [u8; 9] {
    let mut out = [0u8; 9];
    for (to, from) in out.iter_mut().zip(d.get(HEADER + 2..).unwrap_or_default()) {
        *to = *from;
    }
    out
}
