// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! E1.31's sequence numbers: 0 to 255 and round again, per universe.
//!
//! A receiver drops a packet that is a little behind the last one from the
//! same source and universe, and takes one far behind as a source that
//! restarted. Unlike Art-Net, 0 is a number like any other.

/// How far behind a packet may be and still count as late: E1.31's 20.
pub const WINDOW: i8 = 20;

/// Whether `new` is late after `last`: a repeat, or fewer than [`WINDOW`]
/// behind.
#[must_use]
pub const fn is_late(last: u8, new: u8) -> bool {
    #[allow(
        clippy::cast_possible_wrap,
        reason = "the wrapping difference is the point"
    )]
    let behind = new.wrapping_sub(last) as i8;
    behind <= 0 && behind > -WINDOW
}

#[cfg(test)]
mod tests {
    use super::is_late;

    #[test]
    fn a_repeat_or_a_packet_just_behind_is_late_and_zero_is_a_number() {
        assert!(is_late(10, 10));
        assert!(is_late(10, 9));
        assert!(is_late(0, 255), "one behind, across the wrap");
        assert!(!is_late(255, 0), "one ahead, across the wrap");
        assert!(!is_late(100, 80), "twenty behind is a restart");
        assert!(is_late(100, 81));
    }
}
