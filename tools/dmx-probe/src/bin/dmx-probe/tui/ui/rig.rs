// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 1, the rig at a glance: every universe with its frame timing, the
//! event feed, the policy, the nodes and RDM.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{AMBER, CHIP, CYAN, GREEN, MUTED, RED, TEAL, TEXT, muted, strong};
use dmx_probe::engine::model::{Model, Severity, State, Universe};
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

pub(crate) fn draw(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let [left, right] = Layout::horizontal([Constraint::Min(0), Constraint::Length(48)])
        .spacing(2)
        .areas(area);
    let [table, feed] = Layout::vertical([Constraint::Min(0), Constraint::Length(12)])
        .spacing(1)
        .areas(left);
    universes(frame, table, app, model);
    events(frame, feed, model);
    let [policy, nodes, rdm] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Min(0),
        Constraint::Length(7),
    ])
    .spacing(1)
    .areas(right);
    self::policy(frame, policy, model);
    self::nodes(frame, nodes, model);
    self::rdm(frame, rdm, model);
}

fn state(u: &Universe) -> (&'static str, Color) {
    match u.state {
        State::Live => ("live", GREEN),
        State::Idle => ("idle", MUTED),
        State::Failing => ("failing", RED),
    }
}

/// The intervals as bars: low and teal on time, taller and amber when late,
/// flat and red while failing.
pub(crate) fn spark(u: &Universe, width: usize) -> Line<'static> {
    const BARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let skip = u.intervals.len().saturating_sub(width);
    let spans = u.intervals.iter().skip(skip).map(|&ms| {
        let at = (((ms - 20.0) / 2.0).clamp(0.0, 7.0)) as usize;
        let colour = if ms > 24.0 { AMBER } else { TEAL };
        Span::styled(BARS[at].to_string(), Style::new().fg(colour))
    });
    let mut line: Vec<Span<'static>> = spans.collect();
    if u.state == State::Failing {
        line.push(Span::styled("▁▁▁▁", Style::new().fg(RED)));
    }
    Line::from(line)
}

fn universes(frame: &mut Frame<'_>, area: Rect, app: &App, model: &Model) {
    let header = Row::new([
        "name",
        "proto",
        "number",
        "role",
        "state",
        "slots",
        "fps",
        "sources",
        "frame interval",
    ])
    .style(strong(MUTED));
    let rows = model.universes.iter().enumerate().map(|(i, u)| {
        let (s, c) = state(u);
        let sources = match u.merge {
            Some(mode) => format!("{} · {mode}", u.sources.len()),
            None => "—".to_owned(),
        };
        let row = Row::new([
            Cell::from(Span::styled(u.name.clone(), strong(TEXT))),
            Cell::from(Span::styled(u.protocol.name(), muted())),
            Cell::from(u.number.clone()),
            Cell::from(if u.output { "output" } else { "input" }),
            Cell::from(Span::styled(s, strong(c))),
            Cell::from(u.levels.len().to_string()),
            Cell::from(format!("{:.1}", u.fps)),
            Cell::from(sources),
            Cell::from(spark(u, 40)),
        ]);
        if i == app.universe {
            row.style(Style::new().bg(CHIP))
        } else {
            row
        }
    });
    let widths = [
        Constraint::Length(13),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Length(9),
        Constraint::Min(10),
    ];
    let title = format!("Universes · {}", model.universes.len());
    frame.render_widget(
        Table::new(rows, widths)
            .header(header)
            .block(block(title, true)),
        area,
    );
}

fn events(frame: &mut Frame<'_>, area: Rect, model: &Model) {
    let lines: Vec<Line<'_>> = model
        .events
        .iter()
        .take(usize::from(area.height.saturating_sub(2)))
        .map(|e| {
            let c = match e.severity {
                Severity::Good => GREEN,
                Severity::Warn => AMBER,
                Severity::Bad => RED,
                Severity::Info => CYAN,
            };
            Line::from(vec![
                Span::styled(format!("{}  ", e.time), muted()),
                Span::styled(format!("{:<16}", e.kind), strong(c)),
                Span::raw(e.text.clone()),
            ])
        })
        .collect();
    frame.render_widget(Paragraph::new(lines).block(block("Events", false)), area);
}

