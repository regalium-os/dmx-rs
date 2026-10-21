// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Around every tab: the brand and tabs with status chips above, the keys
//! that work below, and the bordered block every panel sits in.

use crate::tui::app::{App, Tab};
use crate::tui::theme::{
    AMBER, BORDER, CHIP, CYAN, GREEN, GROUND, MUTED, RED, TEAL, TEXT, muted, strong,
};
use dmx_probe::engine::model::{Model, State};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

/// A block with its title in the border, cyan when it has the focus.
pub(crate) fn block(title: impl Into<String>, focused: bool) -> Block<'static> {
    let colour = if focused { CYAN } else { BORDER };
    Block::bordered()
        .border_style(Style::new().fg(colour))
        .title(Span::styled(
            format!(" {} ", title.into()),
            strong(if focused { CYAN } else { TEXT }),
        ))
        .style(Style::new().bg(GROUND).fg(TEXT))
}

fn chip(tag: &str, value: &str, colour: Color) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            format!(" {tag} "),
            Style::new().fg(GROUND).bg(colour).bold(),
        ),
        Span::styled(format!(" {value} "), Style::new().fg(TEXT).bg(CHIP)),
        Span::raw(" "),
    ]
}

/// The top line: brand, tabs, and the policy and transports at a glance.
pub(crate) fn header(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let mut left = vec![
        Span::styled(" dmx-probe ", Style::new().fg(GROUND).bg(TEAL).bold()),
        Span::raw("  "),
    ];
    for (i, (tab, title)) in Tab::ALL.iter().enumerate() {
        let label = format!(" {} {title} ", i + 1);
        left.push(if *tab == app.tab {
            Span::styled(label, Style::new().fg(GROUND).bg(CYAN).bold())
        } else {
            Span::styled(label, Style::new().fg(TEXT))
        });
    }
    frame.render_widget(
        Paragraph::new(Line::from(left)).style(Style::new().bg(GROUND)),
        area,
    );

    let p = &model.policy;
    let mut right = Vec::new();
    if p.blackout {
        right.extend(chip("BLACKOUT", "on", RED));
    } else if p.hold {
        right.extend(chip("POLICY", "hold", AMBER));
    } else {
        right.extend(chip("POLICY", "live", GREEN));
    }
    right.extend(chip(
        "MASTER",
        &format!("{}%", u32::from(p.master) * 100 / 255),
        AMBER,
    ));
    right.extend(chip(
        "MCP",
        if model.mcp_writes {
            "writes"
        } else {
            "read-only"
        },
        MUTED,
    ));
    let failing = model
        .universes
        .iter()
        .filter(|u| u.state == State::Failing)
        .count();
    if failing > 0 {
        right.extend(chip("FAILING", &failing.to_string(), RED));
    }
    let width = right.iter().map(Span::width).sum::<usize>();
    let x = area
        .right()
        .saturating_sub(u16::try_from(width).unwrap_or(0));
    let at = Rect {
        x,
        width: area.width.saturating_sub(x - area.x),
        ..area
    };
    frame.render_widget(Paragraph::new(Line::from(right)), at);
}

/// The bottom line: the keys that work on this tab.
pub(crate) fn footer(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let keys: &[(&str, &str)] = match app.tab {
        _ if app.modal.is_some() => &[
            ("enter", "go on"),
            ("esc", "cancel"),
            ("0–9", "type"),
            ("⌫", "erase"),
        ],
        Tab::Universes => &[
            ("←↑↓→", "move"),
            ("shift", "select"),
            ("0–9", "level"),
            ("f", "full"),
            ("z", "zero"),
            ("x", "blackout universe"),
            ("p", "percent"),
            ("[ ]", "universe"),
            ("esc", "back"),
        ],
        Tab::Rdm => &[
            ("↑↓", "select"),
            ("D", "discover"),
            ("a", "address"),
            ("i", "identify"),
            ("esc", "back"),
        ],
        Tab::Traffic => &[("↑↓", "packet"), ("←→", "field"), ("esc", "back")],
        _ => &[
            ("q", "quit"),
            ("?", "help"),
            ("1–6", "tabs"),
            ("↑↓", "select"),
            ("enter", "open"),
            ("b", "blackout"),
            ("h", "hold"),
            ("m", "master"),
        ],
    };
    let mut spans = Vec::new();
    for (k, what) in keys {
        spans.push(Span::styled(
            format!(" {k} "),
            Style::new().fg(GROUND).bg(MUTED).bold(),
        ));
        spans.push(Span::styled(format!(" {what}  "), Style::new().fg(TEXT)));
    }
    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::new().bg(GROUND)),
        area,
    );
    let attached = Span::styled(format!("{} ", model.attached), muted());
    let w = u16::try_from(attached.width()).unwrap_or(0);
    frame.render_widget(
        Paragraph::new(Line::from(attached)),
        Rect {
            x: area.right().saturating_sub(w),
            width: w.min(area.width),
            ..area
        },
    );
}
