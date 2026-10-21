// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why a serial transport could not open, send or receive.

use std::fmt;

/// What the caller must fix.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The port failed, was unplugged, or would not take a write in time.
    Io(std::io::Error),
    /// The port did not answer as an ENTTEC-API widget, so nothing was
    /// sent to it but the probe.
    NotAWidget {
        /// The port.
        path: String,
    },
    /// A universe other than the one a single-port line carries.
    Universe(u32),
    /// Something this transport cannot do yet.
    Unsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "serial port: {e}"),
            Self::NotAWidget { path } => write!(f, "{path} did not answer as a DMX widget"),
            Self::Universe(n) => write!(f, "universe {n}: a serial line carries universe 1"),
            Self::Unsupported(what) => write!(f, "not supported yet: {what}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
