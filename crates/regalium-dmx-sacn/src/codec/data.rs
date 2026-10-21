// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The E1.31 data packet: one universe's start code and slots.
//!
//! After the root layer, a framing layer — source name, priority, sync
//! address, sequence number, options, universe — and a DMP layer whose
//! property values are the start code then the slots. Unlike `ArtDmx`, the
//! start code is carried, so alternate start codes travel too.

use super::cid::Cid;
use super::error::{Error, ErrorKind};
use super::root::{self, FRAMING, VECTOR_DATA, be16};
use super::text::{self, NAME};
use regalium_dmx_core::address::SLOTS;
use regalium_dmx_core::frame::Frame;
use regalium_dmx_core::start_code::StartCode;

/// `VECTOR_E131_DATA_PACKET`.
pub const VECTOR: u32 = 0x0000_0002;
/// Option: preview data, for visualisers; a dimmer ignores it.
pub const PREVIEW: u8 = 0x80;
/// Option: the source is stopping this universe.
pub const TERMINATED: u8 = 0x40;
/// Option: hold output while synchronisation is lost.
pub const FORCE_SYNC: u8 = 0x20;

const NAME_AT: usize = FRAMING + 6;
const PRIORITY: usize = NAME_AT + NAME;
const DMP: usize = PRIORITY + 7;
const VALUES: usize = DMP + 10;
/// The DMP layer's fixed octets after its flags and length: vector
/// `VECTOR_DMP_SET_PROPERTY`, address and data type 0xa1, first property
/// address 0, address increment 1.
const DMP_FIXED: [u8; 6] = [0x02, 0xa1, 0x00, 0x00, 0x00, 0x01];

/// An E1.31 data packet. Borrows the name and the slots.
///
/// `Copy`, `Send` and `Sync`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataPacket<'a> {
    /// The source.
    pub cid: Cid,
    /// The source's name: 64 octets on decode, NUL-padded; cut and padded
    /// to 64 on encode.
    pub source_name: &'a [u8],
    /// 0 to 200; 100 is the default. Receivers merge only the highest.
    pub priority: u8,
    /// The universe whose sync packets release this data; 0 for none.
    pub sync_address: u16,
    /// Numbered per universe, 0 to 255 and round again.
    pub sequence: u8,
    /// [`PREVIEW`], [`TERMINATED`], [`FORCE_SYNC`], kept whole.
    pub options: u8,
    /// The universe, 1 to 63 999.
    pub universe: u16,
    /// The start code, null or alternate.
    pub start_code: StartCode,
    /// The slots after it, at most 512.
    pub slots: &'a [u8],
}

impl<'a> DataPacket<'a> {
    pub(crate) fn decode(d: &'a [u8], cid: Cid) -> Result<Self, Error<'a>> {
        if d.len() < VALUES {
            return Err(Error::truncated(d, VALUES));
        }
        root::pdu(d, FRAMING)?;
        root::pdu(d, DMP)?;
        if d.get(DMP + 2..DMP + 8) != Some(&DMP_FIXED[..]) {
            return Err(Error::at(ErrorKind::Dmp, d, DMP + 2));
        }
        let count = usize::from(be16(d, DMP + 8).unwrap_or_default());
        let held = d.len() - VALUES;
        if count != held {
            let kind = ErrorKind::Length {
                said: count,
                actual: held,
            };
            return Err(Error::at(kind, d, DMP + 8));
        }
        let Some((&code, slots)) = d.get(VALUES..).and_then(<[u8]>::split_first) else {
            return Err(Error::truncated(d, VALUES + 1));
        };
        if slots.len() > SLOTS {
            return Err(Error::at(
                ErrorKind::TooLong { slots: slots.len() },
                d,
                VALUES + 1 + SLOTS,
            ));
        }
        let octet = |at: usize| d.get(at).copied().unwrap_or_default();
        Ok(Self {
            cid,
            source_name: d.get(NAME_AT..PRIORITY).unwrap_or_default(),
            priority: octet(PRIORITY),
            sync_address: be16(d, PRIORITY + 1).unwrap_or_default(),
            sequence: octet(PRIORITY + 3),
            options: octet(PRIORITY + 4),
            universe: be16(d, PRIORITY + 5).unwrap_or_default(),
            start_code: StartCode(code),
            slots,
        })
    }

    /// Appends the whole datagram.
    pub fn encode(&self, out: &mut Vec<u8>) {
        let slots = self.slots.get(..SLOTS).unwrap_or(self.slots);
        let total = VALUES + 1 + slots.len();
        let start = out.len();
        root::write(out, VECTOR_DATA, self.cid, total);
        root::flags_length(out, total - FRAMING);
        out.extend_from_slice(&VECTOR.to_be_bytes());
        text::write(out, self.source_name);
        out.push(self.priority);
        out.extend_from_slice(&self.sync_address.to_be_bytes());
        out.extend_from_slice(&[self.sequence, self.options]);
        out.extend_from_slice(&self.universe.to_be_bytes());
        root::flags_length(out, total - DMP);
        out.extend_from_slice(&DMP_FIXED);
        let count = u16::try_from(1 + slots.len()).unwrap_or(513);
        out.extend_from_slice(&count.to_be_bytes());
        out.push(self.start_code.0);
        out.extend_from_slice(slots);
        debug_assert_eq!(out.len() - start, total);
    }

    /// The start code and slots as the core's frame.
    #[must_use]
    pub fn frame(&self) -> Frame<'a> {
        Frame::within_limit(self.start_code, self.slots)
    }

    /// Whether the option bit `option` is set.
    #[must_use]
    pub const fn has(&self, option: u8) -> bool {
        self.options & option != 0
    }
}
