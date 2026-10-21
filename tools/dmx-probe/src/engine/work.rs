// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM work, done on a thread of its own so a search of several seconds
//! never stalls the model, and its results folded back in.

use super::model::Severity;
use super::rdm::device;
use super::{Done, Worker};
use regalium_dmx::rdm::{Pid, Uid};

impl Worker {
    pub(crate) fn discover(&mut self, universe: String) {
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

    pub(crate) fn rdm_set(
        &mut self,
        universe: String,
        uid: &str,
        pid: Pid,
        data: Vec<u8>,
        what: String,
    ) {
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
}
