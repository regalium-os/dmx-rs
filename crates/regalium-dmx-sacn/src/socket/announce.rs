// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Universe discovery: announcing what this socket sends, and remembering
//! what other sources announce.

use super::{Error, Sacn};
use crate::codec::{Cid, DISCOVERY_UNIVERSE, DiscoveryPacket, PAGE, list, name, text};
use std::time::{Duration, Instant};

/// How often a source announces its universes: E1.31's ten seconds.
pub const ANNOUNCE: Duration = Duration::from_secs(10);

/// Sources remembered before new ones are ignored. Announcements arrive
/// from the network, so the table is bounded.
pub const MAX_SOURCES: usize = 1024;

/// A source, as its discovery packets describe it.
///
/// `Clone`, `Send` and `Sync`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    /// Its CID.
    pub cid: Cid,
    /// Its name, as UTF-8 where it is.
    pub name: String,
    /// Every universe it lists, across its pages, in the order received.
    pub universes: Vec<u16>,
}

#[derive(Debug, Default)]
pub(super) struct Sources(Vec<Source>);

impl Sources {
    /// Records one page. Page 0 starts the source's list afresh.
    pub(super) fn record(&mut self, packet: &DiscoveryPacket<'_>) {
        let at = match self.0.iter().position(|s| s.cid == packet.cid) {
            Some(at) => at,
            None if self.0.len() < MAX_SOURCES => {
                self.0.push(Source {
                    cid: packet.cid,
                    name: String::new(),
                    universes: Vec::new(),
                });
                self.0.len() - 1
            }
            None => return,
        };
        let Some(source) = self.0.get_mut(at) else {
            return;
        };
        source.name = String::from_utf8_lossy(text(packet.source_name)).into_owned();
        if packet.page == 0 {
            source.universes.clear();
        }
        // Bounded: at most 256 pages of 512 universes.
        if source.universes.len() < 256 * PAGE {
            source.universes.extend(packet.universes());
        }
    }
}

impl Sacn {
    /// Every source that has announced its universes, by its latest list.
    #[must_use]
    pub fn sources(&self) -> Vec<Source> {
        self.state().sources.0.clone()
    }

    pub(super) fn record(&self, packet: &DiscoveryPacket<'_>) {
        self.state().sources.record(packet);
    }

    /// Announces the universes being sent now, in pages, on the discovery
    /// universe. [`send`](regalium_dmx_core::transport::Transport::send)
    /// does this every [`ANNOUNCE`]; call it to announce sooner.
    ///
    /// # Errors
    ///
    /// When the socket failed.
    pub fn announce(&self) -> Result<(), Error> {
        let universes: Vec<u16> = {
            let mut state = self.state();
            state.announced = Some(Instant::now());
            state.sending.iter().copied().collect()
        };
        let pages: Vec<&[u16]> = universes.chunks(PAGE).collect();
        let last_page = u8::try_from(pages.len().saturating_sub(1)).unwrap_or(u8::MAX);
        let to = self.destination(DISCOVERY_UNIVERSE);
        for (page, chunk) in pages.iter().enumerate().take(256) {
            let mut octets = Vec::with_capacity(2 * chunk.len());
            list(chunk, &mut octets);
            let packet = DiscoveryPacket {
                cid: self.cid,
                source_name: name(&self.name),
                reserved: [0; 4],
                page: u8::try_from(page).unwrap_or(u8::MAX),
                last_page,
                list: &octets,
            };
            let mut out = Vec::with_capacity(120 + octets.len());
            packet.encode(&mut out);
            self.socket.send_to(&out, to)?;
        }
        Ok(())
    }

    pub(super) fn announce_if_due(&self) -> Result<(), Error> {
        let due = self
            .state()
            .announced
            .is_none_or(|at| at.elapsed() >= ANNOUNCE);
        if due { self.announce() } else { Ok(()) }
    }
}
