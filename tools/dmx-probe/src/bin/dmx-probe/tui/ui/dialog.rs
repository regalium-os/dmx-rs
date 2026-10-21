// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The prompt, the confirmation every write passes, and help.

use super::chrome::block;
use crate::tui::app::{Ask, Modal, Write};
use crate::tui::keys::devices;
use crate::tui::theme::{AMBER, GREEN, GROUND, RED, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

fn centred(area: Rect, width: u16, height: u16) -> Rect {
    let (w, h) = (width.min(area.width), height.min(area.height));
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 3,
        width: w,
        height: h,
    }
}

fn row(label: &str, value: Vec<Span<'static>>) -> Line<'static> {
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

pub(crate) fn modal(frame: &mut Frame<'_>, area: Rect, model: &Model, modal: &Modal) {
    let (title, lines) = match modal {
        Modal::Prompt { ask, entry } => {
            let what = match ask {
                Ask::Level => "Level for the selection, 0–255",
                Ask::Master => "Grand master, 0–255",
                Ask::Address { .. } => "New start address, 1–512",
            };
            (
                what.to_owned(),
                vec![
                    Line::from(vec![
                        Span::styled(format!(" {entry}"), strong(TEXT)),
                        Span::styled("▏", strong(TEXT)),
                    ]),
                    Line::default(),
                    Line::from(Span::styled("enter to review · esc to cancel", muted())),
                ],
            )
        }
        Modal::Confirm(write) => confirm(model, write),
    };
    let at = centred(area, 78, u16::try_from(lines.len()).unwrap_or(10) + 4);
    frame.render_widget(Clear, at);
    let block = block(title, true).border_style(Style::new().fg(AMBER));
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(block),
        at,
    );
}

fn confirm(model: &Model, write: &Write) -> (String, Vec<Line<'static>>) {
    let enter = Line::from(vec![
        Span::styled(" enter ", Style::new().bg(AMBER).fg(GROUND).bold()),
        Span::raw(" send   "),
        Span::styled(" esc ", Style::new().bg(TEXT).fg(GROUND).bold()),
        Span::raw(" cancel"),
    ]);
    let mut lines = match write {
        Write::Levels {
            universe,
            start,
            values,
        } => {
            let last = start + u16::try_from(values.len()).unwrap_or(1) - 1;
            let fixtures: Vec<String> = devices(model, universe)
                .iter()
                .filter(|d| patch::span(d).any(|a| (*start..=last).contains(&a)))
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
            return finish(
                "Write levels?",
                vec![
                    row("universe", vec![Span::raw(format!("universes/{universe}"))]),
                    row(
                        "addresses",
                        vec![Span::raw(format!(
                            "{start}–{last} → {}{}",
                            shown.join(" "),
                            if values.len() > 12 { " …" } else { "" }
                        ))],
                    ),
                    row("what moves", vec![Span::raw(moves)]),
                    policy(model),
                ],
                enter,
            );
        }
        Write::BlackoutUniverse(u) => vec![
            row("universe", vec![Span::raw(format!("universes/{u}"))]),
            row(
                "what moves",
                vec![Span::raw("every light on it goes dark, at the next frame")],
            ),
        ],
        Write::Blackout(_) => vec![
            row(
                "what moves",
                vec![Span::raw("every output returns to its levels at once")],
            ),
            row("master", vec![Span::raw(model.policy.master.to_string())]),
        ],
        Write::Hold(_) => vec![row(
            "what moves",
            vec![Span::raw("writes made during the hold go out at once")],
        )],
        Write::Master(m) => vec![
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
        Write::Address {
            universe,
            uid,
            label,
            from,
            to,
            footprint,
        } => {
            let list = devices(model, universe);
            let last = to + footprint.saturating_sub(1);
            let check = if patch::is_free(list.iter().copied(), uid, *to, *footprint) {
                Span::styled(format!("{to}–{last} is free"), strong(GREEN))
            } else {
                let hit: Vec<String> = list
                    .iter()
                    .filter(|d| &d.uid != uid && patch::span(d).any(|a| (*to..=last).contains(&a)))
                    .map(|d| d.label.clone())
                    .collect();
                Span::styled(
                    format!(
                        "{to}–{last} overlaps {}",
                        if hit.is_empty() {
                            "the end of the universe".to_owned()
                        } else {
                            hit.join(", ")
                        }
                    ),
                    strong(RED),
                )
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
                        "the fixture re-patches at once: it answers the levels at {to}–{last}"
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
        Write::Identify { label, uid, .. } => vec![
            row("device", vec![Span::raw(format!("{label} · {uid}"))]),
            row(
                "what moves",
                vec![Span::raw("it flashes or moves to show itself")],
            ),
        ],
    };
    lines.push(policy(model));
    let title = match write {
        Write::BlackoutUniverse(_) => "Take this universe dark?",
        Write::Blackout(_) => "End blackout?",
        Write::Hold(_) => "Release hold?",
        Write::Master(_) => "Change the master?",
        Write::Address { .. } => "RDM SET · change the start address?",
        Write::Identify { .. } => "RDM SET · identify?",
        Write::Levels { .. } => "Write levels?",
    };
    finish(title, lines, enter)
}

fn finish(
    title: &str,
    mut lines: Vec<Line<'static>>,
    enter: Line<'static>,
) -> (String, Vec<Line<'static>>) {
    lines.push(Line::default());
    lines.push(enter);
    (title.to_owned(), lines)
}

pub(crate) fn help(frame: &mut Frame<'_>, area: Rect) {
    let lines: Vec<Line<'_>> = [
        ("1–6, tab", "tabs"),
        ("b", "blackout every output · at once; ending it asks"),
        ("h", "hold every output · at once; releasing it asks"),
        ("m", "grand master · asks"),
        (
            "Universes",
            "arrows move, shift selects, 0–9 types a level, f full, z zero, x blackout one",
        ),
        ("RDM", "D discovers, a sets the start address, i identifies"),
        ("Traffic", "↑↓ packets, ←→ fields"),
        ("q", "quit"),
    ]
    .into_iter()
    .map(|(k, v)| {
        Line::from(vec![
            Span::styled(format!("{k:<12}"), strong(TEXT)),
            Span::styled(v, muted()),
        ])
    })
    .collect();
    let at = centred(area, 90, 12);
    frame.render_widget(Clear, at);
    frame.render_widget(
        Paragraph::new(lines).block(block("Help · ? closes", true)),
        at,
    );
}
