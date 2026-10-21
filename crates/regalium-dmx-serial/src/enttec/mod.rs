// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The ENTTEC DMX USB Pro widget API: pure, and borrowing.
//!
//! A widget is a microcontroller behind a USB serial bridge. The host sends
//! it messages ([`Message`]) and it generates the DMX512 line itself —
//! break, mark-after-break, refresh — repeating the last packet it was
//! given until it is given another. Replies and received DMX come back as
//! messages on the same stream, framed by a [`Framer`].
//!
//! No I/O here: the port is [`crate::port`]'s.

mod error;
mod framer;
mod message;
mod params;

pub use error::{Error, ErrorKind};
pub use framer::{CAPACITY, Framer};
pub use message::{END, Label, MAX_DATA, Message, OVERHEAD, START};
pub use params::{Params, TICK_NS, serial, serial_bcd};

/// Slots a Send DMX message must carry at least: the API takes 25 to 513
/// octets, start code included, so a shorter universe is padded with dark
/// slots.
pub const MIN_SLOTS: usize = 24;

/// Received DMX status bit: the widget's receive queue overflowed.
pub const QUEUE_OVERFLOW: u8 = 0x01;
/// Received DMX status bit: the line overran the widget's receiver.
pub const OVERRUN: u8 = 0x02;
