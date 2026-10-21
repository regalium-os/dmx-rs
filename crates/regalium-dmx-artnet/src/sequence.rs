// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `ArtDmx`'s sequence number: 1 to 255 and round again, or 0 for a sender
//! that does not number its packets.
//!
//! UDP may reorder datagrams, and a late `ArtDmx` carries older levels than
//! one already applied. A receiver drops a packet that is a little behind
//! the last one from the same source and universe; a packet far behind is a
//! sender that restarted, and is taken. The window is the one ANSI E1.31
//! uses for sACN, which Art-Net does not specify.

/// How far behind a packet may be and still count as late rather than as a
/// restarted sender.
pub const WINDOW: i8 = 20;

/// The number after `sequence`: 255 wraps to 1, and 0 stays 0.
#[must_use]
pub const fn next(sequence: u8) -> u8 {
    match sequence {
        0 => 0,
        255 => 1,
        n => n + 1,
    }
}

/// Whether `new` is late after `last`: a repeat, or a little behind. Never
/// when either is 0, which turns ordering off.
#[must_use]
pub const fn is_late(last: u8, new: u8) -> bool {
    if last == 0 || new == 0 {
        return false;
    }
    #[allow(
        clippy::cast_possible_wrap,
        reason = "the wrapping difference is the point"
    )]
    let behind = new.wrapping_sub(last) as i8;
    behind <= 0 && behind > -WINDOW
}

#[cfg(test)]
mod tests {
    use super::{is_late, next};

    #[test]
    fn numbering_skips_zero_when_it_wraps() {
        assert_eq!(next(1), 2);
        assert_eq!(next(255), 1);
        assert_eq!(next(0), 0);
    }

    #[test]
    fn a_repeat_or_a_packet_just_behind_is_late() {
        assert!(is_late(10, 10));
        assert!(is_late(10, 9));
        assert!(is_late(3, 250), "behind, across the wrap");
        assert!(!is_late(10, 11));
        assert!(!is_late(250, 3), "ahead, across the wrap");
    }

    #[test]
    fn far_behind_is_a_restart_and_zero_turns_ordering_off() {
        assert!(!is_late(100, 50));
        assert!(!is_late(0, 5));
        assert!(!is_late(5, 0));
    }
}
