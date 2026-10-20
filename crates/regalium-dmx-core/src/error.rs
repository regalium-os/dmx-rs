// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why octets did not decode, and why a change was refused.
//!
//! Two types, because they are two situations. An [`Error`] is about bytes
//! that arrived: it borrows them, so a caller logging it logs what the wire
//! carried rather than a description of it. A [`Rejected`] is about a
//! request this process made — a write past slot 512, an RDM packet handed to
//! a merge — and owns everything it says. Both are matched on their kind,
//! never on a message.

use crate::start_code::StartCode;
use core::fmt;

/// What was wrong with octets. Non-exhaustive: RDM adds its own kinds in
/// Phase 6b, and a caller's `match` must not break when it does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// No octets at all: not even a start code.
    Empty,
    /// More data slots than a packet may carry (ANSI E1.11 allows 512).
    TooLong {
        /// Data slots the packet had, after its start code.
        slots: usize,
    },
}

/// A decode failure and the octets it happened at. Borrows the input.
///
/// `Copy`, `Send` and `Sync`; it holds a shared borrow and nothing else.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error<'a> {
    /// What was wrong.
    pub kind: ErrorKind,
    /// The input from the octet that failed to the end of the buffer.
    pub bytes: &'a [u8],
    /// Where `bytes` starts within the buffer decoding was asked of.
    pub offset: usize,
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Empty => f.write_str("empty packet: no start code"),
            ErrorKind::TooLong { slots } => {
                write!(
                    f,
                    "{slots} data slots, more than the 512 a packet may carry"
                )
            }
        }?;
        write!(f, " at offset {}", self.offset)
    }
}

impl core::error::Error for Error<'_> {}

/// A change refused, and why. Nothing was changed when one is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Rejected {
    /// Levels that would run past slot 512, or a length over 512.
    OutOfRange {
        /// The first address asked for, 1 to 512, or 0 for a length.
        start: u16,
        /// How many slots were asked for.
        len: usize,
    },
    /// A packet whose slots are not levels. Driving lights from an RDM
    /// message or a text packet would flash the rig.
    NotLevels(StartCode),
    /// A merge already holding as many sources as it was built to.
    TooManySources {
        /// The limit it was built with.
        limit: usize,
    },
}

impl fmt::Display for Rejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OutOfRange { start, len } => {
                write!(f, "{len} slots from address {start} run past slot 512")
            }
            Self::NotLevels(code) => write!(f, "a {code} packet carries no levels"),
            Self::TooManySources { limit } => {
                write!(f, "the merge already holds its limit of {limit} sources")
            }
        }
    }
}

impl core::error::Error for Rejected {}
