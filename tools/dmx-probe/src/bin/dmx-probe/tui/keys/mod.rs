// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Keys, to app state and engine commands.
//!
//! What stops light acts at once: blackout, hold. What makes light move —
//! a level, the master, releasing blackout or hold, an RDM SET — asks
//! first, in a dialog that says what will move.

mod dialog;
mod grid;

use super::app::{App, Ask, Modal, Tab, Write};
use dmx_probe::engine::Command;
use dmx_probe::engine::model::{Device, Model};
use dmx_probe::engine::patch;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// The devices on `universe`, as the RDM tab lists them.
pub(crate) fn devices<'a>(model: &'a Model, universe: &str) -> Vec<&'a Device> {
    model
        .devices
        .iter()
        .filter(|d| d.universe == universe)
        .collect()
}

/// Handles one key. Returns the command it asks of the engine, if any.
pub(crate) fn key(app: &mut App, model: &Model, key: KeyEvent) -> Option<Command> {
    if let Some(open) = app.modal.take() {
        return dialog::dialog(app, model, open, key);
    }
    let selected = model.universes.get(app.universe);
    let name = selected.map(|u| u.name.clone()).unwrap_or_default();
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Char('?') => app.help = !app.help,
        KeyCode::Char(c @ '1'..='6') if app.tab != Tab::Universes => {
            if let Some((tab, _)) = Tab::ALL.get(usize::from(c as u8 - b'1')) {
                app.tab = *tab;
            }
        }
        KeyCode::Char('b') if !model.policy.blackout => return Some(Command::Blackout(true)),
        KeyCode::Char('b') => app.modal = Some(Modal::Confirm(Write::Blackout(false))),
        KeyCode::Char('h') if !model.policy.hold => return Some(Command::Hold(true)),
        KeyCode::Char('h') => app.modal = Some(Modal::Confirm(Write::Hold(false))),
        KeyCode::Char('m') => {
            app.modal = Some(Modal::Prompt {
                ask: Ask::Master,
                entry: model.policy.master.to_string(),
            });
        }
        KeyCode::Tab => {
            let next =
                (Tab::ALL.iter().position(|t| t.0 == app.tab).unwrap_or(0) + 1) % Tab::ALL.len();
            app.tab = Tab::ALL.get(next).map_or(Tab::Rig, |t| t.0);
        }
        _ => return tab(app, model, key, &name),
    }
    None
}

fn tab(app: &mut App, model: &Model, key: KeyEvent, universe: &str) -> Option<Command> {
    let up = |i: &mut usize| *i = i.saturating_sub(1);
    let down = |i: &mut usize, n: usize| *i = (*i + 1).min(n.saturating_sub(1));
    match (app.tab, key.code) {
        (Tab::Rig, KeyCode::Up) => up(&mut app.universe),
        (Tab::Rig, KeyCode::Down) => down(&mut app.universe, model.universes.len()),
        (Tab::Rig, KeyCode::Enter) => app.tab = Tab::Universes,
        (Tab::Universes, _) => return grid::levels(app, model, key, universe),
        (Tab::Rdm, KeyCode::Up) => up(&mut app.device),
        (Tab::Rdm, KeyCode::Down) => down(&mut app.device, devices(model, universe).len()),
        (Tab::Rdm, KeyCode::Char('D' | 'r')) => {
            return Some(Command::Discover(universe.to_owned()));
        }
        (Tab::Rdm, KeyCode::Char('a')) => {
            let list = devices(model, universe);
            if let Some(d) = list.get(app.device) {
                let free = patch::first_free(list.iter().copied(), &d.uid, d.footprint)
                    .unwrap_or(d.address);
                app.modal = Some(Modal::Prompt {
                    ask: Ask::Address { uid: d.uid.clone() },
                    entry: free.to_string(),
                });
            }
        }
        (Tab::Rdm, KeyCode::Char('i')) => {
            if let Some(d) = devices(model, universe).get(app.device) {
                let w = Write::Identify {
                    universe: universe.to_owned(),
                    uid: d.uid.clone(),
                    label: d.label.clone(),
                    on: true,
                };
                app.modal = Some(Modal::Confirm(w));
            }
        }
        (Tab::Nodes, KeyCode::Up) => up(&mut app.node),
        (Tab::Nodes, KeyCode::Down) => down(&mut app.node, model.nodes.len()),
        (Tab::Traffic, KeyCode::Up) => {
            app.packet = (app.packet + 1).min(model.packets.len().saturating_sub(1));
            app.field = 0;
        }
        (Tab::Traffic, KeyCode::Down) => {
            app.packet = app.packet.saturating_sub(1);
            app.field = 0;
        }
        (Tab::Traffic, KeyCode::Right) => app.field += 1,
        (Tab::Traffic, KeyCode::Left) => app.field = app.field.saturating_sub(1),
        (Tab::Traffic, KeyCode::Char('l')) => app.show_levels = !app.show_levels,
        (_, KeyCode::Esc) => app.tab = Tab::Rig,
        _ => {}
    }
    None
}
