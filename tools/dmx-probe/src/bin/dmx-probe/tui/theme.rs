// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The probe's palette, from the design mock: slate ground, cyan focus, teal
//! brand, amber caution, red fault, green health. Every pair used for text
//! clears 4.5:1 against the ground.

use ratatui::style::{Color, Modifier, Style};

/// The ground.
pub(crate) const GROUND: Color = Color::Rgb(11, 18, 32);
/// Body text.
pub(crate) const TEXT: Color = Color::Rgb(226, 232, 240);
/// Labels and secondary text.
pub(crate) const MUTED: Color = Color::Rgb(148, 163, 184);
/// Block borders, unfocused.
pub(crate) const BORDER: Color = Color::Rgb(51, 65, 85);
/// A chip's value, a selected row.
pub(crate) const CHIP: Color = Color::Rgb(30, 41, 59);
/// Focus: the active tab, a focused block.
pub(crate) const CYAN: Color = Color::Rgb(34, 211, 238);
/// The brand, RDM footprints.
pub(crate) const TEAL: Color = Color::Rgb(20, 184, 166);
/// The second footprint shade.
pub(crate) const DEEP: Color = Color::Rgb(14, 116, 144);
/// Caution: hold, master, a held source.
pub(crate) const AMBER: Color = Color::Rgb(251, 191, 36);
/// Fault: failing, blackout, overlap.
pub(crate) const RED: Color = Color::Rgb(248, 113, 113);
/// Health.
pub(crate) const GREEN: Color = Color::Rgb(74, 222, 128);

/// Body text on the ground.
pub(crate) fn text() -> Style {
    Style::new().fg(TEXT).bg(GROUND)
}

/// Secondary text.
pub(crate) fn muted() -> Style {
    Style::new().fg(MUTED)
}

/// Bold, in `colour`.
pub(crate) fn strong(colour: Color) -> Style {
    Style::new().fg(colour).add_modifier(Modifier::BOLD)
}

/// A level's cell: the ground at 0 to amber at 255, its digits dark on a
/// bright cell.
pub(crate) fn heat(level: u8) -> Style {
    let mix = |from: u8, to: u8| {
        let (from, to, l) = (i32::from(from), i32::from(to), i32::from(level));
        u8::try_from(from + (to - from) * l / 255).unwrap_or(u8::MAX)
    };
    let bg = Color::Rgb(mix(30, 251), mix(41, 191), mix(59, 36));
    Style::new()
        .bg(bg)
        .fg(if level > 150 { GROUND } else { TEXT })
}
