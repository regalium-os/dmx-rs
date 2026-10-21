// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 5, packets: the list, the selected packet's fields as the driver's
//! codecs read them, and its octets with the selected field's lit.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{CHIP, CYAN, GROUND, MUTED, TEAL, TEXT, muted, strong};
use dmx_probe::dissect::{Dissection, dissect};
use dmx_probe::engine::model::Model;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

pub(crate) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let [title, body] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
        .spacing(1)
        .areas(area);
    frame.render_widget(
        Line::from(vec![
            Span::styled("Packets  ", strong(TEXT)),
            Span::styled(
                format!(
                    "{} kept · {} level packets counted, not kept",
                    model.packets.len(),
                    model.level_packets
                ),
                muted(),
            ),
        ]),
        title,
    );
    let [left, right] = Layout::horizontal([Constraint::Min(0), Constraint::Length(70)])
        .spacing(2)
        .areas(body);
    let newest: Vec<_> = model.packets.iter().rev().collect();
    let selected = newest.get(app.packet).copied();
    let rows = newest
        .iter()
        .take(usize::from(left.height))
        .enumerate()
        .map(|(i, p)| {
            let d = dissect(p);
            let row = Row::new([
                Cell::from(Span::styled(p.time.clone(), muted())),
                Cell::from(Span::styled(
                    if p.sent { "→" } else { "←" },
                    strong(if p.sent { TEAL } else { CYAN }),
                )),
                Cell::from(Span::styled(p.protocol.name(), muted())),
                Cell::from(p.peer.clone()),
                Cell::from(d.summary),
                Cell::from(Span::styled(p.octets.len().to_string(), muted())),
            ]);
            if i == app.packet {
                row.style(Style::new().bg(CHIP))
            } else {
                row
            }
        });
    let header = Row::new(["time", "", "proto", "peer", "what", "len"]).style(strong(MUTED));
    let widths = [
        Constraint::Length(12),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(14),
        Constraint::Min(20),
        Constraint::Length(4),
    ];
    frame.render_widget(
        Table::new(rows, widths)
            .header(header)
            .block(block("Traffic", true)),
        left,
    );

    let [tree, octets] = Layout::vertical([Constraint::Min(0), Constraint::Length(12)])
        .spacing(1)
        .areas(right);
    let Some(packet) = selected else {
        frame.render_widget(
            Paragraph::new("Nothing captured yet.").block(block("Packet", false)),
            right,
        );
        return;
    };
    let d: Dissection = dissect(packet);
    let field = app.field.min(d.tree.len().saturating_sub(1));
    let lines: Vec<Line<'_>> = d
        .tree
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let span = f.span.map_or_else(String::new, |(a, b)| {
                if a == b {
                    a.to_string()
                } else {
                    format!("{a}–{b}")
                }
            });
            let indent = "  ".repeat(usize::from(f.depth));
            let style = if i == field {
                Style::new().bg(CYAN).fg(GROUND).bold()
            } else if f.depth == 0 {
                strong(CYAN)
            } else {
                Style::new().fg(TEXT)
            };
            Line::from(vec![
                Span::styled(format!("{indent}{:<52}", f.text), style),
                Span::styled(format!("{span:>9}"), muted()),
            ])
        })
        .collect();
    frame.render_widget(
        Paragraph::new(lines).block(block(
            format!("{} · {} octets", d.summary, packet.octets.len()),
            false,
        )),
        tree,
    );

    let lit = d.tree.get(field).and_then(|f| f.span);
    let lines: Vec<Line<'_>> = packet
        .octets
        .chunks(16)
        .enumerate()
        .take(usize::from(octets.height.saturating_sub(2)))
        .map(|(row, chunk)| {
            let mut spans = vec![Span::styled(format!("{:04x}  ", row * 16), muted())];
            for (i, b) in chunk.iter().enumerate() {
                let at = row * 16 + i;
                let on = lit.is_some_and(|(a, z)| (a..=z).contains(&at));
                let style = if on {
                    Style::new().bg(CYAN).fg(GROUND)
                } else {
                    Style::new().fg(TEXT).bg(Color::Rgb(19, 27, 43))
                };
                spans.push(Span::styled(format!("{b:02x}"), style));
                spans.push(Span::raw(" "));
            }
            Line::from(spans)
        })
        .collect();
    frame.render_widget(Paragraph::new(lines).block(block("Octets", false)), octets);
}
