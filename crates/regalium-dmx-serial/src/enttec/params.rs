// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What a widget says about itself: its parameters and serial number.

use core::time::Duration;

/// The unit the widget times break and mark-after-break in: 10.67 µs.
pub const TICK_NS: u64 = 10_670;

/// A Get Widget Parameters reply.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Params {
    /// The firmware version.
    pub firmware: u16,
    /// The break the widget transmits, in 10.67 µs ticks: 9 to 127.
    pub break_ticks: u8,
    /// The mark-after-break, in ticks: 1 to 127.
    pub mab_ticks: u8,
    /// Packets per second it transmits, 1 to 40, or 0 for as fast as it can.
    pub rate: u8,
}

impl Params {
    /// Reads a reply's data. `None` when it is shorter than the five octets
    /// every reply carries; user configuration after them is ignored.
    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let &[lo, hi, break_ticks, mab_ticks, rate, ..] = data else {
            return None;
        };
        Some(Self {
            firmware: u16::from_le_bytes([lo, hi]),
            break_ticks,
            mab_ticks,
            rate,
        })
    }

    /// The reply's data, with no user configuration.
    #[must_use]
    pub fn encode(&self) -> [u8; 5] {
        let [lo, hi] = self.firmware.to_le_bytes();
        [lo, hi, self.break_ticks, self.mab_ticks, self.rate]
    }

    /// The break, as a duration.
    #[must_use]
    pub const fn break_time(&self) -> Duration {
        Duration::from_nanos(self.break_ticks as u64 * TICK_NS)
    }

    /// The mark-after-break, as a duration.
    #[must_use]
    pub const fn mab_time(&self) -> Duration {
        Duration::from_nanos(self.mab_ticks as u64 * TICK_NS)
    }
}

/// A Get Widget Serial Number reply's four octets of BCD, least significant
/// first, as the number printed on the widget. `None` when the data is not
/// four octets or a digit is not a decimal digit.
#[must_use]
pub fn serial(data: &[u8]) -> Option<u32> {
    let &[a, b, c, d] = data else {
        return None;
    };
    [d, c, b, a].iter().try_fold(0u32, |n, &octet| {
        let (hi, lo) = (octet >> 4, octet & 0x0F);
        (hi < 10 && lo < 10).then(|| n * 100 + u32::from(hi) * 10 + u32::from(lo))
    })
}

/// `serial` as four octets of BCD, least significant first; numbers above
/// 99 999 999 keep their low eight digits.
#[must_use]
pub fn serial_bcd(serial: u32) -> [u8; 4] {
    let mut n = serial % 100_000_000;
    let mut out = [0u8; 4];
    for octet in &mut out {
        let pair = n % 100;
        n /= 100;
        #[allow(clippy::cast_possible_truncation, reason = "pair is below 100")]
        let pair = pair as u8;
        *octet = ((pair / 10) << 4) | (pair % 10);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{Params, serial, serial_bcd};

    #[test]
    fn a_serial_is_bcd_least_significant_first() {
        assert_eq!(serial(&[0x78, 0x56, 0x34, 0x12]), Some(12_345_678));
        assert_eq!(serial_bcd(12_345_678), [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(serial(&[0x7A, 0, 0, 0]), None, "A is not a digit");
        assert_eq!(serial(&[1, 2, 3]), None);
    }

    #[test]
    fn parameters_read_back_and_time_in_ticks() {
        let p = Params {
            firmware: 0x0144,
            break_ticks: 9,
            mab_ticks: 1,
            rate: 40,
        };
        assert_eq!(Params::decode(&p.encode()), Some(p));
        assert_eq!(p.break_time().as_nanos(), 96_030);
        assert_eq!(Params::decode(&[1, 2, 3, 4]), None);
    }
}
