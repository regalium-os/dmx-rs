// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The rig as the probe shows it: plain data, copied out of the runtime
//! every tick, so a view never holds a lock the runtime needs.

mod feed;

pub use feed::{EVENTS, Event, PACKETS, Packet, Severity};

use std::collections::VecDeque;

/// The transport a universe or node is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// Art-Net 4.
    ArtNet,
    /// Streaming ACN.
    Sacn,
    /// A widget or a UART.
    Serial,
}

impl Protocol {
    /// The name the probe prints.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ArtNet => "artnet",
            Self::Sacn => "sacn",
            Self::Serial => "serial",
        }
    }
}

/// What a universe is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Sending or receiving.
    Live,
    /// An input with nothing arriving.
    Idle,
    /// Its transport is failing.
    Failing,
}

/// One merge source of an input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    /// Who, as the transport identifies it.
    pub id: String,
    /// Its priority, 100 where the protocol has none.
    pub priority: u8,
    /// Whether it is at the highest priority present, so merged, not held.
    pub merged: bool,
}

/// One universe.
#[derive(Clone, Debug, PartialEq)]
pub struct Universe {
    /// Its resource ID: `stage-left`.
    pub name: String,
    /// Its transport.
    pub protocol: Protocol,
    /// Its number as the protocol prints it: `0:0:1`, `7`.
    pub number: String,
    /// Whether this probe sends it.
    pub output: bool,
    /// For an input, `htp` or `ltp`.
    pub merge: Option<&'static str>,
    /// What it is doing.
    pub state: State,
    /// Its levels, as many slots as are sent.
    pub levels: Vec<u8>,
    /// Moves exactly when the levels do.
    pub generation: u64,
    /// Frames a second over the last second.
    pub fps: f32,
    /// The last intervals between frames, in milliseconds, oldest first.
    pub intervals: Vec<f32>,
    /// Interval counts in the buckets of [`BUCKETS`].
    pub histogram: [u32; 7],
    /// For an input, its sources.
    pub sources: Vec<Source>,
    /// Where an output goes: an address, a device.
    pub target: String,
}

/// The frame-interval histogram's buckets, in milliseconds, as labels.
pub const BUCKETS: [&str; 7] = ["< 21", "21–22", "22–23", "23–24", "24–25", "25–30", "> 30"];

/// A node: a piece of equipment that carries universes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Its transport.
    pub protocol: Protocol,
    /// Its short name.
    pub name: String,
    /// Where it is: an address or a device path.
    pub address: String,
    /// Its ports, as one line: `out 0:0:1 0:0:2`.
    pub ports: String,
    /// Its firmware, or a dash.
    pub firmware: String,
    /// Whether it answered recently.
    pub reachable: bool,
}

/// An RDM device on a universe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// The universe it is on.
    pub universe: String,
    /// Its UID: `4c55:00000203`.
    pub uid: String,
    /// Its label.
    pub label: String,
    /// Its model.
    pub model: String,
    /// Its start address, 1 to 512, or 0 for none.
    pub address: u16,
    /// Slots it uses.
    pub footprint: u16,
    /// Current personality and how many: `2/4`.
    pub personality: String,
    /// Whether it answered its last request.
    pub responding: bool,
}

/// The output policy, as it stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    /// Every output dark.
    pub blackout: bool,
    /// Every output frozen.
    pub hold: bool,
    /// The grand master, 0 to 255.
    pub master: u8,
}

/// Everything the probe shows, at one moment.
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    /// Every universe, in the order they were made.
    pub universes: Vec<Universe>,
    /// Every node discovery found.
    pub nodes: Vec<Node>,
    /// Every RDM device, by universe.
    pub devices: Vec<Device>,
    /// The output policy.
    pub policy: Policy,
    /// Whether MCP clients may write; the probe never may change it.
    pub mcp_writes: bool,
    /// Newest first, at most [`EVENTS`].
    pub events: VecDeque<Event>,
    /// Newest last, at most [`PACKETS`].
    pub packets: VecDeque<Packet>,
    /// Level packets counted, not kept.
    pub level_packets: u64,
    /// An RDM search in progress or just finished, as one line.
    pub discovery: Option<String>,
    /// What the probe is attached to: `demo rig`, `0.0.0.0`.
    pub attached: String,
}
