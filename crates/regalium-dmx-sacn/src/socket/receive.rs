// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Waiting for data, and the housekeeping done while waiting.

use super::{Error, Sacn};
use crate::codec::{DataPacket, PREVIEW, Packet, TERMINATED};
use crate::sequence;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::transport::Received;
use regalium_dmx_telemetry::shape::{self, UniverseId};
use std::io::ErrorKind;
use std::time::{Duration, Instant};

/// Large enough for any E1.31 packet; the largest, a 512-slot data packet,
/// is 638 octets, and a full discovery page 1144.
const DATAGRAM: usize = 1500;

/// Sequence numbers remembered, per source and universe, before new pairs
/// go unordered: they arrive from the network, so the table is bounded.
const HEARD: usize = 1 << 16;

/// The shortest wait a socket accepts: a zero read timeout means "forever".
const SHORTEST: Duration = Duration::from_millis(1);

impl Sacn {
    pub(super) fn receive_within(
        &self,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<Option<Received>, Error> {
        let deadline = Instant::now() + timeout;
        let mut datagram = [0u8; DATAGRAM];
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            self.socket.set_read_timeout(Some(left.max(SHORTEST)))?;
            match self.socket.recv_from(&mut datagram) {
                Ok((n, _)) => {
                    let got = datagram.get(..n).unwrap_or_default();
                    match Packet::decode(got) {
                        Ok(Packet::Data(data)) => {
                            if let Some(received) = self.levels(&data, buf) {
                                return Ok(Some(received));
                            }
                        }
                        Ok(Packet::Discovery(d)) if d.cid != self.cid => self.record(&d),
                        // Sync, this socket's own discovery, other vectors,
                        // and datagrams that are not E1.31.
                        _ => {}
                    }
                }
                Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                    return Ok(None);
                }
                Err(e) if e.kind() == ErrorKind::ConnectionReset => {}
                Err(e) => return Err(e.into()),
            }
            if left.is_zero() {
                return Ok(None);
            }
        }
    }

    /// The data to deliver, or `None` when it is dropped.
    fn levels(&self, data: &DataPacket<'_>, buf: &mut [u8]) -> Option<Received> {
        // Our own, looped back by multicast; and preview data, which a
        // receiver driving lights ignores.
        if data.cid == self.cid || data.has(PREVIEW) {
            return None;
        }
        let source = data.cid.folded();
        let universe = data.universe;
        let key = (source, universe);
        let mut state = self.state();
        if data.has(TERMINATED) {
            state.heard.remove(&key);
        } else if state
            .heard
            .get(&key)
            .is_some_and(|&last| sequence::is_late(last, data.sequence))
        {
            drop(state);
            let id = UniverseId::new("sacn", universe.into());
            shape::record_sequence_error(self.meter.as_ref(), &id);
            return None;
        } else if state.heard.len() < HEARD || state.heard.contains_key(&key) {
            state.heard.insert(key, data.sequence);
        }
        drop(state);
        let mut len = 0;
        let octets = core::iter::once(&data.start_code.0).chain(data.slots);
        for (to, from) in buf.iter_mut().zip(octets) {
            *to = *from;
            len += 1;
        }
        let received =
            Received::new(universe.into(), SourceId(source), len).priority(data.priority);
        Some(if data.has(TERMINATED) {
            received.ended()
        } else {
            received
        })
    }
}
