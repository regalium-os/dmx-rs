// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

//! Seam spans, through the adapter, into an in-memory SDK exporter: the
//! name, the declared attributes under their keys, and a failure's status.

use crate::Otel;
use opentelemetry::metrics::MeterProvider;
use opentelemetry::trace::{Status, TracerProvider};
use opentelemetry::{Key, Value};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use regalium_dmx_telemetry::{Attribute, Span, SpanId, Tracer, keys};

fn pipeline() -> (Otel, SdkTracerProvider, InMemorySpanExporter) {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    (
        Otel::from_tracer(provider.tracer("test")),
        provider,
        exporter,
    )
}

#[test]
fn a_span_reaches_the_exporter_with_its_declared_attributes() {
    let (otel, provider, exporter) = pipeline();
    let span = Span::start(&otel, "dmx.frame.decode");
    otel.attribute(SpanId(1), Attribute::int(keys::PACKET_OCTETS, 8));
    span.end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "dmx.frame.decode");
    let octets = spans[0]
        .attributes
        .iter()
        .find(|kv| kv.key == Key::from("dmx.packet.octets"));
    assert_eq!(octets.map(|kv| kv.value.clone()), Some(Value::I64(8)));
}

#[test]
fn a_failed_span_ends_with_the_error_status() {
    let (otel, provider, exporter) = pipeline();
    Span::start(&otel, "dmx.frame.decode").fail(&"truncated at offset 0");
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert!(
        matches!(&spans[0].status, Status::Error { description } if description.contains("truncated")),
        "{:?}",
        spans[0].status
    );
}

#[test]
fn without_a_tracer_nothing_is_started_or_kept() {
    let otel = Otel::from_meter(opentelemetry_sdk::metrics::SdkMeterProvider::default().meter("m"));
    assert_eq!(otel.start("dmx.frame.decode", SpanId::NONE), SpanId::NONE);
    otel.attribute(SpanId::NONE, Attribute::int(keys::PACKET_OCTETS, 1));
    otel.end(SpanId::NONE, None);
}

#[test]
fn a_child_span_belongs_to_its_parents_trace_and_names_it_as_parent() {
    let (otel, provider, exporter) = pipeline();
    let cycle = Span::start(&otel, "dmx.output.refresh");
    cycle.child("dmx.api.get_universe").end();
    cycle.end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let child = spans
        .iter()
        .find(|s| s.name == "dmx.api.get_universe")
        .unwrap();
    let parent = spans
        .iter()
        .find(|s| s.name == "dmx.output.refresh")
        .unwrap();
    assert_eq!(
        child.span_context.trace_id(),
        parent.span_context.trace_id()
    );
    assert_eq!(child.parent_span_id, parent.span_context.span_id());
}

#[test]
fn a_remote_traceparent_makes_the_span_its_child() {
    let (otel, provider, exporter) = pipeline();
    let parent = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
    Span::remote(&otel, "dmx.server.call", parent).end();
    Span::remote(&otel, "dmx.server.call", "00-not-a-header").end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let joined = &spans[0];
    assert_eq!(
        joined.span_context.trace_id().to_string(),
        "4bf92f3577b34da6a3ce929d0e0e4736"
    );
    assert_eq!(joined.parent_span_id.to_string(), "00f067aa0ba902b7");
    let fresh = &spans[1];
    assert_ne!(
        fresh.span_context.trace_id(),
        joined.span_context.trace_id(),
        "a malformed header starts a new trace"
    );
}
