// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// A test asserts by panicking; the never-panic lints are not the rule here.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

//! Declared histogram buckets, through the bridge, into an in-memory SDK
//! exporter: a frame interval must land in the bucket the seam declared,
//! not in OpenTelemetry's default 10–25 ms one.

use crate::Otel;
use opentelemetry::metrics::MeterProvider;
use opentelemetry_sdk::metrics::data::{AggregatedMetrics, MetricData};
use opentelemetry_sdk::metrics::{InMemoryMetricExporter, PeriodicReader, SdkMeterProvider};
use regalium_dmx_telemetry::shape::{UniverseId, record_frame_interval};
use regalium_dmx_telemetry::{Instrument, Meter, instruments};

/// The bounds and per-bucket counts of the one histogram named `name`.
fn histogram(record: &dyn Fn(&dyn Meter), name: &str) -> (Vec<f64>, Vec<u64>) {
    let exporter = InMemoryMetricExporter::default();
    let provider = SdkMeterProvider::builder()
        .with_reader(PeriodicReader::builder(exporter.clone()).build())
        .build();
    let live = Otel::from_meter(provider.meter("test"));
    record(&live);
    provider.force_flush().unwrap();
    for resource in exporter.get_finished_metrics().unwrap() {
        for metric in resource
            .scope_metrics()
            .flat_map(opentelemetry_sdk::metrics::data::ScopeMetrics::metrics)
        {
            if let (true, AggregatedMetrics::F64(MetricData::Histogram(h))) =
                (metric.name() == name, metric.data())
            {
                let point = h.data_points().next().expect("one series");
                return (point.bounds().collect(), point.bucket_counts().collect());
            }
        }
    }
    panic!("{name} was not exported");
}

#[test]
fn a_frame_interval_lands_in_the_bucket_the_seam_declared() {
    let wash = UniverseId::new("sacn", 7);
    let (bounds, counts) = histogram(
        &|m| record_frame_interval(m, &wash, 22.7),
        "dmx.frame.interval",
    );
    assert_eq!(bounds, instruments::FRAME_INTERVAL.boundaries);
    // Buckets are (previous, bound]; 22.7 ms falls in (22, 23].
    let at = bounds
        .iter()
        .position(|b| (b - 23.0).abs() < f64::EPSILON)
        .unwrap();
    assert_eq!(counts[at], 1, "{bounds:?} {counts:?}");
}

#[test]
fn a_histogram_declared_without_buckets_keeps_the_sdk_defaults() {
    const PLAIN: Instrument = Instrument::histogram("dmx.test.plain", "ms", "Plain");
    let (bounds, _) = histogram(&|m| m.record(&PLAIN, 3.0, &[]), "dmx.test.plain");
    assert!(
        bounds.contains(&25.0) && !bounds.contains(&23.0),
        "{bounds:?}"
    );
}
