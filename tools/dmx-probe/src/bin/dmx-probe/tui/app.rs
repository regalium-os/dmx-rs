// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What the TUI remembers between frames: where the user is, and what they
//! are being asked. Everything it shows comes from the engine's model.

/// The tabs, in the order their number keys select them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tab {
    Rig,
    Universes,
    Nodes,
    Rdm,
    Traffic,
    Settings,
}

impl Tab {
    pub(crate) const ALL: [(Self, &'static str); 6] = [
        (Self::Rig, "Rig"),
        (Self::Universes, "Universes"),
        (Self::Nodes, "Nodes"),
        (Self::Rdm, "RDM"),
        (Self::Traffic, "Traffic"),
        (Self::Settings, "Settings"),
    ];
}

/// What a typed number becomes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Ask {
    /// A level for the selection.
    Level,
    /// The grand master.
    Master,
    /// A device's new start address.
    Address { uid: String },
}

/// A write, described, waiting for enter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Write {
    Levels {
        universe: String,
        start: u16,
        values: Vec<u8>,
    },
    BlackoutUniverse(String),
    Blackout(bool),
    Hold(bool),
    Master(u8),
    Address {
        universe: String,
        uid: String,
        label: String,
        from: u16,
        to: u16,
        footprint: u16,
    },
    Identify {
        universe: String,
        uid: String,
        label: String,
        on: bool,
    },
}

/// A dialog over the tab.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Modal {
    /// A number being typed.
    Prompt { ask: Ask, entry: String },
    /// A write being confirmed.
    Confirm(Write),
}

/// The TUI's state.
#[derive(Debug)]
pub(crate) struct App {
    pub(crate) tab: Tab,
    /// The universe selected on the Rig tab and shown on Universes.
    pub(crate) universe: usize,
    /// The level grid's cursor, 1 to 512, and the selection's other end.
    pub(crate) cursor: u16,
    pub(crate) anchor: Option<u16>,
    /// Levels as percentages rather than 0 to 255.
    pub(crate) percent: bool,
    pub(crate) device: usize,
    pub(crate) node: usize,
    /// The selected packet, counted from the newest, and its field.
    pub(crate) packet: usize,
    pub(crate) field: usize,
    /// Level packets shown in the Traffic tab.
    pub(crate) show_levels: bool,
    pub(crate) modal: Option<Modal>,
    pub(crate) help: bool,
    pub(crate) quit: bool,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            tab: Tab::Rig,
            universe: 0,
            cursor: 101,
            anchor: Some(108),
            percent: false,
            device: 0,
            node: 0,
            packet: 0,
            field: 0,
            show_levels: false,
            modal: None,
            help: false,
            quit: false,
        }
    }

    /// The selection on the level grid, first to last.
    pub(crate) fn selection(&self) -> (u16, u16) {
        let other = self.anchor.unwrap_or(self.cursor);
        (self.cursor.min(other), self.cursor.max(other))
    }
}
