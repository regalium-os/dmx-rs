// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why a datagram did not decode as Art-Net, and where.
//!
//! As the core's frame error: it borrows the datagram from the octet that
//! failed, so what is logged is what the wire carried.

use core::fmt;

/// What was wrong. Non-exhaustive: `ArtRdm` and `ArtTodData` add their own
/// in Phase 6b.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The first eight octets are not `Art-Net` and a NUL: not Art-Net.
    NotArtNet,
    /// The datagram ends before a field the opcode requires.
    Truncated {
        /// Octets the packet needs at least.
        need: usize,
    },
    /// An `ArtDmx` length over the 512 slots a universe holds.
    TooLong {
        /// The length field.
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
    /// `kind` at `offset` within `datagram`.
    pub(crate) fn at(kind: ErrorKind, datagram: &'a [u8], offset: usize) -> Self {
        Self {
            kind,
            bytes: datagram.get(offset..).unwrap_or_default(),
            offset,
        }
    }

    /// Truncated: the datagram is shorter than `need`.
    pub(crate) fn truncated(datagram: &'a [u8], need: usize) -> Self {
        Self::at(ErrorKind::Truncated { need }, datagram, datagram.len())
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::NotArtNet => f.write_str("not an Art-Net datagram"),
            ErrorKind::Truncated { need } => write!(f, "truncated: needs {need} octets"),
            ErrorKind::TooLong { slots } => write!(f, "ArtDmx length {slots} is over 512"),
        }?;
        write!(f, " at offset {}", self.offset)
    }
}

impl core::error::Error for Error<'_> {}
