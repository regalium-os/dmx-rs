// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! A universe's patch as its RDM devices report it: who owns each address,
//! where two claim the same one, and where there is room.

use super::model::Device;

/// How many devices claim each address, 1 to 512; index 0 is unused.
#[must_use]
pub fn owners<'a>(devices: impl IntoIterator<Item = &'a Device>) -> [u8; 513] {
    let mut owners = [0u8; 513];
    for d in devices {
        for a in span(d) {
            if let Some(n) = owners.get_mut(usize::from(a)) {
                *n = n.saturating_add(1);
            }
        }
    }
    owners
}

/// The addresses `device` uses: its start address and footprint, cut at
/// 512. Empty for a device with no address or no footprint.
pub fn span(device: &Device) -> impl Iterator<Item = u16> {
    let start = device.address;
    let end = if start == 0 { 0 } else { start.saturating_add(device.footprint).min(513) };
    start..end.max(start)
}

/// Each run of addresses claimed twice or more, as `(first, last)`.
#[must_use]
pub fn overlaps(owners: &[u8; 513]) -> Vec<(u16, u16)> {
    let mut runs = Vec::new();
    let mut open: Option<u16> = None;
    for a in 1..=513u16 {
        let shared = owners.get(usize::from(a)).is_some_and(|&n| n > 1);
        match (shared, open) {
            (true, None) => open = Some(a),
            (false, Some(first)) => {
                runs.push((first, a - 1));
                open = None;
            }
            _ => {}
        }
    }
    runs
}

/// The first address from which `len` addresses are all free, ignoring the
/// device being moved (`moving`, its UID).
#[must_use]
pub fn first_free<'a>(devices: impl IntoIterator<Item = &'a Device>, moving: &str, len: u16) -> Option<u16> {
    let owners = owners(devices.into_iter().filter(|d| d.uid != moving));
    let len = len.max(1);
    (1..=513u16.saturating_sub(len)).find(|&start| (start..start + len).all(|a| owners.get(usize::from(a)) == Some(&0)))
}

/// Whether `len` addresses from `start` are free of every device but
/// `moving`, and inside the universe.
#[must_use]
pub fn is_free<'a>(devices: impl IntoIterator<Item = &'a Device>, moving: &str, start: u16, len: u16) -> bool {
    let owners = owners(devices.into_iter().filter(|d| d.uid != moving));
    start >= 1 && u32::from(start) + u32::from(len.max(1)) <= 513 && (start..start + len.max(1)).all(|a| owners.get(usize::from(a)) == Some(&0))
}

#[cfg(test)]
mod tests {
    use super::{first_free, is_free, overlaps, owners};
    use crate::engine::model::Device;

    fn at(uid: &str, address: u16, footprint: u16) -> Device {
        Device { universe: "u".into(), uid: uid.into(), label: String::new(), model: String::new(), address, footprint, personality: String::new(), responding: true }
    }

    #[test]
    fn an_overlap_is_every_run_claimed_twice() {
        let rig = [at("a", 97, 16), at("b", 97, 4), at("c", 200, 10), at("d", 205, 10)];
        assert_eq!(overlaps(&owners(&rig)), [(97, 100), (205, 209)]);
        assert_eq!(overlaps(&owners(&[at("a", 1, 512)])), []);
    }

    #[test]
    fn room_is_found_ignoring_the_device_being_moved() {
        let rig = [at("spots", 1, 304), at("wash", 97, 4), at("more", 305, 16)];
        assert_eq!(first_free(&rig, "wash", 4), Some(321));
        assert!(is_free(&rig, "wash", 321, 4));
        assert!(!is_free(&rig, "wash", 100, 4));
        assert!(!is_free(&rig, "wash", 510, 4), "past 512");
        assert_eq!(first_free(&[at("all", 1, 512)], "x", 1), None);
    }

    #[test]
    fn a_device_without_an_address_claims_nothing() {
        assert_eq!(owners(&[at("a", 0, 16)]).iter().map(|&n| u32::from(n)).sum::<u32>(), 0);
        assert_eq!(owners(&[at("a", 510, 16)]).iter().map(|&n| u32::from(n)).sum::<u32>(), 3, "cut at 512");
    }
}
