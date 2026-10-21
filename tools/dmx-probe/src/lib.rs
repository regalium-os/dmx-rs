// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The field probe's engine: discover nodes, watch universes, run RDM and
//! drive output, behind a TUI and a scriptable CLI.
//!
//! An application, not part of the driver, and shipped on its own. Its
//! engine is a library because `dmx-server` serves the same engine over
//! gRPC, REST and MCP; a second copy would be the first thing to drift.
//!
//! # Layer contract
//!
//! | Module | Owns |
//! | --- | --- |
//! | [`engine`] | the worker that owns the rig, the commands it takes, the model it publishes |
//! | [`sim`] | a rig in memory: a simulated bus, consoles and RDM fixtures |
//! | [`dissect`] | a captured packet as a tree of fields with their offsets |
//!
//! Must not: write to an output except through the runtime's policy, or
//! name an OpenTelemetry type.
//!
//! Allowed imports: the facade, the schema, the seam.
//!
//! # State
//!
//! Phase 8: the engine, the demo rig and the dissector; the TUI in the
//! binary.

pub mod dissect;
pub mod engine;
pub mod sim;
