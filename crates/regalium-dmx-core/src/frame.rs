// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A DMX512 packet as octets: a start code, then up to 512 data slots
//! (ANSI E1.11).
//!
//! On the physical line a packet is framed by a break and a mark-after-break,
//! which the serial transport generates and detects; Art-Net and sACN carry
//! the same start code and slots inside a datagram. Either way, what reaches
//! this module is the octets, and this module is the one place they are
//! read and written.
//!
//! # Lossless
//!
//! Decoding then encoding reproduces the input exactly. Every start code is
//! kept, known or not, and a short packet stays short: a receiver that pads
//! a 24-slot packet to 512 has changed what the console sent.
//!
//! # No span
//!
//! Decoding a frame opens no span. It happens up to 44 times a second per
//! universe; frames are counted and timed on the runtime's metrics, which is
//! where a reader looks for a refresh problem.

use crate::address::{Address, SLOTS};
use crate::error::{Error, ErrorKind};
use crate::start_code::StartCode;

/// One packet: its start code and its data slots. Borrows the slots.
///
/// The invariant — at most 512 slots — holds for every `Frame`, because the
/// only ways to make one, [`Frame::decode`] and [`Frame::new`], check it.
///
/// `Copy`, `Send` and `Sync`: a start code and a shared borrow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame<'a> {
    start_code: StartCode,
    slots: &'a [u8],
}

impl<'a> Frame<'a> {
    /// A frame of `slots` under `start_code`, to encode or merge.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::TooLong`] when `slots` is over 512, with `slots` as the
    /// offending bytes.
    pub fn new(start_code: StartCode, slots: &'a [u8]) -> Result<Self, Error<'a>> {
        if slots.len() > SLOTS {
            return Err(Error {
                kind: ErrorKind::TooLong { slots: slots.len() },
                bytes: slots,
                offset: 0,
            });
        }
        Ok(Self { start_code, slots })
    }

    /// A frame of `slots` the caller already holds to 512 or fewer, as a
    /// universe does by construction. Longer input is cut at 512 rather than
    /// trusted, so the invariant holds even if the caller is wrong.
    pub(crate) fn within_limit(start_code: StartCode, slots: &'a [u8]) -> Self {
        Self {
            start_code,
            slots: slots.get(..SLOTS).unwrap_or(slots),
        }
    }

    /// Reads a packet: the first octet is the start code, the rest are slots.
    ///
    /// Zero-copy: the frame borrows `bytes` and lives no longer than it.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::Empty`] for no octets; [`ErrorKind::TooLong`] for more
    /// than 513, with the error's bytes starting at slot 513, the first one
    /// a packet may not have.
    pub fn decode(bytes: &'a [u8]) -> Result<Self, Error<'a>> {
        let Some((&code, slots)) = bytes.split_first() else {
            return Err(Error {
                kind: ErrorKind::Empty,
                bytes,
                offset: 0,
            });
        };
        if let Some(excess) = slots.get(SLOTS..).filter(|rest| !rest.is_empty()) {
            return Err(Error {
                kind: ErrorKind::TooLong { slots: slots.len() },
                bytes: excess,
                offset: 1 + SLOTS,
            });
        }
        Ok(Self {
            start_code: StartCode(code),
            slots,
        })
    }

    /// Appends the packet's octets to `out`: the start code, then every slot.
    ///
    /// Appends rather than returns, so a refresh loop reuses one buffer and
    /// allocates only the first time.
    pub fn encode(&self, out: &mut Vec<u8>) {
        out.reserve(1 + self.slots.len());
        out.push(self.start_code.0);
        out.extend_from_slice(self.slots);
    }

    /// The start code.
    #[must_use]
    pub const fn start_code(&self) -> StartCode {
        self.start_code
    }

    /// The data slots, 0 to 512 of them, as borrowed.
    #[must_use]
    pub const fn slots(&self) -> &'a [u8] {
        self.slots
    }

    /// Octets on the wire: the start code and the slots.
    #[must_use]
    pub const fn octets(&self) -> usize {
        1 + self.slots.len()
    }

    /// The level at `address`, if this is a null start code packet long
    /// enough to reach it. `None` for an alternate start code: its slots are
    /// not levels, and reading one as a level is the bug this prevents.
    #[must_use]
    pub fn level(&self, address: Address) -> Option<u8> {
        if !self.start_code.is_null() {
            return None;
        }
        self.slots.get(address.index()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::address::Address;
    use crate::error::ErrorKind;
    use crate::start_code::StartCode;

    #[test]
    fn a_level_is_read_only_from_a_null_packet() {
        let null = Frame::decode(&[0x00, 10, 20]).ok();
        let rdm = Frame::decode(&[0xCC, 10, 20]).ok();
        let second = Address::new(2).unwrap_or(Address::FIRST);
        assert_eq!(null.and_then(|f| f.level(second)), Some(20));
        assert_eq!(null.and_then(|f| f.level(Address::LAST)), None);
        assert!(rdm.is_some_and(|f| f.level(second).is_none()));
    }

    #[test]
    fn new_refuses_more_than_512_slots() {
        let slots = [0u8; 513];
        let error = Frame::new(StartCode::NULL, &slots).err();
        assert_eq!(
            error.map(|e| e.kind),
            Some(ErrorKind::TooLong { slots: 513 })
        );
    }
}
