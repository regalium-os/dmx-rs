// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Two sACN sockets on loopback, unicast: what one sends, the other
//! receives with its priority; what is housekeeping stays inside the wait.
//! Multicast itself depends on the host's routes, and is left to the bench.

use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::{Received, Transport};
use regalium_dmx_sacn::Sacn;
use regalium_dmx_sacn::codec::{Cid, DataPacket, PREVIEW, name};
use regalium_dmx_sacn::socket::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

const WAIT: Duration = Duration::from_millis(300);
const RX: Cid = Cid([0xAA; 16]);
const TX: Cid = Cid([0xBB; 16]);

fn loopback() -> SocketAddr {
    "127.0.0.1:0".parse().unwrap()
}

fn receiver() -> Sacn {
    Sacn::builder(loopback(), RX).open().unwrap()
}

fn sender(to: &Sacn, priority: u8) -> Sacn {
    let target = to.local_addr().unwrap();
    Sacn::builder(loopback(), TX)
        .name("Desk")
        .priority(priority)
        .unicast(target)
        .open()
        .unwrap()
}

fn receive(on: &Sacn) -> Option<(Received, Vec<u8>)> {
    let mut buf = [0u8; 513];
    let got = on.receive(&mut buf, WAIT).unwrap()?;
    Some((got, buf[..got.len].to_vec()))
}

#[test]
fn a_frame_arrives_with_its_start_code_and_priority_and_the_universe_is_announced() {
    let rx = receiver();
    let tx = sender(&rx, 150);
    let frame = Frame::new(StartCode(0x17), &[1, 2, 3]).unwrap();
    tx.send(42, &frame).unwrap();
    let (got, octets) = receive(&rx).expect("a frame");
    assert_eq!((got.universe, got.priority, got.ended), (42, 150, false));
    assert_eq!(got.source.0, TX.folded());
    assert_eq!(
        octets,
        [0x17, 1, 2, 3],
        "no padding: sACN carries any length"
    );
    assert_eq!(receive(&rx), None, "the announcement is housekeeping");
    let sources = rx.sources();
    assert_eq!(sources.len(), 1, "{sources:?}");
    assert_eq!((sources[0].cid, sources[0].name.as_str()), (TX, "Desk"));
    assert_eq!(sources[0].universes, [42]);
}

#[test]
fn ending_a_universe_sends_three_terminated_packets() {
    let rx = receiver();
    let tx = sender(&rx, 100);
    tx.end(5).unwrap();
    let ended: Vec<bool> = std::iter::from_fn(|| receive(&rx).map(|(g, _)| g.ended)).collect();
    assert_eq!(ended, [true, true, true]);
}

#[test]
fn a_sockets_own_packets_looped_back_are_dropped() {
    let rx = receiver();
    let me = rx.local_addr().unwrap();
    let echo = Sacn::builder(loopback(), RX).unicast(me).open().unwrap();
    echo.send(1, &Frame::new(StartCode::NULL, &[9]).unwrap())
        .unwrap();
    assert_eq!(receive(&rx), None, "same CID: this source's own data");
}

/// Sends each `(sequence, level, options)` as a raw data packet to `rx`,
/// then returns the first level of each frame `rx` delivers.
fn delivered(rx: &Sacn, packets: &[(u8, u8, u8)]) -> Vec<u8> {
    let raw = UdpSocket::bind(loopback()).unwrap();
    let to = rx.local_addr().unwrap();
    for &(sequence, level, options) in packets {
        let packet = DataPacket {
            cid: TX,
            source_name: name("raw"),
            priority: 100,
            sync_address: 0,
            sequence,
            options,
            universe: 1,
            start_code: StartCode::NULL,
            slots: &[level],
        };
        let mut out = Vec::new();
        packet.encode(&mut out);
        raw.send_to(&out, to).unwrap();
    }
    std::iter::from_fn(|| receive(rx).map(|(_, o)| o[1])).collect()
}

#[test]
fn a_late_packet_is_dropped_and_zero_is_an_ordinary_number() {
    let rx = receiver();
    let packets = [
        (254, 1, 0),
        (255, 2, 0),
        (0, 3, 0),
        (255, 4, 0),
        (100, 5, 0),
        (80, 6, 0),
    ];
    assert_eq!(
        delivered(&rx, &packets),
        [1, 2, 3, 5, 6],
        "255 after 0 is late; 80 is twenty behind 100, a restart"
    );
}

#[test]
fn preview_data_is_dropped() {
    let rx = receiver();
    assert_eq!(delivered(&rx, &[(1, 7, PREVIEW), (2, 8, 0)]), [8]);
}

#[test]
fn universes_outside_1_to_63999_are_refused() {
    let rx = receiver();
    let tx = sender(&rx, 100);
    let frame = Frame::new(StartCode::NULL, &[0]).unwrap();
    for universe in [0, 64_000, 64_214, 70_000] {
        assert!(matches!(tx.send(universe, &frame), Err(Error::Universe(u)) if u == universe));
    }
    assert!(matches!(tx.join(0), Err(Error::Universe(0))));
}
