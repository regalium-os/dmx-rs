// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Waiting for levels, and the housekeeping done while waiting.

use super::{ArtNet, Error};
use crate::codec::{ArtDmx, Packet, Reply, name};
use crate::codec::{LONG_NAME, SHORT_NAME};
use crate::sequence;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::Received;
use regalium_dmx_telemetry::shape::{self, UniverseId};
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

/// Large enough for any Art-Net packet this crate reads; the largest,
/// `ArtDmx` with 512 slots, is 530 octets.
const DATAGRAM: usize = 1500;

/// The shortest wait a socket accepts: a zero read timeout means "forever".
const SHORTEST: Duration = Duration::from_millis(1);

/// A source as the merge knows it: its IPv4 address and UDP port, so two
/// senders on one host stay apart.
pub(crate) fn source(from: SocketAddr) -> SourceId {
    let ip = match from.ip() {
        IpAddr::V4(v4) => u64::from(u32::from(v4)),
        IpAddr::V6(v6) => v6
            .octets()
            .iter()
            .fold(0u64, |h, &b| h.rotate_left(5) ^ u64::from(b)),
    };
    SourceId((ip << 16) | u64::from(from.port()))
}

impl ArtNet {
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
                Ok((n, from)) => {
                    let got = datagram.get(..n).unwrap_or_default();
                    if let Some(received) = self.handle(got, from, buf)? {
                        return Ok(Some(received));
                    }
                }
                Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                    return Ok(None);
                }
                // An ICMP unreachable from an earlier send, on some systems.
                Err(e) if e.kind() == ErrorKind::ConnectionReset => {}
                Err(e) => return Err(e.into()),
            }
            if left.is_zero() {
                return Ok(None);
            }
        }
    }

    /// Levels to deliver, or `None` once the datagram has been handled.
    fn handle(
        &self,
        datagram: &[u8],
        from: SocketAddr,
        buf: &mut [u8],
    ) -> Result<Option<Received>, Error> {
        match Packet::decode(datagram) {
            Ok(Packet::Dmx(dmx)) => Ok(self.levels(&dmx, from, buf)),
            Ok(Packet::Poll(poll)) if poll.target().is_none() => {
                self.reply(from)?;
                Ok(None)
            }
            Ok(Packet::PollReply(reply)) => {
                self.state().nodes.record(from, &reply);
                Ok(None)
            }
            // Targeted polls for ports this controller lacks, syncs, address
            // changes, unknown opcodes and datagrams that are not Art-Net.
            _ => Ok(None),
        }
    }

    fn levels(&self, dmx: &ArtDmx<'_>, from: SocketAddr, buf: &mut [u8]) -> Option<Received> {
        let source = source(from);
        let port = dmx.port_address().get();
        {
            let mut state = self.state();
            let last = state.heard.entry((source, port)).or_insert(0);
            if sequence::is_late(*last, dmx.sequence()) {
                drop(state);
                let id = UniverseId::new("artnet", port.into());
                shape::record_sequence_error(self.meter.as_ref(), &id);
                return None;
            }
            *last = dmx.sequence();
        }
        let (code, slots) = buf.split_first_mut()?;
        *code = StartCode::NULL.0;
        let mut len = 1;
        for (to, from) in slots.iter_mut().zip(dmx.slots()) {
            *to = *from;
            len += 1;
        }
        Some(Received {
            universe: port.into(),
            source,
            len,
        })
    }

    /// Answers a poll, unicast to the poller, as a controller with no ports.
    fn reply(&self, to: SocketAddr) -> Result<(), Error> {
        let ip = match self.socket.local_addr()?.ip() {
            IpAddr::V4(v4) => v4,
            IpAddr::V6(_) => Ipv4Addr::UNSPECIFIED,
        };
        let mut reply = Reply::controller(ip);
        reply.short_name = name(&self.short_name, SHORT_NAME);
        reply.long_name = name(&self.long_name, LONG_NAME);
        let mut out = Vec::with_capacity(crate::codec::REPLY_LEN);
        reply.encode(&mut out);
        self.socket.send_to(&out, to)?;
        Ok(())
    }
}
