// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The level grid: 512 levels as heat, the selection and cursor, and each
//! fixture's RDM footprint underlined beneath its cells.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{CYAN, DEEP, GROUND, MUTED, RED, TEAL, heat, muted, strong};
use dmx_probe::engine::model::{Model, Universe};
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

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

pub(super) fn draw(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &App,
    model: &Model,
    u: &Universe,
    columns: u16,
) {
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
