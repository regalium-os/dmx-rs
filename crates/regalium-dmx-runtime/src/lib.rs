// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The runtime: where the core's decisions meet a clock and a socket.
//!
//! # Layer contract
//!
//! Owns: one refresh loop per output universe, run at its own rate and
//! falling back to a keep-alive when nothing changes; the sources feeding each
//! universe and their loss; RDM transactions and discovery over any
//! transport; the event stream a caller watches.
//!
//! Lifecycle is explicit: `new` constructs and never spawns, `run` blocks
//! until cancelled and does not return before its tasks have stopped, `close`
//! is idempotent. A node going silent is an event, not an error.
//!
//! Allowed imports: the core, every transport, the schema, the seam.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands in Phase 3.
