// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What an `ArtPollReply` says, as fields to read or to send.
//!
//! The layout, by offset from the start of the datagram, is Art-Net 4's.
//! Fields this driver has no use for yet — the macro and remote keys, the
//! default RDM responder, the refresh rate — are left zero on encode and
//! kept, as octets, by [`ArtPollReply`](super::poll_reply::ArtPollReply).

use super::address::{LONG_NAME, SHORT_NAME};
use super::opcode::{self, OpCode, PORT};
use super::text;
use core::net::Ipv4Addr;

/// Octets in a full Art-Net 4 reply.
pub const LEN: usize = 239;
/// Octets of the node report field.
pub const REPORT: usize = 64;

/// `Style`: a DMX node.
pub const ST_NODE: u8 = 0x00;
/// `Style`: a lighting console or controller, as this driver is.
pub const ST_CONTROLLER: u8 = 0x01;
/// `Status2` bit: the node uses 15-bit Port-Addresses (Art-Net 3 and 4).
pub const STATUS2_PORT_ADDRESS_15: u8 = 0x08;

/// An `ArtPollReply`'s fields. Borrows its names.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reply<'a> {
    /// The node's IPv4 address.
    pub ip: Ipv4Addr,
    /// Its UDP port, always 6454 from a conforming node.
    pub port: u16,
    /// The node's firmware revision.
    pub version_info: u16,
    /// The Net of every port in this reply.
    pub net_switch: u8,
    /// The Sub-Net of every port in this reply.
    pub sub_switch: u8,
    /// The OEM code, from Artistic Licence's registry.
    pub oem: u16,
    /// `Status1`: indicators, RDM capability, boot state.
    pub status1: u8,
    /// The ESTA manufacturer code.
    pub esta: u16,
    /// The short name: its first NUL ends it.
    pub short_name: &'a [u8],
    /// The long name, as above.
    pub long_name: &'a [u8],
    /// The node report: a status code and a counter, as text.
    pub report: &'a [u8],
    /// How many ports this reply describes, 0 to 4.
    pub ports: u16,
    /// Each port's type: input, output, and the protocol it speaks.
    pub port_types: [u8; 4],
    /// Each input port's status.
    pub good_input: [u8; 4],
    /// Each output port's status, including whether it is merging.
    pub good_output: [u8; 4],
    /// Each input port's Universe within the Sub-Net, low four bits.
    pub sw_in: [u8; 4],
    /// Each output port's Universe within the Sub-Net, low four bits.
    pub sw_out: [u8; 4],
    /// `Style`: what kind of equipment, [`ST_NODE`] or [`ST_CONTROLLER`].
    pub style: u8,
    /// The MAC address, most significant octet first.
    pub mac: [u8; 6],
    /// The address of the root device, when one box sends several replies.
    pub bind_ip: Ipv4Addr,
    /// Which of the root device's replies this is, from 1.
    pub bind_index: u8,
    /// `Status2`: web configuration, DHCP, 15-bit addressing.
    pub status2: u8,
}

impl Reply<'_> {
    /// Appends the whole datagram: header, these fields at their offsets,
    /// and zeros for the rest, to [`LEN`] octets.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let start = out.len();
        opcode::write(out, OpCode::POLL_REPLY);
        out.extend_from_slice(&self.ip.octets());
        out.extend_from_slice(&self.port.to_le_bytes());
        out.extend_from_slice(&self.version_info.to_be_bytes());
        out.extend_from_slice(&[self.net_switch, self.sub_switch]);
        out.extend_from_slice(&self.oem.to_be_bytes());
        out.extend_from_slice(&[0, self.status1]);
        out.extend_from_slice(&self.esta.to_le_bytes());
        text::write(out, self.short_name, SHORT_NAME);
        text::write(out, self.long_name, LONG_NAME);
        text::write(out, self.report, REPORT);
        out.extend_from_slice(&self.ports.to_be_bytes());
        for quad in [
            self.port_types,
            self.good_input,
            self.good_output,
            self.sw_in,
            self.sw_out,
        ] {
            out.extend_from_slice(&quad);
        }
        // AcnPriority, SwMacro, SwRemote, three spare.
        out.extend_from_slice(&[0; 6]);
        out.push(self.style);
        out.extend_from_slice(&self.mac);
        out.extend_from_slice(&self.bind_ip.octets());
        out.extend_from_slice(&[self.bind_index, self.status2]);
        out.resize(start + LEN, 0);
    }
}

impl Reply<'static> {
    /// A controller's reply: no ports, 15-bit addressing, on port 6454.
    #[must_use]
    pub const fn controller(ip: Ipv4Addr) -> Self {
        Self {
            ip,
            port: PORT,
            version_info: 0,
            net_switch: 0,
            sub_switch: 0,
            oem: 0,
            status1: 0,
            esta: 0,
            short_name: &[],
            long_name: &[],
            report: &[],
            ports: 0,
            port_types: [0; 4],
            good_input: [0; 4],
            good_output: [0; 4],
            sw_in: [0; 4],
            sw_out: [0; 4],
            style: ST_CONTROLLER,
            mac: [0; 6],
            bind_ip: Ipv4Addr::UNSPECIFIED,
            bind_index: 0,
            status2: STATUS2_PORT_ADDRESS_15,
        }
    }
}
