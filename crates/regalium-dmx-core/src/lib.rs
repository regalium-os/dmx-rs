// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The pure DMX protocol core: bytes and a timestamp in, frames and deadlines out.
//!
//! Implements ANSI E1.11 (DMX512-A) and, from Phase 6b, ANSI E1.20 (RDM). It
//! performs no I/O, opens no socket and reads no clock: it is handed octets
//! and the time and returns what should be sent next and when. That is what
//! lets refresh, RDM discovery and timeout scenarios run as table tests on a
//! fake clock, with no hardware and no network.
//!
//! # Layer contract
//!
//! Owns, layering inward:
//!
//! | Module | Owns |
//! | --- | --- |
//! | [`address`] | slot addresses 1 to 512, and the one conversion to an index |
//! | [`start_code`] | the start code, null and alternate |
//! | [`frame`] | E1.11 packets: a start code and up to 512 slots, lossless |
//! | [`universe`] | a universe's levels, sent length and change generation |
//! | [`merge`] | priority, then HTP or LTP, merging sources into one universe |
//! | [`error`] | decode errors that borrow the octets, and refused changes |
//! | [`refresh`] | when to send a universe: the refresh policy as a state machine |
//! | [`transport`] | the trait every transport satisfies |
//! | `rdm` | E1.20 messages: header, checksum, parameter ids, NACK reasons (6b) |
//! | `discovery` | E1.20 binary-search discovery as a state machine (6b) |
//!
//! Must not: perform I/O, hold an operating-system handle, read a clock, log
//! or panic. A malformed frame is an error return, never an abort.
//!
//! Allowed imports: `regalium-dmx-telemetry`.
//!
//! # Decoding is lossless
//!
//! Decoding then re-encoding reproduces the input byte for byte — an unknown
//! start code, a short packet — so equipment that does not follow the
//! standard can be inspected rather than silently normalised.
//!
//! # Spans
//!
//! None for frames, universes or merges: they run per frame, up to 44 times
//! a second per universe, and are measured by the runtime's frame metrics
//! instead. RDM opens `dmx.rdm.{decode,encode}` and `dmx.rdm.discovery` from
//! Phase 6b — work a person waits on.
//!
//! # State
//!
//! Phase 5: [`address`], [`start_code`], [`frame`], [`universe`], [`merge`]
//! with priority, [`error`], [`refresh`] and [`transport`] are built, with
//! the first fixture corpus under `fixtures/`.

pub mod address;
pub mod error;
pub mod frame;
pub mod merge;
pub mod refresh;
pub mod start_code;
pub mod transport;
pub mod universe;
