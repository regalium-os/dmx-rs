// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The patch map: every address coloured by who owns it, red where two
//! claim it, with the overlaps and the first free run spelled out.

use super::chrome::block;
use crate::tui::theme::{CHIP, DEEP, RED, TEAL, TEXT, muted, strong};
use dmx_probe::engine::model::Device;
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub(super) fn draw(
    frame: &mut Frame<'_>,
    area: Rect,
    list: &[&Device],
    selected: Option<&Device>,
    owners: &[u8; 513],
) {
    let mut lines: Vec<Line<'_>> = (0..16u16)
        .map(|row| {
            let cells = (1..=32u16).map(|c| {
                let a = row * 32 + c;
                let colour = match owners.get(usize::from(a)).copied().unwrap_or(0) {
                    0 => CHIP,
                    1 if ((a - 1) / 16).is_multiple_of(2) => TEAL,
                    1 => DEEP,
                    _ => RED,
                };
                Span::styled("█", Style::new().fg(colour))
            });
            Line::from(
                std::iter::once(Span::styled(format!("{:>4} ", row * 32 + 1), muted()))
                    .chain(cells)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    lines.push(Line::default());
    for (first, end) in patch::overlaps(owners) {
        lines.push(Line::from(vec![
            Span::styled(format!("{first}–{end}"), strong(RED)),
            Span::styled(" claimed twice", muted()),
        ]));
    }
    if let Some(d) = selected {
        let free = patch::first_free(list.iter().copied(), &d.uid, d.footprint)
            .map_or_else(|| "none".to_owned(), |a| a.to_string());
        lines.push(Line::from(vec![
            Span::styled(format!("first free run of {}: ", d.footprint), muted()),
            Span::styled(free, strong(TEXT)),
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(block("Patch map · 1–512", false)),
        area,
    );
}
