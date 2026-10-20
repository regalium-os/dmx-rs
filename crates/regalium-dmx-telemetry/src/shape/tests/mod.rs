// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// A test asserts by panicking; the never-panic lints are not the rule here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The universe series. The node and RDM identity series are in [`identity`].

mod identity;

use super::*;
use crate::{AttributeValue, Instrument, Meter};
use std::sync::Mutex;

/// One recording: the instrument's name, its labels as text, its value.
type Recorded = (&'static str, Vec<(&'static str, String)>, f64);

#[derive(Default)]
pub(super) struct Recorder(Mutex<Vec<Recorded>>);

impl Meter for Recorder {
    fn record(&self, instrument: &Instrument, value: f64, attributes: &[Attribute<'_>]) {
        let labels = attributes
            .iter()
            .map(|a| {
                let v = match a.value {
                    AttributeValue::Str(s) => s.to_owned(),
                    AttributeValue::Int(n) => n.to_string(),
                    AttributeValue::Bool(b) => b.to_string(),
                };
                (a.key.name(), v)
            })
            .collect();
        self.0
            .lock()
            .unwrap()
            .push((instrument.name, labels, value));
    }
}

impl Recorder {
    pub(super) fn all(&self) -> Vec<Recorded> {
        self.0.lock().unwrap().clone()
    }
}

const WASH: UniverseId<'static> = UniverseId::new("sacn", 7);

fn ids() -> Vec<(&'static str, String)> {
    vec![
        ("dmx.protocol", "sacn".to_owned()),
        ("dmx.universe", "7".to_owned()),
    ]
}

fn labelled(extra: &[(&'static str, &str)]) -> Vec<(&'static str, String)> {
    let mut want = ids();
    want.extend(extra.iter().map(|(k, v)| (*k, (*v).to_owned())));
    want
}

#[test]
fn a_universe_series_carries_exactly_the_two_ids() {
    let meter = Recorder::default();
    record_frame_interval(&meter, &WASH, 22.7);
    record_universe_slots(&meter, &WASH, 512);
    record_universe_sources(&meter, &WASH, 2);
    record_sequence_error(&meter, &WASH);
    record_source_lost(&meter, &WASH);
    record_rdm_discovered(&meter, &WASH, 12);
    let all = meter.all();
    assert_eq!(all.len(), 6);
    for (name, labels, _) in all {
        assert_eq!(labels, ids(), "{name}");
    }
}

#[test]
fn a_frame_adds_only_its_direction() {
    let meter = Recorder::default();
    record_frame(&meter, &WASH, Direction::Sent);
    let want = labelled(&[("dmx.direction", "sent")]);
    assert_eq!(meter.all(), [("dmx.frames", want, 1.0)]);
}

#[test]
fn universe_info_names_the_universe_on_its_two_ids() {
    let meter = Recorder::default();
    record_universe_info(&meter, &WASH, "stage left wash", "htp");
    let want = labelled(&[
        ("dmx.universe.name", "stage left wash"),
        ("dmx.merge.mode", "htp"),
    ]);
    assert_eq!(meter.all(), [("dmx.universe.info", want, 1.0)]);
}

#[test]
fn a_packet_is_counted_by_protocol_and_direction_only() {
    let meter = Recorder::default();
    record_packet(&meter, "artnet", Direction::Received, 530);
    let labels = vec![
        ("dmx.protocol", "artnet".to_owned()),
        ("dmx.direction", "received".to_owned()),
    ];
    assert_eq!(
        meter.all(),
        [
            ("dmx.transport.packets", labels.clone(), 1.0),
            ("dmx.transport.bytes", labels, 530.0),
        ]
    );
}

#[test]
fn the_timing_histograms_resolve_a_44_hz_refresh() {
    let buckets = crate::instruments::FRAME_INTERVAL.boundaries;
    let around = buckets
        .iter()
        .filter(|b| (20.0..=25.0).contains(*b))
        .count();
    assert!(
        around >= 4,
        "a 22.7 ms refresh needs buckets around it: {buckets:?}"
    );
    assert!(buckets.windows(2).all(|w| w[0] < w[1]), "ascending");
    let rdm = crate::instruments::RDM_REQUEST_DURATION.boundaries;
    assert!(rdm.windows(2).all(|w| w[0] < w[1]), "ascending");
}
