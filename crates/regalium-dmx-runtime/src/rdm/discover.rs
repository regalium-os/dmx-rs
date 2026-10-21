// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Discovery on a line: the core's search, driven over the line.

use super::Controller;
use crate::clock::Clock;
use regalium_dmx_core::rdm::{
    CommandClass, Discovery, MAX_LEN, Next, Pid, Request, Uid, branch_data,
};
use regalium_dmx_core::transport::{Answer, Expect, RdmLine};
use regalium_dmx_telemetry::shape::{self, UniverseId};
use regalium_dmx_telemetry::{Attribute, SpanId, keys};

/// What a discovery found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Discovered {
    /// Every device, in UID order.
    pub devices: Vec<Uid>,
    /// Whether the search ran out of steps first: a noisy line.
    pub incomplete: bool,
}

impl<L: RdmLine, C: Clock> Controller<L, C> {
    /// Finds every device on `universe`'s line: by the line's own table
    /// where it keeps one, as an Art-Net node does, or by searching. Records
    /// the count on `dmx.rdm.discovery.devices`.
    ///
    /// # Errors
    ///
    /// When the line itself failed; silence and collisions are the search's
    /// to handle.
    pub fn discover(&self, universe: u32) -> Result<Discovered, L::Error> {
        let span = self.tracer.start("dmx.rdm.discovery", SpanId::NONE);
        self.tracer
            .attribute(span, Attribute::int(keys::UNIVERSE, universe.into()));
        let result = match self.line.table(universe) {
            Ok(Some(mut devices)) => {
                devices.sort_unstable();
                devices.dedup();
                Ok(Discovered {
                    devices,
                    incomplete: false,
                })
            }
            Ok(None) => self.search(universe),
            Err(e) => Err(e),
        };
        if let Ok(found) = &result {
            let id = UniverseId::new(self.line.protocol(), universe);
            let count = u32::try_from(found.devices.len()).unwrap_or(u32::MAX);
            shape::record_rdm_discovered(self.meter.as_ref(), &id, count);
        }
        let error = result.as_ref().err().map(ToString::to_string);
        self.tracer
            .end(span, error.as_ref().map(|e| e as &dyn std::fmt::Display));
        result
    }

    fn search(&self, universe: u32) -> Result<Discovered, L::Error> {
        let mut discovery = Discovery::new();
        let mut buf = [0u8; MAX_LEN];
        loop {
            match discovery.next() {
                Next::Broadcast(request) => {
                    self.send(universe, &request, Expect::Nothing, &mut buf)?;
                    discovery.sent();
                }
                Next::Branch { lower, upper } => {
                    let data = branch_data(lower, upper);
                    let request = Request {
                        destination: Uid::ALL,
                        sub_device: 0,
                        command_class: CommandClass::DISCOVERY,
                        pid: Pid::DISC_UNIQUE_BRANCH,
                        data: &data,
                    };
                    let (_, answer) = self.send(universe, &request, Expect::Discovery, &mut buf)?;
                    discovery.branched(heard(answer, &buf));
                }
                Next::Mute(uid) => {
                    let request = Request {
                        destination: uid,
                        sub_device: 0,
                        command_class: CommandClass::DISCOVERY,
                        pid: Pid::DISC_MUTE,
                        data: &[],
                    };
                    let (_, answer) = self.send(universe, &request, Expect::Message, &mut buf)?;
                    discovery.muted(heard(answer, &buf));
                }
                Next::Done => {
                    return Ok(Discovered {
                        devices: discovery.found(),
                        incomplete: discovery.incomplete(),
                    });
                }
            }
        }
    }
}

fn heard(answer: Answer, buf: &[u8]) -> Option<&[u8]> {
    match answer {
        Answer::Silent => None,
        Answer::Octets(n) => buf.get(..n),
    }
}
