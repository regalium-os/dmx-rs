// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The physical DMX512 line: 250 kbaud, 8N2, a break and mark-after-break
//! ahead of every packet, as ANSI E1.11 times them.
//!
//! # Layer contract
//!
//! Owns: the ENTTEC DMX USB Pro widget API ([`enttec`]); port discovery
//! ([`discovery`], [`scan`]); the line's timing ([`line`]); a raw serial
//! port ([`port`]); and two transports satisfying the core's trait — an
//! ENTTEC-API widget ([`Enttec`]), which also carries RDM, and a raw UART on an RS-485 transceiver
//! ([`Uart`]).
//!
//! Must not: link C. Ports are enumerated through sysfs and device names,
//! not libudev, and opened through `rustix`; `xtask deps` refuses a native
//! library here. Must not send DMX to a port that has not identified
//! itself.
//!
//! Allowed imports: `regalium-dmx-core`, `regalium-dmx-telemetry`.
//!
//! [`enttec`], [`discovery`] and [`line`] perform no I/O and read no clock;
//! [`port`], [`scan`] and the transports are Unix only, for now.
//!
//! # Spans
//!
//! None yet. `dmx.serial.scan` arrives with the probe, where a scan is a
//! call a person waits on.
//!
//! # State
//!
//! Phase 6: the widget API and its corpus; discovery and the safe probe;
//! the widget and UART transports. Phase 6b: RDM through the widget.

pub mod discovery;
pub mod enttec;
pub mod line;

mod error;
pub use error::Error;

#[cfg(unix)]
pub mod port;
#[cfg(unix)]
pub mod scan;
#[cfg(unix)]
mod uart;
#[cfg(unix)]
mod widget;
#[cfg(unix)]
mod widget_rdm;

#[cfg(unix)]
pub use uart::Uart;
#[cfg(unix)]
pub use widget::{Enttec, KEEP_ALIVE, UNIVERSE, WRITE};
#[cfg(unix)]
pub use widget_rdm::RDM_WINDOW;
