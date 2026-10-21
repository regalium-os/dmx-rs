// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Streaming ACN, ANSI E1.31: DMX512 over UDP port 5568, multicast.
//!
//! # Layer contract
//!
//! Owns: the E1.31 root, framing and DMP layers; synchronisation and
//! universe-discovery packets ([`codec`]); source priority, sequence
//! numbers and the stream-terminated option; the multicast group per
//! universe; and a socket satisfying the core's transport trait ([`Sacn`]).
//!
//! Must not: merge sources. A source's priority travels to the core's
//! merge with its frame, and the merge chooses, so Art-Net and sACN merge
//! identically.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # Spans
//!
//! None yet: data packets run per frame and are measured by the runtime's
//! frame metrics. `dmx.sacn.discovery` arrives with the probe, where
//! discovery is a call a person waits on.
//!
//! # State
//!
//! Phase 5: the codec, with its fixture corpus; the socket, sending to each
//! universe's multicast group, receiving the universes joined, announcing
//! its universes, and ending them with stream-terminated packets.

pub mod codec;
pub mod sequence;
pub mod socket;

pub use socket::Sacn;
