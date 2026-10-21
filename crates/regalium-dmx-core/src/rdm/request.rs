// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A controller's request, and reading what came back as the answer to it.
//!
//! A response belongs to a request when it comes from the request's
//! destination, to its source, with its transaction number, the response
//! command class and the same parameter. Anything else on the line — a
//! late answer to an earlier request, another controller's traffic — is
//! [`Unexpected`], never mistaken for the answer.

use super::codes::{CommandClass, NackReason, Pid, ResponseType};
use super::message::Message;
use super::uid::Uid;
use core::time::Duration;

/// What to ask, before the controller numbers it.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request<'a> {
    /// The device, or a broadcast UID.
    pub destination: Uid,
    /// 0 for the root device.
    pub sub_device: u16,
    /// GET, SET or discovery.
    pub command_class: CommandClass,
    /// The parameter.
    pub pid: Pid,
    /// The parameter data.
    pub data: &'a [u8],
}

impl<'a> Request<'a> {
    /// GET `pid` from `destination`'s root device.
    #[must_use]
    pub const fn get(destination: Uid, pid: Pid) -> Self {
        Self {
            destination,
            sub_device: 0,
            command_class: CommandClass::GET,
            pid,
            data: &[],
        }
    }

    /// SET `pid` on `destination`'s root device to `data`.
    #[must_use]
    pub const fn set(destination: Uid, pid: Pid, data: &'a [u8]) -> Self {
        Self {
            destination,
            sub_device: 0,
            command_class: CommandClass::SET,
            pid,
            data,
        }
    }

    /// The message: from `source`, numbered `transaction`, on port `port`.
    #[must_use]
    pub const fn message(&self, source: Uid, transaction: u8, port: u8) -> Message<'a> {
        Message {
            destination: self.destination,
            source,
            transaction,
            port_or_response: port,
            message_count: 0,
            sub_device: self.sub_device,
            command_class: self.command_class,
            pid: self.pid,
            data: self.data,
        }
    }

    /// Whether a response is expected: never to a broadcast.
    #[must_use]
    pub const fn expects_response(&self) -> bool {
        !self.destination.is_broadcast()
    }
}

/// What a device answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply<'a> {
    /// Done; the answer's data.
    Ack(&'a [u8]),
    /// Ask again after this long, through `QUEUED_MESSAGE`.
    AckTimer(Duration),
    /// Refused, and why.
    Nack(NackReason),
    /// Part of the answer; ask again for the rest.
    AckOverflow(&'a [u8]),
}

/// Why a message is not the answer to a request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Unexpected {
    /// From someone other than the request's destination.
    Source(Uid),
    /// Addressed to someone other than the request's source.
    Destination(Uid),
    /// Another transaction's.
    Transaction(u8),
    /// Not the response command class.
    CommandClass(CommandClass),
    /// Another parameter's.
    Pid(Pid),
    /// A response type E1.20 does not define.
    ResponseType(u8),
    /// An `ACK_TIMER` or `NACK_REASON` without its two octets.
    Data,
}

/// Reads `got` as the answer to `sent`. A `QUEUED_MESSAGE` request's answer
/// carries the queued parameter's id, so for one any PID is accepted.
///
/// # Errors
///
/// [`Unexpected`] when `got` is not the answer to `sent`.
pub fn reply<'a>(sent: &Message<'_>, got: &Message<'a>) -> Result<Reply<'a>, Unexpected> {
    if got.source != sent.destination {
        return Err(Unexpected::Source(got.source));
    }
    if got.destination != sent.source {
        return Err(Unexpected::Destination(got.destination));
    }
    if got.transaction != sent.transaction {
        return Err(Unexpected::Transaction(got.transaction));
    }
    if got.command_class != sent.command_class.response() {
        return Err(Unexpected::CommandClass(got.command_class));
    }
    if got.pid != sent.pid && sent.pid != Pid::QUEUED_MESSAGE {
        return Err(Unexpected::Pid(got.pid));
    }
    let two = || match got.data {
        &[hi, lo] => Ok(u16::from_be_bytes([hi, lo])),
        _ => Err(Unexpected::Data),
    };
    Ok(match ResponseType(got.port_or_response) {
        ResponseType::ACK => Reply::Ack(got.data),
        ResponseType::ACK_TIMER => Reply::AckTimer(Duration::from_millis(100 * u64::from(two()?))),
        ResponseType::NACK_REASON => Reply::Nack(NackReason(two()?)),
        ResponseType::ACK_OVERFLOW => Reply::AckOverflow(got.data),
        ResponseType(other) => return Err(Unexpected::ResponseType(other)),
    })
}
