// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An RDM line in memory: responders on it answer as devices would, and
//! answers to one branch from several of them collide, as on a real line.

use regalium_dmx_core::rdm::{Responder, Response};
use regalium_dmx_core::transport::{Answer, Expect, RdmLine};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub(crate) struct Never;

impl std::fmt::Display for Never {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("never")
    }
}

impl std::error::Error for Never {}

/// The line: its responders, and how many answers to swallow first.
#[derive(Default)]
pub(crate) struct Bench {
    pub(crate) devices: Mutex<Vec<Responder>>,
    pub(crate) drop_next: AtomicU32,
    pub(crate) requests: AtomicU32,
}

impl Bench {
    pub(crate) fn with(devices: Vec<Responder>) -> Self {
        Self {
            devices: Mutex::new(devices),
            ..Self::default()
        }
    }
}

impl RdmLine for Bench {
    type Error = Never;

    fn protocol(&self) -> &'static str {
        "serial"
    }

    fn rdm(&self, _: u32, request: &[u8], expect: Expect, buf: &mut [u8]) -> Result<Answer, Never> {
        self.requests.fetch_add(1, Ordering::Relaxed);
        let answers: Vec<Vec<u8>> = self
            .devices
            .lock()
            .unwrap()
            .iter_mut()
            .filter_map(|d| match d.answer(request) {
                Response::Silent => None,
                Response::Discovery(o) | Response::Message(o) => Some(o),
            })
            .collect();
        if expect == Expect::Nothing || answers.is_empty() {
            return Ok(Answer::Silent);
        }
        let dropping = self.drop_next.load(Ordering::Relaxed);
        if dropping > 0 {
            self.drop_next.store(dropping - 1, Ordering::Relaxed);
            return Ok(Answer::Silent);
        }
        // Several at once: the line carries the OR of what they drive.
        let len = answers.iter().map(Vec::len).max().unwrap_or(0);
        let mixed: Vec<u8> = (0..len)
            .map(|i| {
                answers
                    .iter()
                    .fold(0, |acc, a| acc | a.get(i).copied().unwrap_or(0))
            })
            .collect();
        buf[..mixed.len()].copy_from_slice(&mixed);
        Ok(Answer::Octets(mixed.len()))
    }
}
