// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The data server: the probe's engine served over gRPC, REST/JSON and MCP,
//! every surface generated from `protobuf/`.
//!
//! Writes are refused unless started with `--allow-writes`. In DMX almost
//! every useful call is a write, so writes are further bounded by the
//! safety policy the roadmap describes.
//!
//! # State
//!
//! Phase 0: an empty crate. Lands in Phase 9.
