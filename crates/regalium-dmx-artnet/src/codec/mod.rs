// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The Art-Net 4 packet codec: pure, lossless, and borrowing.
//!
//! [`Packet::decode`] reads any datagram; an opcode this crate does not
//! know decodes as [`Packet::Other`] with its body, so a capture of newer
//! equipment can be inspected and forwarded unchanged. Decoding then
//! encoding reproduces the datagram byte for byte.
//!
//! No I/O: the socket is [`crate::socket`]'s. Opens no span: `ArtDmx` runs
//! per frame, and the rest are counted on `dmx.transport.packets`.

mod address;
mod dmx;
mod error;
mod opcode;
mod poll;
mod poll_reply;
mod port_address;
mod rdm;
mod reply;
mod sync;
mod text;
mod tod;
mod tod_request;

pub use address::{ArtAddress, LONG_NAME, SHORT_NAME};
pub use dmx::ArtDmx;
pub use error::{Error, ErrorKind};
pub use opcode::{ID, OpCode, PORT, VERSION};
pub use poll::{ArtPoll, DIAGNOSTICS, DIAGNOSTICS_UNICAST, REPLY_ON_CHANGE, TARGETED};
pub use poll_reply::ArtPollReply;
pub use port_address::PortAddress;
pub use rdm::{ArtRdm, RDM_VERSION};
pub use reply::{LEN as REPLY_LEN, REPORT, Reply, ST_CONTROLLER, ST_NODE, STATUS2_PORT_ADDRESS_15};
pub use sync::ArtSync;
pub use text::{name, text};
pub use tod::{ArtTodData, BLOCK, TOD_FULL, TOD_NAK};
pub use tod_request::{ATC_FLUSH, ArtTod};

use opcode::HEADER;

/// One Art-Net datagram, decoded. Borrows it.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Packet<'a> {
    /// `ArtDmx`: levels.
    Dmx(ArtDmx<'a>),
    /// `ArtPoll`: discovery.
    Poll(ArtPoll<'a>),
    /// `ArtPollReply`: a node describing itself.
    PollReply(ArtPollReply<'a>),
    /// `ArtSync`: output now.
    Sync(ArtSync<'a>),
    /// `ArtAddress`: reconfiguration.
    Address(ArtAddress<'a>),
    /// `ArtTodRequest`: send your table of RDM devices.
    TodRequest(ArtTod<'a>),
    /// `ArtTodData`: a table of RDM devices.
    TodData(ArtTodData<'a>),
    /// `ArtTodControl`: rebuild your table.
    TodControl(ArtTod<'a>),
    /// `ArtRdm`: one RDM message.
    Rdm(ArtRdm<'a>),
    /// An opcode this crate does not decode yet, and everything after it.
    Other {
        /// The opcode.
        opcode: OpCode,
        /// The datagram after the opcode.
        body: &'a [u8],
    },
}

impl<'a> Packet<'a> {
    /// Reads `datagram`. Zero-copy: the packet borrows it.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::NotArtNet`] when it is not Art-Net at all; otherwise the
    /// opcode's own error, pointing at the octet that failed.
    pub fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        Ok(match opcode::read(datagram)? {
            OpCode::DMX => Self::Dmx(ArtDmx::decode(datagram)?),
            OpCode::POLL => Self::Poll(ArtPoll::decode(datagram)?),
            OpCode::POLL_REPLY => Self::PollReply(ArtPollReply::decode(datagram)?),
            OpCode::SYNC => Self::Sync(ArtSync::decode(datagram)?),
            OpCode::ADDRESS => Self::Address(ArtAddress::decode(datagram)?),
            OpCode::TOD_REQUEST => Self::TodRequest(ArtTod::decode_request(datagram)?),
            OpCode::TOD_DATA => Self::TodData(ArtTodData::decode(datagram)?),
            OpCode::TOD_CONTROL => Self::TodControl(ArtTod::decode_control(datagram)?),
            OpCode::RDM => Self::Rdm(ArtRdm::decode(datagram)?),
            opcode => Self::Other {
                opcode,
                body: datagram.get(HEADER..).unwrap_or_default(),
            },
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Dmx(p) => p.encode(out),
            Self::Poll(p) => p.encode(out),
            Self::PollReply(p) => p.encode(out),
            Self::Sync(p) => p.encode(out),
            Self::Address(p) => p.encode(out),
            Self::TodRequest(p) => p.encode(out, false),
            Self::TodData(p) => p.encode(out),
            Self::TodControl(p) => p.encode(out, true),
            Self::Rdm(p) => p.encode(out),
            Self::Other { opcode, body } => {
                opcode::write(out, *opcode);
                out.extend_from_slice(body);
            }
        }
    }

    /// The opcode.
    #[must_use]
    pub const fn opcode(&self) -> OpCode {
        match self {
            Self::Dmx(_) => OpCode::DMX,
            Self::Poll(_) => OpCode::POLL,
            Self::PollReply(_) => OpCode::POLL_REPLY,
            Self::Sync(_) => OpCode::SYNC,
            Self::Address(_) => OpCode::ADDRESS,
            Self::TodRequest(_) => OpCode::TOD_REQUEST,
            Self::TodData(_) => OpCode::TOD_DATA,
            Self::TodControl(_) => OpCode::TOD_CONTROL,
            Self::Rdm(_) => OpCode::RDM,
            Self::Other { opcode, .. } => *opcode,
        }
    }
}
