// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The prompt, the confirmation every write passes, and help.

use super::chrome::block;
use crate::tui::app::{Ask, Modal};
use crate::tui::theme::{AMBER, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
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

pub(crate) fn modal(frame: &mut Frame<'_>, area: Rect, model: &Model, open: &Modal) {
    let (title, lines) = match open {
        Modal::Prompt { ask, entry } => prompt(ask, entry),
        Modal::Confirm(write) => super::confirm::confirm(model, write),
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

fn prompt(ask: &Ask, entry: &str) -> (String, Vec<Line<'static>>) {
    let what = match ask {
        Ask::Level => "Level for the selection, 0–255",
        Ask::Master => "Grand master, 0–255",
        Ask::Address { .. } => "New start address, 1–512",
    };
    let lines = vec![
        Line::from(vec![
            Span::styled(format!(" {entry}"), strong(TEXT)),
            Span::styled("▏", strong(TEXT)),
        ]),
        Line::default(),
        Line::from(Span::styled("enter to review · esc to cancel", muted())),
    ];
    (what.to_owned(), lines)
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
