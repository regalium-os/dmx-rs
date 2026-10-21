// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Why an RDM request did not get its answer.

use regalium_dmx_core::rdm::NackReason;
use std::fmt;

/// What went wrong, after the controller's retries.
#[derive(Debug)]
#[non_exhaustive]
pub enum RdmError<E> {
    /// The line itself failed.
    Line(E),
    /// No answer, every try: the device is gone, or not on this line.
    NoAnswer,
    /// Answers came, but none decoded as the answer to this request.
    Garbled,
    /// The device refused, and said why.
    Nack(NackReason),
    /// More `ACK_OVERFLOW` parts than any parameter needs.
    Overflow,
    /// `ACK_TIMER`, then the queue never held the answer.
    Queued,
    /// An answer whose data is not what the parameter defines.
    Malformed,
}

impl<E> RdmError<E> {
    /// The outcome as `dmx.outcome` carries it.
    #[must_use]
    pub const fn outcome(&self) -> &'static str {
        match self {
            Self::Line(_) => "error",
            Self::NoAnswer => "timeout",
            Self::Garbled | Self::Malformed => "garbled",
            Self::Nack(_) => "nack",
            Self::Overflow | Self::Queued => "incomplete",
        }
    }
}

impl<E: fmt::Display> fmt::Display for RdmError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line(e) => write!(f, "RDM line: {e}"),
            Self::NoAnswer => f.write_str("no answer"),
            Self::Garbled => f.write_str("no answer decoded"),
            Self::Nack(reason) => match reason.name() {
                Some(name) => write!(f, "refused: {name}"),
                None => write!(f, "refused: reason {:#06x}", reason.0),
            },
            Self::Overflow => f.write_str("too many overflow parts"),
            Self::Queued => f.write_str("the queued answer never came"),
            Self::Malformed => f.write_str("answer data malformed"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for RdmError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Line(e) => Some(e),
            _ => None,
        }
    }
}
