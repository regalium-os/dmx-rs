// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The node series: whether a node answers, on its protocol and id alone;
//! the info series that says who it is; and how many a discovery found.
//!
//! A node is an Art-Net node, an sACN source or a USB-DMX widget: whatever
//! the runtime reaches universes through.

use super::select;
use crate::instruments::{NODE_INFO, NODE_UP, NODES_DISCOVERED};
use crate::{Attribute, Key, Meter, keys};

/// The keys `dmx.node.info` may carry besides the protocol and node id, in
/// the order they are emitted: `ArtPollReply`'s identifying fields, which an
/// sACN source or a widget fills as far as it can, then where it was reached.
///
/// Anything else handed to [`record_node_info`] is dropped, so a universe's
/// label, or a slot level, cannot reach a node's info series.
pub const NODE_INFO_KEYS: [Key; 7] = [
    keys::NODE_SHORT_NAME,
    keys::NODE_LONG_NAME,
    keys::NODE_OEM,
    keys::NODE_ESTA,
    keys::NODE_FIRMWARE,
    keys::NODE_PORTS,
    keys::SERVER_ADDRESS,
];

/// The two labels of every node series: `dmx.protocol`, `dmx.node`.
fn node_labels<'a>(protocol: &'a str, node: &'a str) -> [Attribute<'a>; 2] {
    [
        Attribute::str(keys::PROTOCOL, protocol),
        Attribute::str(keys::NODE, node),
    ]
}

/// Records on `dmx.node.up` whether `node` answers or keeps sending: 1 or 0.
pub fn record_node_up(meter: &dyn Meter, protocol: &str, node: &str, up: bool) {
    meter.record(
        &NODE_UP,
        f64::from(u8::from(up)),
        &node_labels(protocol, node),
    );
}

/// Records `dmx.node.info`, valued 1: `dmx.protocol`, `dmx.node`, then
/// whichever of [`NODE_INFO_KEYS`] `identity` carries, in that order. A key
/// `identity` repeats is taken once, its first value; any other key is
/// ignored, so a producer may hand over a whole
/// [`Traced::attribute_list`](crate::Traced::attribute_list) unfiltered.
///
/// Borrows `identity` for the call only, and allocates nothing.
pub fn record_node_info(meter: &dyn Meter, protocol: &str, node: &str, identity: &[Attribute<'_>]) {
    let head = node_labels(protocol, node);
    let (labels, used) = select::<{ 2 + NODE_INFO_KEYS.len() }>(&head, &NODE_INFO_KEYS, identity);
    meter.record(&NODE_INFO, 1.0, labels.get(..used).unwrap_or(&[]));
}

/// Records on `dmx.discovery.nodes` how many nodes answered `protocol`'s last
/// discovery: an `ArtPoll`, or an sACN universe-discovery period.
pub fn record_nodes_discovered(meter: &dyn Meter, protocol: &str, found: u32) {
    meter.record(
        &NODES_DISCOVERED,
        f64::from(found),
        &[Attribute::str(keys::PROTOCOL, protocol)],
    );
}
