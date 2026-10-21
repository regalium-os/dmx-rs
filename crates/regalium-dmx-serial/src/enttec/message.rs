// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! One ENTTEC DMX USB Pro API message: a start delimiter, a label, a
//! little-endian length, that many octets, and an end delimiter.
//!
//! The same framing carries every request to the widget and every reply
//! from it; the label says which. Widgets from other makers that speak this
//! API (`DMXking`'s, among others) use the same labels.

use super::error::{Error, ErrorKind};
use core::fmt;

/// The first octet of every message.
pub const START: u8 = 0x7E;
/// The last octet of every message.
pub const END: u8 = 0xE7;
/// The most data one message carries: 600 octets, per the API.
pub const MAX_DATA: usize = 600;
/// Octets around the data: start, label, two of length, end.
pub const OVERHEAD: usize = 5;

/// A message label. A newtype over every value, so a vendor's extension
/// label decodes and re-encodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(pub u8);

impl Label {
    /// Get Widget Parameters: firmware, break, mark-after-break, rate.
    pub const GET_PARAMETERS: Self = Self(3);
    /// Set Widget Parameters.
    pub const SET_PARAMETERS: Self = Self(4);
    /// Received DMX Packet: a status octet, then the start code and slots.
    pub const RECEIVED_DMX: Self = Self(5);
    /// Output Only Send DMX Packet: the start code and 24 to 512 slots.
    pub const SEND_DMX: Self = Self(6);
    /// Send RDM Packet (Phase 6b).
    pub const SEND_RDM: Self = Self(7);
    /// Receive DMX on Change: 0 sends every packet received, 1 only changes.
    pub const RECEIVE_ON_CHANGE: Self = Self(8);
    /// Received DMX Change of State.
    pub const CHANGE_OF_STATE: Self = Self(9);
    /// Get Widget Serial Number: four octets of BCD, least significant
    /// first.
    pub const GET_SERIAL: Self = Self(10);
    /// Send RDM Discovery Request (Phase 6b).
    pub const SEND_RDM_DISCOVERY: Self = Self(11);

    /// The label's name in the API, or `None`.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::GET_PARAMETERS => "get-parameters",
            Self::SET_PARAMETERS => "set-parameters",
            Self::RECEIVED_DMX => "received-dmx",
            Self::SEND_DMX => "send-dmx",
            Self::SEND_RDM => "send-rdm",
            Self::RECEIVE_ON_CHANGE => "receive-on-change",
            Self::CHANGE_OF_STATE => "change-of-state",
            Self::GET_SERIAL => "get-serial",
            Self::SEND_RDM_DISCOVERY => "send-rdm-discovery",
            _ => return None,
        })
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "label {}", self.0),
        }
    }
}

/// A message. Borrows its data.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message<'a> {
    /// What the message is.
    pub label: Label,
    /// Its data, at most [`MAX_DATA`] octets.
    pub data: &'a [u8],
}

impl<'a> Message<'a> {
    /// Reads exactly one message from `bytes`, delimiters included.
    /// Zero-copy. For a byte stream, use the [`Framer`](super::Framer).
    ///
    /// # Errors
    ///
    /// When a delimiter is missing, the length is over [`MAX_DATA`], or
    /// `bytes` holds more or fewer octets than the length says.
    pub fn decode(bytes: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some((&start, rest)) = bytes.split_first() else {
            return Err(Error::truncated(bytes, OVERHEAD));
        };
        if start != START {
            return Err(Error::at(ErrorKind::Start, bytes, 0));
        }
        let Some(&[label, lo, hi]) = rest.get(..3) else {
            return Err(Error::truncated(bytes, OVERHEAD));
        };
        let len = usize::from(u16::from_le_bytes([lo, hi]));
        if len > MAX_DATA {
            return Err(Error::at(ErrorKind::TooLong { len }, bytes, 2));
        }
        let need = OVERHEAD + len;
        if bytes.len() < need {
            return Err(Error::truncated(bytes, need));
        }
        if bytes.get(need - 1) != Some(&END) {
            return Err(Error::at(ErrorKind::End, bytes, need - 1));
        }
        if bytes.len() > need {
            return Err(Error::at(ErrorKind::Trailing, bytes, need));
        }
        Ok(Self {
            label: Label(label),
            data: bytes.get(4..need - 1).unwrap_or_default(),
        })
    }

    /// Appends the message, delimiters included. Data past [`MAX_DATA`] is
    /// cut, so the widget never sees a length it must refuse.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let data = self.data.get(..MAX_DATA).unwrap_or(self.data);
        let len = u16::try_from(data.len()).unwrap_or(0);
        out.push(START);
        out.push(self.label.0);
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(data);
        out.push(END);
    }
}
