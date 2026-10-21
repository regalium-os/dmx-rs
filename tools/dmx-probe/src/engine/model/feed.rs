// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What the probe saw happen: the event feed and captured packets.

use super::Protocol;

/// How serious an event is, for its colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Something came back or started.
    Good,
    /// Worth a look.
    Warn,
    /// Something stopped working.
    Bad,
    /// For information.
    Info,
}

/// One line of the event feed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    /// When, as `HH:MM:SS` since the probe started.
    pub time: String,
    /// What kind: `send failed`, `source found`.
    pub kind: String,
    /// What happened.
    pub text: String,
    /// How serious.
    pub severity: Severity,
}

/// A packet seen on the wire, kept whole for the Traffic tab.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    /// When, as `HH:MM:SS.mmm` since the probe started.
    pub time: String,
    /// Whether this probe sent it.
    pub sent: bool,
    /// Its transport.
    pub protocol: Protocol,
    /// Who it went to or came from.
    pub peer: String,
    /// The datagram or message, as on the wire.
    pub octets: Vec<u8>,
}

/// Events kept for the feed.
pub const EVENTS: usize = 200;
/// Packets kept for the Traffic tab.
pub const PACKETS: usize = 500;