fn policy(frame: &mut Frame<'_>, area: Rect, model: &Model) {
    let p = &model.policy;
    let flag = |on: bool, what: &'static str| {
        if on {
            Span::styled("on", strong(if what == "b" { RED } else { AMBER }))
        } else {
            Span::styled("off", strong(GREEN))
        }
    };
    let inner = block("Policy — every write passes here", false);
    let at = inner.inner(area);
    frame.render_widget(inner, area);
    let [b, h, m, mcp] = Layout::vertical([Constraint::Length(1); 4]).areas(at);
    frame.render_widget(
        Line::from(vec![
            Span::styled("blackout    ", muted()),
            flag(p.blackout, "b"),
            Span::styled("  b", muted()),
        ]),
        b,
    );
    frame.render_widget(
        Line::from(vec![
            Span::styled("hold        ", muted()),
            flag(p.hold, "h"),
            Span::styled("  h", muted()),
        ]),
        h,
    );
    let width = usize::from(m.width.saturating_sub(17));
    let full = width * usize::from(p.master) / 255;
    frame.render_widget(
        Line::from(vec![
            Span::styled("master      ", muted()),
            Span::styled("█".repeat(full), Style::new().fg(AMBER)),
            Span::styled("░".repeat(width - full), Style::new().fg(CHIP)),
            Span::styled(format!(" {:>3}", p.master), strong(TEXT)),
        ]),
        m,
    );
    let (word, c) = if model.mcp_writes {
        ("allowed", AMBER)
    } else {
        ("refused", AMBER)
    };
    frame.render_widget(
        Line::from(vec![
            Span::styled("mcp writes  ", muted()),
            Span::styled(word, strong(c)),
            Span::styled("  needs --allow-writes", muted()),
        ]),
        mcp,
    );
}

fn nodes(frame: &mut Frame<'_>, area: Rect, model: &Model) {
    let lines: Vec<Line<'_>> = model
        .nodes
        .iter()
        .map(|n| {
            let (s, c) = if n.reachable {
                ("reachable", GREEN)
            } else {
                ("unreachable", AMBER)
            };
            Line::from(vec![
                Span::styled(format!("{:<7}", n.protocol.name()), muted()),
                Span::raw(format!("{:<28}", format!("{} · {}", n.name, n.address))),
                Span::styled(s, strong(c)),
            ])
        })
        .collect();
    frame.render_widget(
        Paragraph::new(lines).block(block(format!("Nodes · {}", model.nodes.len()), false)),
        area,
    );
}

fn rdm(frame: &mut Frame<'_>, area: Rect, model: &Model) {
    let silent = model.devices.iter().filter(|d| !d.responding).count();
    let overlaps: Vec<String> = model
        .universes
        .iter()
        .flat_map(|u| {
            patch::overlaps(&patch::owners(
                model.devices.iter().filter(|d| d.universe == u.name),
            ))
            .into_iter()
            .map(move |(a, b)| format!("{} {a}–{b}", u.name))
        })
        .collect();
    let lines = vec![
        Line::from(vec![
            Span::styled("devices   ", muted()),
            Span::styled(model.devices.len().to_string(), strong(TEXT)),
        ]),
        Line::from(vec![
            Span::styled("silent    ", muted()),
            Span::styled(
                silent.to_string(),
                strong(if silent > 0 { AMBER } else { GREEN }),
            ),
        ]),
        Line::from(vec![
            Span::styled("overlaps  ", muted()),
            Span::styled(
                overlaps.len().to_string(),
                strong(if overlaps.is_empty() { GREEN } else { RED }),
            ),
            Span::styled(format!("  {}", overlaps.join(", ")), muted()),
        ]),
        Line::from(vec![
            Span::styled("last      ", muted()),
            Span::styled(
                model
                    .discovery
                    .clone()
                    .unwrap_or_else(|| "not yet · 4 then D".to_owned()),
                muted(),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).block(block("RDM", false)), area);
}
