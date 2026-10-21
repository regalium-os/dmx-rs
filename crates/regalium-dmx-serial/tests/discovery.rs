// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Classifying ports before asking them, and the probe's timing, on an
//! injected clock.

use regalium_dmx_serial::discovery::{
    Candidate, Identity, Likely, Probe, TIMEOUT, Usb, Verdict, classify,
};
use regalium_dmx_serial::enttec::{Label, Message, Params};
use std::time::Duration;

fn usb(vid: u16, pid: u16, product: Option<&str>, serial: Option<&str>) -> Candidate {
    Candidate {
        path: "/dev/ttyUSB0".to_owned(),
        usb: Some(Usb {
            vid,
            pid,
            serial: serial.map(str::to_owned),
            manufacturer: Some("FTDI".to_owned()),
            product: product.map(str::to_owned),
        }),
    }
}

fn named(path: &str) -> Candidate {
    Candidate {
        path: path.to_owned(),
        usb: None,
    }
}

#[test]
fn a_port_is_a_widget_only_when_it_says_so() {
    let cases = [
        (
            usb(0x0403, 0x6001, Some("DMX USB PRO"), None),
            Likely::Widget,
        ),
        (
            usb(0x0403, 0x6001, Some("FT232R USB UART"), Some("EN123456")),
            Likely::Widget,
        ),
        (
            usb(0x0403, 0x6001, Some("FT232R USB UART"), Some("A50285BI")),
            Likely::Maybe,
        ),
        (usb(0x0403, 0x6001, None, None), Likely::Maybe),
        (usb(0x10C4, 0xEA60, Some("CP2102 DMX"), None), Likely::No),
        (named("/dev/cu.usbserial-EN172345"), Likely::Widget),
        (named("/dev/cu.usbserial-A10KJ3X"), Likely::Maybe),
        (named("/dev/cu.usbserial-EN"), Likely::Maybe),
        (named("/dev/cu.Bluetooth-Incoming-Port"), Likely::No),
        (named("/dev/ttyS0"), Likely::No),
    ];
    for (port, want) in cases {
        assert_eq!(classify(&port), want, "{port:?}");
    }
}

fn reply(label: Label, data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    Message { label, data }.encode(&mut out);
    out
}

const PARAMS: Params = Params {
    firmware: 0x0144,
    break_ticks: 9,
    mab_ticks: 1,
    rate: 40,
};

#[test]
fn the_probe_asks_for_the_serial_and_parameters_and_nothing_else() {
    let (_, request) = Probe::start(Duration::ZERO);
    assert_eq!(
        request,
        [0x7E, 10, 0, 0, 0xE7, 0x7E, 3, 2, 0, 0, 0, 0xE7],
        "no DMX"
    );
}

#[test]
fn both_replies_identify_a_widget_at_once() {
    let (mut probe, _) = Probe::start(Duration::ZERO);
    let at = Duration::from_millis(5);
    assert_eq!(
        probe.feed(&reply(Label::GET_SERIAL, &[0x56, 0x34, 0x12, 0x00]), at),
        Verdict::Pending { until: TIMEOUT }
    );
    let verdict = probe.feed(&reply(Label::GET_PARAMETERS, &PARAMS.encode()), at);
    assert_eq!(
        verdict,
        Verdict::Widget(Identity {
            serial: Some(123_456),
            params: PARAMS
        })
    );
}

#[test]
fn parameters_alone_identify_a_widget_but_only_at_the_timeout() {
    let (mut probe, _) = Probe::start(Duration::from_secs(10));
    let until = Duration::from_secs(11);
    assert_eq!(
        probe.feed(
            &reply(Label::GET_PARAMETERS, &PARAMS.encode()),
            Duration::from_secs(10)
        ),
        Verdict::Pending { until }
    );
    assert_eq!(
        probe.verdict(until.checked_sub(Duration::from_nanos(1)).unwrap()),
        Verdict::Pending { until }
    );
    assert_eq!(
        probe.verdict(until),
        Verdict::Widget(Identity {
            serial: None,
            params: PARAMS
        })
    );
}

#[test]
fn silence_or_noise_until_the_timeout_is_not_a_widget() {
    let (mut probe, _) = Probe::start(Duration::ZERO);
    assert_eq!(
        probe.feed(b"AT\r\nOK\r\n", Duration::from_millis(10)),
        Verdict::Pending { until: TIMEOUT }
    );
    assert_eq!(probe.verdict(TIMEOUT), Verdict::NotAWidget);
    let (quiet, _) = Probe::start(Duration::ZERO);
    assert_eq!(quiet.verdict(TIMEOUT), Verdict::NotAWidget);
}

#[test]
fn a_serial_alone_is_not_enough() {
    let (mut probe, _) = Probe::start(Duration::ZERO);
    probe.feed(&reply(Label::GET_SERIAL, &[1, 0, 0, 0]), Duration::ZERO);
    assert_eq!(
        probe.verdict(TIMEOUT),
        Verdict::NotAWidget,
        "parameters are what every widget answers"
    );
}
