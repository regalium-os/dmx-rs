// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Every attribute key the driver may record, in one place.
//!
//! Names follow OpenTelemetry convention: dotted, lower case, namespaced
//! `dmx.` except where a general convention already exists (`server.*`,
//! `rpc.*`). Add a key here, with what it identifies, before recording it
//! anywhere. RDM's keys are in [`rdm`] and re-exported here.
//!
//! Each key identifies *which* thing a value belongs to. None carries a slot
//! level: what a universe is sending is recorded as a metric value, or not at
//! all, and never as a label.

mod rdm;

pub use rdm::*;

use crate::Key;

/// Which protocol carried a universe or reached a node: `artnet`, `sacn` or
/// `serial`. Universe numbers overlap between Art-Net and sACN, so a universe
/// is named by this and [`UNIVERSE`] together.
pub const PROTOCOL: Key = Key::declare("dmx.protocol");

/// A universe number, as its protocol numbers it: Art-Net's 15-bit
/// port-address, sACN's 1 to 63999 (ANSI E1.31 §6.2.7), or the number the
/// runtime patched a serial line to.
pub const UNIVERSE: Key = Key::declare("dmx.universe");

/// The name a universe was given when patched, e.g. `stage left wash`.
pub const UNIVERSE_NAME: Key = Key::declare("dmx.universe.name");

/// How a universe's sources are combined: `htp`, `ltp` or `single`.
pub const MERGE_MODE: Key = Key::declare("dmx.merge.mode");

/// `sent` or `received`.
pub const DIRECTION: Key = Key::declare("dmx.direction");

/// A frame's start code (ANSI E1.11): 0 for dimmer data, 0xCC for RDM,
/// anything else alternate. Which start codes a rig carries is worth seeing.
pub const START_CODE: Key = Key::declare("dmx.frame.start_code");

/// Slots a frame carried after its start code, 1 to 512.
pub const SLOTS: Key = Key::declare("dmx.frame.slots");

/// Octets in a packet, header and payload.
pub const PACKET_OCTETS: Key = Key::declare("dmx.packet.octets");

/// A packet's sequence number: Art-Net's and sACN's 8-bit counters, by which
/// a receiver drops a packet that arrives out of order.
pub const SEQUENCE: Key = Key::declare("dmx.sequence");

/// Which node: an Art-Net node's `IP/bind-index`, an sACN source's CID, or a
/// USB widget's serial number. Stable for the node's lifetime.
pub const NODE: Key = Key::declare("dmx.node");

/// A node's short name: Art-Net's 18-octet `ShortName`, an sACN source name.
pub const NODE_SHORT_NAME: Key = Key::declare("dmx.node.short_name");

/// A node's long name: Art-Net's 64-octet `LongName`.
pub const NODE_LONG_NAME: Key = Key::declare("dmx.node.long_name");

/// A node's Art-Net OEM code, assigned by Artistic Licence.
pub const NODE_OEM: Key = Key::declare("dmx.node.oem");

/// A node's ESTA manufacturer id, the same registry RDM's UIDs use.
pub const NODE_ESTA: Key = Key::declare("dmx.node.esta_manufacturer");

/// A node's firmware revision: "every node on 2.x".
pub const NODE_FIRMWARE: Key = Key::declare("dmx.node.firmware");

/// How many DMX ports a node reports.
pub const NODE_PORTS: Key = Key::declare("dmx.node.ports");

/// The `IP:port` a node answers from.
pub const SERVER_ADDRESS: Key = Key::declare("server.address");

/// An Art-Net packet's opcode by name: `ArtDmx`, `ArtPoll`, `ArtSync`, …
pub const ARTNET_OPCODE: Key = Key::declare("dmx.artnet.opcode");

/// An sACN source's priority, 0 to 200 (ANSI E1.31 §6.2.3).
pub const SACN_PRIORITY: Key = Key::declare("dmx.sacn.priority");

/// Whether an sACN packet is marked preview data, for visualisers only.
pub const SACN_PREVIEW: Key = Key::declare("dmx.sacn.preview");

/// The universe an sACN source synchronises on, 0 when it does not.
pub const SACN_SYNC: Key = Key::declare("dmx.sacn.sync_universe");

/// How a request or a send ended: `ok`, `timeout`, `malformed`, or for RDM
/// the response type, `ack`, `ack-timer`, `ack-overflow`, `nack`.
pub const OUTCOME: Key = Key::declare("dmx.outcome");

/// How many times a request was sent before it was answered or abandoned.
pub const ATTEMPTS: Key = Key::declare("dmx.request.attempts");

/// Datagrams a discovery sent.
pub const DATAGRAMS: Key = Key::declare("dmx.discovery.datagrams");

/// Nodes or devices a discovery found.
pub const FOUND: Key = Key::declare("dmx.discovery.found");

/// Which pass of a repeating scenario this is.
pub const CYCLE: Key = Key::declare("dmx.cycle");

/// How an API call ended: `ok`, or its status, `NotFound`, `Unavailable`.
pub const API_STATUS: Key = Key::declare("dmx.api.status");

/// The RPC system, as the OpenTelemetry conventions spell it: `grpc`, for
/// every call the server answers, whichever transport carried it.
pub const RPC_SYSTEM: Key = Key::declare("rpc.system");

/// The fully-qualified service, e.g. `dmx.universe.v1.UniverseService`.
pub const RPC_SERVICE: Key = Key::declare("rpc.service");

/// The method, e.g. `GetUniverse`.
pub const RPC_METHOD: Key = Key::declare("rpc.method");

/// The canonical status code a call ended with, 0 for OK, as an integer.
pub const RPC_STATUS: Key = Key::declare("rpc.grpc.status_code");

/// Which of the server's transports carried a call: `grpc`, `rest` or `mcp`.
pub const TRANSPORT: Key = Key::declare("dmx.server.transport");
