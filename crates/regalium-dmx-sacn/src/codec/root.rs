// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The ACN root layer every E1.31 packet opens with, and the flags-and-length
//! word every layer begins with.
//!
//! The root layer is ANSI E1.17's: a preamble size, a postamble size, the
//! packet identifier `ASC-E1.17` padded with NULs, then a PDU of flags and
//! length, a vector naming what follows, and the sender's CID.

use super::cid::Cid;
use super::error::{Error, ErrorKind};

/// The preamble, postamble and packet identifier: the first 16 octets.
pub const PREAMBLE: [u8; 16] = *b"\x00\x10\x00\x00ASC-E1.17\x00\x00\x00";

/// `VECTOR_ROOT_E131_DATA`: a data packet follows.
pub const VECTOR_DATA: u32 = 0x0000_0004;
/// `VECTOR_ROOT_E131_EXTENDED`: synchronisation or discovery follows.
pub const VECTOR_EXTENDED: u32 = 0x0000_0008;

/// Where the root layer's flags and length are.
pub(crate) const ROOT_PDU: usize = 16;
/// Where the CID is.
const CID: usize = 22;
/// Where the framing layer begins.
pub(crate) const FRAMING: usize = 38;

/// The root layer's vector and CID, checking every fixed field and every
/// length on the way.
pub(crate) fn read(datagram: &[u8]) -> Result<(u32, Cid), Error<'_>> {
    match datagram.get(..PREAMBLE.len()) {
        Some(head) if head == PREAMBLE => {}
        _ if datagram.len() < PREAMBLE.len() && PREAMBLE.starts_with(datagram) => {
            return Err(Error::truncated(datagram, FRAMING));
        }
        _ => return Err(Error::at(ErrorKind::NotAcn, datagram, 0)),
    }
    if datagram.len() < FRAMING {
        return Err(Error::truncated(datagram, FRAMING));
    }
    pdu(datagram, ROOT_PDU)?;
    let vector = be32(datagram, ROOT_PDU + 2).unwrap_or_default();
    let mut cid = [0u8; 16];
    for (to, from) in cid
        .iter_mut()
        .zip(datagram.get(CID..FRAMING).unwrap_or_default())
    {
        *to = *from;
    }
    Ok((vector, Cid(cid)))
}

/// Checks the flags and length at `at`: flags 0x7, and a length that runs
/// exactly to the end of the datagram, as every E1.31 layer does.
pub(crate) fn pdu(datagram: &[u8], at: usize) -> Result<(), Error<'_>> {
    let Some(&[hi, lo]) = datagram.get(at..at + 2) else {
        return Err(Error::truncated(datagram, at + 2));
    };
    if hi >> 4 != 0x7 {
        return Err(Error::at(ErrorKind::Flags { flags: hi >> 4 }, datagram, at));
    }
    let said = usize::from(u16::from_be_bytes([hi & 0x0F, lo]));
    let actual = datagram.len() - at;
    if said != actual {
        return Err(Error::at(ErrorKind::Length { said, actual }, datagram, at));
    }
    Ok(())
}

/// Writes the root layer for `vector` and `cid`. The datagram is `total`
/// octets long when finished; every length is computed from it.
pub(crate) fn write(out: &mut Vec<u8>, vector: u32, cid: Cid, total: usize) {
    out.extend_from_slice(&PREAMBLE);
    flags_length(out, total - ROOT_PDU);
    out.extend_from_slice(&vector.to_be_bytes());
    out.extend_from_slice(&cid.0);
}

/// Writes a flags-and-length word for a layer of `len` octets.
pub(crate) fn flags_length(out: &mut Vec<u8>, len: usize) {
    let len = u16::try_from(len).unwrap_or(0x0FFF) & 0x0FFF;
    out.extend_from_slice(&(0x7000 | len).to_be_bytes());
}

pub(crate) fn be16(datagram: &[u8], at: usize) -> Option<u16> {
    let &[hi, lo] = datagram.get(at..at.checked_add(2)?)? else {
        return None;
    };
    Some(u16::from_be_bytes([hi, lo]))
}

pub(crate) fn be32(datagram: &[u8], at: usize) -> Option<u32> {
    let bytes = datagram.get(at..at.checked_add(4)?)?;
    Some(u32::from_be_bytes(bytes.try_into().ok()?))
}
