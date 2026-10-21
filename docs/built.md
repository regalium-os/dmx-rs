<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Built

What each phase of the [roadmap](roadmap.md) delivered, what it deferred, and
why. The roadmap is the plan; this is the record, in three parts:

- the foundations, Phases 0 to 3, below;
- the protocols, Phases 4 to 6b: [built-protocols.md](built-protocols.md);
- the API, from Phase 7: [built-api.md](built-api.md).

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
