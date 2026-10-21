// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! sACN end to end on loopback, unicast: two consoles at different
//! priorities, one input. The higher holds the universe however bright the
//! lower; when it unpatches, its stream-terminated packets hand the
//! universe over at once, not after the 2.5 s timeout.

use regalium_dmx_core::address::Address;
use regalium_dmx_core::merge::{Merge, Mode};
use regalium_dmx_runtime::{Input, Output, Running, SystemClock};
use regalium_dmx_sacn::Sacn;
use regalium_dmx_sacn::codec::Cid;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn eventually(mut done: impl FnMut() -> bool) -> bool {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if done() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    false
}

fn loopback() -> SocketAddr {
    "127.0.0.1:0".parse().unwrap()
}

/// A console at `priority` sending universe 1 to `to`.
fn console(
    to: SocketAddr,
    cid: u8,
    priority: u8,
    level: u8,
) -> (Output<Arc<Sacn>, SystemClock>, Running) {
    let socket = Sacn::builder(loopback(), Cid([cid; 16]))
        .priority(priority)
        .unicast(to)
        .open()
        .unwrap();
    let (output, _events) = Output::builder(Arc::new(socket), SystemClock::new()).build();
    let patch = output.patch(1);
    patch.set_len(1).unwrap();
    patch.set(Address::FIRST, level);
    let running = output.start().unwrap();
    (output, running)
}

#[test]
fn the_higher_priority_holds_the_universe_and_hands_it_over_when_it_ends() {
    let rx = Arc::new(Sacn::builder(loopback(), Cid([1; 16])).open().unwrap());
    let to = rx.local_addr().unwrap();
    let (input, _events) = Input::builder(Arc::clone(&rx), SystemClock::new()).build();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    let mut listening = input.start().unwrap();

    let (high, mut high_running) = console(to, 2, 150, 20);
    let (_low, mut low_running) = console(to, 3, 100, 255);
    assert!(eventually(|| merged.sources() == 2));
    assert_eq!(
        merged.snapshot().levels(),
        [20],
        "150 beats 100, however bright"
    );

    let handed = Instant::now();
    assert!(high.unpatch(1));
    assert!(eventually(|| merged.snapshot().levels() == [255]));
    assert!(
        handed.elapsed() < Duration::from_secs(2),
        "terminated, not timed out: {:?}",
        handed.elapsed()
    );
    assert_eq!(merged.sources(), 1);

    high_running.close();
    low_running.close();
    listening.close();
}
