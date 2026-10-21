// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Finding every device on a line: E1.20's binary search, as a state
//! machine. It says what to send next; the caller sends it and hands back
//! what came back. No clock: the line's port owns the response window.
//!
//! Un-mute everyone, then search the whole UID range. A branch with no
//! answer is empty. A clean answer is one device: mute it, so it stops
//! answering, and search the same branch again. A garbled answer is
//! several at once: split the branch in two and search each.
//!
//! A device that will not mute would answer every search forever, so after
//! three tries it is recorded and the branch searched around it. The whole
//! search is bounded too, so a line full of noise ends, flagged incomplete.

use super::codes::{CommandClass, Pid};
use super::disc;
use super::message::Message;
use super::request::Request;
use super::uid::Uid;
use std::collections::BTreeSet;

/// Mute attempts before a device is taken as one that will not mute.
pub const MUTE_TRIES: u8 = 3;

/// Steps after which a search gives up, flagged incomplete: far more than
/// the ~1 500 a full line of 512 devices needs.
pub const MAX_STEPS: u32 = 100_000;

/// What the caller does next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    /// Send this, broadcast, expecting nothing back; then call
    /// [`Discovery::sent`].
    Broadcast(Request<'static>),
    /// Send `DISC_UNIQUE_BRANCH` for this range; hand back any octets with
    /// [`Discovery::branched`].
    Branch {
        /// The lowest UID in the branch.
        lower: Uid,
        /// The highest.
        upper: Uid,
    },
    /// Send `DISC_MUTE` to this device; hand back its response with
    /// [`Discovery::muted`].
    Mute(Uid),
    /// Done.
    Done,
}

/// A search in progress. Plain data; `Send` and `Sync`.
#[derive(Clone, Debug)]
pub struct Discovery {
    branches: Vec<(Uid, Uid)>,
    found: BTreeSet<Uid>,
    muting: Option<(Uid, u8)>,
    unmuted: bool,
    steps: u32,
}

impl Default for Discovery {
    fn default() -> Self {
        Self::new()
    }
}

impl Discovery {
    /// A full search: un-mute, then the whole range.
    #[must_use]
    pub fn new() -> Self {
        Self {
            branches: vec![(Uid::MIN, Uid::MAX)],
            found: BTreeSet::new(),
            muting: None,
            unmuted: false,
            steps: 0,
        }
    }

    /// What to do next.
    #[must_use]
    pub fn next(&self) -> Next {
        if !self.unmuted {
            let unmute = Request {
                destination: Uid::ALL,
                sub_device: 0,
                command_class: CommandClass::DISCOVERY,
                pid: Pid::DISC_UN_MUTE,
                data: &[],
            };
            return Next::Broadcast(unmute);
        }
        if self.steps >= MAX_STEPS {
            return Next::Done;
        }
        match (self.muting, self.branches.last()) {
            (Some((uid, _)), _) => Next::Mute(uid),
            (None, Some(&(lower, upper))) => Next::Branch { lower, upper },
            (None, None) => Next::Done,
        }
    }

    /// The broadcast went out.
    pub fn sent(&mut self) {
        self.unmuted = true;
    }

    /// What came back for the current branch: nothing, or octets.
    pub fn branched(&mut self, answer: Option<&[u8]>) {
        self.steps += 1;
        let Some(&(lower, upper)) = self.branches.last() else {
            return;
        };
        match answer.map(disc::decode) {
            None => {
                self.branches.pop();
            }
            Some(Ok(uid)) if (lower..=upper).contains(&uid) && !self.found.contains(&uid) => {
                self.muting = Some((uid, 0));
            }
            // Garbled, out of range, or a device answering again although
            // muted: more than one answered, or something is wrong. Split.
            Some(_) => {
                self.branches.pop();
                if lower < upper {
                    let mid = Uid::from_u64(lower.get() + (upper.get() - lower.get()) / 2);
                    self.branches.push((Uid::from_u64(mid.get() + 1), upper));
                    self.branches.push((lower, mid));
                }
            }
        }
    }

    /// What came back for the current mute: the device's response, if any.
    pub fn muted(&mut self, response: Option<&[u8]>) {
        self.steps += 1;
        let Some((uid, tries)) = self.muting else {
            return;
        };
        let ok = response
            .and_then(|r| Message::decode(r).ok())
            .is_some_and(|m| {
                m.source == uid
                    && m.command_class == CommandClass::DISCOVERY_RESPONSE
                    && m.pid == Pid::DISC_MUTE
            });
        if ok {
            self.found.insert(uid);
            self.muting = None;
        } else if tries + 1 < MUTE_TRIES {
            self.muting = Some((uid, tries + 1));
        } else {
            // It will not mute: keep it, and search either side of it.
            self.found.insert(uid);
            self.muting = None;
            if let Some((lower, upper)) = self.branches.pop() {
                if uid < upper {
                    self.branches.push((Uid::from_u64(uid.get() + 1), upper));
                }
                if lower < uid {
                    self.branches.push((lower, Uid::from_u64(uid.get() - 1)));
                }
            }
        }
    }

    /// Every device found so far, in UID order.
    #[must_use]
    pub fn found(&self) -> Vec<Uid> {
        self.found.iter().copied().collect()
    }

    /// Whether the search ran out of steps before covering every branch.
    #[must_use]
    pub fn incomplete(&self) -> bool {
        self.steps >= MAX_STEPS && !self.branches.is_empty()
    }
}

/// `DISC_UNIQUE_BRANCH`'s request for `lower..=upper`: broadcast, both
/// bounds as data.
#[must_use]
pub fn branch_data(lower: Uid, upper: Uid) -> [u8; 12] {
    let mut data = [0u8; 12];
    for (to, from) in data
        .iter_mut()
        .zip(lower.octets().into_iter().chain(upper.octets()))
    {
        *to = from;
    }
    data
}
