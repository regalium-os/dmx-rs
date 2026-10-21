// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 3, the equipment discovery found.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{AMBER, CHIP, GREEN, MUTED, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

pub(crate) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let [left, right] = Layout::horizontal([Constraint::Min(0), Constraint::Length(50)])
        .spacing(2)
        .areas(area);
    let header =
        Row::new(["proto", "name", "address", "ports", "fw", "state"]).style(strong(MUTED));
    let rows = model.nodes.iter().enumerate().map(|(i, n)| {
        let (s, c) = if n.reachable {
            ("reachable", GREEN)
        } else {
            ("unreachable", AMBER)
        };
        let row = Row::new([
            Cell::from(Span::styled(n.protocol.name(), muted())),
            Cell::from(Span::styled(n.name.clone(), strong(TEXT))),
            Cell::from(n.address.clone()),
            Cell::from(n.ports.clone()),
            Cell::from(n.firmware.clone()),
            Cell::from(Span::styled(s, Style::new().fg(c))),
        ]);
        if i == app.node {
            row.style(Style::new().bg(CHIP))
        } else {
            row
        }
    });
    let widths = [
        Constraint::Length(7),
        Constraint::Length(16),
        Constraint::Length(16),
        Constraint::Length(18),
        Constraint::Length(7),
        Constraint::Min(10),
    ];
    frame.render_widget(
        Table::new(rows, widths)
            .header(header)
            .block(block(format!("Nodes · {}", model.nodes.len()), true)),
        left,
    );
    let lines = match model.nodes.get(app.node) {
        Some(n) => vec![
            Line::from(vec![
                Span::styled("name      ", muted()),
                Span::raw(n.name.clone()),
            ]),
            Line::from(vec![
                Span::styled("protocol  ", muted()),
                Span::raw(n.protocol.name()),
            ]),
            Line::from(vec![
                Span::styled("address   ", muted()),
                Span::raw(n.address.clone()),
            ]),
            Line::from(vec![
                Span::styled("ports     ", muted()),
                Span::raw(n.ports.clone()),
            ]),
            Line::from(vec![
                Span::styled("firmware  ", muted()),
                Span::raw(n.firmware.clone()),
            ]),
            Line::default(),
            Line::from(Span::styled(
                "Discovery never sends DMX: ArtPoll, sACN's announcements, and the serial probe's two widget requests.",
                muted(),
            )),
        ],
        None => vec![Line::from(Span::styled("No nodes found yet.", muted()))],
    };
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(block("Node", false)),
        right,
    );
}
