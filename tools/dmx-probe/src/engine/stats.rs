// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Frame statistics per universe, gathered as a [`Meter`]: the runtime
//! already records every frame interval through the seam, so the probe
//! listens there rather than asking the runtime for anything new.
//!
//! No clock: the frame rate is the inverse of the mean of the latest
//! intervals, and the histogram counts the intervals kept.

use regalium_dmx_telemetry::{Attribute, AttributeValue, Instrument, Meter, instruments, keys};
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, PoisonError};

/// Intervals kept per universe: about 23 seconds at full rate.
pub const KEPT: usize = 1000;

/// Intervals the frame rate is averaged over.
const RATE_OVER: usize = 22;

/// One universe's intervals, in milliseconds, oldest first.
#[derive(Debug, Default)]
struct Series(VecDeque<f32>);

/// Every universe's frame intervals, keyed by protocol and number.
#[derive(Debug, Default)]
pub struct Stats {
    series: Mutex<HashMap<(String, i64), Series>>,
}

impl Stats {
    /// Empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// For `protocol`'s universe `number`: frames a second, the latest
    /// `last` intervals, and the histogram over everything kept.
    #[must_use]
    pub fn of(&self, protocol: &str, number: i64, last: usize) -> (f32, Vec<f32>, [u32; 7]) {
        let series = self.series.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(Series(kept)) = series.get(&(protocol.to_owned(), number)) else {
            return (0.0, Vec::new(), [0; 7]);
        };
        let latest: Vec<f32> = kept.iter().rev().take(RATE_OVER).copied().collect();
        let mean = latest.iter().sum::<f32>() / f32::from(u16::try_from(latest.len()).unwrap_or(1).max(1));
        let fps = if mean > 0.0 { 1000.0 / mean } else { 0.0 };
        let recent = kept.iter().skip(kept.len().saturating_sub(last)).copied().collect();
        let mut histogram = [0u32; 7];
        for &ms in kept {
            let at = match ms {
                ms if ms < 21.0 => 0,
                ms if ms < 22.0 => 1,
                ms if ms < 23.0 => 2,
                ms if ms < 24.0 => 3,
                ms if ms < 25.0 => 4,
                ms if ms < 30.0 => 5,
                _ => 6,
            };
            if let Some(n) = histogram.get_mut(at) {
                *n += 1;
            }
        }
        (fps, recent, histogram)
    }

    /// Forgets a universe whose output has gone quiet, so its rate drops to
    /// zero rather than showing the last one forever.
    pub fn forget(&self, protocol: &str, number: i64) {
        let mut series = self.series.lock().unwrap_or_else(PoisonError::into_inner);
        series.remove(&(protocol.to_owned(), number));
    }
}

impl Meter for Stats {
    fn record(&self, instrument: &Instrument, value: f64, attributes: &[Attribute<'_>]) {
        if instrument.name != instruments::FRAME_INTERVAL.name {
            return;
        }
        let mut protocol = None;
        let mut number = None;
        for a in attributes {
            match a.value {
                AttributeValue::Str(s) if a.key == keys::PROTOCOL => protocol = Some(s),
                AttributeValue::Int(n) if a.key == keys::UNIVERSE => number = Some(n),
                _ => {}
            }
        }
        let (Some(protocol), Some(number)) = (protocol, number) else {
            return;
        };
        let mut series = self.series.lock().unwrap_or_else(PoisonError::into_inner);
        let Series(kept) = series.entry((protocol.to_owned(), number)).or_default();
        if kept.len() == KEPT {
            kept.pop_front();
        }
        #[allow(clippy::cast_possible_truncation, reason = "milliseconds fit an f32")]
        kept.push_back(value as f32);
    }
}
