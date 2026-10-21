// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Keys, to app state and engine commands.
//!
//! What stops light acts at once: blackout, hold. What makes light move —
//! a level, the master, releasing blackout or hold, an RDM SET — asks
//! first, in a dialog that says what will move.

use super::app::{App, Ask, Modal, Tab, Write};
use dmx_probe::engine::Command;
use dmx_probe::engine::model::{Device, Model};
use dmx_probe::engine::patch;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
    if let Some(modal) = app.modal.take() {
        return dialog(app, model, modal, key);
    }
    let selected = model.universes.get(app.universe);
    let name = selected.map(|u| u.name.clone()).unwrap_or_default();
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Char('?') => app.help = !app.help,
        KeyCode::Char(c @ '1'..='6') if app.tab != Tab::Universes => {
            app.tab = Tab::ALL[usize::from(c as u8 - b'1')].0;
        }
        KeyCode::Char('b') if !model.policy.blackout => return Some(Command::Blackout(true)),
        KeyCode::Char('b') => app.modal = Some(Modal::Confirm(Write::Blackout(false))),
        KeyCode::Char('h') if !model.policy.hold => return Some(Command::Hold(true)),
        KeyCode::Char('h') => app.modal = Some(Modal::Confirm(Write::Hold(false))),
        KeyCode::Char('m') => {
            app.modal = Some(Modal::Prompt {
                ask: Ask::Master,
                entry: model.policy.master.to_string(),
            })
        }
        KeyCode::Tab => {
            app.tab =
                Tab::ALL[(Tab::ALL.iter().position(|t| t.0 == app.tab).unwrap_or(0) + 1) % 6].0
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
        (Tab::Universes, _) => return levels(app, model, key, universe),
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

/// The level grid: move, select, type a level, full, zero, blackout.
fn levels(app: &mut App, model: &Model, key: KeyEvent, universe: &str) -> Option<Command> {
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
            })
        }
        KeyCode::Char('f') => {
            app.modal = Some(Modal::Confirm(Write::Levels {
                universe: universe.to_owned(),
                start: first,
                values: run(255),
            }))
        }
        KeyCode::Char('z') => {
            app.modal = Some(Modal::Confirm(Write::Levels {
                universe: universe.to_owned(),
                start: first,
                values: run(0),
            }))
        }
        KeyCode::Char('x') => {
            app.modal = Some(Modal::Confirm(Write::BlackoutUniverse(universe.to_owned())))
        }
        KeyCode::Char('p') => app.percent = !app.percent,
        KeyCode::Char('[') => app.universe = app.universe.saturating_sub(1),
        KeyCode::Char(']') => {
            app.universe = (app.universe + 1).min(model.universes.len().saturating_sub(1))
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

/// A dialog's keys: digits into a prompt, enter to go on, esc to leave.
fn dialog(app: &mut App, model: &Model, modal: Modal, key: KeyEvent) -> Option<Command> {
    match (modal, key.code) {
        (_, KeyCode::Esc) => None,
        (Modal::Confirm(write), KeyCode::Enter) => Some(command(write)),
        (Modal::Prompt { ask, mut entry }, KeyCode::Char(c @ '0'..='9')) => {
            if entry.len() < 3 {
                entry.push(c);
            }
            app.modal = Some(Modal::Prompt { ask, entry });
            None
        }
        (Modal::Prompt { ask, mut entry }, KeyCode::Backspace) => {
            entry.pop();
            app.modal = Some(Modal::Prompt { ask, entry });
            None
        }
        (Modal::Prompt { ask, entry }, KeyCode::Enter) => {
            app.modal = confirm(app, model, &ask, entry.parse().unwrap_or(0)).map(Modal::Confirm);
            None
        }
        (modal, _) => {
            app.modal = Some(modal);
            None
        }
    }
}

/// The write a prompt's number makes, ready to confirm.
fn confirm(app: &App, model: &Model, ask: &Ask, n: u16) -> Option<Write> {
    let universe = model.universes.get(app.universe)?.name.clone();
    let level = u8::try_from(n.min(255)).unwrap_or(255);
    Some(match ask {
        Ask::Level => {
            let (first, last) = app.selection();
            Write::Levels {
                universe,
                start: first,
                values: vec![level; usize::from(last - first + 1)],
            }
        }
        Ask::Master => Write::Master(level),
        Ask::Address { uid } => {
            let d = devices(model, &universe)
                .into_iter()
                .find(|d| &d.uid == uid)?;
            Write::Address {
                universe,
                uid: uid.clone(),
                label: d.label.clone(),
                from: d.address,
                to: n.clamp(1, 512),
                footprint: d.footprint,
            }
        }
    })
}

fn command(write: Write) -> Command {
    match write {
        Write::Levels {
            universe,
            start,
            values,
        } => Command::Write {
            universe,
            start,
            values,
        },
        Write::BlackoutUniverse(universe) => Command::BlackoutUniverse(universe),
        Write::Blackout(on) => Command::Blackout(on),
        Write::Hold(on) => Command::Hold(on),
        Write::Master(m) => Command::Master(m),
        Write::Address {
            universe, uid, to, ..
        } => Command::SetAddress {
            universe,
            uid,
            address: to,
        },
        Write::Identify {
            universe, uid, on, ..
        } => Command::Identify { universe, uid, on },
    }
}
