// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Every tab drawn into a buffer from the demo rig's model, and the keys
//! that write checked to ask before they move a light.

use super::app::{App, Modal, Tab, Write};
use super::{keys, ui};
use dmx_probe::engine::model::Model;
use dmx_probe::engine::{Command, Engine};
use dmx_probe::sim;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

fn model(discover: bool) -> Model {
    let mut engine = Engine::start(sim::demo().unwrap()).unwrap();
    if discover {
        engine.send(Command::Discover("stage-left".into()));
    }
    let start = Instant::now();
    loop {
        if let Some(m) = engine.model() {
            let ready = m.universes.len() == 6 && (!discover || m.devices.len() == 24);
            if ready || start.elapsed() > Duration::from_secs(10) {
                return m.clone();
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// The screen as text, for assertions.
fn screen(app: &App, model: &Model) -> String {
    let mut terminal = Terminal::new(TestBackend::new(170, 46)).unwrap();
    terminal.draw(|f| ui::draw(f, app, model)).unwrap();
    let buffer = terminal.backend().buffer();
    (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
                + "\n"
        })
        .collect()
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn every_tab_draws_what_the_mock_shows() {
    let model = model(true);
    let mut app = App::new();
    let rig = screen(&app, &model);
    for want in [
        "dmx-probe",
        "1 Rig",
        "Universes · 6",
        "stage-left",
        "foh-in",
        "Policy",
        "MASTER",
        "Nodes · 6",
        "overlaps",
    ] {
        assert!(rig.contains(want), "rig lacks {want:?}:\n{rig}");
    }
    app.tab = Tab::Universes;
    let levels = screen(&app, &model);
    assert!(
        levels.contains("universes/stage-left")
            && levels.contains("Selection · 101–108")
            && levels.contains("Frame interval"),
        "{levels}"
    );
    assert!(levels.contains(" 255 255 128"), "the run at 101: {levels}");
    app.tab = Tab::Rdm;
    let rdm = screen(&app, &model);
    assert!(
        rdm.contains("Devices · 24")
            && rdm.contains("Patch map")
            && rdm.contains("97–100 claimed twice"),
        "{rdm}"
    );
    app.tab = Tab::Traffic;
    let traffic = screen(&app, &model);
    assert!(
        traffic.contains("Traffic") && traffic.contains("Octets"),
        "{traffic}"
    );
    app.tab = Tab::Nodes;
    assert!(screen(&app, &model).contains("Node A"));
}

#[test]
fn blackout_acts_at_once_and_ending_it_asks() {
    let mut model = model(false);
    let mut app = App::new();
    assert_eq!(
        keys::key(&mut app, &model, press(KeyCode::Char('b'))),
        Some(Command::Blackout(true))
    );
    model.policy.blackout = true;
    assert_eq!(
        keys::key(&mut app, &model, press(KeyCode::Char('b'))),
        None,
        "asks first"
    );
    assert_eq!(app.modal, Some(Modal::Confirm(Write::Blackout(false))));
    assert!(screen(&app, &model).contains("End blackout?"));
    assert_eq!(
        keys::key(&mut app, &model, press(KeyCode::Enter)),
        Some(Command::Blackout(false))
    );
}

#[test]
fn a_typed_level_is_reviewed_before_it_is_written() {
    let model = model(false);
    let mut app = App::new();
    app.tab = Tab::Universes;
    for c in ['2', '0', '0'] {
        assert_eq!(keys::key(&mut app, &model, press(KeyCode::Char(c))), None);
    }
    assert_eq!(
        keys::key(&mut app, &model, press(KeyCode::Enter)),
        None,
        "the review, not the write"
    );
    let review = screen(&app, &model);
    assert!(
        review.contains("Write levels?") && review.contains("101–108 → 200"),
        "{review}"
    );
    let sent = keys::key(&mut app, &model, press(KeyCode::Enter));
    assert_eq!(
        sent,
        Some(Command::Write {
            universe: "stage-left".into(),
            start: 101,
            values: vec![200; 8]
        })
    );
    assert!(
        keys::key(&mut app, &model, press(KeyCode::Char('f'))).is_none()
            && matches!(app.modal, Some(Modal::Confirm(_)))
    );
    assert_eq!(keys::key(&mut app, &model, press(KeyCode::Esc)), None);
    assert_eq!(app.modal, None, "esc cancels");
}

#[test]
fn an_address_change_offers_the_first_free_run_and_checks_it() {
    let model = model(true);
    let mut app = App::new();
    app.tab = Tab::Rdm;
    let wash = keys::devices(&model, "stage-left")
        .iter()
        .position(|d| d.uid == "4c55:00000203")
        .unwrap();
    app.device = wash;
    keys::key(&mut app, &model, press(KeyCode::Char('a')));
    assert!(
        matches!(&app.modal, Some(Modal::Prompt { entry, .. }) if entry == "321"),
        "{:?}",
        app.modal
    );
    keys::key(&mut app, &model, press(KeyCode::Enter));
    let review = screen(&app, &model);
    assert!(
        review.contains("97  →  321") && review.contains("321–324 is free"),
        "{review}"
    );
    let sent = keys::key(&mut app, &model, press(KeyCode::Enter));
    assert_eq!(
        sent,
        Some(Command::SetAddress {
            universe: "stage-left".into(),
            uid: "4c55:00000203".into(),
            address: 321
        })
    );
}

/// Prints every tab as the demo draws it: `cargo test -p regalium-dmx-probe
/// -- --ignored --nocapture dump`, to see the layout without a terminal.
#[test]
#[ignore = "a tool for looking, not a check"]
fn dump() {
    let model = model(true);
    let mut app = App::new();
    for (tab, _) in Tab::ALL {
        app.tab = tab;
        println!("{}", screen(&app, &model));
    }
}
