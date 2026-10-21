// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The detail of the two writes with most to say: a run of levels, and an
//! RDM device's new start address with its check.

use super::confirm::row;
use crate::tui::keys::devices;
use crate::tui::theme::{GREEN, RED, muted, strong};
use dmx_probe::engine::model::Model;
use dmx_probe::engine::patch;
use ratatui::text::{Line, Span};

pub(super) fn levels(
    model: &Model,
    universe: &str,
    start: u16,
    values: &[u8],
) -> Vec<Line<'static>> {
    let end = start + u16::try_from(values.len()).unwrap_or(1) - 1;
    let fixtures: Vec<String> = devices(model, universe)
        .iter()
        .filter(|d| patch::span(d).any(|a| (start..=end).contains(&a)))
        .map(|d| {
            if d.label.is_empty() {
                d.uid.clone()
            } else {
                d.label.clone()
            }
        })
        .collect();
    let moves = if fixtures.is_empty() {
        "the lights at those addresses, at the next frame".to_owned()
    } else {
        format!("{}, at the next frame", fixtures.join(", "))
    };
    let shown: Vec<String> = values.iter().take(12).map(u8::to_string).collect();
    vec![
        row("universe", vec![Span::raw(format!("universes/{universe}"))]),
        row(
            "addresses",
            vec![Span::raw(format!(
                "{start}–{end} → {}{}",
                shown.join(" "),
                if values.len() > 12 { " …" } else { "" }
            ))],
        ),
        row("what moves", vec![Span::raw(moves)]),
    ]
}

pub(super) fn address(
    model: &Model,
    universe: &str,
    uid: &str,
    label: &str,
    from: u16,
    to: u16,
    footprint: u16,
) -> Vec<Line<'static>> {
    let on = devices(model, universe);
    let end = to + footprint.saturating_sub(1);
    let check = if patch::is_free(on.iter().copied(), uid, to, footprint) {
        Span::styled(format!("{to}–{end} is free"), strong(GREEN))
    } else {
        let hit: Vec<String> = on
            .iter()
            .filter(|d| d.uid != uid && patch::span(d).any(|a| (to..=end).contains(&a)))
            .map(|d| d.label.clone())
            .collect();
        let whom = if hit.is_empty() {
            "the end of the universe".to_owned()
        } else {
            hit.join(", ")
        };
        Span::styled(format!("{to}–{end} overlaps {whom}"), strong(RED))
    };
    vec![
        row(
            "device",
            vec![Span::raw(format!("{label} · {uid} · universes/{universe}"))],
        ),
        row("parameter", vec![Span::raw("DMX_START_ADDRESS · 0x00f0")]),
        row(
            "change",
            vec![
                Span::styled(from.to_string(), strong(RED)),
                Span::raw("  →  "),
                Span::styled(to.to_string(), strong(GREEN)),
            ],
        ),
        row(
            "what moves",
            vec![Span::raw(format!(
                "the fixture re-patches at once: it answers the levels at {to}–{end}"
            ))],
        ),
        row("check", vec![check]),
        row(
            "then",
            vec![Span::styled(
                "reads DEVICE_INFO back: what is shown is what the device kept",
                muted(),
            )],
        ),
    ]
}
