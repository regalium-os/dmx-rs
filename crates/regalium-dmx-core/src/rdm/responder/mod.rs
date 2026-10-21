// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A responder: one RDM device, as the standard says it answers.
//!
//! What the simulator puts on a virtual line, and what the controller's
//! tests talk to. It answers discovery (unique branch, mute, un-mute) and
//! the parameters a controller reads first — `DEVICE_INFO`, the start
//! address, the label, identify, the supported parameters, the queue —
//! and NACKs the rest. Answers too long for one message go out as
//! `ACK_OVERFLOW` parts; one parameter can be made slow, to answer with
//! `ACK_TIMER` and queue its answer.

mod params;

use super::codes::{CommandClass, NackReason, Pid, ResponseType};
use super::device_info::DeviceInfo;
use super::disc;
use super::message::{MAX_DATA, Message};
use super::uid::Uid;

/// One device. Plain data; `Send` and `Sync`.
#[derive(Clone, Debug)]
pub struct Responder {
    uid: Uid,
    info: DeviceInfo,
    label: Vec<u8>,
    identify: bool,
    muted: bool,
    slow: Option<Pid>,
    queued: Option<(Pid, Vec<u8>)>,
    overflow: Option<(Pid, Vec<u8>)>,
    supported: Vec<Pid>,
}

/// What a request drew.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    /// Nothing: not for this device, a broadcast, or a muted device asked
    /// to branch.
    Silent,
    /// A discovery answer: the encoded UID, not a message.
    Discovery(Vec<u8>),
    /// A message, start code to checksum.
    Message(Vec<u8>),
}

impl Responder {
    /// Device `uid`, describing itself by `info`, at `info`'s start address.
    #[must_use]
    pub fn new(uid: Uid, info: DeviceInfo) -> Self {
        Self {
            uid,
            info,
            label: Vec::new(),
            identify: false,
            muted: false,
            slow: None,
            queued: None,
            overflow: None,
            supported: vec![Pid::DEVICE_LABEL, Pid::IDENTIFY_DEVICE],
        }
    }

    /// Labelled `label`, as `DEVICE_LABEL` answers; cut to its 32 octets.
    #[must_use]
    pub fn labelled(mut self, label: &str) -> Self {
        self.label = label
            .as_bytes()
            .get(..32)
            .unwrap_or(label.as_bytes())
            .to_vec();
        self
    }

    /// Answers `pid` with `ACK_TIMER` first, queuing the real answer.
    #[must_use]
    pub const fn slow(mut self, pid: Pid) -> Self {
        self.slow = Some(pid);
        self
    }

    /// Lists `pids` in `SUPPORTED_PARAMETERS`: enough of them overflow.
    #[must_use]
    pub fn supporting(mut self, pids: &[Pid]) -> Self {
        self.supported = pids.to_vec();
        self
    }

    /// The device's UID.
    #[must_use]
    pub const fn uid(&self) -> Uid {
        self.uid
    }

    /// What `DEVICE_INFO` says now: a SET of the start address shows here.
    #[must_use]
    pub const fn info(&self) -> DeviceInfo {
        self.info
    }

    /// Whether it is identifying.
    #[must_use]
    pub const fn identifying(&self) -> bool {
        self.identify
    }

    /// Answers `request`, the octets a controller sent.
    pub fn answer(&mut self, request: &[u8]) -> Response {
        let Ok(m) = Message::decode(request) else {
            return Response::Silent;
        };
        let broadcast =
            m.destination == Uid::ALL || m.destination == Uid::all_of(self.uid.manufacturer());
        if m.destination != self.uid && !broadcast {
            return Response::Silent;
        }
        if m.command_class == CommandClass::DISCOVERY {
            return self.discovery(&m, broadcast);
        }
        let (kind, data, pid) = self.parameter(&m);
        if broadcast {
            return Response::Silent;
        }
        self.respond(&m, kind, &data, pid)
    }

    fn discovery(&mut self, m: &Message<'_>, broadcast: bool) -> Response {
        match m.pid {
            Pid::DISC_UNIQUE_BRANCH => {
                let (Some(lower), Some(upper)) =
                    (Uid::read(m.data), m.data.get(6..).and_then(Uid::read))
                else {
                    return Response::Silent;
                };
                if self.muted || !(lower..=upper).contains(&self.uid) {
                    return Response::Silent;
                }
                let mut out = Vec::new();
                disc::encode(self.uid, 7, &mut out);
                Response::Discovery(out)
            }
            Pid::DISC_MUTE | Pid::DISC_UN_MUTE => {
                self.muted = m.pid == Pid::DISC_MUTE;
                if broadcast {
                    Response::Silent
                } else {
                    self.respond(m, ResponseType::ACK, &[0, 0], m.pid)
                }
            }
            _ => Response::Silent,
        }
    }

    /// A response to `m`, of `kind`, carrying `data`, for `pid`: the
    /// request's own, or a queued answer's.
    fn respond(&self, m: &Message<'_>, kind: ResponseType, data: &[u8], pid: Pid) -> Response {
        let mut out = Vec::new();
        Message {
            destination: m.source,
            source: self.uid,
            transaction: m.transaction,
            port_or_response: kind.0,
            message_count: u8::from(self.queued.is_some()),
            sub_device: m.sub_device,
            command_class: m.command_class.response(),
            pid,
            data,
        }
        .encode(&mut out);
        Response::Message(out)
    }
}

/// A NACK's data.
pub(crate) fn nack(reason: NackReason) -> (ResponseType, Vec<u8>) {
    (ResponseType::NACK_REASON, reason.0.to_be_bytes().to_vec())
}

/// `data`, as one `ACK` or the first of several `ACK_OVERFLOW` parts; the
/// rest is kept for the next ask.
pub(crate) fn parts(
    overflow: &mut Option<(Pid, Vec<u8>)>,
    pid: Pid,
    mut data: Vec<u8>,
) -> (ResponseType, Vec<u8>) {
    if data.len() <= MAX_DATA {
        return (ResponseType::ACK, data);
    }
    let rest = data.split_off(MAX_DATA);
    *overflow = Some((pid, rest));
    (ResponseType::ACK_OVERFLOW, data)
}
