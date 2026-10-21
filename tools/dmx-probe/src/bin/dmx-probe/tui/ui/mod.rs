// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Drawing: a pure function of the app and the model, so every tab is
//! tested by drawing it into a buffer.

pub(crate) mod chrome;
mod confirm;
mod dialog;
mod grid;
mod inspector;
mod nodes;
mod patchmap;
mod rdm;
mod rig;
mod side;
mod traffic;
mod universe;
mod writes;

use super::app::{App, Tab};
use super::theme::{GROUND, text};
use dmx_probe::engine::model::Model;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};

/// Draws everything.
pub(crate) fn draw(frame: &mut Frame<'_>, app: &App, model: &Model) {
    let area = frame.area();
    frame.render_widget(Block::new().style(Style::new().bg(GROUND)), area);
    let [head, body, foot] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .spacing(1)
    .areas(area);
    chrome::header(frame, head, app, model);
    match app.tab {
        Tab::Rig => rig::draw(frame, body, app, model),
        Tab::Universes => universe::draw(frame, body, app, model),
        Tab::Nodes => nodes::draw(frame, body, app, model),
        Tab::Rdm => rdm::draw(frame, body, app, model),
        Tab::Traffic => traffic::draw(frame, body, app, model),
        Tab::Settings => {
            let lines =
                "Settings arrive with the live rig: interfaces, the Art-Net target, the sACN CID.";
            frame.render_widget(
                Paragraph::new(lines)
                    .style(text())
                    .block(chrome::block("Settings", true)),
                body,
            );
        }
    }
    chrome::footer(frame, foot, app, model);
    if let Some(open) = &app.modal {
        dialog::modal(frame, area, model, open);
    }
    if app.help {
        dialog::help(frame, area);
    }
}
