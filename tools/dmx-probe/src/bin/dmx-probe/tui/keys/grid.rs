// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The level grid's keys: move, select, type a level, full, zero, blackout.

use super::super::app::{App, Ask, Modal, Tab, Write};
use dmx_probe::engine::Command;
use dmx_probe::engine::model::Model;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// The level grid: move, select, type a level, full, zero, blackout.
pub(super) fn levels(
    app: &mut App,
    model: &Model,
    key: KeyEvent,
    universe: &str,
) -> Option<Command> {
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);
    let (first, last) = app.selection();
    let run = |v: u8| vec![v; usize::from(last - first + 1)];
    match key.code {
        KeyCode::Left => go(app, shift, -1),
        KeyCode::Right => go(app, shift, 1),
        KeyCode::Up => go(app, shift, -32),
        KeyCode::Down => go(app, shift, 32),
        KeyCode::Char(c @ '0'..='9') => {
            app.modal = Some(Modal::Prompt {
                ask: Ask::Level,
                entry: c.to_string(),
            });
        }
        KeyCode::Char('f') => {
            app.modal = Some(Modal::Confirm(Write::Levels {
                universe: universe.to_owned(),
                start: first,
                values: run(255),
            }));
        }
        KeyCode::Char('z') => {
            app.modal = Some(Modal::Confirm(Write::Levels {
                universe: universe.to_owned(),
                start: first,
                values: run(0),
            }));
        }
        KeyCode::Char('x') => {
            app.modal = Some(Modal::Confirm(Write::BlackoutUniverse(universe.to_owned())));
        }
        KeyCode::Char('p') => app.percent = !app.percent,
        KeyCode::Char('[') => app.universe = app.universe.saturating_sub(1),
        KeyCode::Char(']') => {
            app.universe = (app.universe + 1).min(model.universes.len().saturating_sub(1));
        }
        KeyCode::Esc => app.tab = Tab::Rig,
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Tab => app.tab = Tab::Nodes,
        _ => {}
    }
    None
}

/// Moves the grid's cursor by `by`; with shift, the selection grows from
/// where it was, and without, it collapses to the cursor.
fn go(app: &mut App, shift: bool, by: i32) {
    if shift && app.anchor.is_none() {
        app.anchor = Some(app.cursor);
    } else if !shift {
        app.anchor = None;
    }
    app.cursor = u16::try_from((i32::from(app.cursor) + by).clamp(1, 512)).unwrap_or(1);
}
