// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Discovery's binary search against a simulated line: devices answer a
//! branch when unmuted and in range, collide when several do, and mute
//! when asked — unless one is built not to.

use regalium_dmx_core::rdm::{CommandClass, Discovery, MAX_STEPS, Message, Next, Pid, Uid, disc};
use std::collections::BTreeSet;

const CONTROLLER: Uid = Uid::new(0x7a70, 1);

struct Line {
    devices: Vec<(Uid, bool)>,
    stubborn: BTreeSet<Uid>,
    noise: bool,
}

impl Line {
    fn new(uids: &[Uid]) -> Self {
        Self {
            devices: uids.iter().map(|&u| (u, false)).collect(),
            stubborn: BTreeSet::new(),
            noise: false,
        }
    }

    fn branch(&self, lower: Uid, upper: Uid) -> Option<Vec<u8>> {
        if self.noise {
            return Some(vec![0x13, 0x37]);
        }
        let answering: Vec<Uid> = self
            .devices
            .iter()
            .filter(|(u, muted)| !muted && (lower..=upper).contains(u))
            .map(|(u, _)| *u)
            .collect();
        let mut out = Vec::new();
        match answering.as_slice() {
            [] => return None,
            [one] => disc::encode(*one, 7, &mut out),
            // Several at once: what arrives is garbage.
            _ => out.extend_from_slice(&[0xFE, 0xFE, 0xAA, 0x00, 0x00]),
        }
        Some(out)
    }

    fn mute(&mut self, uid: Uid) -> Option<Vec<u8>> {
        if self.stubborn.contains(&uid) {
            return None;
        }
        let device = self.devices.iter_mut().find(|(u, _)| *u == uid)?;
        device.1 = true;
        let mut out = Vec::new();
        Message {
            destination: CONTROLLER,
            source: uid,
            transaction: 0,
            port_or_response: 0,
            message_count: 0,
            sub_device: 0,
            command_class: CommandClass::DISCOVERY_RESPONSE,
            pid: Pid::DISC_MUTE,
            data: &[0, 0],
        }
        .encode(&mut out);
        Some(out)
    }

    fn unmute_all(&mut self) {
        for d in &mut self.devices {
            d.1 = false;
        }
    }
}

/// Runs a search to the end; returns what it found and how many steps.
fn search(line: &mut Line) -> (Discovery, u32) {
    let mut discovery = Discovery::new();
    let mut steps = 0;
    loop {
        steps += 1;
        match discovery.next() {
            Next::Broadcast(request) => {
                assert_eq!(request.pid, Pid::DISC_UN_MUTE);
                line.unmute_all();
                discovery.sent();
            }
            Next::Branch { lower, upper } => {
                discovery.branched(line.branch(lower, upper).as_deref());
            }
            Next::Mute(uid) => discovery.muted(line.mute(uid).as_deref()),
            Next::Done => return (discovery, steps),
        }
    }
}

/// `n` UIDs spread over the space, from a fixed seed.
fn uids(n: usize) -> Vec<Uid> {
    let mut x: u64 = 0x2545_F491_4F6C_DD1D;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            Uid::from_u64(x % Uid::MAX.get())
        })
        .collect()
}

#[test]
fn an_empty_line_takes_one_branch() {
    let (found, steps) = search(&mut Line::new(&[]));
    assert_eq!(found.found(), []);
    assert_eq!(steps, 3, "un-mute, one empty branch, done");
}

#[test]
fn every_device_on_a_line_of_two_hundred_is_found_once() {
    let mut want = uids(200);
    let (found, steps) = search(&mut Line::new(&want));
    want.sort();
    assert_eq!(found.found(), want);
    assert!(!found.incomplete());
    assert!(steps < 20_000, "{steps} steps");
}

#[test]
fn neighbours_one_uid_apart_are_both_found() {
    let pair = [Uid::new(0x4c55, 7), Uid::new(0x4c55, 8)];
    assert_eq!(search(&mut Line::new(&pair)).0.found(), pair);
}

#[test]
fn a_device_that_will_not_mute_is_kept_and_the_rest_still_found() {
    let mut want = uids(20);
    let mut line = Line::new(&want);
    line.stubborn.insert(want[3]);
    want.sort();
    let (found, _) = search(&mut line);
    assert_eq!(found.found(), want);
}

#[test]
fn a_line_of_noise_ends_flagged_incomplete() {
    let mut line = Line::new(&[]);
    line.noise = true;
    let (found, steps) = search(&mut line);
    assert_eq!(found.found(), []);
    assert!(found.incomplete(), "noise never empties the search");
    assert_eq!(steps, MAX_STEPS + 2, "un-mute, the cap, done");
}
