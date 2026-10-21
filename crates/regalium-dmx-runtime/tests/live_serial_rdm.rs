// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! RDM end to end without hardware: the controller, the widget transport
//! and the real port code, against a fake widget on a pseudo-terminal
//! whose line holds RDM responders — while an output sends levels on the
//! same widget, as a rig does.

use regalium_dmx_core::address::Address;
use regalium_dmx_core::rdm::{DeviceInfo, Pid, Responder, Response, Uid};
use regalium_dmx_runtime::rdm::Controller;
use regalium_dmx_runtime::{Output, SystemClock};
use regalium_dmx_serial::Enttec;
use regalium_dmx_serial::enttec::{Framer, Label, Message, Params, serial_bcd};
use rustix::event::{PollFd, PollFlags, Timespec, poll};
use rustix::fd::OwnedFd;
use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

const ME: Uid = Uid::new(0x7a70, 1);

fn reply(fd: &OwnedFd, label: Label, data: &[u8]) {
    let mut out = Vec::new();
    Message { label, data }.encode(&mut out);
    rustix::io::write(fd, &out).unwrap();
}

/// A widget whose line holds `devices`: it answers the probe, sends RDM
/// requests to the devices, and returns what they said.
struct Widget {
    path: String,
    devices: Arc<Mutex<Vec<Responder>>>,
    levels: Arc<AtomicU32>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Widget {
    fn new(devices: Vec<Responder>) -> Self {
        let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY).unwrap();
        grantpt(&master).unwrap();
        unlockpt(&master).unwrap();
        let path = ptsname(&master, Vec::new()).unwrap().into_string().unwrap();
        let devices = Arc::new(Mutex::new(devices));
        let levels = Arc::new(AtomicU32::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let (d, l, s) = (Arc::clone(&devices), Arc::clone(&levels), Arc::clone(&stop));
        let thread = std::thread::spawn(move || serve(&master, &d, &l, &s));
        Self {
            path,
            devices,
            levels,
            stop,
            thread: Some(thread),
        }
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn serve(master: &OwnedFd, devices: &Mutex<Vec<Responder>>, levels: &AtomicU32, stop: &AtomicBool) {
    let mut framer = Framer::new();
    let mut buf = [0u8; 2048];
    let tick = Timespec {
        tv_sec: 0,
        tv_nsec: 5_000_000,
    };
    while !stop.load(Ordering::Acquire) {
        let mut fds = [PollFd::new(master, PollFlags::IN)];
        if poll(&mut fds, Some(&tick)).unwrap_or(0) == 0 {
            continue;
        }
        let Ok(n) = rustix::io::read(master, &mut buf) else {
            break;
        };
        let mut asked = Vec::new();
        framer.feed(&buf[..n], |m| asked.push((m.label, m.data.to_vec())));
        for (label, data) in asked {
            match label {
                Label::GET_SERIAL => reply(master, label, &serial_bcd(77)),
                Label::GET_PARAMETERS => {
                    reply(
                        master,
                        label,
                        &Params {
                            firmware: 0x0144,
                            break_ticks: 9,
                            mab_ticks: 1,
                            rate: 40,
                        }
                        .encode(),
                    );
                }
                Label::SEND_DMX => {
                    levels.fetch_add(1, Ordering::Relaxed);
                }
                Label::SEND_RDM | Label::SEND_RDM_DISCOVERY => {
                    let heard: Vec<Vec<u8>> = devices
                        .lock()
                        .unwrap()
                        .iter_mut()
                        .filter_map(|d| match d.answer(&data) {
                            Response::Silent => None,
                            Response::Discovery(o) | Response::Message(o) => Some(o),
                        })
                        .collect();
                    let len = heard.iter().map(Vec::len).max().unwrap_or(0);
                    let mut out = vec![0u8];
                    out.extend((0..len).map(|i| {
                        heard
                            .iter()
                            .fold(0, |acc, h| acc | h.get(i).copied().unwrap_or(0))
                    }));
                    let broadcast = data
                        .get(3..9)
                        .and_then(Uid::read)
                        .is_some_and(Uid::is_broadcast);
                    if label == Label::SEND_RDM && heard.is_empty() && broadcast {
                        // Nothing answers a broadcast, and nothing is reported.
                    } else if label == Label::SEND_RDM && heard.is_empty() {
                        // Silence to an addressed request: the firmware's timeout.
                        reply(master, Label::RDM_TIMEOUT, &[]);
                    } else {
                        reply(master, Label::RECEIVED_DMX, &out);
                    }
                }
                _ => {}
            }
        }
    }
}

fn fixture(device: u32) -> Responder {
    let info = DeviceInfo {
        protocol: 0x0100,
        footprint: 8,
        personality: 1,
        personalities: 1,
        start_address: 1,
        ..DeviceInfo::default()
    };
    Responder::new(Uid::new(0x4c55, device), info)
}

#[test]
fn a_controller_discovers_and_readdresses_fixtures_through_the_widget_while_levels_go_out() {
    let widget = Widget::new((1..=6).map(|n| fixture(n * 1_000_003)).collect());
    let enttec = Arc::new(Enttec::open(&widget.path).unwrap());

    let (output, _events) = Output::builder(Arc::clone(&enttec), SystemClock::new()).build();
    let patch = output.patch(1);
    patch.set_len(8).unwrap();
    patch.set(Address::FIRST, 255);
    let mut running = output.start().unwrap();

    let controller = Controller::new(Arc::clone(&enttec), SystemClock::new(), ME);
    let found = controller.discover(1).unwrap();
    assert_eq!(found.devices.len(), 6, "{:?}", found.devices);
    assert!(!found.incomplete);

    for (n, uid) in found.devices.iter().enumerate() {
        let address = 1 + 8 * u16::try_from(n).unwrap();
        controller
            .set(1, *uid, Pid::DMX_START_ADDRESS, &address.to_be_bytes())
            .unwrap();
        assert_eq!(
            controller.device_info(1, *uid).unwrap().start_address,
            address
        );
    }
    running.close();

    let addresses: Vec<u16> = widget
        .devices
        .lock()
        .unwrap()
        .iter()
        .map(|d| d.info().start_address)
        .collect();
    let mut sorted = addresses.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, [1, 9, 17, 25, 33, 41], "each moved, none twice");
    assert!(
        widget.levels.load(Ordering::Relaxed) > 0,
        "levels went out alongside"
    );
}
