// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `dmx-probe tui`: watch and drive the rig.
//!
//! This thread owns the terminal and never touches the runtime: it draws
//! the engine's latest model and sends it commands, so a slow terminal
//! never delays a frame on the wire.

mod app;
mod keys;
mod theme;
mod ui;

#[cfg(test)]
mod tests;

use app::App;
use dmx_probe::engine::Engine;
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

/// How long the loop waits for a key before drawing again.
const FRAME: Duration = Duration::from_millis(50);

/// Runs until the user quits.
///
/// # Errors
///
/// When the terminal cannot be drawn to or read from.
pub(crate) fn run(mut engine: Engine) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = (|| {
        while !app.quit {
            if let Some(model) = engine.model().cloned() {
                terminal.draw(|frame| ui::draw(frame, &app, &model))?;
                if event::poll(FRAME)?
                    && let Event::Key(key) = event::read()?
                    && key.kind == KeyEventKind::Press
                    && let Some(command) = keys::key(&mut app, &model, key)
                {
                    engine.send(command);
                }
            } else {
                std::thread::sleep(FRAME);
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result
}
