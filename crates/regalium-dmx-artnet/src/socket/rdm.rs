// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM through an Art-Net node: the node discovers its port's devices and
//! keeps the table; the controller asks for the table, and sends single
//! requests in `ArtRdm`.
//!
//! Whichever thread reads the socket — an input, or the exchange itself —
//! files `ArtRdm` replies in an inbox and tables by port, so a reply is
//! never lost to a concurrent receive; levels read by an exchange are kept
//! for the next receive.

use super::{ArtNet, Error};
use crate::codec::{ArtRdm, ArtTod, Packet, PortAddress};
use regalium_dmx_core::rdm::Uid;
use regalium_dmx_core::transport::{Answer, Expect, RdmLine};
use std::time::{Duration, Instant};

/// How long a node has to relay an answer: longer than a line's turnaround,
/// for the network and the node's own queue.
pub const RDM_WINDOW: Duration = Duration::from_millis(500);

/// How long nodes have to send their tables.
pub const TOD_WINDOW: Duration = Duration::from_secs(1);

/// `ArtRdm` replies held for an exchange before the oldest are dropped.
pub(crate) const INBOX: usize = 16;

/// UIDs kept per port's table: the most one RDM line can hold, with margin.
pub(crate) const MAX_TOD: usize = 4096;

/// How long one read inside an exchange waits before the inbox is checked.
const SLICE: Duration = Duration::from_millis(10);

impl ArtNet {
    /// Reads the socket for up to `slice`, keeping any levels for receive.
    fn pump(&self, slice: Duration) -> Result<(), Error> {
        let mut buf = [0u8; 513];
        if let Some(got) = self.receive_within(&mut buf, slice)? {
            let octets = buf.get(..got.len).unwrap_or_default().to_vec();
            let mut state = self.state();
            if state.levels.len() == INBOX {
                state.levels.pop_front();
            }
            state.levels.push_back((got, octets));
        }
        Ok(())
    }

    fn take_reply(&self, port: PortAddress) -> Option<Vec<u8>> {
        let mut state = self.state();
        let at = state.inbox.iter().position(
            |d| matches!(Packet::decode(d), Ok(Packet::Rdm(r)) if r.port_address == port),
        )?;
        let datagram = state.inbox.remove(at)?;
        match Packet::decode(&datagram) {
            Ok(Packet::Rdm(reply)) => Some(reply.rdm()),
            _ => None,
        }
    }
}

impl RdmLine for ArtNet {
    type Error = Error;

    fn protocol(&self) -> &'static str {
        "artnet"
    }

    /// Asks the target for `universe`'s table and gathers what nodes send
    /// within [`TOD_WINDOW`]. No answer is an empty table.
    fn table(&self, universe: u32) -> Result<Option<Vec<Uid>>, Error> {
        let port = PortAddress::new(universe).ok_or(Error::PortAddress(universe))?;
        let [low, _] = port.octets();
        let low = [low];
        let mut out = Vec::with_capacity(32);
        ArtTod::request(&port, &low).encode(&mut out, false);
        self.state().tod.remove(&port.get());
        self.socket.send_to(&out, self.target)?;
        let deadline = Instant::now() + TOD_WINDOW;
        while Instant::now() < deadline {
            self.pump(SLICE)?;
        }
        let table = self
            .state()
            .tod
            .get(&port.get())
            .map(|t| t.iter().copied().collect());
        Ok(Some(table.unwrap_or_default()))
    }

    fn rdm(
        &self,
        universe: u32,
        request: &[u8],
        expect: Expect,
        buf: &mut [u8],
    ) -> Result<Answer, Error> {
        let port = PortAddress::new(universe).ok_or(Error::PortAddress(universe))?;
        if expect == Expect::Discovery {
            return Err(Error::Unsupported(
                "RDM discovery: the node keeps the table",
            ));
        }
        while self.take_reply(port).is_some() {}
        let mut out = Vec::with_capacity(request.len() + 24);
        ArtRdm::new(port, request).encode(&mut out);
        self.socket.send_to(&out, self.target)?;
        if expect == Expect::Nothing {
            return Ok(Answer::Silent);
        }
        let deadline = Instant::now() + RDM_WINDOW;
        loop {
            if let Some(reply) = self.take_reply(port) {
                let mut n = 0;
                for (to, from) in buf.iter_mut().zip(&reply) {
                    *to = *from;
                    n += 1;
                }
                return Ok(Answer::Octets(n));
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Ok(Answer::Silent);
            }
            self.pump(left.min(SLICE))?;
        }
    }
}
