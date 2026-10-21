// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The Regalium DMX driver: DMX512, RDM, Art-Net and sACN in pure Rust.
//!
//! The facade a caller depends on. It re-exports with `pub use`, never a
//! newtype, so a third party can implement a transport against the same
//! trait this repository's transports satisfy.
//!
//! # Layer contract
//!
//! Owns: nothing of its own beyond re-exports and feature selection
//! (`artnet`, `sacn`, `serial`, and the opt-in `otel`).
//!
//! Allowed imports: every crate beneath the edge.
//!
//! # State
//!
//! Phase 6b: the core's frames, universes, merging with priority, refresh
//! policy, RDM and transport traits; the runtime's outputs, inputs and RDM
//! controller; Art-Net 4, sACN, and the serial line.

pub use regalium_dmx_artnet as artnet;
pub use regalium_dmx_core::{
    address, error, frame, merge, rdm, refresh, start_code, transport, universe,
};
pub use regalium_dmx_runtime as runtime;
pub use regalium_dmx_sacn as sacn;
pub use regalium_dmx_serial as serial;
