// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Telemetry seam for the DMX driver: the traits every layer records
//! through, and nothing else.
//!
//! Observability goes through the the-protobuf-project telemetry SDK, never
//! through an OpenTelemetry import. This crate is the whole seam — it says
//! what the layers need, and `regalium-dmx-telemetry-otel` attaches the SDK
//! to it. It is bacnet-rs's seam, unchanged in mechanism; the [`keys`],
//! [`instruments`] and [`shape`] are DMX's own.
//!
//! That is [`Meter`]: declared [`Instrument`]s recorded with [`Attribute`]s
//! built from the declared [`keys`]; and [`Tracer`], whose spans record only
//! what a type declares with [`trace_attrs!`], nested parent to child; and
//! [`Logger`], whose lines carry the span they happened in.
//!
//! # Why this exists before the protocol does
//!
//! Spans at layer boundaries are a design constraint, not an observability
//! feature bolted on later. A span wrapping `frame::decode` forces `decode`
//! to take a tracer, which forces its caller to have one. Retrofitting that
//! means changing every signature in the core.
//!
//! # Layer contract
//!
//! Owns: the meter, tracer and logger traits and their no-op implementations,
//! the span guard, instrument and attribute declarations, the attribute key
//! constants, the `trace_attrs!` declaration macro, and the [`shape`] of
//! every universe, node and RDM series.
//!
//! Must not: perform I/O, read a clock, allocate on a no-op path, or name a
//! type from any telemetry implementation. This crate has no dependencies and
//! `cargo xtask purity` fails if one appears.
//!
//! Allowed imports: none.
//!
//! # Attributes are declared, never set ad hoc
//!
//! An attribute is declared next to the field it describes with the
//! `trace_attrs!` macro. An undeclared field is never recorded, and that is
//! the safety mechanism rather than an oversight: there is no
//! `set_attribute` for a well-meaning change in the refresh loop to call.
//!
//! # Spans
//!
//! Span names are `dmx.<layer>.<operation>` — `dmx.frame.decode`,
//! `dmx.rdm.discovery`, `dmx.artnet.poll`. Each crate's own documentation
//! lists the spans it opens.
//!
//! **Never open a span per frame.** A universe refreshes at up to 44 Hz;
//! twenty of them is 880 spans a second for nothing a reader can use. Frames
//! are counted and timed on [`instruments::FRAMES`] and
//! [`instruments::FRAME_INTERVAL`]; spans are for discovery, RDM, patching
//! and API calls — work a person waits on.
//!
//! # Metrics
//!
//! Instrument names are `dmx.<thing>.<measure>` and labels are only ever
//! [`keys`]: which universe, node or responder — never a slot level. Record
//! universe, node and RDM series through [`shape`], which fixes their labels
//! for every producer.

mod attribute;
pub mod instruments;
pub mod keys;
mod logs;
mod meter;
pub mod shape;
mod trace;
mod traced;

pub use attribute::{Attribute, AttributeValue, Key};
pub use logs::{Level, Logger, NoopLogger};
pub use meter::{Instrument, Kind, Meter, NoopMeter};
pub use trace::{NoopTracer, Span, SpanId, Tracer};
pub use traced::{Field, Traced};
