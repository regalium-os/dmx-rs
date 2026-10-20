// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The shape of every universe, node and RDM series: which labels each one
//! carries, decided once, here, for every producer.
//!
//! bacnet-rs learned this the expensive way: a building's whole device
//! identity on every point series was 138,000 Prometheus series, and the
//! dashboards stopped answering. So here a universe series carries exactly
//! its two ids — protocol and universe number — a node series its protocol
//! and node id, and an RDM series its UID. Everything else about each (names,
//! firmware, patch) lives on one *info* series valued 1, joined on those ids.
//!
//! # Why recorders rather than label lists
//!
//! The functions here take a [`UniverseId`], a node id or a UID, never a
//! slice of labels, so a producer *cannot* put a node's name on
//! `dmx.frames`: there is no argument to put it in. The probe watching a rig
//! and the simulator serving one call the same functions, which keeps a
//! dashboard built against one working against the other.
//!
//! # Concurrency
//!
//! Every function here is a pure translation into calls of
//! [`Meter::record`](crate::Meter::record) and holds no state, so calling it
//! from several threads is as correct as the meter is — and every meter must
//! accept concurrent calls.
//!
//! Nothing here allocates: labels are fixed-size arrays of borrowed
//! [`Attribute`]s, so the no-op path stays free.

mod node;
mod rdm;
mod universe;

#[cfg(test)]
mod tests;

pub use node::{NODE_INFO_KEYS, record_node_info, record_node_up, record_nodes_discovered};
pub use rdm::{
    RDM_DEVICE_INFO_KEYS, record_rdm_device_info, record_rdm_device_up, record_rdm_discovered,
    record_rdm_request,
};
pub use universe::{
    record_frame, record_frame_interval, record_packet, record_sequence_error, record_source_lost,
    record_universe_info, record_universe_slots, record_universe_sources,
};

use crate::{Attribute, Key, keys};

/// Which universe a series is about: the two ids every universe series
/// carries, and the join key from it to `dmx_universe_info`.
///
/// Both, because the number alone is ambiguous: Art-Net's port-address 1 and
/// sACN's universe 1 are different universes, and a rig may carry both.
///
/// Borrows the protocol's name, so building one allocates nothing. `Copy`,
/// `Send` and `Sync`; it is plain data and safe to share.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UniverseId<'a> {
    /// `artnet`, `sacn` or `serial`: `dmx.protocol`.
    pub protocol: &'a str,
    /// The universe as that protocol numbers it: `dmx.universe`.
    pub universe: u32,
}

impl<'a> UniverseId<'a> {
    /// Universe `universe` of `protocol`.
    #[must_use]
    pub const fn new(protocol: &'a str, universe: u32) -> Self {
        Self { protocol, universe }
    }
}

/// Which way a frame or packet went.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Out of this process: a controller's output.
    Sent,
    /// Into it: a node's input, or a merge source.
    Received,
}

impl Direction {
    /// The name `dmx.direction` carries.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sent => "sent",
            Self::Received => "received",
        }
    }
}

/// The labels of every universe series, exactly: `dmx.protocol` and
/// `dmx.universe`, in that order.
#[must_use]
pub fn universe_labels<'a>(id: &UniverseId<'a>) -> [Attribute<'a>; 2] {
    [
        Attribute::str(keys::PROTOCOL, id.protocol),
        Attribute::int(keys::UNIVERSE, id.universe.into()),
    ]
}

/// An info series' labels: `head`, then whichever of `allowed` `identity`
/// carries, in `allowed`'s order — the first value of each, nothing else.
///
/// Returns the array and how much of it is filled, so an info series is
/// built without allocating however many keys a producer knows. `N` is
/// sized by the caller to `head` plus `allowed`, so nothing is ever cut.
fn select<'a, const N: usize>(
    head: &[Attribute<'a>],
    allowed: &[Key],
    identity: &[Attribute<'a>],
) -> ([Attribute<'a>; N], usize) {
    // Fills the slots past `used`, which are never emitted.
    const UNUSED: Attribute<'static> = Attribute::bool(keys::SACN_PREVIEW, false);
    let chosen = allowed
        .iter()
        .filter_map(|key| identity.iter().find(|a| a.key == *key));
    let mut labels = [UNUSED; N];
    let mut used = 0;
    for a in head.iter().chain(chosen) {
        if let Some(slot) = labels.get_mut(used) {
            *slot = *a;
            used += 1;
        }
    }
    (labels, used)
}
