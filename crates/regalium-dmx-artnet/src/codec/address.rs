// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtAddress` (`OpAddress`, 0x6000): reconfigure a node.
//!
//! A controller sends one to rename a node or move its ports to other
//! Port-Addresses. Every field has a "leave it alone" value, documented on
//! the field; a value with the top bit set is "program this".
//!
//! Sending one changes a node's configuration, so in this driver it is a
//! write like any other, and goes through the runtime's output policy.

use super::error::Error;
use super::opcode::{self, HEADER, OpCode, be16};
use super::text;

/// Octets of the short name field.
pub const SHORT_NAME: usize = 18;
/// Octets of the long name field.
pub const LONG_NAME: usize = 64;

const LEN: usize = HEADER + 2 + 2 + SHORT_NAME + LONG_NAME + 4 + 4 + 3;

/// An `ArtAddress` packet. Borrows its names.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtAddress<'a> {
    /// The protocol version the sender claims.
    pub version: u16,
    /// The new Net, with bit 7 set; 0x7F leaves it alone.
    pub net_switch: u8,
    /// Which of a node's port groups this is for, from 1; 0 means 1.
    pub bind_index: u8,
    /// The new short name; empty leaves it alone. Exactly 18 octets on
    /// decode; cut and padded to 18 on encode.
    pub short_name: &'a [u8],
    /// The new long name; empty leaves it alone. 64 octets, as above.
    pub long_name: &'a [u8],
    /// Each input port's new Universe, bit 7 set; 0x7F leaves it.
    pub sw_in: [u8; 4],
    /// Each output port's new Universe, bit 7 set; 0x7F leaves it.
    pub sw_out: [u8; 4],
    /// The new Sub-Net, bit 7 set; 0x7F leaves it alone.
    pub sub_switch: u8,
    /// The new sACN priority; 255 leaves it alone.
    pub acn_priority: u8,
    /// A command: merge mode, cancel merge, identify, and others.
    pub command: u8,
    /// Octets after the packet, which a receiver ignores.
    pub trailing: &'a [u8],
}

impl<'a> ArtAddress<'a> {
    /// Reads the body of a datagram whose opcode was `OpAddress`.
    pub(crate) fn decode(d: &'a [u8]) -> Result<Self, Error<'a>> {
        let field = |at: usize, len: usize| d.get(at..at + len).unwrap_or_default();
        let quad = |at: usize| <[u8; 4]>::try_from(field(at, 4)).unwrap_or_default();
        let octet = |at: usize| d.get(at).copied().unwrap_or_default();
        if d.len() < LEN {
            return Err(Error::truncated(d, LEN));
        }
        let names = HEADER + 4;
        let ports = names + SHORT_NAME + LONG_NAME;
        Ok(Self {
            version: be16(d, HEADER).unwrap_or_default(),
            net_switch: octet(HEADER + 2),
            bind_index: octet(HEADER + 3),
            short_name: field(names, SHORT_NAME),
            long_name: field(names + SHORT_NAME, LONG_NAME),
            sw_in: quad(ports),
            sw_out: quad(ports + 4),
            sub_switch: octet(ports + 8),
            acn_priority: octet(ports + 9),
            command: octet(ports + 10),
            trailing: d.get(LEN..).unwrap_or_default(),
        })
    }

    /// Appends the whole datagram, header included.
    pub fn encode(&self, out: &mut Vec<u8>) {
        opcode::write(out, OpCode::ADDRESS);
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&[self.net_switch, self.bind_index]);
        text::write(out, self.short_name, SHORT_NAME);
        text::write(out, self.long_name, LONG_NAME);
        out.extend_from_slice(&self.sw_in);
        out.extend_from_slice(&self.sw_out);
        out.extend_from_slice(&[self.sub_switch, self.acn_priority, self.command]);
        out.extend_from_slice(self.trailing);
    }
}
