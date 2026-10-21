// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! RDM over Art-Net on loopback: a fake node keeps its port's table of
//! responders and relays `ArtRdm` to them; the controller asks for the
//! table and re-addresses each device — while an input reads the same
//! socket, so replies it happens to read must still reach the controller.

use regalium_dmx_artnet::ArtNet;
use regalium_dmx_artnet::codec::{ArtRdm, ArtTodData, Packet, PortAddress};
use regalium_dmx_core::merge::{Merge, Mode};
use regalium_dmx_core::rdm::{DeviceInfo, Pid, Responder, Response, Uid};
use regalium_dmx_runtime::rdm::Controller;
use regalium_dmx_runtime::{Input, SystemClock};
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const ME: Uid = Uid::new(0x7a70, 1);

/// The node: one port, 0:0:1, and the devices on it.
fn node(socket: &UdpSocket, devices: &Mutex<Vec<Responder>>, stop: &AtomicBool) {
    socket
        .set_read_timeout(Some(Duration::from_millis(10)))
        .unwrap();
    let port = PortAddress::new(1).unwrap();
    let mut buf = [0u8; 1500];
    while !stop.load(Ordering::Acquire) {
        let Ok((n, from)) = socket.recv_from(&mut buf) else {
            continue;
        };
        let mut out = Vec::new();
        match Packet::decode(&buf[..n]) {
            Ok(Packet::TodRequest(_)) => {
                let uids: Vec<Uid> = devices.lock().unwrap().iter().map(Responder::uid).collect();
                let mut list = Vec::new();
                ArtTodData::new(port, &uids, &mut list).encode(&mut out);
            }
            Ok(Packet::Rdm(request)) => {
                let answer = devices.lock().unwrap().iter_mut().find_map(|d| {
                    match d.answer(&request.rdm()) {
                        Response::Message(m) => Some(m),
                        Response::Silent | Response::Discovery(_) => None,
                    }
                });
                let Some(answer) = answer else { continue };
                ArtRdm::new(port, &answer).encode(&mut out);
            }
            _ => continue,
        }
        socket.send_to(&out, from).unwrap();
    }
}

#[test]
fn a_controller_reads_a_nodes_table_and_readdresses_its_devices() {
    let info = DeviceInfo {
        protocol: 0x0100,
        footprint: 4,
        start_address: 1,
        ..DeviceInfo::default()
    };
    let devices = Arc::new(Mutex::new(
        (1..=4)
            .map(|n| Responder::new(Uid::new(0x4c55, n), info))
            .collect::<Vec<_>>(),
    ));
    let stop = Arc::new(AtomicBool::new(false));
    let node_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let node_addr = node_socket.local_addr().unwrap();
    let thread = {
        let (devices, stop) = (Arc::clone(&devices), Arc::clone(&stop));
        std::thread::spawn(move || node(&node_socket, &devices, &stop))
    };

    let art = Arc::new(
        ArtNet::builder("127.0.0.1:0".parse().unwrap(), node_addr)
            .open()
            .unwrap(),
    );
    let (input, _events) = Input::builder(Arc::clone(&art), SystemClock::new()).build();
    let _merged = input.listen(1, Merge::new(Mode::Htp)).unwrap();
    let mut listening = input.start().unwrap();

    let controller = Controller::new(Arc::clone(&art), SystemClock::new(), ME);
    let found = controller.discover(1).unwrap();
    assert_eq!(
        found.devices,
        (1..=4).map(|n| Uid::new(0x4c55, n)).collect::<Vec<_>>()
    );
    for (n, uid) in found.devices.iter().enumerate() {
        let address = 100 + 4 * u16::try_from(n).unwrap();
        controller
            .set(1, *uid, Pid::DMX_START_ADDRESS, &address.to_be_bytes())
            .unwrap();
        assert_eq!(
            controller.device_info(1, *uid).unwrap().start_address,
            address
        );
    }

    listening.close();
    stop.store(true, Ordering::Release);
    thread.join().unwrap();
    let addresses: Vec<u16> = devices
        .lock()
        .unwrap()
        .iter()
        .map(|d| d.info().start_address)
        .collect();
    assert_eq!(addresses, [100, 104, 108, 112]);
}
