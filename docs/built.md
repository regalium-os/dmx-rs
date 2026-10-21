<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Built

What each phase of the [roadmap](roadmap.md) delivered, what it deferred, and
why. The roadmap is the plan; this is the record.

## Phase 0, in detail

Built in this commit:

- Twelve crates, empty but for their layer contracts.
- `xtask` gates ported from bacnet-rs: `size`, `licence`, `arch`, `purity`,
  `deps`, each with negative tests. The layer table names DMX's crates; the
  native-code rule anticipates libudev beneath `serial` and refuses it.
- `justfile`, CI (`conformance`, `test`, `fixtures`, `lint`, `deny`, `cross`),
  `deny.toml`, the pinned toolchain.

Not yet ported from bacnet-rs, each waiting on what it checks:

| Piece | Waits on |
| --- | --- |
| `openapi` gate, `buf` configs, AIP lint workflow | the schema tree (Phase 7) |
| `version` gate, changelog, release workflow | the first release |
| Bazel build and container images | a binary worth shipping (Phase 8) |
| Dependabot and auto-merge | the first external dependency (Phase 1) |

## Phase 1, in detail

The seam is bacnet-rs's, unchanged in mechanism: declared keys, declared
instruments, `trace_attrs!`, no-op defaults, no dependencies. What is DMX's
own:

- **Keys** (`keys/`): universe, protocol, node, Art-Net and sACN fields, and
  RDM's UID, PID, command class and `DEVICE_INFO` identity.
- **Series** (`instruments.rs`, `shape/`): frames and frame interval per
  universe; slots, sources, sequence errors and lost sources; packets and
  octets per protocol; node up and info; RDM requests (counted per UID,
  timed per PID), device up and info, discovery counts.
- **Histogram buckets** on `Instrument`, carried through the adapter: the
  frame-interval histogram resolves 22–25 ms, where OpenTelemetry's defaults
  jump from 10 to 25.
- **No span per frame, and no slot levels as a series**, both stated in the
  seam's contract.

The adapter pins the same SDK commit as bacnet-rs, so both export alike.

## Phase 2, in detail

- **Addresses** are 1 to 512 and convert to an index in one place, so the
  slot-1-versus-octet-1 bug has nowhere to live.
- **Frames** borrow their slots and are lossless: any start code is kept, a
  short packet stays short, more than 512 slots is an error pointing at the
  first slot too many.
- **Universes** hold 512 levels, a sent length and a generation that moves
  exactly when something a receiver would see changes: the refresh loop's
  one-integer test for "send now or wait for keep-alive".
- **Merging**: HTP per slot; LTP per slot *by change*, so a console resending
  the same levels does not steal slots from the one that moved a fader.
  Sources are bounded (8 by default) because they arrive from the network.
- Alternate start codes are never applied or merged as levels.
- No spans: all of this runs per frame (see the seam's contract).
- Corpus `fixtures/frame/`: null, alternate and malformed packets, built
  from the standard.

## Phase 3, in detail

- **The refresh policy** is three numbers — interval, repeats, keep-alive —
  and a pure state machine (`core::refresh`) that answers *send now* or
  *idle until T* from the time and the universe's generation. Serial
  refreshes continuously; network transports send on change and keep alive,
  each supplying its own policy through `Transport::refresh`.
- **The transport trait** (`core::transport`) carries frames, never protocol
  headers, and blocks with a timeout, as bacnet-rs's datalinks do.
- **One output thread per transport** (`runtime::Output`), plain `std`, on an
  injected clock: `SystemClock` for a rig, `ManualClock` for tests, where an
  hour of refresh runs in milliseconds with every frame at its exact time.
- A level change wakes the loop; it is sent at the next interval, never
  sooner, never at the keep-alive.
- A failing transport is one `SendFailed` event and one log line, then one
  `SendRecovered`; events never block the loop (drop and count).
- Every send records `dmx.frames`, `dmx.frame.interval`, packets and slots.
- Receiving and merging are deferred to Phase 4, where a real receiver
  exercises them.

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

