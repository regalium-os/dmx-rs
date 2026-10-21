// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What a surface asks the engine to do. Every write is one of these, and
//! each goes through the runtime's output policy; none is applied by the
//! surface itself.

/// One request to the engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Every output dark, or not.
    Blackout(bool),
    /// Every output frozen, or not.
    Hold(bool),
    /// The grand master, 0 to 255.
    Master(u8),
    /// Writes `values` into an output universe from `start`, 1 to 512.
    Write {
        /// The universe's name.
        universe: String,
        /// The first address.
        start: u16,
        /// The levels.
        values: Vec<u8>,
    },
    /// One output universe's every level to zero.
    BlackoutUniverse(String),
    /// Finds the RDM devices on a universe.
    Discover(String),
    /// Moves an RDM device's start address.
    SetAddress {
        /// The universe it is on.
        universe: String,
        /// Its UID, `4c55:00000203`.
        uid: String,
        /// The new start address, 1 to 512.
        address: u16,
    },
    /// Makes an RDM device identify itself, or stop.
    Identify {
        /// The universe it is on.
        universe: String,
        /// Its UID.
        uid: String,
        /// On or off.
        on: bool,
    },
}
