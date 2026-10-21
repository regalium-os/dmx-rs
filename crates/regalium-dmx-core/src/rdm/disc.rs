// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The answer to `DISC_UNIQUE_BRANCH`: not an RDM message, but a UID
//! encoded so that two devices answering at once garble each other
//! detectably (ANSI E1.20).
//!
//! Up to seven 0xFE preamble octets, a 0xAA separator, then each of the
//! UID's six octets twice — once OR 0xAA, once OR 0x55 — and the 16-bit
//! sum of those twelve octets, encoded the same way. AND-ing a pair gives
//! the octet back; a pair whose bits are not set where the masks say is
//! a collision.

use super::error::{Error, ErrorKind};
use super::uid::Uid;

/// A preamble octet.
pub const PREAMBLE: u8 = 0xFE;
/// The separator after the preamble.
pub const SEPARATOR: u8 = 0xAA;
/// The most preamble octets a device sends.
pub const MAX_PREAMBLE: usize = 7;
/// Octets after the separator: 12 of UID, 4 of checksum.
pub const BODY: usize = 16;

fn pair(out: &mut Vec<u8>, octet: u8) {
    out.extend_from_slice(&[octet | 0xAA, octet | 0x55]);
}

/// Appends `uid`'s response, with `preamble` 0xFE octets (at most 7).
pub fn encode(uid: Uid, preamble: usize, out: &mut Vec<u8>) {
    out.resize(out.len() + preamble.min(MAX_PREAMBLE), PREAMBLE);
    out.push(SEPARATOR);
    let start = out.len();
    for octet in uid.octets() {
        pair(out, octet);
    }
    let sum = super::message::checksum(out.get(start..).unwrap_or_default());
    for octet in sum.to_be_bytes() {
        pair(out, octet);
    }
}

/// Reads a response. Octets after the sixteen are ignored: a widget may
/// pass on line noise after the response.
///
/// # Errors
///
/// [`ErrorKind::Encoding`] for a missing separator, a pair with the wrong
/// bits, or a checksum that does not match: a collision.
pub fn decode(bytes: &[u8]) -> Result<Uid, Error<'_>> {
    let lead = bytes
        .iter()
        .take(MAX_PREAMBLE)
        .take_while(|&&b| b == PREAMBLE)
        .count();
    if bytes.get(lead) != Some(&SEPARATOR) {
        return Err(Error::at(ErrorKind::Encoding, bytes, lead));
    }
    let start = lead + 1;
    let Some(body) = bytes.get(start..start + BODY) else {
        return Err(Error::truncated(bytes, start + BODY));
    };
    let mut octets = [0u8; 8];
    let (pairs, _) = body.as_chunks::<2>();
    for (i, (out, &[a, b])) in octets.iter_mut().zip(pairs).enumerate() {
        let at = start + 2 * i;
        if a & 0xAA != 0xAA || b & 0x55 != 0x55 {
            return Err(Error::at(ErrorKind::Encoding, bytes, at));
        }
        *out = a & b;
    }
    let [uid @ .., hi, lo] = octets;
    let sum = super::message::checksum(body.get(..12).unwrap_or_default());
    if sum != u16::from_be_bytes([hi, lo]) {
        return Err(Error::at(ErrorKind::Encoding, bytes, start + 12));
    }
    Ok(Uid::from_octets(uid))
}
