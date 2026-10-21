// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The boundary every transport satisfies: send one universe's frame, or
//! wait a bounded time for one to arrive.
//!
//! The core declares it because the core is the consumer: the runtime drives
//! the refresh state machine, which says what to send and how long to wait,
//! and a transport is anything that can do those two things. Art-Net, sACN
//! and the serial line each implement it in their own crate, and a vendor's
//! protocol implements it outside this repository without this one being
//! involved.
//!
//! # What crosses it
//!
//! A [`Frame`]: a start code and its slots. A transport adds its own header
//! on the way out and strips it on the way in, so the runtime never sees an
//! Art-Net opcode or an sACN root layer, and the core's frame codec is the
//! only one there is.
//!
//! # Why blocking with a timeout
//!
//! As in bacnet-rs: [`Transport::receive`] takes the time left until the next
//! deadline and gives up when it passes. That needs no executor, so a
//! transport is plain `std`, and each output path keeps its own thread and
//! timing rather than sharing an executor with whatever else the caller runs.

use crate::frame::Frame;
use crate::refresh::Refresh;
use core::time::Duration;

mod received;
mod shared;

pub use received::Received;

/// How long a merge source may be silent before it is dropped, unless a
/// transport says otherwise: the ten seconds after which Art-Net 4 cancels
/// a merge. sACN's network data loss timeout is shorter, and its transport
/// overrides this.
pub const SOURCE_TIMEOUT: Duration = Duration::from_secs(10);

/// A transport: one way of carrying DMX512 frames.
///
/// # Concurrency
///
/// Methods take `&self` so one thread can receive while another sends; an
/// implementation says in its own docs whether that is safe for it.
pub trait Transport {
    /// What the caller must fix when the transport cannot work: a closed
    /// socket, a widget unplugged. Never a quiet receiver; DMX has no
    /// acknowledgement, so a send that left this process succeeded.
    type Error: std::error::Error + Send + Sync + 'static;

    /// The protocol's name as `dmx.protocol` carries it: `artnet`, `sacn`
    /// or `serial`.
    fn protocol(&self) -> &'static str;

    /// How this transport's receivers expect to be refreshed. A patch may
    /// override it.
    fn refresh(&self) -> Refresh {
        Refresh::default()
    }

    /// How long a source may be silent before a merge drops it.
    fn source_timeout(&self) -> Duration {
        SOURCE_TIMEOUT
    }

    /// Starts receiving `universe`, as sACN joins its multicast group. The
    /// runtime calls it when an input listens to the universe; a transport
    /// that receives everything anyway leaves the default, which does
    /// nothing.
    ///
    /// # Errors
    ///
    /// When the transport could not arrange to receive it.
    fn join(&self, universe: u32) -> Result<(), Self::Error> {
        let _ = universe;
        Ok(())
    }

    /// Stops receiving `universe`; the opposite of [`Transport::join`].
    ///
    /// # Errors
    ///
    /// When the transport itself failed.
    fn leave(&self, universe: u32) -> Result<(), Self::Error> {
        let _ = universe;
        Ok(())
    }

    /// Says `universe` will not be sent again, as sACN's stream-terminated
    /// packets do, so receivers drop this source at once rather than after
    /// their timeout. The runtime calls it on unpatch and on close.
    ///
    /// # Errors
    ///
    /// When the transport itself failed.
    fn end(&self, universe: u32) -> Result<(), Self::Error> {
        let _ = universe;
        Ok(())
    }

    /// Sends `frame` as universe `universe`, numbered as this protocol
    /// numbers universes.
    ///
    /// # Errors
    ///
    /// When the transport itself failed.
    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Self::Error>;

    /// Waits at most `timeout` for the next frame, copying its start code and
    /// slots into `buf` (513 octets always suffice). `Ok(None)` when the time
    /// passed with nothing to deliver. Protocol housekeeping — a poll reply,
    /// a discovery packet — happens inside the wait and is never returned.
    ///
    /// # Errors
    ///
    /// When the transport itself failed.
    fn receive(&self, buf: &mut [u8], timeout: Duration) -> Result<Option<Received>, Self::Error>;
}
