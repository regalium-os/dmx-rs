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
    Frame {
        universe: u32,
        source: u64,
        octets: Vec<u8>,
    },
    Fail,
}

#[derive(Default)]
pub(crate) struct Script(pub(crate) Mutex<VecDeque<Next>>);

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

    fn send(&self, _: u32, _: &Frame<'_>) -> Result<(), Down> {
        Ok(())
    }

    fn receive(&self, buf: &mut [u8], _: Duration) -> Result<Option<Received>, Down> {
        match self.0.lock().unwrap().pop_front() {
            None => Ok(None),
            Some(Next::Fail) => Err(Down),
            Some(Next::Frame {
                universe,
                source,
                octets,
            }) => {
                buf[..octets.len()].copy_from_slice(&octets);
                Ok(Some(Received {
                    universe,
                    source: SourceId(source),
                    len: octets.len(),
                }))
            }
        }
    }
}

impl Script {
    pub(crate) fn frame(&self, universe: u32, source: u64, octets: &[u8]) {
        let octets = octets.to_vec();
        self.0.lock().unwrap().push_back(Next::Frame {
            universe,
            source,
            octets,
        });
    }
}
