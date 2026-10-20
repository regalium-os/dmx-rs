// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The pure DMX protocol core: bytes and a timestamp in, frames and deadlines out.
//!
//! Implements ANSI E1.11 (DMX512-A) and ANSI E1.20 (RDM). It performs no I/O,
//! opens no socket and reads no clock: it is handed octets and the time and
//! returns what should be sent next and when. That is what lets refresh,
//! RDM discovery and timeout scenarios run as table tests on a fake clock,
//! with no hardware and no network.
//!
//! # Layer contract
//!
//! Owns, layering inward:
//!
//! | Module | Owns |
//! | --- | --- |
//! | `frame` | E1.11 packets: the start code and up to 512 slots, alternate start codes |
//! | `universe` | a universe's 512 slots, their change tracking and refresh deadline |
//! | `merge` | HTP and LTP merging of several sources into one universe |
//! | `rdm` | E1.20 messages: header, checksum, parameter ids, NACK reasons |
//! | `discovery` | E1.20 §7 binary-search discovery, mute and un-mute, as a state machine |
//! | `transport` | the trait every transport satisfies: send a frame, receive one |
//!
//! Must not: perform I/O, hold an operating-system handle, read a clock, log
//! or panic. A malformed frame is an error return, never an abort.
//!
//! Allowed imports: `regalium-dmx-telemetry`.
//!
//! # Decoding is lossless
//!
//! Decoding then re-encoding reproduces the input byte for byte, a bad RDM
//! checksum included, so equipment that does not follow the standard can be
//! inspected rather than silently normalised.
//!
//! # Spans
//!
//! `dmx.frame.{decode,encode}`, `dmx.rdm.{decode,encode}`,
//! `dmx.rdm.discovery`, `dmx.merge.apply`.
//!
//! # State
//!
//! Phase 0: an empty crate. The table above lands from Phase 2.
