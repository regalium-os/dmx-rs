// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The event feed: runtime events, and the engine's own notes, as lines.

use super::Worker;
use super::model::{EVENTS, Event, Severity};
use regalium_dmx::runtime::Event as RuntimeEvent;

impl Worker {
    /// Every runtime event, as a line of the feed.
    pub(crate) fn events(&mut self) {
        let mut lines = Vec::new();
        for events in &self.rig.events {
            while let Some(event) = events.try_next() {
                lines.push(event);
            }
        }
        for event in lines {
            let name = |protocol: &str, universe: u32| {
                self.rig
                    .entries
                    .iter()
                    .find(|e| e.protocol.name() == protocol && e.number == universe)
                    .map_or_else(|| format!("{protocol} {universe}"), |e| e.name.clone())
            };
            let (kind, text, severity) = match event {
                RuntimeEvent::SendFailed {
                    protocol,
                    universe,
                    error,
                } => {
                    self.failing.insert(name(protocol, universe));
                    (
                        "send failed",
                        format!("{} · {error}", name(protocol, universe)),
                        Severity::Bad,
                    )
                }
                RuntimeEvent::SendRecovered { protocol, universe } => {
                    self.failing.remove(&name(protocol, universe));
                    ("send recovered", name(protocol, universe), Severity::Good)
                }
                RuntimeEvent::ReceiveFailed { protocol, error } => (
                    "receive failed",
                    format!("{protocol} · {error}"),
                    Severity::Bad,
                ),
                RuntimeEvent::ReceiveRecovered { protocol } => {
                    ("receive recovered", protocol.to_owned(), Severity::Good)
                }
                RuntimeEvent::SourceFound {
                    protocol,
                    universe,
                    source,
                } => (
                    "source found",
                    format!("{} · {:x}", name(protocol, universe), source.0),
                    Severity::Good,
                ),
                RuntimeEvent::SourceLost {
                    protocol,
                    universe,
                    source,
                } => (
                    "source lost",
                    format!("{} · {:x}", name(protocol, universe), source.0),
                    Severity::Warn,
                ),
                other => ("event", format!("{other:?}"), Severity::Info),
            };
            self.note(kind, &text, severity);
        }
    }

    /// Adds a line to the feed, newest first.
    pub(crate) fn note(&mut self, kind: &str, text: &str, severity: Severity) {
        if self.feed.len() == EVENTS {
            self.feed.pop_back();
        }
        let time = self
            .rig
            .capture
            .time()
            .get(..8)
            .unwrap_or_default()
            .to_owned();
        self.feed.push_front(Event {
            time,
            kind: kind.to_owned(),
            text: text.to_owned(),
            severity,
        });
    }
}
