// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used)]

//! The refresh policy as a table on a fake clock: at each millisecond, given
//! the universe's generation, the refresher says send or idle until when.

use core::time::Duration;
use regalium_dmx_core::refresh::{Refresh, Refresher, Step};

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn idle(n: u64) -> Step {
    Step::Idle { until: ms(n) }
}

/// Runs `(time ms, generation, expected)` rows through one refresher.
fn table(policy: Refresh, rows: &[(u64, u64, Step)]) {
    let mut r = Refresher::new(policy);
    for (i, &(at, generation, want)) in rows.iter().enumerate() {
        assert_eq!(r.next(ms(at), generation), want, "row {i}: t={at} ms");
    }
}

#[test]
fn the_first_step_is_always_a_send() {
    table(Refresh::default(), &[(0, 0, Step::Send)]);
}

#[test]
fn a_continuous_refresh_sends_every_interval_whether_or_not_anything_changed() {
    table(
        Refresh::continuous(ms(25)),
        &[
            (0, 0, Step::Send),
            (10, 0, idle(25)),
            (25, 0, Step::Send),
            (49, 0, idle(50)),
            (50, 0, Step::Send),
        ],
    );
}

#[test]
fn a_change_waits_for_the_interval_then_sends_and_repeats() {
    table(
        Refresh::on_change(ms(25), ms(1000), 2),
        &[
            (0, 0, Step::Send),
            // Two repeats of the first send, an interval apart.
            (25, 0, Step::Send),
            (50, 0, Step::Send),
            // Repeats spent: idle until the keep-alive.
            (60, 0, idle(1050)),
            // A change at 60 ms: due 25 ms after the last send, not before.
            (60, 1, idle(75)),
            (75, 1, Step::Send),
            (100, 1, Step::Send),
            (125, 1, Step::Send),
            (130, 1, idle(1125)),
        ],
    );
}

#[test]
fn unchanged_levels_are_resent_at_the_keep_alive() {
    table(
        Refresh::on_change(ms(25), ms(1000), 0),
        &[
            (0, 7, Step::Send),
            (999, 7, idle(1000)),
            (1000, 7, Step::Send),
            (2000, 7, Step::Send),
        ],
    );
}

#[test]
fn a_change_during_repeats_restarts_them() {
    table(
        Refresh::on_change(ms(25), ms(1000), 2),
        &[
            (0, 0, Step::Send),
            (25, 0, Step::Send),
            // Changed with one repeat left: a fresh send and two fresh repeats.
            (50, 1, Step::Send),
            (75, 1, Step::Send),
            (100, 1, Step::Send),
            (125, 1, idle(1100)),
        ],
    );
}

#[test]
fn an_idle_step_changes_nothing_so_asking_again_is_safe() {
    let mut r = Refresher::new(Refresh::continuous(ms(25)));
    assert_eq!(r.next(ms(0), 0), Step::Send);
    for _ in 0..10 {
        assert_eq!(r.next(ms(5), 0), idle(25));
    }
    assert_eq!(r.last_sent(), Some(ms(0)));
}

#[test]
fn a_clock_that_steps_backwards_waits_rather_than_floods() {
    let mut r = Refresher::new(Refresh::continuous(ms(25)));
    assert_eq!(r.next(ms(100), 0), Step::Send);
    assert_eq!(r.next(ms(40), 1), idle(125));
}

#[test]
fn a_keep_alive_shorter_than_the_interval_is_raised_to_it() {
    let p = Refresh::on_change(ms(25), ms(5), 0);
    assert_eq!(p.keep_alive, ms(25));
}

#[test]
fn the_default_is_continuous_at_about_44_hz() {
    let p = Refresh::default();
    assert_eq!(p.interval, p.keep_alive);
    let hz = 1.0 / p.interval.as_secs_f64();
    assert!((43.9..44.1).contains(&hz), "{hz} Hz");
}
