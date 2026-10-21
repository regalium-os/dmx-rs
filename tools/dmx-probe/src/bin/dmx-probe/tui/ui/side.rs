// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The Rig tab's side panels: the policy, the nodes, RDM at a glance.

use super::chrome::block;
use crate::tui::theme::{AMBER, CHIP, GREEN, RED, TEXT, muted, strong};
use dmx_probe::engine::model::Model;
use dmx_probe::engine::patch;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub(super) fn policy(frame: &mut Frame<'_>, area: Rect, model: &Model) {
    let rules = &model.policy;
    let flag = |on: bool, colour| {
        if on {
            Span::styled("on", strong(colour))
        } else {
            Span::styled("off", strong(GREEN))
        }
    };
    let outer = block("Policy — every write passes here", false);
    let inside = outer.inner(area);
    frame.render_widget(outer, area);
    let [blackout, hold, master, mcp] = Layout::vertical([Constraint::Length(1); 4]).areas(inside);
    frame.render_widget(
        Line::from(vec![
            Span::styled("blackout    ", muted()),
            flag(rules.blackout, RED),
            Span::styled("  b", muted()),
        ]),
        blackout,
    );
    frame.render_widget(
        Line::from(vec![
            Span::styled("hold        ", muted()),
            flag(rules.hold, AMBER),
            Span::styled("  h", muted()),
        ]),
        hold,
    );
    let width = usize::from(master.width.saturating_sub(17));
    let full = width * usize::from(rules.master) / 255;
    let bar = vec![
        Span::styled("master      ", muted()),
        Span::styled("█".repeat(full), Style::new().fg(AMBER)),
        Span::styled("░".repeat(width - full), Style::new().fg(CHIP)),
        Span::styled(format!(" {:>3}", rules.master), strong(TEXT)),
    ];
    frame.render_widget(Line::from(bar), master);
    let word = if model.mcp_writes {
        "allowed"
    } else {
        "refused"
    };
    frame.render_widget(
        Line::from(vec![
            Span::styled("mcp writes  ", muted()),
            Span::styled(word, strong(AMBER)),
            Span::styled("  needs --allow-writes", muted()),
        ]),
        mcp,
    );
}

pub(super) fn nodes(frame: &mut Frame<'_>, area: Rect, model: &Model) {
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

pub(super) fn rdm(frame: &mut Frame<'_>, area: Rect, model: &Model) {
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
