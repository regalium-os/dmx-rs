// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net's fixed-width, NUL-terminated ASCII fields: node names and the
//! node report.

/// The field's text: everything before the first NUL, or all of it.
#[must_use]
pub fn text(field: &[u8]) -> &[u8] {
    field.split(|&b| b == 0).next().unwrap_or_default()
}

/// Writes `value` into exactly `width` octets: cut at `width`, padded with
/// NULs. A decoded field goes back unchanged; a name this crate composes is
/// cut to leave its terminating NUL before it gets here (see [`name`]).
pub(crate) fn write(out: &mut Vec<u8>, value: &[u8], width: usize) {
    let value = value.get(..width).unwrap_or(value);
    out.extend_from_slice(value);
    out.resize(out.len() + (width - value.len()), 0);
}

/// `value` cut to fit a field of `width` octets with its NUL, at a
/// character boundary, as a name to send.
#[must_use]
pub fn name(value: &str, width: usize) -> &[u8] {
    let mut end = value.len().min(width.saturating_sub(1));
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value.as_bytes().get(..end).unwrap_or_default()
}
