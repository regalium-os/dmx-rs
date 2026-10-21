// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why the sACN transport could not send or receive.

use std::fmt;

/// What the caller must fix. A malformed datagram from the network is not
/// one: it is dropped.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The socket failed, or a multicast group could not be joined.
    Io(std::io::Error),
    /// A universe outside 1 to 63 999.
    Universe(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "sACN socket: {e}"),
            Self::Universe(n) => write!(f, "universe {n} is outside sACN's 1 to 63999"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Universe(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
