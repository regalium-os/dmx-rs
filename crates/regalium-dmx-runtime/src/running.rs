// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! An output on a thread of its own, and the handle that stops and joins it.

use crate::clock::Clock;
use crate::output::Output;
use regalium_dmx_core::transport::Transport;
use std::thread::JoinHandle;

impl<T, C> Output<T, C>
where
    T: Transport + Send + Sync + 'static,
    C: Clock + 'static,
{
    /// Runs the output on a thread of its own, named for its protocol.
    ///
    /// # Errors
    ///
    /// When the operating system cannot start a thread.
    pub fn start(&self) -> std::io::Result<Running> {
        let output = self.clone();
        let thread = std::thread::Builder::new()
            .name(format!("dmx-output-{}", self.inner.transport.protocol()))
            .spawn(move || output.run())?;
        let output = self.clone();
        Ok(Running {
            close: Box::new(move || output.close()),
            thread: Some(thread),
        })
    }
}

/// An output running on its own thread. Closing, or dropping, stops the
/// loop and joins the thread; no orphan outlives it.
pub struct Running {
    close: Box<dyn Fn() + Send + Sync>,
    thread: Option<JoinHandle<()>>,
}

impl Running {
    /// Stops the loop and waits for its thread. Idempotent.
    pub fn close(&mut self) {
        (self.close)();
        if let Some(thread) = self.thread.take() {
            // A panic inside the loop has already stopped it; there is
            // nothing further to do with the payload.
            let _ = thread.join();
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        self.close();
    }
}
