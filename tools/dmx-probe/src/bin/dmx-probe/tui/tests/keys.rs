// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! The keys that write, checked to ask before they move a light.

use super::super::app::{App, Modal, Tab, Write};
use super::super::keys;
use super::{model, press, screen};
use dmx_probe::engine::Command;
use ratatui::crossterm::event::KeyCode;

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
