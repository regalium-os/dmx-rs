// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Finding widgets: enumerate the ports, classify them, probe those
//! allowed. See [`crate::discovery`] for why the probe is safe.
//!
//! Enumeration reads the filesystem only. On Linux, `/sys/class/tty` and
//! the USB device directory above each port give vendor and product ids,
//! serial number and product string — what libudev would, without it. On
//! macOS the platform's USB registry needs `IOKit`, which is C, so only the
//! device names under `/dev` are used.

use crate::discovery::{Candidate, Identity, Likely, Probe, Usb, Verdict, classify};
use crate::port::{Framing, Port};
use std::time::{Duration, Instant};

/// The framing a probe opens a port with. A USB widget ignores the rate;
/// 57 600 is one every driver accepts.
const PROBE_FRAMING: Framing = Framing {
    baud: 57_600,
    two_stop: false,
};

/// A port that answered as a widget.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    /// The port, as enumerated.
    pub port: Candidate,
    /// What the widget said about itself.
    pub identity: Identity,
}

/// A scan's options. Probes only [`Likely::Widget`] ports unless told
/// otherwise.
#[derive(Clone, Copy, Debug, Default)]
pub struct Scan {
    maybe: bool,
}

impl Scan {
    /// A scan probing only ports that say they are widgets.
    #[must_use]
    pub const fn new() -> Self {
        Self { maybe: false }
    }

    /// Also probes [`Likely::Maybe`] ports: generic USB serial bridges,
    /// which then receive the probe's 11 octets whatever is attached.
    #[must_use]
    pub const fn probe_maybe(mut self, yes: bool) -> Self {
        self.maybe = yes;
        self
    }

    /// Enumerates, classifies and probes, one port after another. Ports
    /// that cannot be opened — in use, or not permitted — are skipped.
    #[must_use]
    pub fn run(&self) -> Vec<Found> {
        ports()
            .into_iter()
            .filter(|port| match classify(port) {
                Likely::Widget => true,
                Likely::Maybe => self.maybe,
                Likely::No => false,
            })
            .filter_map(|port| {
                let identity = probe(&port.path).ok()??;
                Some(Found { port, identity })
            })
            .collect()
    }
}

/// Probes the port at `path`: `Some` with what it said if it is a widget.
///
/// # Errors
///
/// When the port cannot be opened, written or read.
pub fn probe(path: &str) -> std::io::Result<Option<Identity>> {
    let port = Port::open(path, PROBE_FRAMING)?;
    identify(&port)
}

/// Probes an open port.
pub(crate) fn identify(port: &Port) -> std::io::Result<Option<Identity>> {
    let start = Instant::now();
    let (mut probe, request) = Probe::start(Duration::ZERO);
    port.write_all(&request, Duration::from_millis(500))?;
    let mut buf = [0u8; 256];
    loop {
        let now = start.elapsed();
        let left = match probe.verdict(now) {
            Verdict::Widget(identity) => return Ok(Some(identity)),
            Verdict::NotAWidget => return Ok(None),
            Verdict::Pending { until } => until.saturating_sub(now),
        };
        let n = port.read(&mut buf, left)?;
        probe.feed(buf.get(..n).unwrap_or_default(), start.elapsed());
    }
}

/// Every serial port the platform lists, with what it says about each.
#[must_use]
pub fn ports() -> Vec<Candidate> {
    #[cfg(target_os = "linux")]
    return linux();
    #[cfg(not(target_os = "linux"))]
    return by_name();
}

/// Linux: each tty with a device behind it, and that device's USB
/// attributes when it is on USB.
#[cfg(target_os = "linux")]
fn linux() -> Vec<Candidate> {
    let Ok(entries) = std::fs::read_dir("/sys/class/tty") else {
        return Vec::new();
    };
    let mut ports: Vec<Candidate> = entries
        .flatten()
        .filter_map(|entry| {
            let device = std::fs::canonicalize(entry.path().join("device")).ok()?;
            let name = entry.file_name().into_string().ok()?;
            let usb = device.ancestors().take(5).find_map(usb);
            Some(Candidate {
                path: format!("/dev/{name}"),
                usb,
            })
        })
        .collect();
    ports.sort_by(|a, b| a.path.cmp(&b.path));
    ports
}

/// The USB attributes in `dir`, if it is a USB device's directory.
#[cfg(target_os = "linux")]
fn usb(dir: &std::path::Path) -> Option<Usb> {
    let read = |file: &str| {
        let text = std::fs::read_to_string(dir.join(file)).ok()?;
        Some(text.trim().to_owned()).filter(|t| !t.is_empty())
    };
    let hex = |file: &str| u16::from_str_radix(&read(file)?, 16).ok();
    Some(Usb {
        vid: hex("idVendor")?,
        pid: hex("idProduct")?,
        serial: read("serial"),
        manufacturer: read("manufacturer"),
        product: read("product"),
    })
}

/// Elsewhere: USB serial bridges by their device names, with no USB
/// attributes; [`classify`] reads what it can from the name.
#[cfg(not(target_os = "linux"))]
fn by_name() -> Vec<Candidate> {
    let Ok(entries) = std::fs::read_dir("/dev") else {
        return Vec::new();
    };
    let mut ports: Vec<Candidate> = entries
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with("cu.usbserial-") || name.starts_with("cu.usbmodem"))
        .map(|name| Candidate {
            path: format!("/dev/{name}"),
            usb: None::<Usb>,
        })
        .collect();
    ports.sort_by(|a, b| a.path.cmp(&b.path));
    ports
}
