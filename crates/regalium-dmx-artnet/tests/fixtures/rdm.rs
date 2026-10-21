// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What the RDM packets of the corpus mean: `rdm.hex`.

use super::octets;
use regalium_dmx_artnet::codec::Packet;
use regalium_dmx_core::rdm::Message;
use std::fmt::Write as _;

/// The meaning of an `ArtRdm`, `ArtTodRequest`, `ArtTodControl` or
/// `ArtTodData`; anything else as its debug form.
pub(crate) fn meaning(packet: &Packet<'_>) -> String {
    match packet {
        Packet::Rdm(r) => {
            let rdm = r.rdm();
            let inner = Message::decode(&rdm).map_or_else(
                |_| "not rdm".to_owned(),
                |m| {
                    format!(
                        "{} tn {} pid {}",
                        m.command_class.name().unwrap_or("?"),
                        m.transaction,
                        m.pid
                    )
                },
            );
            format!(
                "rdm v{} rdmver {} port {} command {}; {inner}",
                r.version, r.rdm_version, r.port_address, r.command
            )
        }
        Packet::TodRequest(t) | Packet::TodControl(t) => {
            let kind = if matches!(packet, Packet::TodRequest(_)) {
                "tod-request"
            } else {
                "tod-control"
            };
            let mut out = format!(
                "{kind} v{} net {} command {} addresses {}",
                t.version,
                t.net,
                t.command,
                octets(t.addresses)
            );
            if !t.trailing.is_empty() {
                let _ = write!(out, "; {} trailing", t.trailing.len());
            }
            out
        }
        Packet::TodData(t) => {
            let uids: Vec<String> = t.uids().map(|u| u.to_string()).collect();
            let uids = if uids.is_empty() {
                "none".to_owned()
            } else {
                uids.join(" ")
            };
            format!(
                "tod-data v{} port {} bind {} response {} port-address {} total {} block {} uids {uids}",
                t.version, t.port, t.bind_index, t.response, t.port_address, t.total, t.block
            )
        }
        other => format!("{other:?}"),
    }
}
