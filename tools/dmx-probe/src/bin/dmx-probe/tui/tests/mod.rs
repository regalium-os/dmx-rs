// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! Every tab drawn into a buffer from the demo rig's model, and the keys
//! that write checked to ask before they move a light.

mod keys;

use super::app::{App, Tab};
use super::ui;
use dmx_probe::engine::model::Model;
use dmx_probe::engine::{Command, Engine};
use dmx_probe::sim;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

pub(super) fn model(discover: bool) -> Model {
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
pub(super) fn screen(app: &App, model: &Model) -> String {
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

pub(super) fn press(code: KeyCode) -> KeyEvent {
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
