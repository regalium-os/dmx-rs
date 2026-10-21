// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The RDM controller against responders on an in-memory line, on a
//! manual clock: discovery, GET, SET, NACK, retries, overflow and timers.

mod common;

use common::bench::Bench;
use regalium_dmx_core::rdm::{DeviceInfo, NackReason, Pid, Responder, Uid};
use regalium_dmx_runtime::rdm::{Controller, RdmError};
use regalium_dmx_runtime::{Clock, ManualClock};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

const ME: Uid = Uid::new(0x7a70, 1);

fn info(address: u16) -> DeviceInfo {
    DeviceInfo {
        protocol: 0x0100,
        model: 7,
        footprint: 16,
        personality: 1,
        personalities: 2,
        start_address: address,
        ..DeviceInfo::default()
    }
}

type Rig = (
    Controller<Arc<Bench>, Arc<ManualClock>>,
    Arc<Bench>,
    Arc<ManualClock>,
);

fn rig(devices: Vec<Responder>) -> Rig {
    let bench = Arc::new(Bench::with(devices));
    let clock = Arc::new(ManualClock::new());
    (
        Controller::new(Arc::clone(&bench), Arc::clone(&clock), ME),
        bench,
        clock,
    )
}

fn fixture(device: u32, address: u16) -> Responder {
    Responder::new(Uid::new(0x4c55, device), info(address))
}

#[test]
fn discovery_finds_every_device_and_then_each_answers_device_info() {
    let devices: Vec<Responder> = (0..24)
        .map(|n| fixture(n * 7_919 + 3, 1 + 16 * u16::try_from(n).unwrap()))
        .collect();
    let mut want: Vec<Uid> = devices.iter().map(Responder::uid).collect();
    want.sort();
    let (controller, _, _) = rig(devices);
    let found = controller.discover(1).unwrap();
    assert_eq!(found.devices, want);
    assert!(!found.incomplete);
    let first = controller.device_info(1, want[0]).unwrap();
    assert_eq!((first.footprint, first.personalities), (16, 2));
}

#[test]
fn a_set_moves_the_fixture_and_an_out_of_range_set_is_refused() {
    let (controller, bench, _) = rig(vec![fixture(1, 1)]);
    let uid = Uid::new(0x4c55, 1);
    controller
        .set(1, uid, Pid::DMX_START_ADDRESS, &101u16.to_be_bytes())
        .unwrap();
    assert_eq!(bench.devices.lock().unwrap()[0].info().start_address, 101);
    let refused = controller.set(1, uid, Pid::DMX_START_ADDRESS, &600u16.to_be_bytes());
    assert!(matches!(
        refused,
        Err(RdmError::Nack(NackReason::DATA_OUT_OF_RANGE))
    ));
    let unknown = controller.get(1, uid, Pid(0x8123));
    assert!(matches!(
        unknown,
        Err(RdmError::Nack(NackReason::UNKNOWN_PID))
    ));
}

#[test]
fn a_lost_answer_is_retried_and_a_missing_device_is_no_answer() {
    let (controller, bench, _) = rig(vec![fixture(1, 1)]);
    bench.drop_next.store(2, Ordering::Relaxed);
    assert!(
        controller.device_info(1, Uid::new(0x4c55, 1)).is_ok(),
        "the third try answers"
    );
    let before = bench.requests.load(Ordering::Relaxed);
    assert!(matches!(
        controller.device_info(1, Uid::new(0x4c55, 99)),
        Err(RdmError::NoAnswer)
    ));
    assert_eq!(
        bench.requests.load(Ordering::Relaxed) - before,
        3,
        "one try and two retries"
    );
}

#[test]
fn an_overflowing_answer_is_joined_from_its_parts() {
    let pids: Vec<Pid> = (0..150).map(|n| Pid(0x8000 + n)).collect();
    let (controller, _, _) = rig(vec![fixture(1, 1).supporting(&pids)]);
    let data = controller
        .get(1, Uid::new(0x4c55, 1), Pid::SUPPORTED_PARAMETERS)
        .unwrap();
    assert_eq!(data.len(), 300, "231 and 69, joined");
    assert_eq!(&data[298..], [0x80, 149]);
}

#[test]
fn an_ack_timer_waits_on_the_clock_then_reads_the_queue() {
    let (controller, bench, clock) = rig(vec![fixture(1, 42).slow(Pid::DMX_START_ADDRESS)]);
    let before = clock.now();
    let data = controller
        .get(1, Uid::new(0x4c55, 1), Pid::DMX_START_ADDRESS)
        .unwrap();
    assert_eq!(data, 42u16.to_be_bytes());
    assert!(
        clock.now().checked_sub(before).unwrap() >= Duration::from_millis(100),
        "waited the 100 ms it asked for"
    );
    assert_eq!(
        bench.requests.load(Ordering::Relaxed),
        2,
        "the request, then the queue"
    );
}

#[test]
fn a_broadcast_set_reaches_every_device_and_waits_for_nothing() {
    let (controller, bench, _) = rig(vec![fixture(1, 1), fixture(2, 1)]);
    let data = controller
        .set(1, Uid::ALL, Pid::IDENTIFY_DEVICE, &[1])
        .unwrap();
    assert_eq!(data, []);
    assert!(
        bench
            .devices
            .lock()
            .unwrap()
            .iter()
            .all(Responder::identifying)
    );
}
