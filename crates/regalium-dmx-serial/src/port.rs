// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A serial port, raw: no line discipline, no echo, and every read and
//! write bounded by a timeout.
//!
//! Through `rustix`, which calls the operating system directly and links
//! nothing, so the deps gate holds: no libudev, no C. Unix only; Windows
//! ports arrive later.
//!
//! # Concurrency
//!
//! `Send` and `Sync`. One thread may read while another writes; two
//! writers would interleave their octets, so the transports above hold a
//! lock around each message.

use rustix::event::{PollFd, PollFlags, Timespec, poll};
use rustix::fd::OwnedFd;
use rustix::fs::{Mode, OFlags, open};
use rustix::io::Errno;
use rustix::termios::{
    ControlModes, OptionalActions, QueueSelector, tcdrain, tcflush, tcgetattr, tcsetattr,
};
use std::io;
use std::time::{Duration, Instant};

/// How the line is framed: data bits are always eight, parity none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Framing {
    /// The baud rate.
    pub baud: u32,
    /// Two stop bits, as DMX512 uses, or one.
    pub two_stop: bool,
}

/// An open serial port.
#[derive(Debug)]
pub struct Port {
    fd: OwnedFd,
    path: String,
}

impl Port {
    /// Opens `path` without making it the controlling terminal, in raw
    /// mode, with `framing`, and discards anything already received.
    ///
    /// # Errors
    ///
    /// When it cannot be opened, or is not a serial port.
    pub fn open(path: &str, framing: Framing) -> io::Result<Self> {
        let flags = OFlags::RDWR | OFlags::NOCTTY | OFlags::NONBLOCK | OFlags::CLOEXEC;
        let fd = open(path, flags, Mode::empty())?;
        let port = Self {
            fd,
            path: path.to_owned(),
        };
        port.frame(framing)?;
        tcflush(&port.fd, QueueSelector::IFlush)?;
        Ok(port)
    }

    /// The path it was opened by.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Changes the framing, after what has been written has gone out.
    ///
    /// # Errors
    ///
    /// When the driver refuses the rate or the settings.
    pub fn frame(&self, framing: Framing) -> io::Result<()> {
        let mut termios = tcgetattr(&self.fd)?;
        termios.make_raw();
        termios.control_modes |= ControlModes::CLOCAL | ControlModes::CREAD;
        termios
            .control_modes
            .set(ControlModes::CSTOPB, framing.two_stop);
        termios.set_speed(framing.baud)?;
        tcsetattr(&self.fd, OptionalActions::Drain, &termios)?;
        Ok(())
    }

    /// Waits until everything written has been transmitted.
    ///
    /// # Errors
    ///
    /// When the port failed.
    pub fn drain(&self) -> io::Result<()> {
        Ok(tcdrain(&self.fd)?)
    }

    /// Reads what has arrived into `buf`, waiting at most `timeout` for
    /// something to. `Ok(0)` when nothing came.
    ///
    /// # Errors
    ///
    /// When the port failed, or was unplugged.
    pub fn read(&self, buf: &mut [u8], timeout: Duration) -> io::Result<usize> {
        if !self.wait(PollFlags::IN, timeout)? {
            return Ok(0);
        }
        match rustix::io::read(&self.fd, buf) {
            Ok(0) => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "port closed")),
            Ok(n) => Ok(n),
            Err(Errno::AGAIN | Errno::INTR) => Ok(0),
            Err(e) => Err(e.into()),
        }
    }

    /// Writes all of `bytes`, waiting at most `timeout` in all.
    ///
    /// # Errors
    ///
    /// [`io::ErrorKind::TimedOut`] when the port would not take them in
    /// time, as a widget that stopped reading; otherwise the port's error.
    pub fn write_all(&self, mut bytes: &[u8], timeout: Duration) -> io::Result<()> {
        let deadline = Instant::now() + timeout;
        while !bytes.is_empty() {
            match rustix::io::write(&self.fd, bytes) {
                Ok(n) => bytes = bytes.get(n..).unwrap_or_default(),
                Err(Errno::AGAIN | Errno::INTR) => {
                    let left = deadline.saturating_duration_since(Instant::now());
                    if left.is_zero() || !self.wait(PollFlags::OUT, left)? {
                        return Err(io::ErrorKind::TimedOut.into());
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    /// Whether the port became ready for `flags` within `timeout`. A hang-up
    /// or error counts as ready, so the read or write that follows reports it.
    fn wait(&self, flags: PollFlags, timeout: Duration) -> io::Result<bool> {
        let timeout = Timespec {
            tv_sec: i64::try_from(timeout.as_secs()).unwrap_or(i64::MAX),
            tv_nsec: i64::from(timeout.subsec_nanos()),
        };
        let mut fds = [PollFd::new(&self.fd, flags)];
        match poll(&mut fds, Some(&timeout)) {
            Ok(n) => Ok(n > 0),
            Err(Errno::INTR) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}
