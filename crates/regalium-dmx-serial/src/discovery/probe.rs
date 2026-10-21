// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Asking a port whether it is a widget: a state machine on an injected
//! clock, so every timing rule is a table test.

use crate::enttec::{self, Framer, Label, Message, Params};
use core::time::Duration;

/// How long a port has to answer. A widget replies within milliseconds; a
/// second allows for a USB bridge that is slow to start.
pub const TIMEOUT: Duration = Duration::from_secs(1);

/// What a widget said about itself.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Identity {
    /// The serial number, when it answered that request. Some compatible
    /// widgets answer only the parameters.
    pub serial: Option<u32>,
    /// Its parameters.
    pub params: Params,
}

/// Where a probe stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// No answer yet; give up at `until`.
    Pending {
        /// When the probe times out.
        until: Duration,
    },
    /// The port answered as a widget.
    Widget(Identity),
    /// The port did not answer as a widget in time. It may be anything.
    NotAWidget,
}

/// One probe of one port. Plain data; `Send` and `Sync`.
#[derive(Clone, Debug)]
pub struct Probe {
    until: Duration,
    framer: Framer,
    serial: Option<u32>,
    params: Option<Params>,
}

impl Probe {
    /// A probe begun at `now`, and the octets to write to the port: Get
    /// Widget Serial Number, then Get Widget Parameters asking for no user
    /// configuration. No DMX.
    #[must_use]
    pub fn start(now: Duration) -> (Self, Vec<u8>) {
        let mut request = Vec::with_capacity(11);
        Message {
            label: Label::GET_SERIAL,
            data: &[],
        }
        .encode(&mut request);
        Message {
            label: Label::GET_PARAMETERS,
            data: &[0, 0],
        }
        .encode(&mut request);
        let probe = Self {
            until: now.saturating_add(TIMEOUT),
            framer: Framer::new(),
            serial: None,
            params: None,
        };
        (probe, request)
    }

    /// Takes octets read from the port at `now`.
    pub fn feed(&mut self, bytes: &[u8], now: Duration) -> Verdict {
        let (serial, params) = (&mut self.serial, &mut self.params);
        self.framer.feed(bytes, |m| match m.label {
            Label::GET_SERIAL => *serial = serial.or(enttec::serial(m.data)),
            Label::GET_PARAMETERS => *params = params.or(Params::decode(m.data)),
            _ => {}
        });
        self.verdict(now)
    }

    /// Where the probe stands at `now`, with nothing new read. A widget is
    /// declared as soon as both replies are in; at the timeout, the
    /// parameters alone suffice.
    #[must_use]
    pub fn verdict(&self, now: Duration) -> Verdict {
        match (self.params, self.serial) {
            (Some(params), Some(serial)) => Verdict::Widget(Identity {
                serial: Some(serial),
                params,
            }),
            (Some(params), None) if now >= self.until => Verdict::Widget(Identity {
                serial: None,
                params,
            }),
            (None, _) if now >= self.until => Verdict::NotAWidget,
            _ => Verdict::Pending { until: self.until },
        }
    }
}
