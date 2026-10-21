// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 4, RDM on the selected universe: the devices, one device, and the
//! patch map — who owns each address, red where two claim it.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::keys::devices;
use crate::tui::theme::{AMBER, CHIP, DEEP, GREEN, MUTED, RED, TEAL, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

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

    let header =
        Row::new(["uid", "label", "model", "addr", "fp", "pers", "state"]).style(strong(MUTED));
    let rows = list.iter().enumerate().map(|(i, d)| {
        let clash = patch::span(d).any(|a| owners.get(usize::from(a)).is_some_and(|&n| n > 1));
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
                strong(if clash { RED } else { TEXT }),
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
        left,
    );

    let [detail, map] = Layout::vertical([Constraint::Length(10), Constraint::Min(0)])
        .spacing(1)
        .areas(right);
    let selected = list.get(app.device).copied();
    let lines = match selected {
        Some(d) => {
            let last = d.address + d.footprint.saturating_sub(1);
            let clash = patch::span(d).any(|a| owners.get(usize::from(a)).is_some_and(|&n| n > 1));
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
                        "  {}–{last}{}",
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
    frame.render_widget(Paragraph::new(lines).block(block(title, false)), detail);

    let mut lines: Vec<Line<'_>> = (0..16u16)
        .map(|row| {
            let spans = (1..=32u16).map(|c| {
                let a = row * 32 + c;
                let n = owners.get(usize::from(a)).copied().unwrap_or(0);
                let colour = match n {
                    0 => CHIP,
                    1 if ((a - 1) / 16) % 2 == 0 => TEAL,
                    1 => DEEP,
                    _ => RED,
                };
                Span::styled("█", Style::new().fg(colour))
            });
            Line::from(
                std::iter::once(Span::styled(format!("{:>4} ", row * 32 + 1), muted()))
                    .chain(spans)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    lines.push(Line::default());
    for (a, b) in patch::overlaps(&owners) {
        lines.push(Line::from(vec![
            Span::styled(format!("{a}–{b}"), strong(RED)),
            Span::styled(" claimed twice", muted()),
        ]));
    }
    if let Some(d) = selected {
        let free = patch::first_free(list.iter().copied(), &d.uid, d.footprint)
            .map_or_else(|| "none".to_owned(), |a| a.to_string());
        lines.push(Line::from(vec![
            Span::styled(format!("first free run of {}: ", d.footprint), muted()),
            Span::styled(free, strong(TEXT)),
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(block("Patch map · 1–512", false)),
        map,
    );
}
