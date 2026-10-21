// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! One step of the output loop: ask each universe's refresher, send what is
//! due, and record what happened.
//!
//! The universe's lock is held only to copy its levels into the slot's
//! buffer; the send happens with it released, so a serial write that takes a
//! whole frame time never blocks a caller setting a level. What is copied
//! passes through the output policy on the way (see [`crate::policy`]).

use crate::clock::{Clock, lock};
use crate::event::Event;
use crate::levels::{Slot, SlotState};
use crate::output::Inner;
use crate::policy::{Rules, scale};
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::refresh::Step;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::Transport;
use regalium_dmx_core::universe::Universe;
use regalium_dmx_telemetry::shape::{self, Direction, UniverseId};
use regalium_dmx_telemetry::{Attribute, Level, SpanId, keys};
use std::sync::Arc;
use std::time::Duration;

/// Fills the slot's buffer with what goes out under `rules`: zeros in
/// blackout, the held frame under hold, else the levels scaled by master.
fn compose(rules: &Rules, universe: &Universe, state: &mut SlotState) {
    if rules.hold && !rules.blackout && !state.held.is_empty() {
        state.buf.clone_from(&state.held);
        return;
    }
    state.buf.clear();
    universe.frame().encode(&mut state.buf);
    let levels = state.buf.first() == Some(&StartCode::NULL.0);
    if let Some((_, slots)) = state.buf.split_first_mut() {
        if rules.blackout {
            slots.fill(0);
            return;
        }
        if levels && rules.master != u8::MAX {
            for slot in slots {
                *slot = scale(*slot, rules.master);
            }
        }
    }
    state.held.clone_from(&state.buf);
}

impl<T: Transport, C: Clock> Inner<T, C> {
    pub(crate) fn step(&self) -> Option<Duration> {
        let now = self.clock.now();
        let slots: Vec<Arc<Slot>> = lock(&self.slots).clone();
        slots
            .iter()
            .map(|slot| self.refresh(slot, now))
            .reduce(Duration::min)
    }

    /// Sends `slot` if it is due at `now`; returns its next deadline.
    fn refresh(&self, slot: &Slot, now: Duration) -> Duration {
        let mut state = lock(&slot.state);
        let state = &mut *state;
        let previous = state.refresher.last_sent();
        let rules = self.policy.rules();
        let generation = {
            let universe = lock(&slot.universe);
            // What a receiver would see moves with the policy, and with the
            // levels unless they are held.
            if rules.generation != state.policy_seen {
                state.policy_seen = rules.generation;
                state.counter += 1;
            }
            if !rules.hold && universe.generation() != state.universe_seen {
                state.universe_seen = universe.generation();
                state.counter += 1;
            }
            let generation = state.counter;
            match state.refresher.next(now, generation) {
                Step::Idle { until } => return until,
                Step::Send => {
                    compose(&rules, &universe, state);
                    generation
                }
            }
        };
        if let Ok(frame) = Frame::decode(&state.buf) {
            let sent = self.transport.send(slot.number, &frame);
            if self.after_send(
                slot.number,
                &mut state.failing,
                sent.map_err(|e| e.to_string()),
            ) {
                self.record(slot.number, &frame, previous.map(|p| now.saturating_sub(p)));
            }
        }
        // Just recorded as sent at `now`, so this is the next deadline.
        match state.refresher.next(now, generation) {
            Step::Idle { until } => until,
            Step::Send => now.saturating_add(state.refresher.policy().interval),
        }
    }

    /// Reports the edge when a universe starts or stops failing; nothing in
    /// between, so a dead port is one event and one log line, not 44 a second.
    /// Returns whether the send succeeded.
    fn after_send(&self, universe: u32, failing: &mut bool, sent: Result<(), String>) -> bool {
        let protocol = self.transport.protocol();
        let ok = sent.is_ok();
        match (sent, *failing) {
            (Ok(()), false) | (Err(_), true) => {}
            (Ok(()), true) => {
                *failing = false;
                self.events
                    .emit(Event::SendRecovered { protocol, universe });
                self.log(Level::Info, &"send recovered", protocol, universe);
            }
            (Err(error), false) => {
                *failing = true;
                self.log(
                    Level::Warn,
                    &format!("send failed: {error}"),
                    protocol,
                    universe,
                );
                self.events.emit(Event::SendFailed {
                    protocol,
                    universe,
                    error,
                });
            }
        }
        ok
    }

    /// The frame's series: counted, timed since the last send, its packet
    /// and its slot count.
    fn record(&self, universe: u32, frame: &Frame<'_>, since: Option<Duration>) {
        let protocol = self.transport.protocol();
        let id = UniverseId::new(protocol, universe);
        let meter = self.meter.as_ref();
        shape::record_frame(meter, &id, Direction::Sent);
        if let Some(since) = since {
            shape::record_frame_interval(meter, &id, since.as_secs_f64() * 1000.0);
        }
        shape::record_packet(meter, protocol, Direction::Sent, frame.octets());
        let slots = u16::try_from(frame.slots().len()).unwrap_or(u16::MAX);
        shape::record_universe_slots(meter, &id, slots);
    }

    fn log(&self, level: Level, message: &dyn std::fmt::Display, protocol: &str, universe: u32) {
        if self.logger.enabled(level) {
            self.logger.log(
                level,
                message,
                SpanId::NONE,
                &[
                    Attribute::str(keys::PROTOCOL, protocol),
                    Attribute::int(keys::UNIVERSE, universe.into()),
                ],
            );
        }
    }
}
