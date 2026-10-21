// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The nodes that have answered a poll, as their replies said.

use crate::codec::{ArtPollReply, Packet};
use std::net::SocketAddr;

/// A node, by the last reply it sent. Owns a copy of that datagram.
///
/// `Clone`, `Send` and `Sync`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    from: SocketAddr,
    datagram: Vec<u8>,
}

impl Node {
    /// Where the reply came from.
    #[must_use]
    pub const fn from(&self) -> SocketAddr {
        self.from
    }

    /// The reply, decoded again from the copy; `None` only if the copy is
    /// not a reply, which [`Nodes::record`] does not allow.
    #[must_use]
    pub fn reply(&self) -> Option<ArtPollReply<'_>> {
        match Packet::decode(&self.datagram) {
            Ok(Packet::PollReply(reply)) => Some(reply),
            _ => None,
        }
    }

    fn bind_index(&self) -> Option<u8> {
        self.reply().map(|r| r.fields().bind_index)
    }
}

/// Nodes kept before new ones are ignored. Replies arrive from the
/// network, so the table is bounded; a rig of a thousand nodes is large.
pub const MAX_NODES: usize = 1024;

/// Every node heard from, one entry per sender and bind index: a box with
/// eight ports answers with two replies, and both are kept.
#[derive(Debug, Default)]
pub(crate) struct Nodes(Vec<Node>);

impl Nodes {
    /// Records `reply`, replacing what the same sender said before.
    pub(crate) fn record(&mut self, from: SocketAddr, reply: &ArtPollReply<'_>) {
        let mut datagram = Vec::with_capacity(reply.len());
        reply.encode(&mut datagram);
        let node = Node { from, datagram };
        let index = node.bind_index();
        let known = self
            .0
            .iter()
            .position(|n| n.from == from && n.bind_index() == index);
        let full = self.0.len() >= MAX_NODES;
        match known.and_then(|at| self.0.get_mut(at)) {
            Some(known) => *known = node,
            None if !full => self.0.push(node),
            None => {}
        }
    }

    pub(crate) fn all(&self) -> Vec<Node> {
        self.0.clone()
    }
}
