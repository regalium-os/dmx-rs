// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Streaming ACN, ANSI E1.31: DMX512 over UDP port 5568, multicast.
//!
//! # Layer contract
//!
//! Owns: the E1.31 root, framing and DMP layers; synchronisation and
//! universe-discovery packets; source priority, sequence numbers and the
//! stream-terminated option; source loss after E1.31 §6.7.1's timeout; and a
//! multicast socket satisfying the core's transport trait.
//!
//! Must not: merge sources. Priority selects a source; combining what is left
//! is the core's `merge`, so Art-Net and sACN merge identically.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # Spans
//!
//! `dmx.sacn.{decode,encode,send,discovery}`.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands in Phase 5.
