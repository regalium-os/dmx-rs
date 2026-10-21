// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 2, one universe: all 512 levels as a heat grid, each fixture's RDM
//! footprint underlined beneath its cells, the selection, and frame timing.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{GREEN, MUTED, RED, TEXT, muted, strong};
use dmx_probe::engine::model::{Model, State};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

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
    super::grid::draw(frame, grid, app, model, u, columns);
    super::inspector::draw(frame, side, app, model, u);
}
