// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Tab 1, the rig at a glance: every universe with its frame timing, the
//! event feed, the policy, the nodes and RDM.

use super::chrome::block;
use crate::tui::app::App;
use crate::tui::theme::{AMBER, CHIP, CYAN, GREEN, MUTED, RED, TEAL, TEXT, muted, strong};
use dmx_probe::engine::model::{Model, Severity, State, Universe};
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
    super::side::policy(frame, policy, model);
    super::side::nodes(frame, nodes, model);
    super::side::rdm(frame, rdm, model);
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
    /// Where each taller bar starts, in milliseconds.
    const STEPS: [f32; 7] = [21.0, 22.0, 23.0, 24.0, 25.0, 27.0, 30.0];
    let skip = u.intervals.len().saturating_sub(width);
    let spans = u.intervals.iter().skip(skip).map(|&ms| {
        let at = STEPS.iter().take_while(|&&t| ms >= t).count();
        let colour = if ms > 24.0 { AMBER } else { TEAL };
        Span::styled(
            BARS.get(at).copied().unwrap_or('█').to_string(),
            Style::new().fg(colour),
        )
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
