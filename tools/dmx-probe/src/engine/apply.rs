// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Commands, applied; RDM work, done on a thread of its own so a search of
//! several seconds never stalls the model; and runtime events, as feed
//! lines.

use super::model::{EVENTS, Event, Severity};
use super::rdm::device;
use super::{Command, Done, Worker};
use regalium_dmx::address::Address;
use regalium_dmx::rdm::{Pid, Uid};
use regalium_dmx::runtime::Event as RuntimeEvent;

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

    fn number(&self, universe: &str) -> Option<u32> {
        self.rig
            .entries
            .iter()
            .find(|e| e.name == universe)
            .map(|e| e.number)
    }

    fn discover(&mut self, universe: String) {
        let (Some(port), Some(number)) =
            (self.rig.rdm.get(&universe).cloned(), self.number(&universe))
        else {
            self.note(
                "refused",
                &format!("{universe}: no RDM line"),
                Severity::Bad,
            );
            return;
        };
        self.discovery = Some(format!("{universe} · searching…"));
        let done = self.done.0.clone();
        std::thread::spawn(move || {
            let started = std::time::Instant::now();
            let found = port.discover(number);
            let message = match found {
                Ok(uids) => {
                    let devices = uids
                        .iter()
                        .filter_map(|&uid| {
                            port.info(number, uid)
                                .ok()
                                .map(|i| device(&universe, uid, &i, port.label(number, uid)))
                        })
                        .collect::<Vec<_>>();
                    let line = format!(
                        "{universe} · {} found · {:.1} s",
                        devices.len(),
                        started.elapsed().as_secs_f32()
                    );
                    Done::Found {
                        universe,
                        devices,
                        line,
                    }
                }
                Err(e) => Done::Found {
                    universe: universe.clone(),
                    devices: Vec::new(),
                    line: format!("{universe} · failed: {e}"),
                },
            };
            let _ = done.send(message);
        });
    }

    fn rdm_set(&mut self, universe: String, uid: &str, pid: Pid, data: Vec<u8>, what: String) {
        let parsed = uid.split_once(':').and_then(|(m, d)| {
            Some(Uid::new(
                u16::from_str_radix(m, 16).ok()?,
                u32::from_str_radix(d, 16).ok()?,
            ))
        });
        let (Some(port), Some(number), Some(uid)) = (
            self.rig.rdm.get(&universe).cloned(),
            self.number(&universe),
            parsed,
        ) else {
            self.note(
                "refused",
                &format!("{universe}: no RDM line, or no such device"),
                Severity::Bad,
            );
            return;
        };
        let done = self.done.0.clone();
        std::thread::spawn(move || {
            let result = port.set(number, uid, pid, &data);
            // Read back, so what is shown is what the device kept.
            let device = port
                .info(number, uid)
                .ok()
                .map(|i| device(&universe, uid, &i, port.label(number, uid)));
            let (line, ok) = match result {
                Ok(()) => (format!("{uid} · {what}"), true),
                Err(e) => (format!("{uid} · {what} refused: {e}"), false),
            };
            let _ = done.send(Done::Changed {
                universe,
                device,
                line,
                ok,
            });
        });
    }

    pub(crate) fn finished(&mut self, done: Done) {
        match done {
            Done::Found {
                universe,
                devices,
                line,
            } => {
                self.devices.retain(|d| d.universe != universe);
                self.devices.extend(devices);
                self.note("rdm", &line, Severity::Info);
                self.discovery = Some(line);
            }
            Done::Changed {
                universe,
                device,
                line,
                ok,
            } => {
                if let Some(device) = device {
                    match self
                        .devices
                        .iter_mut()
                        .find(|d| d.universe == universe && d.uid == device.uid)
                    {
                        Some(known) => *known = device,
                        None => self.devices.push(device),
                    }
                }
                self.note(
                    "rdm set",
                    &line,
                    if ok { Severity::Good } else { Severity::Bad },
                );
            }
        }
    }

    /// Every runtime event, as a line of the feed.
    pub(crate) fn events(&mut self) {
        let mut lines = Vec::new();
        for events in &self.rig.events {
            while let Some(event) = events.try_next() {
                lines.push(event);
            }
        }
        for event in lines {
            let name = |protocol: &str, universe: u32| {
                self.rig
                    .entries
                    .iter()
                    .find(|e| e.protocol.name() == protocol && e.number == universe)
                    .map_or_else(|| format!("{protocol} {universe}"), |e| e.name.clone())
            };
            let (kind, text, severity) = match event {
                RuntimeEvent::SendFailed {
                    protocol,
                    universe,
                    error,
                } => {
                    self.failing.insert(name(protocol, universe));
                    (
                        "send failed",
                        format!("{} · {error}", name(protocol, universe)),
                        Severity::Bad,
                    )
                }
                RuntimeEvent::SendRecovered { protocol, universe } => {
                    self.failing.remove(&name(protocol, universe));
                    ("send recovered", name(protocol, universe), Severity::Good)
                }
                RuntimeEvent::ReceiveFailed { protocol, error } => (
                    "receive failed",
                    format!("{protocol} · {error}"),
                    Severity::Bad,
                ),
                RuntimeEvent::ReceiveRecovered { protocol } => {
                    ("receive recovered", protocol.to_owned(), Severity::Good)
                }
                RuntimeEvent::SourceFound {
                    protocol,
                    universe,
                    source,
                } => (
                    "source found",
                    format!("{} · {:x}", name(protocol, universe), source.0),
                    Severity::Good,
                ),
                RuntimeEvent::SourceLost {
                    protocol,
                    universe,
                    source,
                } => (
                    "source lost",
                    format!("{} · {:x}", name(protocol, universe), source.0),
                    Severity::Warn,
                ),
                other => ("event", format!("{other:?}"), Severity::Info),
            };
            self.note(kind, &text, severity);
        }
    }

    /// Adds a line to the feed, newest first.
    pub(crate) fn note(&mut self, kind: &str, text: &str, severity: Severity) {
        if self.feed.len() == EVENTS {
            self.feed.pop_back();
        }
        let time = self
            .rig
            .capture
            .time()
            .get(..8)
            .unwrap_or_default()
            .to_owned();
        self.feed.push_front(Event {
            time,
            kind: kind.to_owned(),
            text: text.to_owned(),
            severity,
        });
    }
}
