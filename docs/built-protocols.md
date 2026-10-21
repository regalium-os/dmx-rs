<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Built: the protocols

Phases 4 to 6b of the [roadmap](roadmap.md): Art-Net, sACN, the serial line
and RDM. The foundations are in [built.md](built.md).

## Phase 4, in detail

- **The codec** (`artnet::codec`): `ArtDmx`, `ArtPoll`, `ArtPollReply`,
  `ArtSync`, `ArtAddress`, borrowing and lossless; an unknown opcode decodes
  as `Other` with its body. A short `ArtPollReply` from older equipment
  decodes, its missing fields zero. `ArtDmx` carries the core's frame.
- **The sequence rule**: a packet a little behind the last from the same
  source and universe is dropped and counted; one far behind is a restart.
  The window is E1.31's, since Art-Net specifies none.
- **The socket** (`artnet::ArtNet`) satisfies the transport trait: it sends
  to one target, pads odd lengths as Art-Net 4 requires, answers untargeted
  polls as a portless controller, and records replies in a bounded node
  table. Sending and receiving share one socket through an `Arc`.
- **The input** (`runtime::Input`): receive, merge per listened universe,
  and drop a source silent for the transport's source timeout (Art-Net's
  10 s by default), with found, lost, failed and recovered as events.
- **First live output**: two consoles on output threads, merged HTP by an
  input thread, over real UDP on loopback.
- Corpus `fixtures/` in the Art-Net crate: every packet, unknown opcodes and
  malformed datagrams, **built from the specification's layout, not yet
  captured**. A Wireshark capture from real nodes is the second opinion
  still owed, together with the spec's exact keep-alive figure (1 s is used).
- **Not yet**: unicast routing to the nodes subscribed to a universe,
  sending `ArtSync` after a batch, and `ArtRdm`/`ArtTodData` (Phase 6b).

## Phase 5, in detail

- **The codec** (`sacn::codec`): data, synchronisation and universe
  discovery, borrowing and lossless. Every fixed field and every layer's
  length is checked, as E1.31 tells receivers to; an unknown vector decodes
  as `Other`. Alternate start codes travel, unlike Art-Net.
- **Priority is the core merge's**: only sources at the highest priority
  present merge, by HTP or LTP; the rest are held and take over when the
  higher leave. Art-Net sources all sit at the default, 100.
- **The transport trait grew**: `join`/`leave`, which an input calls as it
  listens, for multicast membership; `end`, which an output calls on unpatch
  and close, for sACN's three stream-terminated packets; and `Received`
  carries priority and whether the source ended.
- **The socket** (`sacn::Sacn`): sends to each universe's multicast group
  (or one unicast target), numbers packets per universe, announces its
  universes every 10 s, records other sources' announcements, and drops
  preview data, late packets and its own looped-back packets. A source is
  lost after 2.5 s, or at once when it terminates.
- **Live**: a priority-150 console holds a universe against a brighter
  priority-100 one, and hands it over within milliseconds when it unpatches.
- Corpus `fixtures/` in the sACN crate, **built from the standard's layout,
  not yet captured**.
- **Not yet**: `SO_REUSEADDR` (std cannot set it, so one process per host
  holds port 5568), applying synchronisation, per-address priority (start
  code 0xDD), and a multicast test on a real interface.

## Phase 6, in detail

- **The widget API** (`serial::enttec`): messages, a resynchronising
  stream framer bounded to one message, widget parameters, BCD serial
  numbers. Pure. Corpus `fixtures/`: requests, replies, malformed.
- **Discovery** (`serial::discovery`, `serial::scan`): enumerate (sysfs on
  Linux, for VID:PID, serial and product, without libudev; device names on
  macOS), classify (widget, maybe, no), probe. The probe is a state machine
  on an injected clock and sends only Get Serial and Get Parameters, never
  DMX. Generic FTDI cables are *maybe*, and probed only when the caller
  allows.
- **The port** (`serial::port`): raw termios and poll through `rustix`,
  which links nothing; the deps gate and `cargo deny` pass.
- **Transports**: `Enttec` probes on open and refuses a port that does not
  answer; pads to the API's 24-slot minimum; receives clean packets only.
  `Uart` makes the break from a zero octet at 57 600 baud (156 µs low,
  17 µs mark), then sends at 250 kbaud 8N2, refreshing continuously.
- **Tested without hardware**: a fake widget on a pseudo-terminal answers
  the probe, receives DMX and sends it back, through the real port code.
- **Not yet**: Windows ports; USB ids on macOS (IOKit is C); receiving on a
  raw UART; the Pro Mk2's second port; and the bench — a real widget and a
  scope on the UART's break.


## Phase 6b, in detail

- **The core's RDM** (`core::rdm`), pure: UIDs; the message codec and its
  checksum; command classes, response types, PIDs and NACK reasons, each a
  newtype over every value; `DEVICE_INFO`; the discovery answer's own
  encoding, where a collision is a decode error; matching an answer to its
  question field by field; and discovery's binary search as a state
  machine — mute retries, a device that will not mute, a noisy line
  bounded and flagged. Corpus `fixtures/rdm/`.
- **A responder** (`core::rdm::Responder`): one device answering as E1.20
  says, with `ACK_OVERFLOW` and `ACK_TIMER` on request. The simulator's
  fixtures, and every RDM test's.
- **The line trait** (`core::transport::RdmLine`): send a request, collect
  the window's answer; or hand over the far end's own table.
- **The controller** (`runtime::rdm::Controller`): discovery, GET, SET;
  retries, overflow joined, `ACK_TIMER` followed through the queue on the
  injected clock; `dmx.rdm.request` and `dmx.rdm.discovery` spans and the
  RDM series. The one path every RDM write takes, where the output policy
  will gate it.
- **The widget** carries RDM on labels 7 and 11, draining stale answers
  before each request. **Art-Net** carries it in `ArtRdm`, with the node's
  table from `ArtTodRequest`/`ArtTodData`; replies read by a concurrent
  input reach the controller through an inbox. Corpus `fixtures/rdm.hex`.
- **Tested end to end without hardware**: six fixtures discovered and
  re-addressed through the widget on a pty while levels go out; four
  through a fake Art-Net node over UDP while an input shares the socket.
- **Not yet**: sub-devices beyond the root, proxies, `ACK_TIMER` on SET,
  E1.33. Unverified against equipment: whether widget firmware reports a
  timeout (label 12) for broadcasts, and the `QUEUED_MESSAGE` status type
  (0x04) a controller should ask with.
