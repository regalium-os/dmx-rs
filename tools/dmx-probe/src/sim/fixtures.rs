// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A line of RDM fixtures in memory: the core's responders, answering as
//! E1.20 devices do, colliding when several answer one branch.

use crate::engine::Capture;
use crate::engine::model::Protocol;
use regalium_dmx::rdm::{DeviceInfo, Responder, Response, Uid};
use regalium_dmx::transport::{Answer, Expect, RdmLine};
use std::sync::{Arc, Mutex, PoisonError};

/// The fixtures on one line.
pub struct Line {
    devices: Mutex<Vec<Responder>>,
    capture: Arc<Capture>,
}

/// Never: the line in memory does not fail.
#[derive(Debug)]
pub struct Never;

impl std::fmt::Display for Never {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("never")
    }
}

impl std::error::Error for Never {}

/// The rig the mock shows: nineteen 16-slot spots from address 1, and five
/// 4-slot washes after them — one of which, Wash 3, sits on Spot 7 at 97.
#[must_use]
pub fn stage() -> Vec<Responder> {
    let mut devices = Vec::new();
    for n in 1..=19u16 {
        let info = DeviceInfo {
            protocol: 0x0100,
            model: 0x0110,
            category: 0x0101,
            software: 0x0003_0000,
            footprint: 16,
            personality: 2,
            personalities: 4,
            start_address: 1 + 16 * (n - 1),
            sub_devices: 0,
            sensors: 0,
        };
        devices.push(
            Responder::new(Uid::new(0x4c55, 0x100 + u32::from(n)), info)
                .labelled(&format!("Spot {n}")),
        );
    }
    for (n, address) in [(1u32, 305u16), (2, 309), (3, 97), (4, 313), (5, 317)] {
        let info = DeviceInfo {
            protocol: 0x0100,
            model: 0x0203,
            category: 0x0101,
            software: 0x0002_0100,
            footprint: 4,
            personality: 1,
            personalities: 3,
            start_address: address,
            sub_devices: 0,
            sensors: 1,
        };
        devices
            .push(Responder::new(Uid::new(0x4c55, 0x200 + n), info).labelled(&format!("Wash {n}")));
    }
    devices
}

impl Line {
    /// `devices` on a line, its traffic recorded on `capture`.
    #[must_use]
    pub fn new(devices: Vec<Responder>, capture: &Arc<Capture>) -> Self {
        Self {
            devices: Mutex::new(devices),
            capture: Arc::clone(capture),
        }
    }
}

impl RdmLine for Line {
    type Error = Never;

    fn protocol(&self) -> &'static str {
        "serial"
    }

    fn rdm(&self, _: u32, request: &[u8], expect: Expect, buf: &mut [u8]) -> Result<Answer, Never> {
        self.capture
            .record(true, Protocol::Serial, "line", request, false);
        let answers: Vec<Vec<u8>> = self
            .devices
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter_mut()
            .filter_map(|d| match d.answer(request) {
                Response::Silent => None,
                Response::Discovery(o) | Response::Message(o) => Some(o),
            })
            .collect();
        if expect == Expect::Nothing || answers.is_empty() {
            return Ok(Answer::Silent);
        }
        // Several at once: the line carries the OR of what they drive.
        let len = answers.iter().map(Vec::len).max().unwrap_or(0);
        let heard: Vec<u8> = (0..len)
            .map(|i| {
                answers
                    .iter()
                    .fold(0, |acc, a| acc | a.get(i).copied().unwrap_or(0))
            })
            .collect();
        if expect == Expect::Message {
            self.capture
                .record(false, Protocol::Serial, "line", &heard, false);
        }
        let mut n = 0;
        for (to, from) in buf.iter_mut().zip(&heard) {
            *to = *from;
            n += 1;
        }
        Ok(Answer::Octets(n))
    }
}
