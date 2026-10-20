// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The telemetry seam: the traits every layer records through.
//!
//! Spans, metrics and logs are named here and made elsewhere. Every crate in
//! the workspace may import this one, which is why it imports nothing: a
//! dependency here is a dependency everywhere.
//!
//! # Layer contract
//!
//! Owns: the `Tracer`, `Meter` and `Logger` traits with no-op defaults, the
//! attribute types, and `trace_attrs!`, so an undeclared field cannot be
//! recorded.
//!
//! Must not: depend on anything, OpenTelemetry above all. The
//! `regalium-dmx-telemetry-otel` adapter attaches the the-protobuf-project SDK
//! to these traits; nothing else may.
//!
//! Allowed imports: none.
//!
//! # Spans
//!
//! Opens none. Spans are named `dmx.<layer>.<operation>` by the layer that
//! opens them.
//!
//! # State
//!
//! Phase 0: an empty crate. The seam is ported from bacnet-rs in Phase 1.
