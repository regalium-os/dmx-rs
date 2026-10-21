// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! How sources combine, and who a source is.

/// How slots from several sources combine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Highest takes precedence: each slot is the highest level any source
    /// sends. Right for intensities; wrong for a pan, where it picks a side.
    Htp,
    /// Latest takes precedence: each slot follows the source that last
    /// changed it.
    Ltp,
}

impl Mode {
    /// The name `dmx.merge.mode` carries: `htp` or `ltp`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Htp => "htp",
            Self::Ltp => "ltp",
        }
    }
}

/// Who a source is, as the transport that received it numbers it: an
/// Art-Net sender's address, an sACN CID mapped to an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(pub u64);
