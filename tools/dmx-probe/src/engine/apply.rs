// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Commands, applied. RDM work is [`super::work`]'s; the feed is
//! [`super::feed`]'s.

use super::model::Severity;
use super::{Command, Worker};
use regalium_dmx::address::Address;
use regalium_dmx::rdm::Pid;

impl Worker {
    pub(crate) fn apply(&mut self, command: Command) {
        let policy = &self.rig.policy;
        match command {
            Command::Blackout(on) => {
                if policy.set_blackout(on) {
                    self.note(
                        "policy",
                        if on { "blackout on" } else { "blackout off" },
                        Severity::Warn,
                    );
                }
            }
            Command::Hold(on) => {
                if policy.set_hold(on) {
                    self.note(
                        "policy",
                        if on { "hold on" } else { "hold off" },
                        Severity::Warn,
                    );
                }
            }
            Command::Master(master) => {
                let before = policy.rules().master;
                if policy.set_master(master) {
                    self.note(
                        "policy",
                        &format!("master {before} → {master}"),
                        Severity::Warn,
                    );
                }
            }
            Command::Write {
                universe,
                start,
                values,
            } => {
                let written = self
                    .levels(&universe)
                    .zip(Address::new(start))
                    .map(|(l, a)| l.write(a, &values));
                if !matches!(written, Some(Ok(_))) {
                    self.note(
                        "refused",
                        &format!("{universe}: not an output, or past 512"),
                        Severity::Bad,
                    );
                }
            }
            Command::BlackoutUniverse(universe) => {
                if let Some(levels) = self.levels(&universe) {
                    levels.blackout();
                    self.note("blackout", &universe, Severity::Warn);
                }
            }
            Command::Discover(universe) => self.discover(universe),
            Command::SetAddress {
                universe,
                uid,
                address,
            } => {
                self.rdm_set(
                    universe,
                    &uid,
                    Pid::DMX_START_ADDRESS,
                    address.to_be_bytes().to_vec(),
                    format!("start address → {address}"),
                );
            }
            Command::Identify { universe, uid, on } => {
                let what = if on { "identify on" } else { "identify off" };
                self.rdm_set(
                    universe,
                    &uid,
                    Pid::IDENTIFY_DEVICE,
                    vec![u8::from(on)],
                    what.to_owned(),
                );
            }
        }
    }

    fn levels(&self, universe: &str) -> Option<&regalium_dmx::runtime::Levels> {
        self.rig
            .entries
            .iter()
            .find(|e| e.name == universe)?
            .levels
            .as_ref()
    }

    pub(crate) fn number(&self, universe: &str) -> Option<u32> {
        self.rig
            .entries
            .iter()
            .find(|e| e.name == universe)
            .map(|e| e.number)
    }
}
