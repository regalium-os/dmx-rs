// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A dialog's keys, and the write each prompt's number makes.

use super::super::app::{App, Ask, Modal, Write};
use super::devices;
use dmx_probe::engine::Command;
use dmx_probe::engine::model::Model;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// A dialog's keys: digits into a prompt, enter to go on, esc to leave.
pub(super) fn dialog(app: &mut App, model: &Model, open: Modal, key: KeyEvent) -> Option<Command> {
    match (open, key.code) {
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
        (unchanged, _) => {
            app.modal = Some(unchanged);
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
