// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why octets did not decode as RDM, and where. Borrows them, as the
//! frame error does.

use core::fmt;

/// What was wrong.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Not start code 0xCC and sub-start code 0x01.
    NotRdm,
    /// The octets end before the message, or the response, does.
    Truncated {
        /// Octets needed.
        need: usize,
    },
    /// The message length is under 24, or disagrees with the data length.
    Length {
        /// The message length field.
        said: usize,
        /// What the data length implies.
        implied: usize,
    },
    /// The checksum is not the sum of the octets before it.
    Checksum {
        /// The checksum sent.
        said: u16,
        /// The sum of what arrived.
        computed: u16,
    },
    /// Octets after the checksum.
    Trailing,
    /// A discovery response without its separator, or with an octet that
    /// is not a valid encoding: two devices answering at once, usually.
    Encoding,
}

/// A decode failure and the octets it happened at.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error<'a> {
    /// What was wrong.
    pub kind: ErrorKind,
    /// The input from the octet that failed to its end.
    pub bytes: &'a [u8],
    /// Where `bytes` starts within the input.
    pub offset: usize,
}

impl<'a> Error<'a> {
    pub(crate) fn at(kind: ErrorKind, input: &'a [u8], offset: usize) -> Self {
        Self {
            kind,
            bytes: input.get(offset..).unwrap_or_default(),
            offset,
        }
    }

    pub(crate) fn truncated(input: &'a [u8], need: usize) -> Self {
        Self::at(ErrorKind::Truncated { need }, input, input.len())
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::NotRdm => f.write_str("not RDM: start codes are not 0xcc 0x01"),
            ErrorKind::Truncated { need } => write!(f, "truncated: needs {need} octets"),
            ErrorKind::Length { said, implied } => {
                write!(f, "message length {said}, but the data implies {implied}")
            }
            ErrorKind::Checksum { said, computed } => {
                write!(
                    f,
                    "checksum {said:#06x}, but the octets sum to {computed:#06x}"
                )
            }
            ErrorKind::Trailing => f.write_str("octets after the checksum"),
            ErrorKind::Encoding => f.write_str("not a discovery response: a collision"),
        }?;
        write!(f, " at offset {}", self.offset)
    }
}

impl core::error::Error for Error<'_> {}
