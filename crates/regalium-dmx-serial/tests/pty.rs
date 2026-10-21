// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The widget transport against a fake widget on a pseudo-terminal: the
//! real port code, the real probe, no hardware.

use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;
use regalium_dmx_core::transport::Transport;
use regalium_dmx_serial::enttec::{Framer, Label, Message, Params, serial_bcd};
use regalium_dmx_serial::{Enttec, Error};
use rustix::event::{PollFd, PollFlags, Timespec, poll};
use rustix::fd::OwnedFd;
use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

const PARAMS: Params = Params {
    firmware: 0x0144,
    break_ticks: 9,
    mab_ticks: 1,
    rate: 40,
};

/// Every message the fake heard: label and data.
type Heard = Arc<Mutex<Vec<(u8, Vec<u8>)>>>;

/// How the fake answers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Widget,
    Silent,
}

/// The pty's far end, played by a thread: it answers the probe as a widget
/// would, and records every message the host sends.
struct Fake {
    path: String,
    master: Arc<OwnedFd>,
    heard: Heard,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

fn send(fd: &OwnedFd, label: Label, data: &[u8]) {
    let mut out = Vec::new();
    Message { label, data }.encode(&mut out);
    rustix::io::write(fd, &out).unwrap();
}

impl Fake {
    fn new(kind: Kind) -> Self {
        let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY).unwrap();
        grantpt(&master).unwrap();
        unlockpt(&master).unwrap();
        let path = ptsname(&master, Vec::new()).unwrap().into_string().unwrap();
        let master = Arc::new(master);
        let heard = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let (master, heard, stop) =
                (Arc::clone(&master), Arc::clone(&heard), Arc::clone(&stop));
            std::thread::spawn(move || {
                let mut framer = Framer::new();
                let mut buf = [0u8; 1024];
                let tick = Timespec {
                    tv_sec: 0,
                    tv_nsec: 10_000_000,
                };
                while !stop.load(Ordering::Acquire) {
                    let mut fds = [PollFd::new(&*master, PollFlags::IN)];
                    if poll(&mut fds, Some(&tick)).unwrap_or(0) == 0 {
                        continue;
                    }
                    let Ok(n) = rustix::io::read(&*master, &mut buf) else {
                        break;
                    };
                    let mut asked = Vec::new();
                    framer.feed(&buf[..n], |m| asked.push((m.label.0, m.data.to_vec())));
                    for (label, data) in asked {
                        if kind == Kind::Widget {
                            match Label(label) {
                                Label::GET_SERIAL => {
                                    send(&master, Label::GET_SERIAL, &serial_bcd(4_242));
                                }
                                Label::GET_PARAMETERS => {
                                    send(&master, Label::GET_PARAMETERS, &PARAMS.encode());
                                }
                                _ => {}
                            }
                        }
                        heard.lock().unwrap().push((label, data));
                    }
                }
            })
        };
        Self {
            path,
            master,
            heard,
            stop,
            thread: Some(thread),
        }
    }

    fn heard(&self) -> Vec<(u8, Vec<u8>)> {
        self.heard.lock().unwrap().clone()
    }

    /// Waits up to two seconds for a message with `label`.
    fn wait_for(&self, label: Label) -> Option<Vec<u8>> {
        for _ in 0..200 {
            if let Some((_, data)) = self.heard().into_iter().find(|m| m.0 == label.0) {
                return Some(data);
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        None
    }
}

impl Drop for Fake {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

#[test]
fn a_widget_is_identified_then_sent_dmx_padded_to_24_slots() {
    let fake = Fake::new(Kind::Widget);
    let widget = Enttec::open(&fake.path).unwrap();
    assert_eq!(widget.identity().serial, Some(4_242));
    assert_eq!(widget.identity().params, PARAMS);
    let labels: Vec<u8> = fake.heard().iter().map(|m| m.0).collect();
    assert_eq!(labels, [10, 3], "the probe, and nothing before it");

    widget
        .send(1, &Frame::new(StartCode::NULL, &[255, 0, 7]).unwrap())
        .unwrap();
    let dmx = fake.wait_for(Label::SEND_DMX).expect("a Send DMX message");
    assert_eq!(dmx.len(), 25, "start code and 24 slots");
    assert_eq!(&dmx[..4], [0x00, 255, 0, 7]);
    assert!(matches!(
        widget.send(2, &Frame::new(StartCode::NULL, &[]).unwrap()),
        Err(Error::Universe(2))
    ));
}

#[test]
fn received_dmx_with_a_clean_status_arrives_as_a_frame_and_a_torn_one_does_not() {
    let fake = Fake::new(Kind::Widget);
    let widget = Enttec::open(&fake.path).unwrap();
    widget.join(1).unwrap();
    assert_eq!(fake.wait_for(Label::RECEIVE_ON_CHANGE), Some(vec![0]));
    send(&fake.master, Label::RECEIVED_DMX, &[0x02, 0x00, 1]);
    send(&fake.master, Label::RECEIVED_DMX, &[0x00, 0x00, 10, 20]);
    let mut buf = [0u8; 513];
    let mut got = None;
    for _ in 0..50 {
        if let Some(r) = widget.receive(&mut buf, Duration::from_millis(20)).unwrap() {
            got = Some((r, buf[..r.len].to_vec()));
            break;
        }
    }
    let (r, octets) = got.expect("a frame");
    assert_eq!(octets, [0x00, 10, 20], "the overrun packet was dropped");
    assert_eq!((r.universe, r.source.0), (1, 4_242));
}

#[test]
fn a_port_that_does_not_answer_is_refused_and_sent_nothing_but_the_probe() {
    let fake = Fake::new(Kind::Silent);
    let refused = Enttec::open(&fake.path);
    assert!(matches!(refused, Err(Error::NotAWidget { .. })));
    std::thread::sleep(Duration::from_millis(50));
    let labels: Vec<u8> = fake.heard().iter().map(|m| m.0).collect();
    assert_eq!(labels, [10, 3]);
}
