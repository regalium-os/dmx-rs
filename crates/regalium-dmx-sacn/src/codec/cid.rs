// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The CID: the 16-octet UUID naming an E1.31 source for its whole life.

use core::fmt;

/// A component identifier. A source keeps one CID across restarts, so a
/// receiver recognises it; choosing and storing it is the caller's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cid(pub [u8; 16]);

impl Cid {
    /// The CID folded to the 64 bits the merge identifies sources by:
    /// FNV-1a over all sixteen octets, so CIDs differing anywhere fold
    /// apart. Two that fold alike would merge as one source; for distinct
    /// UUIDs that is about a one-in-2^64 chance.
    #[must_use]
    pub fn folded(&self) -> u64 {
        self.0.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &octet| {
            (hash ^ u64::from(octet)).wrapping_mul(0x0100_0000_01b3)
        })
    }
}

/// The UUID's usual form: `8-4-4-4-12` hexadecimal digits.
impl fmt::Display for Cid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                f.write_str("-")?;
            }
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}
