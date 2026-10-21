// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 4, RDM on the selected universe: the devices, one device, and the
//! patch map — who owns each address, red where two claim it.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::keys::devices;
use crate::tui::theme::{AMBER, CHIP, GREEN, MUTED, RED, TEXT, muted, strong};
use dmx_probe::engine::model::{Device, Model};
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table, Wrap};

pub(crate) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let universe = model
        .universes
        .get(app.universe)
        .map(|u| u.name.clone())
        .unwrap_or_default();
    let list = devices(model, &universe);
    let owners = patch::owners(list.iter().copied());
    let [title, body] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
        .spacing(1)
        .areas(area);
    let status = model
        .discovery
        .clone()
        .unwrap_or_else(|| "not searched yet · D discovers".to_owned());
    frame.render_widget(
        Line::from(vec![
            Span::styled(format!("universes/{universe} · devices   "), strong(TEXT)),
            Span::styled(status, muted()),
        ]),
        title,
    );
    let [left, right] = Layout::horizontal([Constraint::Min(0), Constraint::Length(56)])
        .spacing(2)
        .areas(body);
    table(frame, left, app, &list, &owners);
    let [top, bottom] = Layout::vertical([Constraint::Length(10), Constraint::Min(0)])
        .spacing(1)
        .areas(right);
    let selected = list.get(app.device).copied();
    detail(frame, top, selected, &owners);
    super::patchmap::draw(frame, bottom, &list, selected, &owners);
}

fn claims_twice(d: &Device, owners: &[u8; 513]) -> bool {
    patch::span(d).any(|a| owners.get(usize::from(a)).is_some_and(|&n| n > 1))
}

fn table(frame: &mut Frame<'_>, area: Rect, app: &App, list: &[&Device], owners: &[u8; 513]) {
    let header =
        Row::new(["uid", "label", "model", "addr", "fp", "pers", "state"]).style(strong(MUTED));
    let rows = list.iter().enumerate().map(|(i, d)| {
        let (s, c) = if d.responding {
            ("responding", GREEN)
        } else {
            ("silent", AMBER)
        };
        let row = Row::new([
            Cell::from(Span::styled(d.uid.clone(), muted())),
            Cell::from(Span::styled(
                if d.label.is_empty() {
                    "—".to_owned()
                } else {
                    d.label.clone()
                },
                strong(TEXT),
            )),
            Cell::from(d.model.clone()),
            Cell::from(Span::styled(
                d.address.to_string(),
                strong(if claims_twice(d, owners) { RED } else { TEXT }),
            )),
            Cell::from(d.footprint.to_string()),
            Cell::from(d.personality.clone()),
            Cell::from(Span::styled(s, Style::new().fg(c))),
        ]);
        if i == app.device {
            row.style(Style::new().bg(CHIP))
        } else {
            row
        }
    });
    let widths = [
        Constraint::Length(15),
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Min(8),
    ];
    frame.render_widget(
        Table::new(rows, widths)
            .header(header)
            .block(block(format!("Devices · {}", list.len()), true)),
        area,
    );
}

fn detail(frame: &mut Frame<'_>, area: Rect, selected: Option<&Device>, owners: &[u8; 513]) {
    let lines = match selected {
        Some(d) => {
            let end = d.address + d.footprint.saturating_sub(1);
            let clash = claims_twice(d, owners);
            vec![
                Line::from(vec![
                    Span::styled("uid            ", muted()),
                    Span::raw(d.uid.clone()),
                ]),
                Line::from(vec![
                    Span::styled("model          ", muted()),
                    Span::raw(d.model.clone()),
                ]),
                Line::from(vec![
                    Span::styled("personality    ", muted()),
                    Span::raw(format!("{} · footprint {}", d.personality, d.footprint)),
                ]),
                Line::from(vec![
                    Span::styled("start address  ", muted()),
                    Span::styled(
                        d.address.to_string(),
                        strong(if clash { RED } else { TEXT }),
                    ),
                    Span::raw(format!(
                        "  {}–{end}{}",
                        d.address,
                        if clash { " · overlaps" } else { "" }
                    )),
                ]),
                Line::from(vec![
                    Span::styled("state          ", muted()),
                    Span::styled(
                        if d.responding { "responding" } else { "silent" },
                        Style::new().fg(if d.responding { GREEN } else { AMBER }),
                    ),
                ]),
                Line::default(),
                Line::from(Span::styled(
                    "a address · i identify · every SET asks, then reads back",
                    muted(),
                )),
            ]
        }
        None => vec![Line::from(Span::styled(
            "No devices. D discovers this universe's line.",
            muted(),
        ))],
    };
    let title = selected.map_or_else(
        || "Device".to_owned(),
        |d| {
            format!(
                "{} · {}",
                if d.label.is_empty() {
                    "device"
                } else {
                    &d.label
                },
                d.uid
            )
        },
    );
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(block(title, false)),
        area,
    );
}
