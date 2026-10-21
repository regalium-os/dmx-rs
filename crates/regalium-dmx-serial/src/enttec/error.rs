// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why octets did not decode as an ENTTEC API message, and where.

use core::fmt;

/// What was wrong.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The first octet is not the start delimiter, 0x7E.
    Start,
    /// The octet after the data is not the end delimiter, 0xE7.
    End,
    /// The length is over the API's 600.
    TooLong {
        /// The length field.
        len: usize,
    },
    /// The octets end before the message does.
    Truncated {
        /// Octets the message needs.
        need: usize,
    },
    /// Octets after the end delimiter: more than one message.
    Trailing,
}

/// A decode failure and the octets it happened at. Borrows the input.
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
            ErrorKind::Start => f.write_str("no start delimiter 0x7e"),
            ErrorKind::End => f.write_str("no end delimiter 0xe7"),
            ErrorKind::TooLong { len } => write!(f, "length {len} is over 600"),
            ErrorKind::Truncated { need } => write!(f, "truncated: needs {need} octets"),
            ErrorKind::Trailing => f.write_str("octets after the end delimiter"),
        }?;
        write!(f, " at offset {}", self.offset)
    }
}

impl core::error::Error for Error<'_> {}
