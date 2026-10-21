// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The model, copied out of the rig.

use super::Worker;
use super::model::{self, Model, Source, State, Universe};

/// Intervals the sparkline shows.
const SPARK: usize = 40;

impl Worker {
    pub(crate) fn snapshot(&self) -> Model {
        let universes = self.rig.entries.iter().map(|e| {
            let (fps, intervals, histogram) =
                self.rig
                    .stats
                    .of(e.protocol.name(), i64::from(e.number), SPARK);
            let (levels, generation, sources) = match (&e.levels, &e.merged) {
                (Some(l), _) => {
                    let u = l.snapshot();
                    (u.levels().to_vec(), u.generation(), Vec::new())
                }
                (None, Some((m, _))) => {
                    let u = m.snapshot();
                    let sources = m
                        .source_list()
                        .into_iter()
                        .map(|(id, priority, merged)| Source {
                            id: format!("{:x}", id.0),
                            priority,
                            merged,
                        })
                        .collect();
                    (u.levels().to_vec(), u.generation(), sources)
                }
                (None, None) => (Vec::new(), 0, Vec::new()),
            };
            let failing = self.failing.contains(&e.name);
            let state = if failing {
                State::Failing
            } else if e.merged.is_some() && sources.is_empty() {
                State::Idle
            } else {
                State::Live
            };
            Universe {
                name: e.name.clone(),
                protocol: e.protocol,
                number: e.label.clone(),
                output: e.levels.is_some(),
                merge: e.merged.as_ref().map(|(_, mode)| *mode),
                state,
                levels,
                generation,
                fps: if failing { 0.0 } else { fps },
                intervals,
                histogram,
                sources,
                target: e.target.clone(),
            }
        });
        let rules = self.rig.policy.rules();
        Model {
            universes: universes.collect(),
            nodes: self.rig.nodes.clone(),
            devices: self.devices.clone(),
            policy: model::Policy {
                blackout: rules.blackout,
                hold: rules.hold,
                master: rules.master,
            },
            mcp_writes: false,
            events: self.feed.clone(),
            packets: self.rig.capture.packets(),
            level_packets: self.rig.capture.level_packets(),
            discovery: self.discovery.clone(),
            attached: self.rig.attached.clone(),
        }
    }
}
