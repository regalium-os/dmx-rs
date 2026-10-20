// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The rig simulator: Art-Net nodes, sACN receivers and RDM responders,
//! served through the driver so a simulated rig exercises exactly the code a
//! real controller talks to.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands beside the transports it simulates.
