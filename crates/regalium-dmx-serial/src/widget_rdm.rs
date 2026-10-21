// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM through the widget: it sends the request between DMX packets, times
//! the turnaround, and returns what it heard as Received DMX.
//!
//! The exchange holds the port's reader for its whole window, so the answer
//! cannot be taken by a concurrent receive; DMX that arrives meanwhile is
//! queued for that receive, not lost.

use crate::Error;
use crate::enttec::Label;
use crate::widget::{Enttec, PENDING, UNIVERSE, lock};
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::{Answer, Expect, RdmLine};
use std::time::{Duration, Instant};

/// How long the widget has to return an answer: its own turnaround is a few
/// milliseconds; the rest is USB.
pub const RDM_WINDOW: Duration = Duration::from_millis(100);

impl RdmLine for Enttec {
    type Error = Error;

    fn protocol(&self) -> &'static str {
        "serial"
    }

    fn rdm(
        &self,
        universe: u32,
        request: &[u8],
        expect: Expect,
        buf: &mut [u8],
    ) -> Result<Answer, Error> {
        if universe != UNIVERSE {
            return Err(Error::Universe(universe));
        }
        let label = match expect {
            Expect::Discovery => Label::SEND_RDM_DISCOVERY,
            Expect::Nothing | Expect::Message => Label::SEND_RDM,
        };
        let mut state = lock(&self.framer);
        let mut chunk = [0u8; 256];
        // Whatever is already waiting answers an earlier request — a late
        // reply, or a timeout reported for a broadcast — and must not be
        // read as this one's answer. Levels among it are kept for receive.
        loop {
            let n = self.port.read(&mut chunk, Duration::ZERO)?;
            if n == 0 {
                break;
            }
            let (framer, pending) = &mut *state;
            framer.feed(chunk.get(..n).unwrap_or_default(), |m| {
                if let (Label::RECEIVED_DMX, Some((0, heard @ [0x00, ..]))) =
                    (m.label, m.data.split_first())
                {
                    queue(pending, heard);
                }
            });
        }
        self.write(label, request)?;
        if expect == Expect::Nothing {
            return Ok(Answer::Silent);
        }
        let deadline = Instant::now() + RDM_WINDOW;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Ok(Answer::Silent);
            }
            let n = self.port.read(&mut chunk, left)?;
            let (framer, pending) = &mut *state;
            let mut answer: Option<Answer> = None;
            framer.feed(chunk.get(..n).unwrap_or_default(), |m| {
                if answer.is_some() {
                    return;
                }
                match (m.label, m.data.split_first()) {
                    (Label::RDM_TIMEOUT, _) => answer = Some(Answer::Silent),
                    (Label::RECEIVED_DMX, Some((_, heard))) => match (expect, heard.first()) {
                        (Expect::Discovery, None) => answer = Some(Answer::Silent),
                        (Expect::Message, Some(&code)) if code == StartCode::RDM.0 => {
                            answer = Some(copy(heard, buf));
                        }
                        (Expect::Discovery, Some(&code)) if code != StartCode::NULL.0 => {
                            answer = Some(copy(heard, buf));
                        }
                        // Levels arriving during the window: for receive.
                        _ => queue(pending, heard),
                    },
                    _ => {}
                }
            });
            if let Some(answer) = answer {
                return Ok(answer);
            }
        }
    }
}

/// Keeps received levels for the next receive, dropping the oldest when full.
fn queue(pending: &mut std::collections::VecDeque<Vec<u8>>, heard: &[u8]) {
    if pending.len() == PENDING {
        pending.pop_front();
    }
    pending.push_back(heard.to_vec());
}

fn copy(heard: &[u8], buf: &mut [u8]) -> Answer {
    let mut n = 0;
    for (to, from) in buf.iter_mut().zip(heard) {
        *to = *from;
        n += 1;
    }
    Answer::Octets(n)
}
