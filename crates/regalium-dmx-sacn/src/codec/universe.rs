// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Universe numbers, and the multicast group each is sent to.

use core::net::Ipv4Addr;

/// The UDP port E1.31 uses: 5568.
pub const PORT: u16 = 5568;

/// The highest data universe. 0 and everything above are not data
/// universes; 64 214 is discovery's.
pub const MAX_UNIVERSE: u16 = 63_999;

/// The universe discovery packets are sent on.
pub const DISCOVERY_UNIVERSE: u16 = 64_214;

/// The multicast group for `universe`: 239.255, then the universe's two
/// octets. `None` for universe 0, which has none.
#[must_use]
pub const fn multicast(universe: u16) -> Option<Ipv4Addr> {
    if universe == 0 {
        return None;
    }
    let [hi, lo] = universe.to_be_bytes();
    Some(Ipv4Addr::new(239, 255, hi, lo))
}

#[cfg(test)]
mod tests {
    use super::{DISCOVERY_UNIVERSE, multicast};
    use core::net::Ipv4Addr;

    #[test]
    fn a_universe_is_sent_to_239_255_and_its_two_octets() {
        assert_eq!(multicast(1), Some(Ipv4Addr::new(239, 255, 0, 1)));
        assert_eq!(multicast(0x1234), Some(Ipv4Addr::new(239, 255, 0x12, 0x34)));
        assert_eq!(
            multicast(DISCOVERY_UNIVERSE),
            Some(Ipv4Addr::new(239, 255, 250, 214))
        );
        assert_eq!(multicast(0), None);
    }
}
