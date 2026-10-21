// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why a datagram did not decode as E1.31, and where.
//!
//! As the core's frame error: it borrows the datagram from the octet that
//! failed, so what is logged is what the wire carried.

use core::fmt;

/// What was wrong. Non-exhaustive: E1.33 (`RDMnet`) shares the root layer and
/// may add kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The preamble, postamble or ACN packet identifier is wrong: not ACN.
    NotAcn,
    /// The datagram ends before a field the packet requires.
    Truncated {
        /// Octets the packet needs at least.
        need: usize,
    },
    /// A layer's flags are not 0x7, as every E1.31 PDU's are.
    Flags {
        /// The flags nibble.
        flags: u8,
    },
    /// A layer's length disagrees with the octets that follow it.
    Length {
        /// What the length field says.
        said: usize,
        /// What the datagram holds from the layer's start.
        actual: usize,
    },
    /// The DMP layer's fixed fields — vector, address and data type, first
    /// address, increment — are not the values E1.31 requires.
    Dmp,
    /// More than a start code and 512 slots.
    TooLong {
        /// Data slots, after the start code.
        slots: usize,
    },
}

/// A decode failure and the octets it happened at. Borrows the datagram.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error<'a> {
    /// What was wrong.
    pub kind: ErrorKind,
    /// The datagram from the octet that failed to its end.
    pub bytes: &'a [u8],
    /// Where `bytes` starts within the datagram.
    pub offset: usize,
}

impl<'a> Error<'a> {
    pub(crate) fn at(kind: ErrorKind, datagram: &'a [u8], offset: usize) -> Self {
        Self {
            kind,
            bytes: datagram.get(offset..).unwrap_or_default(),
            offset,
        }
    }

    pub(crate) fn truncated(datagram: &'a [u8], need: usize) -> Self {
        Self::at(ErrorKind::Truncated { need }, datagram, datagram.len())
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::NotAcn => f.write_str("not an ACN datagram"),
            ErrorKind::Truncated { need } => write!(f, "truncated: needs {need} octets"),
            ErrorKind::Flags { flags } => write!(f, "PDU flags {flags:#x}, not 0x7"),
            ErrorKind::Length { said, actual } => {
                write!(f, "PDU length {said}, but {actual} octets follow")
            }
            ErrorKind::Dmp => f.write_str("DMP layer fields are not E1.31's"),
            ErrorKind::TooLong { slots } => write!(f, "{slots} slots, over 512"),
        }?;
        write!(f, " at offset {}", self.offset)
    }
}

impl core::error::Error for Error<'_> {}
