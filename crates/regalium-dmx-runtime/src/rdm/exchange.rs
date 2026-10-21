// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! One request, to its answer: retries, overflow and the queue.

use super::{Controller, RETRIES, RdmError};
use crate::clock::Clock;
use regalium_dmx_core::rdm::{CommandClass, MAX_LEN, Message, Pid, Reply, Request, reply};
use regalium_dmx_core::transport::{Answer, Expect, RdmLine};
use regalium_dmx_telemetry::shape;
use regalium_dmx_telemetry::{Attribute, SpanId, keys};
use std::sync::atomic::Ordering;
use std::time::Duration;

/// `ACK_OVERFLOW` parts accepted before the answer is given up on.
pub const MAX_PARTS: usize = 16;

/// Times the queue is asked after `ACK_TIMER` before giving up.
pub const QUEUE_POLLS: usize = 5;

/// `QUEUED_MESSAGE`'s status type: `STATUS_ERROR`, asking for everything
/// queued.
const STATUS_ERROR: u8 = 0x04;

impl<L: RdmLine, C: Clock> Controller<L, C> {
    /// Sends `request` and returns the answer's data. A broadcast returns
    /// at once with none, as nothing answers one.
    ///
    /// # Errors
    ///
    /// [`RdmError::Nack`] when refused; [`RdmError::NoAnswer`] or
    /// [`RdmError::Garbled`] after [`RETRIES`]; [`RdmError::Line`] when the
    /// line failed.
    pub fn request(
        &self,
        universe: u32,
        request: &Request<'_>,
    ) -> Result<Vec<u8>, RdmError<L::Error>> {
        let span = self.tracer.start("dmx.rdm.request", SpanId::NONE);
        let uid = request.destination.to_string();
        let pid = request.pid.to_string();
        self.tracer
            .attribute(span, Attribute::str(keys::RDM_UID, &uid));
        self.tracer
            .attribute(span, Attribute::str(keys::RDM_PID, &pid));
        let started = self.clock.now();
        let result = self.follow(universe, request);
        let elapsed = self.clock.now().saturating_sub(started).as_secs_f64() * 1000.0;
        let command = request.command_class.name().unwrap_or("other");
        let outcome = result.as_ref().map_or_else(RdmError::outcome, |_| "ack");
        shape::record_rdm_request(self.meter.as_ref(), command, &pid, outcome, &uid, elapsed);
        let error = result.as_ref().err().map(ToString::to_string);
        self.tracer
            .end(span, error.as_ref().map(|e| e as &dyn std::fmt::Display));
        result
    }

    /// Asks, joining overflow parts and following the queue after a timer.
    /// From the queue, only an answer for the parameter asked is taken; any
    /// other is another request's, and the queue is asked again.
    fn follow(&self, universe: u32, request: &Request<'_>) -> Result<Vec<u8>, RdmError<L::Error>> {
        if !request.expects_response() {
            self.send(universe, request, Expect::Nothing, &mut [0; 0])
                .map_err(RdmError::Line)?;
            return Ok(Vec::new());
        }
        let status = [STATUS_ERROR];
        let queue = Request {
            pid: Pid::QUEUED_MESSAGE,
            command_class: CommandClass::GET,
            data: &status,
            ..*request
        };
        let mut data = Vec::new();
        let (mut asking, mut parts, mut polls) = (*request, 0, 0);
        loop {
            let (done, pid, part) = self.once(universe, &asking)?;
            let ours = pid == request.pid;
            // A timer's data is its wait, not part of the answer.
            if ours && matches!(done, Done::Ack | Done::Overflow) {
                data.extend_from_slice(&part);
            }
            match done {
                Done::Ack if ours => return Ok(data),
                Done::Overflow if ours => {
                    parts += 1;
                    if parts >= MAX_PARTS {
                        return Err(RdmError::Overflow);
                    }
                    continue;
                }
                Done::Timer(wait) => self.pause(wait),
                // The queue held something else, or nothing yet.
                Done::Ack | Done::Overflow => self.pause(Duration::from_millis(100)),
            }
            asking = queue;
            polls += 1;
            if polls > QUEUE_POLLS {
                return Err(RdmError::Queued);
            }
        }
    }

    fn pause(&self, wait: Duration) {
        self.clock
            .wait(&self.wake, self.clock.now().saturating_add(wait));
    }

    /// One question, retried, to one reply: how it ended, the parameter it
    /// answered, and its data.
    fn once(
        &self,
        universe: u32,
        request: &Request<'_>,
    ) -> Result<(Done, Pid, Vec<u8>), RdmError<L::Error>> {
        let mut buf = [0u8; MAX_LEN];
        let mut heard = false;
        for _ in 0..=RETRIES {
            let (tn, answer) = self
                .send(universe, request, Expect::Message, &mut buf)
                .map_err(RdmError::Line)?;
            let Answer::Octets(n) = answer else { continue };
            heard = true;
            let Ok(got) = Message::decode(buf.get(..n).unwrap_or_default()) else {
                continue;
            };
            let sent = request.message(self.uid, tn, self.port);
            let done = match reply(&sent, &got) {
                Ok(Reply::Nack(reason)) => return Err(RdmError::Nack(reason)),
                Ok(Reply::AckTimer(wait)) => Done::Timer(wait.max(Duration::from_millis(100))),
                Ok(Reply::AckOverflow(_)) => Done::Overflow,
                Ok(Reply::Ack(_)) => Done::Ack,
                Err(_) => continue,
            };
            return Ok((done, got.pid, got.data.to_vec()));
        }
        Err(if heard {
            RdmError::Garbled
        } else {
            RdmError::NoAnswer
        })
    }

    /// Numbers and sends `request`; returns the transaction used and the
    /// answer.
    pub(crate) fn send(
        &self,
        universe: u32,
        request: &Request<'_>,
        expect: Expect,
        buf: &mut [u8],
    ) -> Result<(u8, Answer), L::Error> {
        let tn = self.transaction.fetch_add(1, Ordering::Relaxed);
        let mut out = Vec::with_capacity(MAX_LEN);
        request.message(self.uid, tn, self.port).encode(&mut out);
        Ok((tn, self.line.rdm(universe, &out, expect, buf)?))
    }
}

/// How one exchange ended.
enum Done {
    Ack,
    Overflow,
    Timer(Duration),
}
