// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtPollReply` (`OpPollReply`, 0x2100): a node describing itself.
//!
//! Unlike every other packet, it carries no protocol version, and its port
//! and ESTA code are little-endian. Replies from older equipment are
//! shorter than Art-Net 4's 239 octets, and a receiver must accept them, so
//! only the address and port are required; a field past the end reads as
//! zero.

use super::error::Error;
use super::opcode::{HEADER, be16};
use super::reply::Reply;
use core::net::Ipv4Addr;

/// The address and the port: what a reply must carry to be any use.
const MIN: usize = HEADER + 6;

/// An `ArtPollReply` as received. Borrows the whole datagram, so it is
/// lossless whatever revision sent it.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtPollReply<'a> {
    datagram: &'a [u8],
}

impl<'a> ArtPollReply<'a> {
    /// Reads a datagram whose opcode was `OpPollReply`.
    pub(crate) fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        if datagram.len() < MIN {
            return Err(Error::truncated(datagram, MIN));
        }
        Ok(Self { datagram })
    }

    /// Appends the datagram as it arrived.
    pub fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.datagram);
    }

    /// Octets received; 239 from an Art-Net 4 node.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.datagram.len()
    }

    /// Never: a reply has at least its address and port.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.datagram.is_empty()
    }

    /// What the reply says. Borrows the names from the datagram.
    #[must_use]
    pub fn fields(&self) -> Reply<'a> {
        let d = self.datagram;
        let field = |at: usize, len: usize| d.get(at..at + len).unwrap_or_default();
        let octet = |at: usize| d.get(at).copied().unwrap_or_default();
        let quad = |at: usize| <[u8; 4]>::try_from(field(at, 4)).unwrap_or_default();
        let le16 = |at: usize| u16::from_le_bytes([octet(at), octet(at + 1)]);
        let be = |at: usize| be16(d, at).unwrap_or_default();
        Reply {
            ip: Ipv4Addr::from(quad(10)),
            port: le16(14),
            version_info: be(16),
            net_switch: octet(18),
            sub_switch: octet(19),
            oem: be(20),
            status1: octet(23),
            esta: le16(24),
            short_name: field(26, 18),
            long_name: field(44, 64),
            report: field(108, 64),
            ports: be(172),
            port_types: quad(174),
            good_input: quad(178),
            good_output: quad(182),
            sw_in: quad(186),
            sw_out: quad(190),
            style: octet(200),
            mac: <[u8; 6]>::try_from(field(201, 6)).unwrap_or_default(),
            bind_ip: Ipv4Addr::from(quad(207)),
            bind_index: octet(211),
            status2: octet(212),
        }
    }
}
