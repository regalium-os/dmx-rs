// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Two Art-Net sockets on loopback: what one sends, the other receives, and
//! what is housekeeping stays inside the wait.

use regalium_dmx_artnet::ArtNet;
use regalium_dmx_artnet::codec::{ArtDmx, PortAddress, text};
use regalium_dmx_artnet::socket::Error;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::{Received, Transport};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

const WAIT: Duration = Duration::from_secs(2);

fn loopback() -> SocketAddr {
    "127.0.0.1:0".parse().unwrap()
}

/// A socket that sends to `target`.
fn open(target: SocketAddr) -> ArtNet {
    ArtNet::builder(loopback(), target)
        .name("bench", "Test bench")
        .open()
        .unwrap()
}

/// A receiver, and a sender aimed at it.
fn pair() -> (ArtNet, ArtNet) {
    let rx = open(loopback());
    let tx = open(rx.local_addr().unwrap());
    (rx, tx)
}

fn receive(on: &ArtNet) -> Option<(Received, Vec<u8>)> {
    let mut buf = [0u8; 513];
    let got = on.receive(&mut buf, WAIT).unwrap()?;
    Some((got, buf[..got.len].to_vec()))
}

#[test]
fn a_frame_sent_arrives_as_the_same_frame_padded_to_an_even_length() {
    let (rx, tx) = pair();
    let slots = [10, 20, 30];
    tx.send(0x1234, &Frame::new(StartCode::NULL, &slots).unwrap())
        .unwrap();
    let (got, octets) = receive(&rx).expect("a frame");
    assert_eq!(got.universe, 0x1234);
    assert_eq!(octets, [0x00, 10, 20, 30, 0], "one dark slot of padding");
}

#[test]
fn a_poll_is_answered_inside_the_wait_and_the_reply_recorded() {
    let (rx, tx) = pair();
    tx.poll().unwrap();
    assert_eq!(
        rx.receive(&mut [0; 513], Duration::from_millis(300))
            .unwrap(),
        None
    );
    assert_eq!(
        tx.receive(&mut [0; 513], Duration::from_millis(300))
            .unwrap(),
        None
    );
    let nodes = tx.nodes();
    assert_eq!(nodes.len(), 1, "{nodes:?}");
    assert_eq!(nodes[0].from(), rx.local_addr().unwrap());
    let fields = nodes[0].reply().unwrap().fields();
    assert_eq!(text(fields.short_name), b"bench");
    assert_eq!(fields.style, 1, "a controller");
}

/// Sends each `(sequence, level)` as raw `ArtDmx` to `rx`, then returns
/// the first level of each frame `rx` delivers.
fn delivered(rx: &ArtNet, packets: &[(u8, u8)], garbage: bool) -> Vec<u8> {
    let raw = UdpSocket::bind(loopback()).unwrap();
    let to = rx.local_addr().unwrap();
    let port = PortAddress::new(1).unwrap();
    if garbage {
        raw.send_to(b"not Art-Net at all", to).unwrap();
    }
    for &(sequence, level) in packets {
        let mut out = Vec::new();
        ArtDmx::new(port, sequence, &[level, 0]).encode(&mut out);
        raw.send_to(&out, to).unwrap();
    }
    let mut seen = Vec::new();
    while let Some((_, octets)) = receive_for(rx, Duration::from_millis(200)) {
        seen.push(octets[1]);
    }
    seen
}

fn receive_for(on: &ArtNet, timeout: Duration) -> Option<(Received, Vec<u8>)> {
    let mut buf = [0u8; 513];
    let got = on.receive(&mut buf, timeout).unwrap()?;
    Some((got, buf[..got.len].to_vec()))
}

#[test]
fn a_late_packet_is_dropped_and_a_restarted_sender_is_not() {
    let rx = open(loopback());
    let packets = [(10, 1), (9, 2), (10, 3), (11, 4), (100, 5), (50, 6)];
    assert_eq!(
        delivered(&rx, &packets, false),
        [1, 4, 5, 6],
        "9 and the repeated 10 are late; 50 is far enough behind to be a restart"
    );
}

#[test]
fn a_datagram_that_is_not_art_net_is_skipped_inside_the_wait() {
    let rx = open(loopback());
    assert_eq!(delivered(&rx, &[(0, 7)], true), [7]);
}

#[test]
fn a_wait_with_nothing_to_deliver_is_none_not_an_error() {
    let rx = open(loopback());
    let mut buf = [0u8; 513];
    assert_eq!(rx.receive(&mut buf, Duration::ZERO).unwrap(), None);
    assert_eq!(
        rx.receive(&mut buf, Duration::from_millis(20)).unwrap(),
        None
    );
}

#[test]
fn a_universe_past_15_bits_or_an_alternate_start_code_is_refused() {
    let (_rx, tx) = pair();
    let levels = Frame::new(StartCode::NULL, &[0, 0]).unwrap();
    assert!(matches!(
        tx.send(32_768, &levels),
        Err(Error::PortAddress(32_768))
    ));
    let rdm = Frame::new(StartCode(0xCC), &[1, 2]).unwrap();
    assert!(matches!(
        tx.send(1, &rdm),
        Err(Error::StartCode(StartCode(0xCC)))
    ));
}

#[test]
fn each_universe_is_numbered_from_one_on_its_own() {
    let rx = open(loopback());
    let raw = UdpSocket::bind(loopback()).unwrap();
    let tx = open(rx.local_addr().unwrap());
    let _ = raw;
    let frame = Frame::new(StartCode::NULL, &[0, 0]).unwrap();
    for universe in [1, 2, 1] {
        tx.send(universe, &frame).unwrap();
    }
    // Delivered in order, so none was taken for late: universe 2's first
    // packet is its own 1, not universe 1's 2.
    let universes: Vec<u32> = std::iter::from_fn(|| receive(&rx).map(|(g, _)| g.universe))
        .take(3)
        .collect();
    assert_eq!(universes, [1, 2, 1]);
}
