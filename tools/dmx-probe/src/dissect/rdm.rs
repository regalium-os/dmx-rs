// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! RDM: a message, wherever it travels, and a discovery answer.

use super::{Dissection, Field, refused};
use regalium_dmx::rdm::{DeviceInfo, Message, NackReason, Pid, ResponseType, disc};

/// An RDM message, start code first. `base` is where its start code would
/// sit in the datagram it came in: an `ArtRdm` drops it, so 23 there.
pub(super) fn message(octets: &[u8], base: usize) -> Dissection {
    let m = match Message::decode(octets) {
        Ok(m) => m,
        Err(e) => return refused("RDM", &e, base + e.offset, base + octets.len()),
    };
    let at = |from: usize, to: usize| Some((base + from, base + to));
    let class = m.command_class.name().unwrap_or("class?");
    let response = m.command_class.0 & 1 == 1;
    let what = match (response, ResponseType(m.port_or_response), m.data) {
        (true, ResponseType::NACK_REASON, &[hi, lo]) => format!("NACK {}", NackReason(u16::from_be_bytes([hi, lo])).name().unwrap_or("?")),
        (true, ResponseType::ACK_TIMER, &[hi, lo]) => format!("ACK_TIMER {} ms", 100 * u32::from(u16::from_be_bytes([hi, lo]))),
        (true, ResponseType::ACK_OVERFLOW, _) => "ACK_OVERFLOW".to_owned(),
        (true, _, _) => "ACK".to_owned(),
        (false, _, _) => format!("{class}"),
    };
    let summary = format!("{what} {} · {} → {} · tn {}", m.pid, m.source, m.destination, m.transaction);
    let len = octets.len();
    let mut tree = vec![
        Field::new(0, format!("RDM · {class}{}", if response { format!(" · {what}") } else { String::new() }), at(0, len - 1)),
        Field::new(1, format!("destination {}", m.destination), at(3, 8)),
        Field::new(1, format!("source {}", m.source), at(9, 14)),
        Field::new(1, format!("transaction {}", m.transaction), at(15, 15)),
        Field::new(1, format!("pid {} · {} octets", m.pid, m.data.len()), at(21, 23)),
        Field::new(1, "checksum matches", at(len - 2, len - 1)),
    ];
    if m.pid == Pid::DEVICE_INFO && response {
        if let Some(i) = DeviceInfo::decode(m.data) {
            tree.push(Field::new(1, "DEVICE_INFO", at(24, 42)));
            tree.push(Field::new(2, format!("model {:#06x} · category {:#06x}", i.model, i.category), at(26, 29)));
            tree.push(Field::new(2, format!("footprint {} · personality {} of {}", i.footprint, i.personality, i.personalities), at(34, 37)));
            tree.push(Field::new(2, format!("start address {}", i.start_address), at(38, 39)));
            tree.push(Field::new(2, format!("sub-devices {} · sensors {}", i.sub_devices, i.sensors), at(40, 42)));
        }
    }
    Dissection { summary, levels: false, tree }
}

/// A `DISC_UNIQUE_BRANCH` answer: one UID, or a collision.
pub(super) fn discovery(octets: &[u8]) -> Dissection {
    match disc::decode(octets) {
        Ok(uid) => Dissection {
            summary: format!("discovery answer · {uid}"),
            levels: false,
            tree: vec![Field::new(0, "RDM · discovery answer", Some((0, octets.len() - 1))), Field::new(1, format!("uid {uid}"), None)],
        },
        Err(e) => refused("RDM discovery answer · collision", &e, e.offset, octets.len()),
    }
}
