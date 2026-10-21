// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Remote Device Management, ANSI E1.20: pure, lossless and borrowing.
//!
//! RDM shares the DMX512 line, as packets under start code 0xCC, between
//! the levels. A controller asks; one device answers, inside a turnaround
//! window the line's port owns. This module is the octets and the logic:
//! [`Message`] and its checksum, [`Uid`], the [`codes`], the discovery
//! response's own encoding ([`disc`]), [`DeviceInfo`], matching an answer
//! to its question ([`reply`]), and discovery's binary search
//! ([`Discovery`]); and a [`Responder`], one device answering as the
//! standard says, for the simulator and for tests. No I/O and no clock.

pub mod codes;
mod device_info;
pub mod disc;
mod discovery;
mod error;
mod message;
mod request;
mod responder;
mod uid;

pub use codes::{CommandClass, NackReason, Pid, ResponseType};
pub use device_info::{DeviceInfo, LEN as DEVICE_INFO_LEN};
pub use discovery::{Discovery, MAX_STEPS, MUTE_TRIES, Next, branch_data};
pub use error::{Error, ErrorKind};
pub use message::{HEADER, MAX_DATA, MAX_LEN, Message, SUB_START, checksum};
pub use request::{Reply, Request, Unexpected, reply};
pub use responder::{Responder, Response};
pub use uid::Uid;
