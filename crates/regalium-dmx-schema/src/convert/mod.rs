// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! From what the core knows to the messages, and back where a request
//! names something the core acts on.
//!
//! | Module | Converts |
//! | --- | --- |
//! | [`names`] | an RDM UID to a device ID and back; a PID to a parameter ID and back |
//! | [`device`] | `DEVICE_INFO` and a UID to an [`rdm::Device`](crate::rdm::Device) |
//! | [`levels`] | a core `Universe` to [`universe::Levels`](crate::universe::Levels); merge modes both ways |

pub mod device;
pub mod levels;
pub mod names;
