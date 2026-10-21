// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net 4: DMX512 over UDP port 6454.
//!
//! # Layer contract
//!
//! Owns: the Art-Net packet codec ([`codec`]: `ArtDmx`, `ArtPoll`,
//! `ArtPollReply`, `ArtSync`, `ArtAddress`), the 15-bit Port-Address, the
//! sequence rule, node discovery, and a UDP socket satisfying the core's
//! transport trait ([`ArtNet`]).
//!
//! Must not: reimplement a DMX frame or an RDM message. Art-Net carries the
//! core's types; it does not redefine them. The codec performs no I/O; only
//! [`socket`] touches the network.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # Spans
//!
//! None yet. `ArtDmx` runs per frame and is measured by the runtime's frame
//! metrics; `dmx.artnet.poll` arrives when discovery is a call a person
//! waits on, with the probe.
//!
//! # State
//!
//! Phase 4: the codec, with its fixture corpus; the socket, sending to one
//! target, answering polls and recording replies. `ArtRdm` and `ArtTodData`
//! arrive with RDM in Phase 6b.

pub mod codec;
pub mod sequence;
pub mod socket;

pub use socket::ArtNet;
