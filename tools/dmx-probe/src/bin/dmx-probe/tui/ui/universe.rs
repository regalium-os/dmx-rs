// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 2, one universe: all 512 levels as a heat grid, each fixture's RDM
//! footprint underlined beneath its cells, the selection, and frame timing.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{
    AMBER, CHIP, CYAN, DEEP, GREEN, GROUND, MUTED, RED, TEAL, TEXT, heat, muted, strong,
};
use dmx_probe::engine::model::{BUCKETS, Model, State, Universe};
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

/// A cell: a space and three digits.
const CELL: u16 = 4;

/// The narrowest the panel beside the grid may be.
const PANEL: u16 = 44;

pub(crate) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let Some(u) = model.universes.get(app.universe) else {
        frame.render_widget(
            Paragraph::new("No universes.").block(block("Levels", true)),
            area,
        );
        return;
    };
    let [title, body] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
        .spacing(1)
        .areas(area);
    let (word, colour) = match u.state {
        State::Live => ("live", GREEN),
        State::Idle => ("idle", MUTED),
        State::Failing => ("failing", RED),
    };
    let role = if u.output {
        format!("output · → {}", u.target)
    } else {
        format!("input · {}", u.merge.unwrap_or(""))
    };
    frame.render_widget(
        Line::from(vec![
            Span::styled(format!("universes/{}  ", u.name), strong(TEXT)),
            Span::styled(
                format!("{} {} · {role}  ", u.protocol.name(), u.number),
                muted(),
            ),
            Span::styled(format!("{word}  "), strong(colour)),
            Span::styled(
                format!(
                    "{:.1} fps · {} slots · generation {}",
                    u.fps,
                    u.levels.len(),
                    u.generation
                ),
                muted(),
            ),
        ]),
        title,
    );
    // Thirty-two columns where the panel still gets its width; sixteen,
    // twice as tall, where it would not.
    let columns: u16 = if body.width >= 32 * CELL + 10 + PANEL {
        32
    } else {
        16
    };
    let [grid, side] = Layout::horizontal([
        Constraint::Length(columns * CELL + 8),
        Constraint::Min(PANEL),
    ])
    .spacing(2)
    .areas(body);
    levels(frame, grid, app, model, u, columns);
    panel(frame, side, app, model, u);
}

/// Each address's footprint colour: alternating per fixture, red where two
/// claim it, none where nothing is patched.
fn bands(model: &Model, u: &Universe) -> [Option<Color>; 513] {
    let devices: Vec<_> = model
        .devices
        .iter()
        .filter(|d| d.universe == u.name)
        .collect();
    let owners = patch::owners(devices.iter().copied());
    let mut bands = [None; 513];
    for (n, d) in devices.iter().enumerate() {
        for a in patch::span(d) {
            if let Some(b) = bands.get_mut(usize::from(a)) {
                *b = Some(if owners.get(usize::from(a)).is_some_and(|&o| o > 1) {
                    RED
                } else if n % 2 == 0 {
                    TEAL
                } else {
                    DEEP
                });
            }
        }
    }
    bands
}

fn levels(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model, u: &Universe, columns: u16) {
    let bands = bands(model, u);
    let (first, last) = app.selection();
    let mut lines = vec![Line::from(
        std::iter::once(Span::styled("      ", muted()))
            .chain((1..=columns).map(|c| Span::styled(format!("{c:>4}"), muted())))
            .collect::<Vec<_>>(),
    )];
    for row in 0..(512 / columns) {
        let start = row * columns + 1;
        let mut spans = vec![Span::styled(format!("{start:>5} "), muted())];
        for a in start..start + columns {
            let level = u.levels.get(usize::from(a - 1)).copied();
            let text = match (level, app.percent) {
                (None, _) => "   ·".to_owned(),
                (Some(l), false) => format!("{l:>4}"),
                (Some(l), true) => format!("{:>4}", u32::from(l) * 100 / 255),
            };
            let mut style = level.map_or(Style::new().fg(MUTED).bg(GROUND), heat);
            if (first..=last).contains(&a) {
                style = style
                    .bg(Color::Rgb(8, 80, 96))
                    .fg(CYAN)
                    .add_modifier(Modifier::BOLD);
            }
            if a == app.cursor {
                style = Style::new()
                    .bg(CYAN)
                    .fg(GROUND)
                    .add_modifier(Modifier::BOLD);
            }
            if let Some(Some(band)) = bands.get(usize::from(a)) {
                style = style
                    .add_modifier(Modifier::UNDERLINED)
                    .underline_color(*band);
            }
            spans.push(Span::styled(text, style));
        }
        lines.push(Line::from(spans));
    }
    let legend = Line::from(vec![
        Span::styled("  0 ", heat(0)),
        Span::styled(" 128 ", heat(128)),
        Span::styled(" 255 ", heat(255)),
        Span::styled("  underline: RDM footprint, ", muted()),
        Span::styled("red", strong(RED)),
        Span::styled(" where two overlap", muted()),
    ]);
    lines.push(Line::default());
    lines.push(legend);
    frame.render_widget(
        Paragraph::new(lines).block(block(
            format!("Levels · 1–512{}", if app.percent { " · %" } else { "" }),
            true,
        )),
        area,
    );
}

fn panel(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model, u: &Universe) {
    let [selection, refresh, timing] = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Min(0),
    ])
    .spacing(1)
    .areas(area);
    let (first, last) = app.selection();
    let run: Vec<String> = (first..=last)
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
            lines.push(Line::from(vec![
                Span::styled("fixture  ", muted()),
                Span::raw(format!(
                    "{} · {}",
                    if d.label.is_empty() {
                        "unlabelled"
                    } else {
                        &d.label
                    },
                    d.uid
                )),
            ]));
            lines.push(Line::from(vec![
                Span::styled("its slots", muted()),
                Span::raw(format!(
                    " {}–{} · personality {}",
                    d.address,
                    d.address + d.footprint.saturating_sub(1),
                    d.personality
                )),
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
            .block(block(format!("Selection · {first}–{last}"), false)),
        selection,
    );

    let (word, c) = if u.state == State::Failing {
        ("failing", RED)
    } else {
        ("ok", GREEN)
    };
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
            Span::raw(
                u.sources
                    .iter()
                    .map(|s| {
                        format!(
                            "{} @{}{}",
                            s.id,
                            s.priority,
                            if s.merged { "" } else { " held" }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        ]),
        Line::from(vec![
            Span::styled("last send", muted()),
            Span::styled(format!(" {word}"), strong(c)),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(block("Refresh", false)),
        refresh,
    );

    let max = u.histogram.iter().copied().max().unwrap_or(0).max(1);
    let width = usize::from(timing.width.saturating_sub(20));
    let lines: Vec<Line<'_>> = BUCKETS
        .iter()
        .zip(u.histogram)
        .map(|(label, n)| {
            let bar = usize::try_from(u64::from(n) * width as u64 / u64::from(max)).unwrap_or(0);
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
        timing,
    );
}
