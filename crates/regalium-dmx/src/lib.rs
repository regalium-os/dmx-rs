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
//! Phase 4: the core's frames, universes, merging, refresh policy and
//! transport trait; the runtime's outputs and inputs; and Art-Net 4.

pub use regalium_dmx_artnet as artnet;
pub use regalium_dmx_core::{
    address, error, frame, merge, refresh, start_code, transport, universe,
};
pub use regalium_dmx_runtime as runtime;
