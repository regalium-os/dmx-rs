// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The one place OpenTelemetry is named: the seam's traits implemented over
//! the the-protobuf-project telemetry SDK.
//!
//! # Layer contract
//!
//! Owns: the tracer, meter and logger bridges, and the exporter's lifecycle.
//!
//! Must not: be imported beneath the facade's opt-in `otel` feature or the
//! applications. Only here may the SDK's audited zstd and lz4 appear.
//!
//! Allowed imports: `regalium-dmx-telemetry`, the SDK, OpenTelemetry.
//!
//! # State
//!
//! Phase 0: an empty crate. Ported from bacnet-rs in Phase 1.
