// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The live engine: a worker thread that owns the rig — outputs, inputs,
//! RDM controllers — takes [`Command`]s, and publishes a [`Model`] ten times
//! a second.
//!
//! A surface never touches the runtime. It sends a command and draws the
//! latest model, so a slow terminal never delays a frame, and the server
//! (Phase 9) drives the same engine the TUI does.

mod apply;
mod capture;
mod command;
mod feed;
pub mod model;
pub mod patch;
mod rdm;
mod snapshot;
mod stats;
mod work;

pub use capture::Capture;
pub use command::Command;
pub use model::Model;
pub use rdm::RdmPort;
pub use stats::{KEPT, Stats};

use model::{Device, Event, Node, Protocol};
use regalium_dmx::runtime::{Events, Levels, Merged, Policy, Running};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

/// How often the model is published.
pub const TICK: Duration = Duration::from_millis(100);

/// One universe the rig sends or merges.
pub struct Entry {
    /// Its resource ID.
    pub name: String,
    /// Its transport.
    pub protocol: Protocol,
    /// Its number, as the transport numbers it.
    pub number: u32,
    /// Its number as printed: `0:0:1`.
    pub label: String,
    /// An output's levels.
    pub levels: Option<Levels>,
    /// An input's merge, and its mode's name.
    pub merged: Option<(Merged, &'static str)>,
    /// Where an output goes.
    pub target: String,
}

/// Everything the engine owns. Built by [`crate::sim::demo`] or from a live
/// network.
pub struct Rig {
    /// The universes, in the order shown.
    pub entries: Vec<Entry>,
    /// The policy every output shares.
    pub policy: Arc<Policy>,
    /// Frame statistics, fed by every output's and input's meter.
    pub stats: Arc<Stats>,
    /// Packets seen.
    pub capture: Arc<Capture>,
    /// Each output's and input's events.
    pub events: Vec<Events>,
    /// RDM, by universe name.
    pub rdm: HashMap<String, Arc<dyn RdmPort>>,
    /// Nodes found.
    pub nodes: Vec<Node>,
    /// Threads that run while the rig lives: outputs, inputs, simulations.
    pub running: Vec<Running>,
    /// Threads the simulation runs, stopped with the rig.
    pub background: Vec<std::thread::JoinHandle<()>>,
    /// Set when the rig stops, for those threads to see.
    pub stop: Arc<AtomicBool>,
    /// What the probe is attached to.
    pub attached: String,
}

/// The surface's end of the engine.
pub struct Engine {
    commands: Sender<Command>,
    models: Receiver<Model>,
    latest: Option<Model>,
}

impl Engine {
    /// Starts the worker over `rig`.
    ///
    /// # Errors
    ///
    /// When the operating system cannot start a thread.
    pub fn start(rig: Rig) -> std::io::Result<Self> {
        let (commands, inbox) = mpsc::channel();
        let (publish, models) = mpsc::channel();
        std::thread::Builder::new()
            .name("dmx-engine".into())
            .spawn(move || Worker::new(rig).run(&inbox, &publish))?;
        Ok(Self {
            commands,
            models,
            latest: None,
        })
    }

    /// Asks for `command`. Never blocks.
    pub fn send(&self, command: Command) {
        let _ = self.commands.send(command);
    }

    /// The newest model, if one has been published.
    pub fn model(&mut self) -> Option<&Model> {
        while let Ok(model) = self.models.try_recv() {
            self.latest = Some(model);
        }
        self.latest.as_ref()
    }
}

/// What finished on an RDM thread.
pub(crate) enum Done {
    Found {
        universe: String,
        devices: Vec<Device>,
        line: String,
    },
    Changed {
        universe: String,
        device: Option<Device>,
        line: String,
        ok: bool,
    },
}

/// The worker: the rig, and what it has learnt.
pub(crate) struct Worker {
    pub(crate) rig: Rig,
    pub(crate) devices: Vec<Device>,
    pub(crate) feed: VecDeque<Event>,
    pub(crate) discovery: Option<String>,
    /// Universes whose transport is failing now, by name.
    pub(crate) failing: HashSet<String>,
    pub(crate) done: (Sender<Done>, Receiver<Done>),
}

impl Worker {
    fn new(rig: Rig) -> Self {
        Self {
            rig,
            devices: Vec::new(),
            feed: VecDeque::new(),
            discovery: None,
            failing: HashSet::new(),
            done: mpsc::channel(),
        }
    }

    /// Until the surface goes away.
    fn run(mut self, inbox: &Receiver<Command>, publish: &Sender<Model>) {
        loop {
            match inbox.recv_timeout(TICK) {
                Ok(command) => self.apply(command),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
            while let Ok(command) = inbox.try_recv() {
                self.apply(command);
            }
            while let Ok(done) = self.done.1.try_recv() {
                self.finished(done);
            }
            self.events();
            if publish.send(self.snapshot()).is_err() {
                break;
            }
        }
        self.rig.stop.store(true, Ordering::Release);
        for mut running in self.rig.running.drain(..) {
            running.close();
        }
        for thread in self.rig.background.drain(..) {
            let _ = thread.join();
        }
    }
}
