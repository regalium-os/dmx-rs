// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! One step of the input loop: receive, merge, expire, record.

use super::merged::{Listening, MergeState};
use super::{Inner, MAX_WAIT};
use crate::clock::{Clock, lock};
use crate::event::Event;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::transport::{Received, Transport};
use regalium_dmx_telemetry::shape::{self, Direction, UniverseId};
use regalium_dmx_telemetry::{Attribute, Level, SpanId, keys};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

impl<T: Transport, C: Clock> Inner<T, C> {
    pub(crate) fn step(&self, timeout: Duration) -> bool {
        let mut buf = [0u8; 513];
        let received = self.transport.receive(&mut buf, timeout);
        let ok = received.is_ok();
        match received {
            Ok(Some(got)) => {
                self.recovered();
                self.deliver(got, buf.get(..got.len).unwrap_or_default());
            }
            Ok(None) => self.recovered(),
            Err(e) => self.failed(&e.to_string()),
        }
        self.expire(self.clock.now());
        ok
    }

    /// How long the next receive may block: until the next source would
    /// time out, and never more than [`MAX_WAIT`].
    pub(crate) fn next_wait(&self) -> Duration {
        let now = self.clock.now();
        let timeout = self.transport.source_timeout();
        lock(&self.listening)
            .iter()
            .flat_map(|l| lock(&l.state).heard.iter().map(|h| h.1).collect::<Vec<_>>())
            .map(|heard| heard.saturating_add(timeout).saturating_sub(now))
            .fold(MAX_WAIT, Duration::min)
    }

    fn deliver(&self, got: Received, octets: &[u8]) {
        let protocol = self.transport.protocol();
        shape::record_packet(
            self.meter.as_ref(),
            protocol,
            Direction::Received,
            octets.len(),
        );
        let Some(listening) = self.find(got.universe) else {
            return;
        };
        // RDM and other alternate start codes arrive with Phase 6b; a merge
        // refuses them, so they are dropped here.
        let Ok(frame) = Frame::decode(octets) else {
            return;
        };
        let now = self.clock.now();
        let id = UniverseId::new(protocol, got.universe);
        let mut state = lock(&listening.state);
        if got.ended {
            self.drop_source(&id, got.source, &mut state);
            return;
        }
        let before = state.merge.sources();
        // A source past the merge's limit is not merged; the ones already
        // there keep the universe.
        if state
            .merge
            .update_at(got.source, got.priority, &frame)
            .is_err()
        {
            return;
        }
        match state.heard.iter_mut().find(|h| h.0 == got.source) {
            Some(heard) => heard.1 = now,
            None => state.heard.push((got.source, now)),
        }
        if state.merge.sources() != before {
            self.source_event(&id, got.source, true, &state);
        }
        let MergeState {
            merge, universe, ..
        } = &mut *state;
        merge.render(universe);
        let meter = self.meter.as_ref();
        shape::record_frame(meter, &id, Direction::Received);
        if let Some(last) = state.last_frame.replace(now) {
            shape::record_frame_interval(
                meter,
                &id,
                now.saturating_sub(last).as_secs_f64() * 1000.0,
            );
        }
        shape::record_universe_slots(meter, &id, state.universe.len());
    }

    /// Drops every source silent longer than the transport's timeout.
    fn expire(&self, now: Duration) {
        let timeout = self.transport.source_timeout();
        let protocol = self.transport.protocol();
        let all: Vec<Arc<Listening>> = lock(&self.listening).clone();
        for listening in all {
            let mut state = lock(&listening.state);
            let id = UniverseId::new(protocol, listening.number);
            let silent: Vec<SourceId> = state
                .heard
                .iter()
                .filter(|h| now.saturating_sub(h.1) > timeout)
                .map(|h| h.0)
                .collect();
            for source in silent {
                self.drop_source(&id, source, &mut state);
            }
        }
    }

    /// Takes `source` out of the merge, timed out or ended, and re-renders.
    fn drop_source(&self, id: &UniverseId<'_>, source: SourceId, state: &mut MergeState) {
        state.heard.retain(|h| h.0 != source);
        if state.merge.remove(source) {
            shape::record_source_lost(self.meter.as_ref(), id);
            self.source_event(id, source, false, state);
            let MergeState {
                merge, universe, ..
            } = state;
            merge.render(universe);
        }
    }

    fn find(&self, universe: u32) -> Option<Arc<Listening>> {
        lock(&self.listening)
            .iter()
            .find(|l| l.number == universe)
            .cloned()
    }

    fn source_event(&self, id: &UniverseId<'_>, source: SourceId, found: bool, state: &MergeState) {
        let (protocol, universe) = (self.transport.protocol(), id.universe);
        let sources = u32::try_from(state.merge.sources()).unwrap_or(u32::MAX);
        shape::record_universe_sources(self.meter.as_ref(), id, sources);
        let event = if found {
            Event::SourceFound {
                protocol,
                universe,
                source,
            }
        } else {
            self.log(Level::Info, &format!("source {} lost", source.0), universe);
            Event::SourceLost {
                protocol,
                universe,
                source,
            }
        };
        self.events.emit(event);
    }

    fn recovered(&self) {
        if self.failing.swap(false, Ordering::AcqRel) {
            let protocol = self.transport.protocol();
            self.events.emit(Event::ReceiveRecovered { protocol });
        }
    }

    fn failed(&self, error: &str) {
        if !self.failing.swap(true, Ordering::AcqRel) {
            let protocol = self.transport.protocol();
            self.log(Level::Warn, &format!("receive failed: {error}"), 0);
            let error = error.to_owned();
            self.events.emit(Event::ReceiveFailed { protocol, error });
        }
    }

    fn log(&self, level: Level, message: &dyn std::fmt::Display, universe: u32) {
        if self.logger.enabled(level) {
            let attributes = [
                Attribute::str(keys::PROTOCOL, self.transport.protocol()),
                Attribute::int(keys::UNIVERSE, universe.into()),
            ];
            self.logger.log(level, message, SpanId::NONE, &attributes);
        }
    }
}
