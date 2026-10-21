// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Art-Net and sACN datagrams.

use super::{Dissection, Field, rdm, refused};
use regalium_dmx::artnet::codec::{Packet as ArtPacket, text};
use regalium_dmx::sacn::codec::{Packet as SacnPacket, TERMINATED, text as sacn_text};

fn lossy(field: &[u8]) -> String {
    String::from_utf8_lossy(text(field)).into_owned()
}

fn plain(summary: String, layer: String, fields: Vec<Field>) -> Dissection {
    let mut tree = vec![Field::new(0, layer, None)];
    tree.extend(fields);
    Dissection { summary, levels: false, tree }
}

/// An Art-Net datagram.
pub(super) fn artnet(octets: &[u8]) -> Dissection {
    let packet = match ArtPacket::decode(octets) {
        Ok(p) => p,
        Err(e) => return refused("Art-Net", &e, e.offset, octets.len()),
    };
    let head = |name: &str| format!("Art-Net · {name} ({:#06x})", packet.opcode().0);
    match packet {
        ArtPacket::Dmx(d) => Dissection {
            summary: format!("ArtDmx {} · seq {} · {} slots", d.port_address(), d.sequence(), d.slots().len()),
            levels: true,
            tree: vec![
                Field::new(0, head("ArtDmx"), Some((0, 17))),
                Field::new(1, format!("sequence {}", d.sequence()), Some((12, 12))),
                Field::new(1, format!("port {}", d.port_address()), Some((14, 15))),
                Field::new(1, format!("{} slots", d.slots().len()), Some((18, 17 + d.slots().len()))),
            ],
        },
        ArtPacket::Poll(p) => plain("ArtPoll".to_owned(), head("ArtPoll"), vec![Field::new(1, format!("flags {:#04x}", p.flags), Some((12, 12)))]),
        ArtPacket::PollReply(r) => {
            let f = r.fields();
            plain(
                format!("ArtPollReply · {} · {}", lossy(f.short_name), f.ip),
                head("ArtPollReply"),
                vec![
                    Field::new(1, format!("address {}:{}", f.ip, f.port), Some((10, 15))),
                    Field::new(1, format!("short name “{}”", lossy(f.short_name)), Some((26, 43))),
                    Field::new(1, format!("long name “{}”", lossy(f.long_name)), Some((44, 107))),
                    Field::new(1, format!("style {} · ports {}", f.style, f.ports), Some((172, 200))),
                ],
            )
        }
        ArtPacket::Rdm(r) => {
            let inner = r.rdm();
            let mut d = rdm::message(&inner, 23);
            d.summary = format!("ArtRdm {} · {}", r.port_address, d.summary);
            let mut tree = vec![Field::new(0, head("ArtRdm"), Some((0, 23))), Field::new(1, format!("port {}", r.port_address), Some((21, 23)))];
            tree.extend(d.tree);
            Dissection { tree, ..d }
        }
        ArtPacket::TodData(t) => plain(format!("ArtTodData {} · {} uids", t.port_address, t.uids().count()), head("ArtTodData"), t.uids().map(|u| Field::new(1, u.to_string(), None)).collect()),
        ArtPacket::TodRequest(t) => plain(format!("ArtTodRequest · net {}", t.net), head("ArtTodRequest"), Vec::new()),
        other => plain(format!("{}", other.opcode()), head("other"), Vec::new()),
    }
}

/// An sACN datagram.
pub(super) fn sacn(octets: &[u8]) -> Dissection {
    let packet = match SacnPacket::decode(octets) {
        Ok(p) => p,
        Err(e) => return refused("sACN", &e, e.offset, octets.len()),
    };
    match packet {
        SacnPacket::Data(d) => Dissection {
            summary: format!("data · univ {} · priority {} · seq {}{}", d.universe, d.priority, d.sequence, if d.has(TERMINATED) { " · terminated" } else { "" }),
            levels: d.start_code.0 == 0 && !d.has(TERMINATED),
            tree: vec![
                Field::new(0, "sACN · data", Some((0, 37))),
                Field::new(1, format!("source {}", d.cid), Some((22, 37))),
                Field::new(1, format!("priority {}", d.priority), Some((108, 108))),
                Field::new(1, format!("universe {}", d.universe), Some((113, 114))),
                Field::new(1, format!("{} slots", d.slots.len()), Some((126, 125 + d.slots.len()))),
            ],
        },
        SacnPacket::Discovery(d) => {
            let universes: Vec<String> = d.universes().map(|u| u.to_string()).collect();
            let name = String::from_utf8_lossy(sacn_text(d.source_name)).into_owned();
            plain(format!("discovery · {name} · univ {}", universes.join(" ")), "sACN · universe discovery".to_owned(), vec![Field::new(1, format!("source {} “{name}”", d.cid), Some((22, 107)))])
        }
        SacnPacket::Sync(s) => plain(format!("sync · address {}", s.sync_address), "sACN · sync".to_owned(), Vec::new()),
        _ => plain("sACN · other".to_owned(), "sACN · other".to_owned(), Vec::new()),
    }
}
