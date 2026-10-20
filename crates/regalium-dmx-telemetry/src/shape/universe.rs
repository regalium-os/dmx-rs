// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The universe series: frames, their timing, slots, sources and losses, on
//! the two universe ids; the info series that names the universe; and the
//! transport's packet and octet counts, on protocol and direction alone.

use super::{Direction, UniverseId, universe_labels};
use crate::instruments::{
    BYTES, FRAME_INTERVAL, FRAMES, PACKETS, SEQUENCE_ERRORS, SOURCES_LOST, UNIVERSE_INFO,
    UNIVERSE_SLOTS, UNIVERSE_SOURCES,
};
use crate::{Attribute, Meter, keys};

/// Counts one frame of universe `id` on `dmx.frames`, labelled by the
/// universe ids and `direction`.
pub fn record_frame(meter: &dyn Meter, id: &UniverseId<'_>, direction: Direction) {
    let [p, u] = universe_labels(id);
    meter.record(
        &FRAMES,
        1.0,
        &[p, u, Attribute::str(keys::DIRECTION, direction.name())],
    );
}

/// Records on `dmx.frame.interval` the time since universe `id`'s previous
/// frame, in milliseconds.
///
/// Kept per universe, unlike every other histogram here, because jitter is
/// per output path: one universe flickering on a starved refresh task is the
/// fault, and a histogram averaged across the rig would hide it. A rig's
/// universes number in the tens, so this is hundreds of series, not tens of
/// thousands.
pub fn record_frame_interval(meter: &dyn Meter, id: &UniverseId<'_>, elapsed_ms: f64) {
    meter.record(&FRAME_INTERVAL, elapsed_ms, &universe_labels(id));
}

/// Records on `dmx.universe.slots` how many slots universe `id`'s last frame
/// carried after its start code.
pub fn record_universe_slots(meter: &dyn Meter, id: &UniverseId<'_>, slots: u16) {
    meter.record(&UNIVERSE_SLOTS, f64::from(slots), &universe_labels(id));
}

/// Records on `dmx.universe.sources` how many sources are merged into
/// universe `id`.
pub fn record_universe_sources(meter: &dyn Meter, id: &UniverseId<'_>, sources: u32) {
    meter.record(&UNIVERSE_SOURCES, f64::from(sources), &universe_labels(id));
}

/// Counts on `dmx.input.sequence_errors` one packet for universe `id`
/// dropped for arriving out of sequence.
pub fn record_sequence_error(meter: &dyn Meter, id: &UniverseId<'_>) {
    meter.record(&SEQUENCE_ERRORS, 1.0, &universe_labels(id));
}

/// Counts on `dmx.input.sources_lost` one source that stopped sending to
/// universe `id`. Which source is in the trace and the log line, not here:
/// sACN CIDs are per console session, and a label per CID would grow without
/// bound across a season of shows.
pub fn record_source_lost(meter: &dyn Meter, id: &UniverseId<'_>) {
    meter.record(&SOURCES_LOST, 1.0, &universe_labels(id));
}

/// Records `dmx.universe.info`, valued 1: the universe ids, the `name` it was
/// patched with (empty when it has none) and its `merge` mode, `htp`, `ltp`
/// or `single`.
pub fn record_universe_info(meter: &dyn Meter, id: &UniverseId<'_>, name: &str, merge: &str) {
    let [p, u] = universe_labels(id);
    meter.record(
        &UNIVERSE_INFO,
        1.0,
        &[
            p,
            u,
            Attribute::str(keys::UNIVERSE_NAME, name),
            Attribute::str(keys::MERGE_MODE, merge),
        ],
    );
}

/// Counts one packet of `octets` on `protocol`'s socket or port, on
/// `dmx.transport.packets` and `dmx.transport.bytes`, labelled by protocol
/// and direction only: what the wire carries, whichever universe it was for.
pub fn record_packet(meter: &dyn Meter, protocol: &str, direction: Direction, octets: usize) {
    let labels = [
        Attribute::str(keys::PROTOCOL, protocol),
        Attribute::str(keys::DIRECTION, direction.name()),
    ];
    meter.record(&PACKETS, 1.0, &labels);
    // A packet is at most a few hundred octets; the conversion is exact.
    #[allow(clippy::cast_precision_loss)]
    meter.record(&BYTES, octets as f64, &labels);
}
