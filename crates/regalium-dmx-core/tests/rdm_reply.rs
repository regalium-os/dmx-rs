// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! An answer is matched to its question, field by field, or refused.

use regalium_dmx_core::rdm::{
    CommandClass, Message, NackReason, Pid, Reply, Request, ResponseType, Uid, Unexpected, reply,
};
use std::time::Duration;

const ME: Uid = Uid::new(0x7a70, 1);
const DEVICE: Uid = Uid::new(0x4c55, 0x1234_5678);

fn asked() -> Message<'static> {
    Request::get(DEVICE, Pid::DEVICE_INFO).message(ME, 9, 1)
}

fn answer(kind: ResponseType, data: &[u8]) -> Message<'_> {
    Message {
        destination: ME,
        source: DEVICE,
        transaction: 9,
        port_or_response: kind.0,
        message_count: 0,
        sub_device: 0,
        command_class: CommandClass::GET_RESPONSE,
        pid: Pid::DEVICE_INFO,
        data,
    }
}

#[test]
fn each_response_type_reads_as_its_reply() {
    assert_eq!(
        reply(&asked(), &answer(ResponseType::ACK, &[1, 2])),
        Ok(Reply::Ack(&[1, 2]))
    );
    assert_eq!(
        reply(&asked(), &answer(ResponseType::ACK_TIMER, &[0, 15])),
        Ok(Reply::AckTimer(Duration::from_millis(1500)))
    );
    assert_eq!(
        reply(&asked(), &answer(ResponseType::NACK_REASON, &[0, 0])),
        Ok(Reply::Nack(NackReason::UNKNOWN_PID))
    );
    assert_eq!(
        reply(&asked(), &answer(ResponseType::ACK_OVERFLOW, &[7])),
        Ok(Reply::AckOverflow(&[7]))
    );
}

#[test]
fn an_answer_to_someone_else_or_another_question_is_refused() {
    let ok = answer(ResponseType::ACK, &[]);
    let cases = [
        (
            Message {
                source: Uid::new(1, 1),
                ..ok
            },
            Unexpected::Source(Uid::new(1, 1)),
        ),
        (
            Message {
                destination: Uid::new(2, 2),
                ..ok
            },
            Unexpected::Destination(Uid::new(2, 2)),
        ),
        (
            Message {
                transaction: 8,
                ..ok
            },
            Unexpected::Transaction(8),
        ),
        (
            Message {
                command_class: CommandClass::SET_RESPONSE,
                ..ok
            },
            Unexpected::CommandClass(CommandClass::SET_RESPONSE),
        ),
        (
            Message {
                pid: Pid::DEVICE_LABEL,
                ..ok
            },
            Unexpected::Pid(Pid::DEVICE_LABEL),
        ),
        (
            Message {
                port_or_response: 9,
                ..ok
            },
            Unexpected::ResponseType(9),
        ),
        (answer(ResponseType::NACK_REASON, &[1]), Unexpected::Data),
    ];
    for (got, why) in cases {
        assert_eq!(reply(&asked(), &got), Err(why));
    }
}

#[test]
fn a_queued_message_answer_carries_the_queued_parameter() {
    let queued = Request::get(DEVICE, Pid::QUEUED_MESSAGE).message(ME, 9, 1);
    let got = Message {
        pid: Pid::SENSOR_VALUE,
        ..answer(ResponseType::ACK, &[1])
    };
    assert_eq!(reply(&queued, &got), Ok(Reply::Ack(&[1])));
}

#[test]
fn a_broadcast_expects_no_answer() {
    assert!(!Request::get(Uid::ALL, Pid::DEVICE_INFO).expects_response());
    assert!(!Request::get(Uid::all_of(0x4c55), Pid::DEVICE_INFO).expects_response());
    assert!(Request::get(DEVICE, Pid::DEVICE_INFO).expects_response());
}
