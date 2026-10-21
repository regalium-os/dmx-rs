// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An RDM message (ANSI E1.20): 24 octets of header, up to 231 of
//! parameter data, and a 16-bit checksum.
//!
//! The header: start code 0xCC, sub-start code 0x01, the message length
//! (header and data, not the checksum), destination and source UIDs, a
//! transaction number, the port id (in a request) or response type (in a
//! response), a queued message count, the sub-device, the command class,
//! the parameter id, and the data's length. The checksum is the 16-bit sum
//! of every octet before it, start code included, sent high octet first.

use super::codes::{CommandClass, Pid};
use super::error::{Error, ErrorKind};
use super::uid::Uid;

/// The message's sub-start code.
pub const SUB_START: u8 = 0x01;
/// Header octets, start code to data length.
pub const HEADER: usize = 24;
/// The most parameter data one message carries.
pub const MAX_DATA: usize = 231;
/// The longest message: header, data and checksum.
pub const MAX_LEN: usize = HEADER + MAX_DATA + 2;

/// An RDM message. Borrows its parameter data.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message<'a> {
    /// Who it is for.
    pub destination: Uid,
    /// Who sent it.
    pub source: Uid,
    /// Numbered by the controller; a response repeats its request's.
    pub transaction: u8,
    /// The port id in a request, from 1; the response type in a response.
    pub port_or_response: u8,
    /// In a response, how many queued messages the device holds.
    pub message_count: u8,
    /// 0 for the root device, 0xFFFF for every sub-device.
    pub sub_device: u16,
    /// GET, SET, discovery, or a response to one.
    pub command_class: CommandClass,
    /// The parameter.
    pub pid: Pid,
    /// The parameter data, at most [`MAX_DATA`] octets.
    pub data: &'a [u8],
}

/// The 16-bit sum of `octets`.
#[must_use]
pub fn checksum(octets: &[u8]) -> u16 {
    octets
        .iter()
        .fold(0u16, |sum, &b| sum.wrapping_add(u16::from(b)))
}

impl<'a> Message<'a> {
    /// Reads exactly one message, start code to checksum. Zero-copy.
    ///
    /// # Errors
    ///
    /// When the start codes, a length or the checksum are wrong, or
    /// octets are missing or left over.
    pub fn decode(bytes: &'a [u8]) -> Result<Self, Error<'a>> {
        match bytes {
            [0xCC, SUB_START, ..] => {}
            [] | [0xCC] => return Err(Error::truncated(bytes, HEADER + 2)),
            _ => return Err(Error::at(ErrorKind::NotRdm, bytes, 0)),
        }
        let Some(header) = bytes.get(..HEADER) else {
            return Err(Error::truncated(bytes, HEADER + 2));
        };
        let at = |i: usize| header.get(i).copied().unwrap_or_default();
        let said = usize::from(at(2));
        let implied = HEADER + usize::from(at(23));
        if said != implied {
            return Err(Error::at(ErrorKind::Length { said, implied }, bytes, 2));
        }
        let Some(sum) = bytes.get(said..said + 2) else {
            return Err(Error::truncated(bytes, said + 2));
        };
        let said_sum = u16::from_be_bytes([
            sum.first().copied().unwrap_or_default(),
            sum.get(1).copied().unwrap_or_default(),
        ]);
        let computed = checksum(bytes.get(..said).unwrap_or_default());
        if said_sum != computed {
            let kind = ErrorKind::Checksum {
                said: said_sum,
                computed,
            };
            return Err(Error::at(kind, bytes, said));
        }
        if bytes.len() > said + 2 {
            return Err(Error::at(ErrorKind::Trailing, bytes, said + 2));
        }
        Ok(Self {
            destination: Uid::read(header.get(3..).unwrap_or_default()).unwrap_or_default(),
            source: Uid::read(header.get(9..).unwrap_or_default()).unwrap_or_default(),
            transaction: at(15),
            port_or_response: at(16),
            message_count: at(17),
            sub_device: u16::from_be_bytes([at(18), at(19)]),
            command_class: CommandClass(at(20)),
            pid: Pid(u16::from_be_bytes([at(21), at(22)])),
            data: bytes.get(HEADER..said).unwrap_or_default(),
        })
    }

    /// Appends the message, start code to checksum. Data past
    /// [`MAX_DATA`] is cut, so no device sees a length it must refuse.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let data = self.data.get(..MAX_DATA).unwrap_or(self.data);
        let start = out.len();
        let pdl = u8::try_from(data.len()).unwrap_or(0);
        let len = u8::try_from(HEADER + data.len()).unwrap_or(0);
        out.extend_from_slice(&[0xCC, SUB_START, len]);
        out.extend_from_slice(&self.destination.octets());
        out.extend_from_slice(&self.source.octets());
        out.extend_from_slice(&[self.transaction, self.port_or_response, self.message_count]);
        out.extend_from_slice(&self.sub_device.to_be_bytes());
        out.push(self.command_class.0);
        out.extend_from_slice(&self.pid.0.to_be_bytes());
        out.push(pdl);
        out.extend_from_slice(data);
        let sum = checksum(out.get(start..).unwrap_or_default());
        out.extend_from_slice(&sum.to_be_bytes());
    }
}
