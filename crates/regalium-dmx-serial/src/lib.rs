// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The physical DMX512 line: 250 kbaud, 8N2, a break and mark-after-break
//! ahead of every packet (E1.11 §8).
//!
//! # Layer contract
//!
//! Owns: the ENTTEC DMX USB Pro widget protocol, and a raw RS-485 port that
//! generates the break itself; port discovery; RDM turnaround on the line.
//! Each satisfies the core's transport trait.
//!
//! Must not: link C. Ports are enumerated through sysfs and the platform's
//! own APIs, not libudev; `xtask deps` refuses a native library here.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # Spans
//!
//! `dmx.serial.{open,send,receive}`.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands in Phase 6.
