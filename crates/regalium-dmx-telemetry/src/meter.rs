// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Metrics: declared instruments, recorded with declared attributes.
//!
//! An instrument is a `const` declared once beside the code it measures, with
//! its unit and description, so a dashboard author reads what a series means
//! from its declaration rather than guessing from its name. A recording names
//! the instrument and passes [`Attribute`]s built only from declared
//! [`Key`](crate::Key)s — there is no way to label a series with an ad hoc
//! string.

use crate::Attribute;

/// How a recorded value combines with the ones before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Monotonic total: each recording is added. Negative values are a bug
    /// and an adapter may drop them.
    Counter,
    /// Current value: each recording replaces the last, per attribute set.
    /// What a universe's source count is.
    Gauge,
    /// Distribution: each recording is one sample, e.g. a round-trip time.
    Histogram,
}

/// A metric, declared once as a `const`.
///
/// Names follow OpenTelemetry convention, `dmx.<thing>.<measure>`, and are
/// translated by the exporter (Prometheus turns the dots into underscores and
/// appends the unit).
///
/// `PartialEq` but not `Eq`: a histogram's boundaries are `f64`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instrument {
    /// The series name, e.g. `dmx.output.frames`.
    pub name: &'static str,
    /// UCUM unit, e.g. `ms`, `By`, or `1` for a dimensionless count.
    pub unit: &'static str,
    /// What one recorded value means.
    pub description: &'static str,
    /// How recordings combine.
    pub kind: Kind,
    /// A histogram's bucket boundaries, in its unit; empty for every other
    /// kind, and for a histogram content with OpenTelemetry's defaults.
    pub boundaries: &'static [f64],
}

impl Instrument {
    /// Declares a counter.
    #[must_use]
    pub const fn counter(
        name: &'static str,
        unit: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            unit,
            description,
            kind: Kind::Counter,
            boundaries: &[],
        }
    }

    /// Declares a gauge.
    #[must_use]
    pub const fn gauge(name: &'static str, unit: &'static str, description: &'static str) -> Self {
        Self {
            name,
            unit,
            description,
            kind: Kind::Gauge,
            boundaries: &[],
        }
    }

    /// Declares a histogram.
    #[must_use]
    pub const fn histogram(
        name: &'static str,
        unit: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            unit,
            description,
            kind: Kind::Histogram,
            boundaries: &[],
        }
    }
}

impl Instrument {
    /// Declares a histogram with its own bucket `boundaries`, ascending.
    ///
    /// For a measure the defaults cannot resolve: OpenTelemetry's buckets
    /// jump from 10 to 25 ms, and a full DMX universe refreshes every
    /// 22.7 ms at its fastest (about 44 Hz under ANSI E1.11's timing), where
    /// a 2 ms slip is visible flicker.
    #[must_use]
    pub const fn histogram_with(
        name: &'static str,
        unit: &'static str,
        description: &'static str,
        boundaries: &'static [f64],
    ) -> Self {
        Self {
            boundaries,
            ..Self::histogram(name, unit, description)
        }
    }
}

/// Where recordings go. Declared here, by the consumer of the boundary, and
/// satisfied by an adapter — `regalium-dmx-telemetry-otel` binds it to the
/// the-protobuf-project telemetry SDK.
///
/// # Concurrency
///
/// `Send + Sync` because a driver records from its socket thread and its
/// refresh threads at once. Implementations must accept concurrent calls; a
/// recording is fire-and-forget and must never block on export.
pub trait Meter: Send + Sync {
    /// Records `value` against `instrument`, labelled by `attributes`.
    ///
    /// Infallible on purpose: a metric that cannot be exported is the
    /// telemetry pipeline's problem to report, never a reason for a frame to a
    /// dimmer to be late.
    fn record(&self, instrument: &Instrument, value: f64, attributes: &[Attribute<'_>]);
}

/// A meter that records nothing, for callers that configure no telemetry.
/// `Default`, zero-sized, and free: it costs one empty call.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopMeter;

impl Meter for NoopMeter {
    fn record(&self, _: &Instrument, _: f64, _: &[Attribute<'_>]) {}
}

#[cfg(test)]
mod tests {
    use super::{Instrument, Kind, Meter, NoopMeter};
    use crate::{Attribute, keys};

    const REQUESTS: Instrument = Instrument::counter("dmx.test.requests", "1", "Requests sent");

    #[test]
    fn an_instrument_declares_its_kind_at_compile_time() {
        assert_eq!(REQUESTS.kind, Kind::Counter);
        assert_eq!(Instrument::gauge("g", "1", "").kind, Kind::Gauge);
        assert_eq!(Instrument::histogram("h", "ms", "").kind, Kind::Histogram);
    }

    #[test]
    fn the_noop_meter_accepts_anything() {
        let meter: &dyn Meter = &NoopMeter;
        meter.record(&REQUESTS, 1.0, &[Attribute::int(keys::UNIVERSE, 7)]);
    }
}
