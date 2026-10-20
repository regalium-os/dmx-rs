// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The instruments every application on the driver records into: one
//! declaration of each series, so the probe watching a rig and the simulator
//! serving one produce the same `dmx_frames_total`, and a dashboard
//! built against either works against both.
//!
//! Series are labelled by *which* universe, node or responder — never by
//! what it carries. A universe is [`PROTOCOL`](crate::keys::PROTOCOL) and
//! [`UNIVERSE`](crate::keys::UNIVERSE); who it, a node or a responder is, is
//! said once on an info series valued 1 and joined on those ids.
//!
//! Slot levels are deliberately not a series. Twenty universes of 512 slots
//! is ten thousand gauges sampled at 44 Hz for a value a dashboard cannot
//! use; the probe shows levels live, and a trace or a capture keeps them.
//!
//! Record these through [`shape`](crate::shape), whose recorders attach
//! exactly the labels documented here and no others.

use crate::meter::Instrument;

/// Frame-interval buckets, in ms: dense around 22.7 ms, a full universe at
/// 44 Hz, then out to Art-Net's and sACN's keep-alive intervals.
const FRAME_BUCKETS: &[f64] = &[
    1.0, 2.0, 5.0, 10.0, 20.0, 22.0, 23.0, 24.0, 25.0, 30.0, 50.0, 100.0, 250.0, 500.0, 1000.0,
    2500.0, 5000.0,
];

/// RDM round-trip buckets, in ms: a responder on a wire answers within a
/// few milliseconds; through an Art-Net or sACN gateway, within hundreds.
const RDM_BUCKETS: &[f64] = &[
    1.0, 2.0, 3.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0,
];

/// DMX frames, by universe and direction.
pub const FRAMES: Instrument = Instrument::counter(
    "dmx.frames",
    "{frame}",
    "DMX512 frames sent or received, by universe and direction",
);
/// Time between consecutive frames of one universe: the refresh rate and
/// its jitter. The one histogram kept per universe; see
/// [`record_frame_interval`](crate::shape::record_frame_interval).
pub const FRAME_INTERVAL: Instrument = Instrument::histogram_with(
    "dmx.frame.interval",
    "ms",
    "Time since the universe's previous frame",
    FRAME_BUCKETS,
);
/// Slots in a universe's last frame, 1 to 512.
pub const UNIVERSE_SLOTS: Instrument = Instrument::gauge(
    "dmx.universe.slots",
    "{slot}",
    "Slots in the universe's last frame",
);
/// Sources currently merged into a universe.
pub const UNIVERSE_SOURCES: Instrument = Instrument::gauge(
    "dmx.universe.sources",
    "{source}",
    "Sources merged into the universe",
);
/// Always 1: what a universe is. Labels: the universe ids, its name and
/// merge mode.
pub const UNIVERSE_INFO: Instrument = Instrument::gauge(
    "dmx.universe.info",
    "{info}",
    "Always 1; labelled with the universe's name and merge mode",
);
/// Packets dropped for arriving out of sequence.
pub const SEQUENCE_ERRORS: Instrument = Instrument::counter(
    "dmx.input.sequence_errors",
    "{packet}",
    "Packets dropped for arriving out of sequence",
);
/// Sources lost: an sACN source silent past its timeout, or terminated.
pub const SOURCES_LOST: Instrument = Instrument::counter(
    "dmx.input.sources_lost",
    "{source}",
    "Sources that stopped sending to the universe",
);
/// Packets on a socket or port, by protocol and direction.
pub const PACKETS: Instrument = Instrument::counter(
    "dmx.transport.packets",
    "{packet}",
    "Packets sent and received, by protocol and direction",
);
/// Octets on a socket or port, by protocol and direction.
pub const BYTES: Instrument = Instrument::counter(
    "dmx.transport.bytes",
    "By",
    "Octets sent and received, by protocol and direction",
);
/// 1 while a node answers polls or keeps sending.
pub const NODE_UP: Instrument =
    Instrument::gauge("dmx.node.up", "{flag}", "1 while the node answers or sends");
/// Always 1: who a node is. See [`NODE_INFO_KEYS`](crate::shape::NODE_INFO_KEYS).
pub const NODE_INFO: Instrument = Instrument::gauge(
    "dmx.node.info",
    "{info}",
    "Always 1; labelled with the node's identity and where it was reached",
);
/// Nodes that answered the last discovery, by protocol.
pub const NODES_DISCOVERED: Instrument = Instrument::gauge(
    "dmx.discovery.nodes",
    "{node}",
    "Nodes that answered the last poll",
);
/// RDM requests, by command class, outcome and responder.
pub const RDM_REQUESTS: Instrument = Instrument::counter(
    "dmx.rdm.requests",
    "{request}",
    "RDM requests, by command class, outcome and responder",
);
/// RDM round-trip time, retries included, by command class, parameter and
/// outcome; not by responder (see [`crate::shape::record_rdm_request`]).
pub const RDM_REQUEST_DURATION: Instrument = Instrument::histogram_with(
    "dmx.rdm.request.duration",
    "ms",
    "Time from RDM request to final response or give-up",
    RDM_BUCKETS,
);
/// 1 while a responder answers.
pub const RDM_DEVICE_UP: Instrument = Instrument::gauge(
    "dmx.rdm.device.up",
    "{flag}",
    "1 while the responder answers",
);
/// Always 1: who a responder is. See
/// [`RDM_DEVICE_INFO_KEYS`](crate::shape::RDM_DEVICE_INFO_KEYS).
pub const RDM_DEVICE_INFO: Instrument = Instrument::gauge(
    "dmx.rdm.device.info",
    "{info}",
    "Always 1; labelled with the responder's identity and patch",
);
/// Responders the last RDM discovery found, by universe.
pub const RDM_DEVICES_DISCOVERED: Instrument = Instrument::gauge(
    "dmx.rdm.discovery.devices",
    "{device}",
    "Responders the last RDM discovery found",
);
/// Time a server took to answer one call, by `rpc.service`, `rpc.method`,
/// `rpc.grpc.status_code` and transport: the OpenTelemetry RPC semantic
/// conventions' own name and unit. A server stream is one call.
pub const RPC_SERVER_DURATION: Instrument = Instrument::histogram(
    "rpc.server.duration",
    "ms",
    "Time to answer a call, or for a stream to close",
);
