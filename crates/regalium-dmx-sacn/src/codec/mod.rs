// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The E1.31 packet codec: pure, lossless, and borrowing.
//!
//! [`Packet::decode`] checks every fixed field and every layer's length,
//! as E1.31 tells a receiver to, and refuses a datagram that fails; one
//! with a vector this crate does not know decodes as [`Packet::Other`].
//! Decoding then encoding reproduces the datagram byte for byte.
//!
//! No I/O: the socket is [`crate::socket`]'s. No span: data packets run per
//! frame, and are measured by the runtime's frame metrics.

mod cid;
mod data;
mod discovery;
mod error;
mod root;
mod sync;
mod text;
mod universe;

pub use cid::Cid;
pub use data::{DataPacket, FORCE_SYNC, PREVIEW, TERMINATED, VECTOR as VECTOR_DATA_PACKET};
pub use discovery::{
    DiscoveryPacket, PAGE, VECTOR as VECTOR_DISCOVERY, VECTOR_LIST as VECTOR_UNIVERSE_LIST, list,
};
pub use error::{Error, ErrorKind};
pub use root::{PREAMBLE, VECTOR_DATA, VECTOR_EXTENDED};
pub use sync::{SyncPacket, VECTOR as VECTOR_SYNC};
pub use text::{NAME, name, text};
pub use universe::{DISCOVERY_UNIVERSE, MAX_UNIVERSE, PORT, multicast};

use root::{FRAMING, be32};

/// One E1.31 datagram, decoded. Borrows it.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Packet<'a> {
    /// Levels, or another start code's data.
    Data(DataPacket<'a>),
    /// Release synchronised data.
    Sync(SyncPacket),
    /// A source's list of universes.
    Discovery(DiscoveryPacket<'a>),
    /// A valid root layer with a vector this crate does not decode.
    Other {
        /// The root vector.
        root: u32,
        /// The framing vector, or 0 if the datagram is too short to have one.
        framing: u32,
        /// The whole datagram.
        datagram: &'a [u8],
    },
}

impl<'a> Packet<'a> {
    /// Reads `datagram`. Zero-copy: the packet borrows it.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::NotAcn`] when it is not ACN at all; otherwise the
    /// layer's own error, pointing at the octet that failed.
    pub fn decode(datagram: &'a [u8]) -> Result<Self, Error<'a>> {
        let (vector, cid) = root::read(datagram)?;
        let framing = be32(datagram, FRAMING + 2).unwrap_or_default();
        Ok(match (vector, framing) {
            (VECTOR_DATA, data::VECTOR) => Self::Data(DataPacket::decode(datagram, cid)?),
            (VECTOR_EXTENDED, sync::VECTOR) => Self::Sync(SyncPacket::decode(datagram, cid)?),
            (VECTOR_EXTENDED, discovery::VECTOR) => {
                Self::Discovery(DiscoveryPacket::decode(datagram, cid)?)
            }
            (root, framing) => Self::Other {
                root,
                framing,
                datagram,
            },
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Data(p) => p.encode(out),
            Self::Sync(p) => p.encode(out),
            Self::Discovery(p) => p.encode(out),
            Self::Other { datagram, .. } => out.extend_from_slice(datagram),
        }
    }
}
