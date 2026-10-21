// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! The first live output: two consoles send Art-Net over loopback, each on
//! its own output thread, and an input merges them on its own thread, all
//! on the system clock.

use regalium_dmx_artnet::ArtNet;
use regalium_dmx_core::address::Address;
use regalium_dmx_core::merge::{Merge, Mode};
use regalium_dmx_runtime::{Input, Levels, Output, Running, SystemClock};
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

fn socket(target: SocketAddr) -> Arc<ArtNet> {
    let bind = "127.0.0.1:0".parse().unwrap();
    Arc::new(ArtNet::builder(bind, target).open().unwrap())
}

/// A console sending universe 1 to `to`, with these levels.
fn console(to: SocketAddr, levels: &[u8]) -> (Levels, Running) {
    let (output, _events) = Output::builder(socket(to), SystemClock::new()).build();
    let patch = output.patch(1);
    patch.set_len(u16::try_from(levels.len()).unwrap()).unwrap();
    patch.write(Address::FIRST, levels).unwrap();
    (patch, output.start().unwrap())
}

#[test]
fn two_consoles_over_art_net_merge_highest_first_on_a_listening_input() {
    let rx = socket("127.0.0.1:9".parse().unwrap());
    let to = rx.local_addr().unwrap();
    let (input, _events) = Input::builder(Arc::clone(&rx), SystemClock::new()).build();
    let merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    let mut listening = input.start().unwrap();

    let (a, mut a_running) = console(to, &[200, 0, 30, 0]);
    let (_b, mut b_running) = console(to, &[100, 150, 0, 0]);
    assert!(
        eventually(|| merged.snapshot().levels() == [200, 150, 30, 0]),
        "{:?}",
        merged.snapshot().levels()
    );
    assert_eq!(merged.sources(), 2);

    // A change on one console reaches the merge at the next interval.
    a.set(Address::new(4).unwrap(), 77);
    assert!(eventually(
        || merged.snapshot().levels() == [200, 150, 30, 77]
    ));

    a_running.close();
    b_running.close();
    listening.close();
    assert!(input.is_closed());
}
