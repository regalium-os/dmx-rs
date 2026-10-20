// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Slot addresses, 1 to 512, as a console, a patch sheet and a fixture's
//! address switches number them.
//!
//! On the wire a slot has no number: it is the *n*th octet after the start
//! code. The address is a human convention, one-based, and an off-by-one
//! between "slot 1" and "octet 1" is the commonest patching bug there is. So
//! the conversion happens in exactly one place, [`Address::index`], and every
//! other API takes an `Address`, never a bare integer.

use core::fmt;
use core::num::NonZeroU16;

/// How many data slots a DMX512 packet may carry after its start code
/// (ANSI E1.11): the size of a universe.
pub const SLOTS: usize = 512;

/// A slot address, 1 to 512. Construction checks the range, so an `Address`
/// in hand always names a slot that exists.
///
/// `Copy`, `Send` and `Sync`: plain data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(NonZeroU16);

impl Address {
    /// Slot 1, the first after the start code.
    pub const FIRST: Self = Self(NonZeroU16::MIN);

    /// Slot 512, the last a universe has.
    pub const LAST: Self = match NonZeroU16::new(512) {
        Some(n) => Self(n),
        None => Self::FIRST,
    };

    /// The address `n`, or `None` outside 1 to 512.
    #[must_use]
    pub const fn new(n: u16) -> Option<Self> {
        if n as usize > SLOTS {
            return None;
        }
        match NonZeroU16::new(n) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    /// The address as a console shows it, 1 to 512.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    /// The zero-based offset of this slot among a packet's data slots: what
    /// address 1 is to an array. The one place the conversion happens.
    #[must_use]
    pub fn index(self) -> usize {
        usize::from(self.0.get()) - 1
    }

    /// The address `count` slots after this one, or `None` past 512.
    #[must_use]
    pub fn offset(self, count: usize) -> Option<Self> {
        let n = usize::from(self.0.get()).checked_add(count)?;
        Self::new(u16::try_from(n).ok()?)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, SLOTS};

    #[test]
    fn only_one_to_512_are_addresses() {
        assert_eq!(Address::new(0), None);
        assert_eq!(Address::new(1), Some(Address::FIRST));
        assert_eq!(Address::new(512), Some(Address::LAST));
        assert_eq!(Address::new(513), None);
    }

    #[test]
    fn address_1_is_index_0_and_512_is_the_last_index() {
        assert_eq!(Address::FIRST.index(), 0);
        assert_eq!(Address::LAST.index(), SLOTS - 1);
    }

    #[test]
    fn an_offset_past_the_universe_is_none() {
        assert_eq!(Address::FIRST.offset(511), Some(Address::LAST));
        assert_eq!(Address::FIRST.offset(512), None);
        assert_eq!(Address::LAST.offset(usize::MAX), None);
    }
}
