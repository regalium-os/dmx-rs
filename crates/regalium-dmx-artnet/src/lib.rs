// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net 4: DMX512 and RDM over UDP port 6454.
//!
//! # Layer contract
//!
//! Owns: the Art-Net packet codec (`ArtDmx`, `ArtPoll`/`ArtPollReply`,
//! `ArtSync`, `ArtAddress`, `ArtTodRequest`/`ArtTodData`/`ArtRdm`), the
//! 15-bit port-address, node discovery, and a UDP socket satisfying the
//! core's transport trait.
//!
//! Must not: reimplement a DMX frame or an RDM message. Art-Net carries the
//! core's types; it does not redefine them.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # Spans
//!
//! `dmx.artnet.{decode,encode,poll,send}`.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands in Phase 4.
