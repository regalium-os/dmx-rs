// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The DMX512 line's timing (ANSI E1.11), and how a plain UART meets it.
//!
//! A line is 250 kbaud, 8 data bits, no parity, 2 stop bits. Each packet
//! is led by a break — the line held low — and a mark-after-break, held
//! high. A UART cannot hold the line low on request with useful precision
//! from user space, so [`BreakPlan`] makes the break out of an ordinary
//! character: a zero octet at a lower baud rate is a start bit and eight
//! zero bits, low, then a stop bit, high.

use core::time::Duration;

/// The line's data rate.
pub const BAUD: u32 = 250_000;

/// The shortest break a transmitter sends: 92 µs.
pub const MIN_BREAK: Duration = Duration::from_micros(92);

/// The shortest mark-after-break a transmitter sends: 12 µs.
pub const MIN_MAB: Duration = Duration::from_micros(12);

/// A break made of a zero octet at a lower baud rate, 8N1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakPlan {
    /// The baud rate the zero octet is sent at.
    pub baud: u32,
}

impl BreakPlan {
    /// 57 600 baud: a standard rate every driver accepts, giving a 156 µs
    /// break and a 17 µs mark, both inside E1.11's limits with margin.
    pub const STANDARD: Self = Self { baud: 57_600 };

    /// How long the line is low: the start bit and eight zeros.
    #[must_use]
    pub const fn low(&self) -> Duration {
        Duration::from_nanos(match 9_000_000_000u64.checked_div(self.baud as u64) {
            Some(n) => n,
            None => 0,
        })
    }

    /// How long it is high after: the stop bit. Switching the rate back
    /// takes longer still, which only lengthens the mark; E1.11 allows up
    /// to a second.
    #[must_use]
    pub const fn mark(&self) -> Duration {
        Duration::from_nanos(match 1_000_000_000u64.checked_div(self.baud as u64) {
            Some(n) => n,
            None => 0,
        })
    }

    /// Whether this plan meets E1.11's minimums.
    #[must_use]
    pub const fn meets_e1_11(&self) -> bool {
        self.baud > 0
            && self.low().as_nanos() >= MIN_BREAK.as_nanos()
            && self.mark().as_nanos() >= MIN_MAB.as_nanos()
    }
}

#[cfg(test)]
mod tests {
    use super::BreakPlan;

    #[test]
    fn the_standard_plan_meets_e1_11_and_a_fast_one_does_not() {
        let p = BreakPlan::STANDARD;
        assert_eq!((p.low().as_micros(), p.mark().as_micros()), (156, 17));
        assert!(p.meets_e1_11());
        assert!(!BreakPlan { baud: 115_200 }.meets_e1_11(), "a 78 µs break");
        assert!(!BreakPlan { baud: 0 }.meets_e1_11());
    }
}
