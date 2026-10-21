// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Messages out of a byte stream, as they arrive from the widget a few
//! octets at a time.
//!
//! Resynchronising: octets before a start delimiter are skipped, and a
//! message whose end delimiter is missing is dropped and the search starts
//! again one octet after its start, so line noise or a reply cut by a
//! reconnect costs that message and no more. Bounded: it never holds more
//! than one maximum message.

use super::message::{END, MAX_DATA, Message, OVERHEAD, START};

/// The most a framer buffers: one message of the largest size.
pub const CAPACITY: usize = MAX_DATA + OVERHEAD;

/// The stream's state. Plain data; `Send` and `Sync`.
#[derive(Clone, Debug, Default)]
pub struct Framer {
    buf: Vec<u8>,
    skipped: u64,
}

impl Framer {
    /// An empty framer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buf: Vec::new(),
            skipped: 0,
        }
    }

    /// Octets thrown away so far while resynchronising.
    #[must_use]
    pub const fn skipped(&self) -> u64 {
        self.skipped
    }

    /// Takes `bytes` and calls `each` with every whole message they
    /// complete, in order. A message borrows the framer only for the call.
    /// Between calls, the framer holds less than [`CAPACITY`] octets.
    pub fn feed(&mut self, bytes: &[u8], mut each: impl FnMut(Message<'_>)) {
        self.buf.extend_from_slice(bytes);
        loop {
            // Up to the next start delimiter.
            let lead = self
                .buf
                .iter()
                .position(|&b| b == START)
                .unwrap_or(self.buf.len());
            self.skip(lead);
            let Some(&[_, _, lo, hi]) = self.buf.get(..4) else {
                return;
            };
            let len = usize::from(u16::from_le_bytes([lo, hi]));
            if len > MAX_DATA {
                self.skip(1);
                continue;
            }
            let need = OVERHEAD + len;
            match self.buf.get(need - 1) {
                None => return,
                Some(&END) => {
                    if let Some(Ok(message)) = self.buf.get(..need).map(Message::decode) {
                        each(message);
                    }
                    self.buf.drain(..need);
                }
                // The start delimiter led nowhere: look again after it.
                Some(_) => self.skip(1),
            }
        }
    }

    fn skip(&mut self, n: usize) {
        let n = n.min(self.buf.len());
        self.buf.drain(..n);
        self.skipped += n as u64;
    }
}
