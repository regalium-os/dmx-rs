// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The boundary an RDM-capable line satisfies: send one request, and
//! collect what comes back in the response window.
//!
//! The window is the line's to time, not the caller's: on a widget the
//! widget's firmware times the turnaround; on Art-Net the node does. The
//! runtime's controller numbers requests, matches answers, retries and
//! follows `ACK_TIMER` and `ACK_OVERFLOW` above this.

use crate::rdm::Uid;
use std::sync::Arc;

/// What the request expects back.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Expect {
    /// Nothing: a broadcast. The line sends and returns at once.
    Nothing,
    /// An RDM message, start code to checksum.
    Message,
    /// A `DISC_UNIQUE_BRANCH` answer: the encoded UID, or a collision.
    Discovery,
}

/// What came back.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Answer {
    /// Nothing within the response window.
    Silent,
    /// This many octets, written to the caller's buffer as received.
    Octets(usize),
}

/// A line that carries RDM.
///
/// # Concurrency
///
/// `&self`, so a controller can share the line with the output loop; one
/// request at a time is the line's to enforce, as the widget transport does.
pub trait RdmLine {
    /// What the caller must fix: as [`Transport::Error`](super::Transport::Error).
    type Error: std::error::Error + Send + Sync + 'static;

    /// The protocol's name as `dmx.protocol` carries it.
    fn protocol(&self) -> &'static str;

    /// The devices on `universe` as the far end discovered them itself — an
    /// Art-Net node's table — or `None` when the controller must search, as
    /// on a line it drives. The default searches.
    ///
    /// # Errors
    ///
    /// When the transport itself failed.
    fn table(&self, universe: u32) -> Result<Option<Vec<Uid>>, Self::Error> {
        let _ = universe;
        Ok(None)
    }

    /// Sends `request` — a whole RDM packet, start code 0xCC to checksum —
    /// on `universe`, and collects what arrives in the window into `buf`
    /// ([`MAX_LEN`](crate::rdm::MAX_LEN) octets suffice).
    ///
    /// # Errors
    ///
    /// When the line itself failed. Silence is [`Answer::Silent`], not an
    /// error.
    fn rdm(
        &self,
        universe: u32,
        request: &[u8],
        expect: Expect,
        buf: &mut [u8],
    ) -> Result<Answer, Self::Error>;
}

impl<T: RdmLine + ?Sized> RdmLine for Arc<T> {
    type Error = T::Error;

    fn protocol(&self) -> &'static str {
        (**self).protocol()
    }

    fn table(&self, universe: u32) -> Result<Option<Vec<Uid>>, Self::Error> {
        (**self).table(universe)
    }

    fn rdm(
        &self,
        universe: u32,
        request: &[u8],
        expect: Expect,
        buf: &mut [u8],
    ) -> Result<Answer, Self::Error> {
        (**self).rdm(universe, request, expect, buf)
    }
}
