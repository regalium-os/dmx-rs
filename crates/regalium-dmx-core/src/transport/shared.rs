// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! One transport behind an `Arc`, shared by an output and an input.

use super::{Received, Transport};
use crate::frame::Frame;
use crate::refresh::Refresh;
use core::time::Duration;
use std::sync::Arc;

/// A shared transport is a transport, so one socket can feed an output and
/// an input at once: each holds an `Arc` of it.
impl<T: Transport + ?Sized> Transport for Arc<T> {
    type Error = T::Error;

    fn protocol(&self) -> &'static str {
        (**self).protocol()
    }

    fn refresh(&self) -> Refresh {
        (**self).refresh()
    }

    fn source_timeout(&self) -> Duration {
        (**self).source_timeout()
    }

    fn join(&self, universe: u32) -> Result<(), Self::Error> {
        (**self).join(universe)
    }

    fn leave(&self, universe: u32) -> Result<(), Self::Error> {
        (**self).leave(universe)
    }

    fn end(&self, universe: u32) -> Result<(), Self::Error> {
        (**self).end(universe)
    }

    fn send(&self, universe: u32, frame: &Frame<'_>) -> Result<(), Self::Error> {
        (**self).send(universe, frame)
    }

    fn receive(&self, buf: &mut [u8], timeout: Duration) -> Result<Option<Received>, Self::Error> {
        (**self).receive(buf, timeout)
    }
}
