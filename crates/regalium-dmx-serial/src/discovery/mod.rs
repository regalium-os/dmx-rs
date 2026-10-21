// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Which serial ports might be DMX widgets, and asking them, safely.
//!
//! Discovery is three steps, and only the last touches a port:
//!
//! 1. **Enumerate** the serial ports, with whatever the platform says about
//!    each: on Linux the USB vendor and product ids, serial number and
//!    product string from sysfs; on macOS only the device name
//!    ([`crate::scan`]).
//! 2. **Classify** each ([`classify`]): a widget, maybe one, or not.
//! 3. **Probe** those allowed ([`Probe`]): send the two API requests every
//!    ENTTEC-API widget answers — serial number and parameters — and wait
//!    for well-formed replies.
//!
//! # Why the probe is safe
//!
//! It never sends DMX: nothing reaches a light until a port has answered
//! as a widget. What it sends is two short API requests, 11 octets in all.
//! Even so, the most common widget bridge (FTDI's FT232R, 0403:6001) is
//! also on every generic USB serial cable, so a port that is only *maybe*
//! a widget is probed only when the caller allows it: those 11 octets
//! reach whatever is on the cable.

mod probe;

pub use probe::{Identity, Probe, TIMEOUT, Verdict};

/// What the platform says about the USB device behind a port.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Usb {
    /// The vendor id.
    pub vid: u16,
    /// The product id.
    pub pid: u16,
    /// The serial number string, if the device has one.
    pub serial: Option<String>,
    /// The manufacturer string.
    pub manufacturer: Option<String>,
    /// The product string.
    pub product: Option<String>,
}

/// A serial port, as enumerated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    /// The device to open: `/dev/ttyUSB0`, `/dev/cu.usbserial-EN123456`.
    pub path: String,
    /// The USB device behind it, where the platform says.
    pub usb: Option<Usb>,
}

/// How likely a port is to be a widget, before asking it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Likely {
    /// Says it is one: probed by default.
    Widget,
    /// Could be one, or a generic cable: probed only when allowed.
    Maybe,
    /// Not a widget this driver knows: never probed.
    No,
}

/// FTDI's vendor id.
pub const FTDI: u16 = 0x0403;

/// USB ids of the bridges ENTTEC-API widgets are built on. FT232R is
/// also every generic cable's, so it is [`Likely::Maybe`] unless the
/// product string or serial says more.
pub const KNOWN: &[(u16, u16, &str)] = &[(FTDI, 0x6001, "FTDI FT232R")];

/// Classifies `port` from what enumeration found.
///
/// - Known ids with a product string naming DMX, or an ENTTEC serial
///   (`EN` and digits, as ENTTEC programs them): [`Likely::Widget`].
/// - Known ids otherwise: [`Likely::Maybe`].
/// - No USB information (macOS): by device name, the same way — a serial
///   bridge whose name carries an ENTTEC serial is a widget, any other is
///   maybe one.
/// - Anything else: [`Likely::No`].
#[must_use]
pub fn classify(port: &Candidate) -> Likely {
    match &port.usb {
        Some(usb) if KNOWN.iter().any(|&(v, p, _)| v == usb.vid && p == usb.pid) => {
            let says_dmx = usb
                .product
                .as_deref()
                .is_some_and(|p| p.to_ascii_uppercase().contains("DMX"));
            if says_dmx || usb.serial.as_deref().is_some_and(enttec_serial) {
                Likely::Widget
            } else {
                Likely::Maybe
            }
        }
        Some(_) => Likely::No,
        None => match port
            .path
            .rsplit('/')
            .next()
            .and_then(|n| n.strip_prefix("cu.usbserial-"))
        {
            Some(serial) if enttec_serial(serial) => Likely::Widget,
            Some(_) => Likely::Maybe,
            None => Likely::No,
        },
    }
}

/// Whether `serial` is ENTTEC's form: `EN` and digits.
fn enttec_serial(serial: &str) -> bool {
    serial
        .strip_prefix("EN")
        .is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
}
