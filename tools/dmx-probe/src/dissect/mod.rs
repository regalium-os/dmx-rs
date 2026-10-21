// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A captured packet as people read it: a one-line summary, and a tree of
//! fields, each with the octets it came from.
//!
//! Every decode is the driver's own codec, so what the probe shows is what
//! the driver understood; a packet the codec refuses shows the refusal and
//! the offset it points at.

mod network;
mod rdm;

use crate::engine::model::{Packet, Protocol};

/// One line of the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    /// How deep: 0 for a layer, 1 for its fields, 2 for a parameter's.
    pub depth: u8,
    /// What it says.
    pub text: String,
    /// The octets it came from, first to last inclusive, if known.
    pub span: Option<(usize, usize)>,
}

impl Field {
    pub(crate) fn new(depth: u8, text: impl Into<String>, span: Option<(usize, usize)>) -> Self {
        Self { depth, text: text.into(), span }
    }
}

/// A packet, read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dissection {
    /// One line, for the packet list.
    pub summary: String,
    /// Whether it only carries levels, which the list hides by default.
    pub levels: bool,
    /// Its fields.
    pub tree: Vec<Field>,
}

/// Reads `packet` with the codec for its protocol.
#[must_use]
pub fn dissect(packet: &Packet) -> Dissection {
    match packet.protocol {
        Protocol::ArtNet => network::artnet(&packet.octets),
        Protocol::Sacn => network::sacn(&packet.octets),
        Protocol::Serial => line(&packet.octets),
    }
}

/// What travels on a serial line: a DMX512 packet, an RDM message, or a
/// discovery answer.
fn line(octets: &[u8]) -> Dissection {
    match octets.first() {
        Some(0xCC) => rdm::message(octets, 0),
        Some(0xFE | 0xAA) => rdm::discovery(octets),
        Some(&code) => Dissection {
            summary: format!("DMX512 · start {code:#04x} · {} slots", octets.len() - 1),
            levels: code == 0,
            tree: vec![Field::new(0, format!("DMX512 · start code {code:#04x}"), Some((0, 0))), Field::new(1, format!("{} slots", octets.len() - 1), Some((1, octets.len().saturating_sub(1))))],
        },
        None => Dissection { summary: "empty".to_owned(), levels: false, tree: Vec::new() },
    }
}

/// A refusal, as the tree shows it.
pub(crate) fn refused(what: &str, error: &dyn std::fmt::Display, offset: usize, len: usize) -> Dissection {
    Dissection {
        summary: format!("{what} · malformed: {error}"),
        levels: false,
        tree: vec![Field::new(0, format!("{what} · refused"), None), Field::new(1, error.to_string(), Some((offset, len.saturating_sub(1).max(offset))))],
    }
}
