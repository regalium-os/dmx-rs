// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What each write says before it is sent: the change, what moves, the
//! check, and how the policy will treat it.

use crate::tui::app::Write;
use crate::tui::theme::{AMBER, GREEN, GROUND, RED, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
use ratatui::style::Style;
use ratatui::text::{Line, Span};

pub(super) fn row(label: &str, value: Vec<Span<'static>>) -> Line<'static> {
    let mut spans = vec![Span::styled(format!("{label:<12}"), muted())];
    spans.extend(value);
    Line::from(spans)
}

/// How the policy will treat a write of levels.
fn policy(model: &Model) -> Line<'static> {
    let p = &model.policy;
    let (text, colour) = if p.blackout {
        (
            "blackout is on · kept, and sent when it ends".to_owned(),
            RED,
        )
    } else if p.hold {
        (
            "hold is on · kept, and sent when hold ends".to_owned(),
            AMBER,
        )
    } else {
        (format!("live · master {}", p.master), GREEN)
    };
    row("policy", vec![Span::styled(text, strong(colour))])
}

/// What a write changes and moves, for the dialog that asks.
pub(super) fn confirm(model: &Model, write: &Write) -> (String, Vec<Line<'static>>) {
    let (title, mut lines) = match write {
        Write::Levels {
            universe,
            start,
            values,
        } => (
            "Write levels?",
            super::writes::levels(model, universe, *start, values),
        ),
        Write::BlackoutUniverse(u) => (
            "Take this universe dark?",
            vec![
                row("universe", vec![Span::raw(format!("universes/{u}"))]),
                row(
                    "what moves",
                    vec![Span::raw("every light on it goes dark, at the next frame")],
                ),
            ],
        ),
        Write::Blackout(_) => (
            "End blackout?",
            vec![
                row(
                    "what moves",
                    vec![Span::raw("every output returns to its levels at once")],
                ),
                row("master", vec![Span::raw(model.policy.master.to_string())]),
            ],
        ),
        Write::Hold(_) => (
            "Release hold?",
            vec![row(
                "what moves",
                vec![Span::raw("writes made during the hold go out at once")],
            )],
        ),
        Write::Master(m) => (
            "Change the master?",
            vec![
                row(
                    "master",
                    vec![Span::raw(format!(
                        "{} → {m} ({}%)",
                        model.policy.master,
                        u32::from(*m) * 100 / 255
                    ))],
                ),
                row(
                    "what moves",
                    vec![Span::raw("every level on every output scales")],
                ),
            ],
        ),
        Write::Address {
            universe,
            uid,
            label,
            from,
            to,
            footprint,
        } => (
            "RDM SET · change the start address?",
            super::writes::address(model, universe, uid, label, *from, *to, *footprint),
        ),
        Write::Identify { label, uid, .. } => (
            "RDM SET · identify?",
            vec![
                row("device", vec![Span::raw(format!("{label} · {uid}"))]),
                row(
                    "what moves",
                    vec![Span::raw("it flashes or moves to show itself")],
                ),
            ],
        ),
    };
    lines.push(policy(model));
    lines.push(Line::default());
    lines.push(Line::from(vec![
        Span::styled(" enter ", Style::new().bg(AMBER).fg(GROUND).bold()),
        Span::raw(" send   "),
        Span::styled(" esc ", Style::new().bg(TEXT).fg(GROUND).bold()),
        Span::raw(" cancel"),
    ]));
    (title.to_owned(), lines)
}
