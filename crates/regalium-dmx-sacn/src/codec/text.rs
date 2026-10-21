// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! E1.31's 64-octet, NUL-terminated UTF-8 source name.

/// Octets of the source name field.
pub const NAME: usize = 64;

/// The name's text: everything before the first NUL, or all of it.
#[must_use]
pub fn text(field: &[u8]) -> &[u8] {
    field.split(|&b| b == 0).next().unwrap_or_default()
}

/// `value` cut to fit the name field with its NUL, at a character
/// boundary, as a name to send.
#[must_use]
pub fn name(value: &str) -> &[u8] {
    let mut end = value.len().min(NAME - 1);
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value.as_bytes().get(..end).unwrap_or_default()
}

/// Writes `value` into exactly [`NAME`] octets: cut, and padded with NULs.
pub(crate) fn write(out: &mut Vec<u8>, value: &[u8]) {
    let value = value.get(..NAME).unwrap_or(value);
    out.extend_from_slice(value);
    out.resize(out.len() + (NAME - value.len()), 0);
}
