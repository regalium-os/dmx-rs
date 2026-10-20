// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The seam's [`Tracer`], over OpenTelemetry spans.
//!
//! The seam names a span by number so it carries no SDK type; this side
//! keeps the OpenTelemetry span under that number until it ends, and starts
//! a child inside its parent's context, so a trace nests the way the work
//! did. A pipeline with tracing off hands out [`SpanId::NONE`].

use super::Otel;
use opentelemetry::global::{BoxedSpan, BoxedTracer};
use opentelemetry::propagation::TextMapPropagator as _;
use opentelemetry::trace::{Span as _, Status, TraceContextExt, Tracer as _};
use opentelemetry::{Context, KeyValue};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use regalium_dmx_telemetry::{Attribute, AttributeValue, SpanId, Tracer};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

/// The tracer, and the spans the seam has started and not yet ended.
#[derive(Default)]
pub(crate) struct Spans {
    tracer: Option<BoxedTracer>,
    open: Mutex<HashMap<u64, BoxedSpan>>,
    next: AtomicU64,
}

impl Spans {
    pub(crate) fn new(tracer: Option<BoxedTracer>) -> Self {
        Self {
            tracer,
            ..Self::default()
        }
    }

    /// The trace and span ids of an open span, in hex, for a log line to
    /// carry so a reader can go from it to its trace.
    pub(crate) fn ids(&self, span: SpanId) -> Option<(String, String)> {
        let open = self.open.lock().ok()?;
        let context = open.get(&span.0)?.span_context().clone();
        Some((
            context.trace_id().to_string(),
            context.span_id().to_string(),
        ))
    }
}

impl Otel {
    /// Starts `name` inside `within`, or inside the open span `parent` when
    /// `within` is `None`.
    fn begin(&self, name: &'static str, parent: SpanId, within: Option<Context>) -> SpanId {
        let Some(tracer) = &self.spans.tracer else {
            return SpanId::NONE;
        };
        let _context = self.runtime.as_ref().map(tokio::runtime::Runtime::enter);
        let Ok(mut open) = self.spans.open.lock() else {
            return SpanId::NONE;
        };
        let within = within.unwrap_or_else(|| {
            open.get(&parent.0).map_or_else(Context::new, |p| {
                Context::new().with_remote_span_context(p.span_context().clone())
            })
        });
        let id = self.spans.next.fetch_add(1, Ordering::Relaxed) + 1;
        open.insert(id, tracer.start_with_context(name, &within));
        SpanId(id)
    }
}

impl Tracer for Otel {
    fn start(&self, name: &'static str, parent: SpanId) -> SpanId {
        self.begin(name, parent, None)
    }

    /// Parsed by OpenTelemetry's own W3C propagator, which rejects a
    /// malformed or all-zero header; the span then starts a new trace.
    fn start_remote(&self, name: &'static str, traceparent: &str) -> SpanId {
        let carrier =
            std::collections::HashMap::from([("traceparent".to_owned(), traceparent.to_owned())]);
        let extracted = TraceContextPropagator::new().extract(&carrier);
        let within = extracted
            .span()
            .span_context()
            .is_valid()
            .then_some(extracted);
        self.begin(name, SpanId::NONE, within)
    }

    fn attribute(&self, span: SpanId, attribute: Attribute<'_>) {
        let Ok(mut open) = self.spans.open.lock() else {
            return;
        };
        if let Some(s) = open.get_mut(&span.0) {
            let key = attribute.key.name();
            s.set_attribute(match attribute.value {
                AttributeValue::Str(v) => KeyValue::new(key, v.to_owned()),
                AttributeValue::Int(v) => KeyValue::new(key, v),
                AttributeValue::Bool(v) => KeyValue::new(key, v),
            });
        }
    }

    fn end(&self, span: SpanId, error: Option<&dyn Display>) {
        let removed = self
            .spans
            .open
            .lock()
            .ok()
            .and_then(|mut open| open.remove(&span.0));
        let Some(mut s) = removed else {
            return;
        };
        let _context = self.runtime.as_ref().map(tokio::runtime::Runtime::enter);
        if let Some(e) = error {
            let message = e.to_string();
            s.add_event(
                "exception",
                vec![KeyValue::new("exception.message", message.clone())],
            );
            s.set_status(Status::error(message));
        } else {
            s.set_status(Status::Ok);
        }
        s.end();
    }
}
