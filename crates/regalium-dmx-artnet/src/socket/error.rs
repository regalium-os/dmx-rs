// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why the Art-Net transport could not send or receive.

use regalium_dmx_core::start_code::StartCode;
use std::fmt;

/// What the caller must fix. A malformed datagram from the network is not
/// one: it is dropped, since a node cannot be made to stop sending it.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The socket failed.
    Io(std::io::Error),
    /// A universe above Art-Net's 15-bit Port-Address range.
    PortAddress(u32),
    /// A frame that `ArtDmx` cannot carry: only null start code data goes
    /// out as levels, and RDM has its own packets (Phase 6b).
    StartCode(StartCode),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Art-Net socket: {e}"),
            Self::PortAddress(n) => write!(f, "universe {n} is over Art-Net's 32767"),
            Self::StartCode(code) => write!(f, "ArtDmx cannot carry start code {code}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::PortAddress(_) | Self::StartCode(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
