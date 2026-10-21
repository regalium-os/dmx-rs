// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What an output or an input tells its caller: a transport that started
//! failing and one that recovered; a merge source that appeared, and one
//! that fell silent.
//!
//! Edge-triggered: a dead port is reported once when it dies and once when
//! it comes back, never 44 times a second. Delivered through a bounded queue
//! the output never blocks on: when the reader is slow, new events are
//! dropped and counted ([`crate::Output::dropped_events`]), because a refresh
//! that waits for a log reader is a refresh that flickers.

use regalium_dmx_core::merge::SourceId;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::time::Duration;

/// Events held for a slow reader before new ones are dropped.
pub const EVENT_QUEUE: usize = 64;

/// Something the caller may want to act on. Non-exhaustive: RDM devices
/// found arrive with Phase 6b.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// Sending `universe` failed, and keeps failing until
    /// [`Event::SendRecovered`]. The loop keeps trying at the refresh rate.
    SendFailed {
        /// The transport's protocol.
        protocol: &'static str,
        /// The universe, as the protocol numbers it.
        universe: u32,
        /// The transport's error, as it described it.
        error: String,
    },
    /// Sending `universe` works again.
    SendRecovered {
        /// The transport's protocol.
        protocol: &'static str,
        /// The universe, as the protocol numbers it.
        universe: u32,
    },
    /// Receiving failed, and keeps failing until
    /// [`Event::ReceiveRecovered`]. The input retries every
    /// [`RETRY`](crate::RETRY).
    ReceiveFailed {
        /// The transport's protocol.
        protocol: &'static str,
        /// The transport's error, as it described it.
        error: String,
    },
    /// Receiving works again.
    ReceiveRecovered {
        /// The transport's protocol.
        protocol: &'static str,
    },
    /// A new source joined `universe`'s merge.
    SourceFound {
        /// The transport's protocol.
        protocol: &'static str,
        /// The universe, as the protocol numbers it.
        universe: u32,
        /// The source, as the transport identifies it.
        source: SourceId,
    },
    /// A source was silent for the transport's source timeout and left the
    /// merge. When it was the last, the universe goes dark.
    SourceLost {
        /// The transport's protocol.
        protocol: &'static str,
        /// The universe, as the protocol numbers it.
        universe: u32,
        /// The source, as the transport identified it.
        source: SourceId,
    },
}

/// The receiving end of an output's events. `Send`, not `Sync`: one reader.
#[derive(Debug)]
pub struct Events {
    rx: Receiver<Event>,
}

impl Events {
    /// The next event, if one is waiting.
    #[must_use]
    pub fn try_next(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }

    /// The next event, waiting at most `timeout`. `None` when the time
    /// passed, or when the output is gone and nothing is left.
    #[must_use]
    pub fn next_timeout(&self, timeout: Duration) -> Option<Event> {
        self.rx.recv_timeout(timeout).ok()
    }
}

/// The sending end, held by the output.
#[derive(Debug)]
pub(crate) struct Sink {
    tx: SyncSender<Event>,
    dropped: AtomicU64,
}

impl Sink {
    pub(crate) fn new() -> (Self, Events) {
        let (tx, rx) = std::sync::mpsc::sync_channel(EVENT_QUEUE);
        let sink = Self {
            tx,
            dropped: AtomicU64::new(0),
        };
        (sink, Events { rx })
    }

    /// Queues `event`, or counts it dropped. Never blocks.
    pub(crate) fn emit(&self, event: Event) {
        match self.tx.try_send(event) {
            Ok(()) | Err(TrySendError::Disconnected(_)) => {}
            Err(TrySendError::Full(_)) => {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub(crate) fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }
}
