// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The field probe's engine: discover nodes, watch universes, run RDM and
//! drive output, behind a TUI and a scriptable CLI.
//!
//! An application, not part of the driver, and shipped on its own. Its engine
//! is a library because `dmx-server` serves the same engine over gRPC, REST
//! and MCP; a second copy would be the first thing to drift.
//!
//! # State
//!
//! Phase 0: an empty crate. The TUI lands in Phase 8.
