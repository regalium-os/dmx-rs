// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A transport whose receives return what the test queued, in order: a
//! frame, a failure, or nothing once the queue is empty.

use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::merge::SourceId;
use regalium_dmx_core::transport::{Received, Transport};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;

/// What the next receive returns: a frame, or a failure.
pub(crate) enum Next {
    Frame(Received, Vec<u8>),
    Fail,
}

/// The queue, and every universe joined (`true`) or left (`false`).
#[derive(Default)]
pub(crate) struct Script {
    queue: Mutex<VecDeque<Next>>,
    pub(crate) membership: Mutex<Vec<(bool, u32)>>,
}

#[derive(Debug)]
pub(crate) struct Down;

impl std::fmt::Display for Down {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("network down")
    }
}

impl std::error::Error for Down {}

impl Transport for Script {
    type Error = Down;

    fn protocol(&self) -> &'static str {
        "artnet"
    }

    fn join(&self, universe: u32) -> Result<(), Down> {
        self.membership.lock().unwrap().push((true, universe));
        Ok(())
    }

    fn leave(&self, universe: u32) -> Result<(), Down> {
        self.membership.lock().unwrap().push((false, universe));
        Ok(())
    }

    fn send(&self, _: u32, _: &Frame<'_>) -> Result<(), Down> {
        Ok(())
    }

    fn receive(&self, buf: &mut [u8], _: Duration) -> Result<Option<Received>, Down> {
        match self.queue.lock().unwrap().pop_front() {
            None => Ok(None),
            Some(Next::Fail) => Err(Down),
            Some(Next::Frame(got, octets)) => {
                buf[..octets.len()].copy_from_slice(&octets);
                Ok(Some(got))
            }
        }
    }
}

impl Script {
    /// Queues `octets` for `universe` from `source`, at the default priority.
    pub(crate) fn frame(&self, universe: u32, source: u64, octets: &[u8]) {
        self.push(
            Received::new(universe, SourceId(source), octets.len()),
            octets,
        );
    }

    /// Queues `octets` as `got` describes them.
    pub(crate) fn push(&self, got: Received, octets: &[u8]) {
        let next = Next::Frame(got, octets.to_vec());
        self.queue.lock().unwrap().push_back(next);
    }

    /// Queues a failed receive.
    pub(crate) fn fail(&self) {
        self.queue.lock().unwrap().push_back(Next::Fail);
    }
}
