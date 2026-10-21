// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The packets the probe saw, for the Traffic tab.
//!
//! Level packets — `ArtDmx`, sACN data — go by at 44 Hz a universe and
//! would push everything else out of any buffer, so they are counted, not
//! kept; everything else is kept whole, newest last.

use super::model::{PACKETS, Packet, Protocol};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::Instant;

/// What the probe saw. `Send` and `Sync`: transports record from their own
/// threads.
#[derive(Debug)]
pub struct Capture {
    start: Instant,
    kept: Mutex<VecDeque<Packet>>,
    levels: AtomicU64,
}

impl Default for Capture {
    fn default() -> Self {
        Self::new()
    }
}

impl Capture {
    /// Empty, its clock starting now.
    #[must_use]
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            kept: Mutex::new(VecDeque::new()),
            levels: AtomicU64::new(0),
        }
    }

    /// Records a packet. `levels` marks one of the many that only carry
    /// levels, which is counted rather than kept.
    pub fn record(&self, sent: bool, protocol: Protocol, peer: &str, octets: &[u8], levels: bool) {
        if levels {
            self.levels.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let packet = Packet {
            time: self.time(),
            sent,
            protocol,
            peer: peer.to_owned(),
            octets: octets.to_vec(),
        };
        let mut kept = self.kept.lock().unwrap_or_else(PoisonError::into_inner);
        if kept.len() == PACKETS {
            kept.pop_front();
        }
        kept.push_back(packet);
    }

    /// Everything kept, oldest first.
    #[must_use]
    pub fn packets(&self) -> VecDeque<Packet> {
        self.kept
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Level packets counted so far.
    #[must_use]
    pub fn level_packets(&self) -> u64 {
        self.levels.load(Ordering::Relaxed)
    }

    /// The time since the probe started, as `HH:MM:SS.mmm`.
    #[must_use]
    pub fn time(&self) -> String {
        let t = self.start.elapsed();
        let s = t.as_secs();
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            s / 3600,
            (s / 60) % 60,
            s % 60,
            t.subsec_millis()
        )
    }
}
