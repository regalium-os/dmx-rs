// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Beside the level grid: the selection and the fixture under the cursor,
//! the refresh, and the frame-interval histogram.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{AMBER, CHIP, GREEN, RED, TEAL, muted, strong};
use dmx_probe::engine::model::{BUCKETS, Model, State, Universe};
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

pub(super) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model, u: &Universe) {
    let [top, middle, bottom] = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Min(0),
    ])
    .spacing(1)
    .areas(area);
    selection(frame, top, app, model, u);
    refresh(frame, middle, u);
    timing(frame, bottom, u);
}

fn selection(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model, u: &Universe) {
    let (first, end) = app.selection();
    let run: Vec<String> = (first..=end)
        .take(16)
        .map(|a| {
            u.levels
                .get(usize::from(a - 1))
                .map_or("·".to_owned(), u8::to_string)
        })
        .collect();
    let owner = model
        .devices
        .iter()
        .find(|d| d.universe == u.name && patch::span(d).any(|a| a == app.cursor));
    let mut lines = vec![Line::from(vec![
        Span::styled("levels   ", muted()),
        Span::raw(run.join(" ")),
    ])];
    match owner {
        Some(d) => {
            let name = if d.label.is_empty() {
                "unlabelled"
            } else {
                &d.label
            };
            lines.push(Line::from(vec![
                Span::styled("fixture  ", muted()),
                Span::raw(format!("{name} · {}", d.uid)),
            ]));
            let span = format!(
                " {}–{} · personality {}",
                d.address,
                d.address + d.footprint.saturating_sub(1),
                d.personality
            );
            lines.push(Line::from(vec![
                Span::styled("its slots", muted()),
                Span::raw(span),
            ]));
            lines.push(Line::from(vec![
                Span::styled("here     ", muted()),
                Span::raw(format!(
                    "slot {} of {}",
                    app.cursor - d.address + 1,
                    d.footprint
                )),
            ]));
        }
        None => lines.push(Line::from(Span::styled(
            "no RDM device here · discover on tab 4",
            muted(),
        ))),
    }
    let write = if u.output {
        "type a level, enter writes · asks first"
    } else {
        "an input: read only"
    };
    lines.push(Line::default());
    lines.push(Line::from(Span::styled(write, muted())));
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(block(format!("Selection · {first}–{end}"), false)),
        area,
    );
}

fn refresh(frame: &mut Frame<'_>, area: Rect, u: &Universe) {
    let (word, c) = if u.state == State::Failing {
        ("failing", RED)
    } else {
        ("ok", GREEN)
    };
    let sources: Vec<String> = u
        .sources
        .iter()
        .map(|s| {
            format!(
                "{} @{}{}",
                s.id,
                s.priority,
                if s.merged { "" } else { " held" }
            )
        })
        .collect();
    let lines = vec![
        Line::from(vec![
            Span::styled("target   ", muted()),
            Span::raw(if u.target.is_empty() {
                "—".to_owned()
            } else {
                u.target.clone()
            }),
        ]),
        Line::from(vec![
            Span::styled("sources  ", muted()),
            Span::raw(sources.join(", ")),
        ]),
        Line::from(vec![
            Span::styled("last send", muted()),
            Span::styled(format!(" {word}"), strong(c)),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).block(block("Refresh", false)), area);
}

fn timing(frame: &mut Frame<'_>, area: Rect, u: &Universe) {
    let max = u.histogram.iter().copied().max().unwrap_or(0).max(1);
    let width = usize::from(area.width.saturating_sub(20));
    let lines: Vec<Line<'_>> = BUCKETS
        .iter()
        .zip(u.histogram)
        .map(|(label, n)| {
            let bar = usize::try_from(u64::from(n) * width as u64 / u64::from(max))
                .unwrap_or(0)
                .min(width);
            let colour = if *label == "22–23" { TEAL } else { AMBER };
            Line::from(vec![
                Span::styled(format!("{label:>6} "), muted()),
                Span::styled("█".repeat(bar), Style::new().fg(colour)),
                Span::styled("░".repeat(width - bar), Style::new().fg(CHIP)),
                Span::raw(format!(" {n:>5}")),
            ])
        })
        .collect();
    frame.render_widget(
        Paragraph::new(lines).block(block("Frame interval, ms", false)),
        area,
    );
}
