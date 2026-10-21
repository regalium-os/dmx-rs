// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The runtime: where the core's decisions meet a clock and a transport.
//!
//! The core's [`Refresher`](regalium_dmx_core::refresh::Refresher) says send
//! now or idle until when. This crate owns the clock, the waiting and the
//! sending that act on it: one [`Output`] per transport, refreshing every
//! universe patched to it, woken early when a level changes.
//!
//! # Lifecycle is explicit
//!
//! [`Output::builder`] constructs and starts nothing, so a caller can build,
//! patch, inspect and discard. [`Output::run`] blocks until
//! [`Output::close`]; [`Output::start`] runs the same loop on exactly one
//! thread of its own, and the [`Running`] it returns joins that thread on
//! close or drop. Close is idempotent. A transport that fails is an
//! [`Event`], reported once when it starts and once when it stops — never an
//! error return, since the loop keeps trying and the caller has nothing to
//! retry.
//!
//! # Why a thread per output, not an executor
//!
//! A universe is late when its thread is late. With one thread per
//! transport, an output's timing depends on its own work alone, never on
//! whatever else shares an executor with it. The waiting is a condition
//! variable on the injected [`Clock`]; what it achieves is on
//! `dmx.frame.interval`.
//!
//! # Layer contract
//!
//! Owns: the clock, the waiting, each output's thread, the patch, and the
//! event queue's backpressure policy (drop and count, never block).
//!
//! Must not: decode a protocol header, or decide when to send. The first is
//! a transport's; the second is the core's refresher's, and a runtime that
//! second-guesses it has a timing rule no table test covers.
//!
//! Allowed imports: `regalium-dmx-core`, every transport, the schema, the
//! seam.
//!
//! # Spans
//!
//! None per frame. Frames are recorded on `dmx.frames`,
//! `dmx.frame.interval`, `dmx.transport.packets` and `dmx.universe.slots`.
//!
//! # State
//!
//! Phase 6b: [`Output`] and [`Input`] over any transport, on [`SystemClock`]
//! or [`ManualClock`]; [`Levels`] and [`Merged`] handles; send and receive
//! failure, sources found and sources lost as [`Event`]s; and the RDM
//! [`rdm::Controller`] over any RDM line.

mod builder;
mod clock;
mod event;
mod input;
mod levels;
mod output;
mod pump;
pub mod rdm;
mod running;

pub use builder::OutputBuilder;
pub use clock::{Clock, ManualClock, SystemClock, Wake};
pub use event::{EVENT_QUEUE, Event, Events};
pub use input::{Input, InputBuilder, MAX_WAIT, Merged, RETRY};
pub use levels::Levels;
pub use output::{MIN_INTERVAL, Output};
pub use running::Running;
