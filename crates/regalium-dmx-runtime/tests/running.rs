// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The output on its own thread, on a manual clock that jumps to each
//! deadline: the rate is exact, a change wakes it, and close joins it.

mod common;

use common::{ms, rig};
use regalium_dmx_core::address::Address;
use regalium_dmx_core::refresh::Refresh;
use regalium_dmx_runtime::Output;
use regalium_dmx_telemetry::{Attribute, AttributeValue, Instrument, Meter};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Waits, in real time, until `done` holds or five seconds pass.
fn eventually(mut done: impl FnMut() -> bool) -> bool {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if done() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    false
}

#[test]
fn a_running_output_keeps_its_interval_exactly_and_close_joins_it() {
    let (transport, clock, wire) = rig(Refresh::continuous(ms(25)));
    let (output, _events) = Output::builder(transport, clock).build();
    let _levels = output.patch(1);
    let mut running = output.start().unwrap();
    assert!(eventually(|| wire.sent().len() >= 40), "the thread sends");
    running.close();
    running.close();
    assert!(output.is_closed());
    assert_eq!(
        *wire.ended.lock().unwrap(),
        [1],
        "closing ends the universe"
    );
    let stopped = wire.sent().len();
    std::thread::sleep(Duration::from_millis(20));
    assert_eq!(wire.sent().len(), stopped, "nothing after close");
    let sent = wire.sent();
    for pair in sent.windows(2) {
        assert_eq!(
            pair[1].at.saturating_sub(pair[0].at),
            ms(25),
            "{:?}",
            pair[1].at
        );
    }
}

#[test]
fn a_change_on_another_thread_reaches_the_wire() {
    let (transport, clock, wire) = rig(Refresh::on_change(ms(25), ms(60_000), 0));
    let (output, _events) = Output::builder(transport, clock).build();
    let levels = output.patch(1);
    levels.set_len(1).unwrap();
    let _running = output.start().unwrap();
    let setter = std::thread::spawn(move || levels.set(Address::FIRST, 200));
    assert!(setter.join().unwrap());
    assert!(eventually(|| wire
        .sent()
        .iter()
        .any(|s| s.octets == [0x00, 200])));
}

#[derive(Default)]
struct Series(Mutex<Vec<(&'static str, f64, Vec<String>)>>);

impl Meter for Series {
    fn record(&self, i: &Instrument, value: f64, attributes: &[Attribute<'_>]) {
        let labels = attributes
            .iter()
            .map(|a| match a.value {
                AttributeValue::Str(s) => format!("{}={s}", a.key.name()),
                AttributeValue::Int(n) => format!("{}={n}", a.key.name()),
                AttributeValue::Bool(b) => format!("{}={b}", a.key.name()),
            })
            .collect();
        self.0.lock().unwrap().push((i.name, value, labels));
    }
}

#[test]
fn every_send_is_counted_and_timed_on_the_universe_series() {
    let (transport, clock, _wire) = rig(Refresh::continuous(ms(25)));
    let series = Arc::new(Series::default());
    let (output, _events) = Output::builder(transport, clock.clone())
        .meter(series.clone())
        .build();
    let _levels = output.patch(4);
    output.step();
    clock.set(ms(25));
    output.step();
    let all = series.0.lock().unwrap().clone();
    let frames = all.iter().filter(|r| r.0 == "dmx.frames").count();
    let intervals: Vec<f64> = all
        .iter()
        .filter(|r| r.0 == "dmx.frame.interval")
        .map(|r| r.1)
        .collect();
    assert_eq!(frames, 2);
    assert_eq!(intervals, [25.0], "the first send has no interval");
    let labels = &all.iter().find(|r| r.0 == "dmx.frames").unwrap().2;
    assert!(
        labels.contains(&"dmx.protocol=serial".to_owned()),
        "{labels:?}"
    );
    assert!(labels.contains(&"dmx.universe=4".to_owned()), "{labels:?}");
}
