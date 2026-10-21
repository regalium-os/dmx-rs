// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtDmx` (`OpDmx`, 0x5000): one universe's levels.
//!
//! After the header: the protocol version, a sequence number, the physical
//! input it came from, the Port-Address as `SubUni` then `Net`, a big-endian
//! length, and that many slots. The start code is not carried: `ArtDmx` is
//! always null start code data.

use super::error::{Error, ErrorKind};
use super::opcode::{self, HEADER, OpCode, VERSION, be16};
use super::port_address::PortAddress;
use regalium_dmx_core::address::SLOTS;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;

/// Octets before the first slot.
const DATA: usize = HEADER + 8;

/// An `ArtDmx` packet. Borrows the slots, and any octets after them.
///
/// Lossless: it keeps the `Net` octet's unused top bit, an odd length and
/// trailing octets, all of which Art-Net 4 says a sender should not send and
/// some do.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtDmx<'a> {
    version: u16,
    sequence: u8,
    physical: u8,
    sub_uni: u8,
    net: u8,
    slots: &'a [u8],
    trailing: &'a [u8],
}

impl<'a> ArtDmx<'a> {
    /// `slots` for `port_address`, Art-Net 4, sequence `sequence` (0 turns
    /// a receiver's reordering off). `slots` is cut at 512.
    #[must_use]
    pub fn new(port_address: PortAddress, sequence: u8, slots: &'a [u8]) -> Self {
        let [sub_uni, net] = port_address.octets();
        Self {
            version: VERSION,
            sequence,
            physical: 0,
            sub_uni,
            net,
            slots: slots.get(..SLOTS).unwrap_or(slots),
            trailing: &[],
        }
    }

    /// Reads the body of a datagram whose opcode was `OpDmx`.
    pub(crate) fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some(&[_, _, sequence, physical, sub_uni, net, _, _]) = datagram.get(HEADER..DATA)
        else {
            return Err(Error::truncated(datagram, DATA));
        };
        let len = usize::from(be16(datagram, DATA - 2).unwrap_or_default());
        if len > SLOTS {
            return Err(Error::at(
                ErrorKind::TooLong { slots: len },
                datagram,
                DATA - 2,
            ));
        }
        let Some(slots) = datagram.get(DATA..DATA + len) else {
            return Err(Error::truncated(datagram, DATA + len));
        };
        Ok(Self {
            version: be16(datagram, HEADER).unwrap_or_default(),
            sequence,
            physical,
            sub_uni,
            net,
            slots,
            trailing: datagram.get(DATA + len..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram, header included.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::DMX);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&[self.sequence, self.physical, self.sub_uni, self.net]);
        let len = u16::try_from(self.slots.len()).unwrap_or(u16::MAX);
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(self.slots);
        out.extend_from_slice(self.trailing);
    }

    /// The protocol version the sender claims.
    #[must_use]
    pub const fn version(&self) -> u16 {
        self.version
    }

    /// The sequence number, 1 to 255, or 0 when the sender does not order.
    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }

    /// The sender's physical input, for information only.
    #[must_use]
    pub const fn physical(&self) -> u8 {
        self.physical
    }

    /// The universe.
    #[must_use]
    pub const fn port_address(&self) -> PortAddress {
        PortAddress::from_octets(self.sub_uni, self.net)
    }

    /// The levels, as borrowed.
    #[must_use]
    pub const fn slots(&self) -> &'a [u8] {
        self.slots
    }

    /// Octets after the slots, which a receiver ignores.
    #[must_use]
    pub const fn trailing(&self) -> &'a [u8] {
        self.trailing
    }

    /// The levels as the core's frame: a null start code and the slots.
    #[must_use]
    pub fn frame(&self) -> Frame<'a> {
        Frame::within_limit(StartCode::NULL, self.slots)
    }
}
