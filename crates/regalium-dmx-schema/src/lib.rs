// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The protobuf model: Rust types generated from `protobuf/`, and the
//! conversions between them and the core's protocol types.
//!
//! # Layer contract
//!
//! Owns: the generated messages under `generated/`, and `From`/`TryFrom`
//! between them and `regalium-dmx-core`.
//!
//! Must not: be imported by the core. prost never enters the pure graph.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! # State
//!
//! Phase 0: an empty crate. The schema tree is designed in Phase 7.
