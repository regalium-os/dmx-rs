<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Roadmap

Everything dmx-rs is to be, in the order it comes in. Each crate's `lib.rs`
layer contract is the authority on what it owns; the costs of this plan are
in [drawbacks.md](drawbacks.md).

**Built** means merged, tested and run in CI. Everything else is designed, not
written.

## How a phase ends

Each protocol phase ends with a committed corpus of hex-dumped traffic that
decodes and re-encodes byte for byte, malformed records included. **No phase
starts before the previous corpus passes.** `just fixtures` is the gate.

Captures come from real equipment where we have it, and from Wireshark's
Art-Net, sACN and RDM dissectors as the second opinion where we do not.

## Phases

| Phase | Delivers | Crates | State |
| --- | --- | --- | --- |
| 0 | workspace, layer contracts, conformance gates with negative tests, CI | all, `xtask` | **built** |
| 1 | the telemetry seam and its OTel adapter, ported from bacnet-rs; DMX's keys, instruments and series shapes | `telemetry`, `telemetry-otel` | **built** |
| 2 | DMX512 frames, universes, HTP/LTP merge; first corpus | `core` | **built** |
| 3 | refresh loops on an injected clock; the transport trait; events | `core`, `runtime` | **built** |
| 4 | Art-Net 4: ArtDmx, ArtPoll/Reply, ArtSync, ArtAddress; first live output; the receive and merge loop | `artnet`, `runtime` | next |
| 5 | sACN: data, sync, universe discovery, priority, source loss | `sacn` | |
| 6 | the physical line: ENTTEC DMX USB Pro, RS-485 break generation; port auto-discovery | `serial` | |
| 6b | RDM: codec, discovery, GET/SET, over serial, Art-Net and sACN (E1.33 later) | `core`, transports | |
| 7 | **the schema tree** — designed together, see below | `schema`, `protobuf/` | to discuss |
| 8 | the field probe: TUI and CLI | `dmx-probe` | |
| 9 | the server: gRPC, REST, MCP generated from the schema | `dmx-server` | |
| — | the rig simulator, growing with each transport | `dmx-sim` | alongside 4–6b |
| — | fuzz targets for every decoder | `fuzz/` | `frame` **built**; one per codec after |
| — | Grafana dashboard and observability stack, as `telemetry.just` | `tools/` | after 8 |

RDM is 6b rather than 2 on purpose: it is the hardest part of the protocol
(discovery collisions, turnaround timing, vendor NACKs) and it needs a working
transport to test against real fixtures.

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
  from the standard. Captured traffic arrives with Art-Net in Phase 4.

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

## Phase 7: the schema, to discuss

The protos drive the API, the generated Rust model and every MCP tool, so they
are designed before the probe and server, not grown out of them. The open
questions, for the session where we design them:

- **Resources.** A first cut: `nodes/{node}/ports/{port}`, `universes/{universe}`,
  `fixtures/{fixture}` (patch: universe, address, profile),
  `profiles/{profile}`, `rdm/devices/{device}/parameters/{parameter}`,
  `scenes/{scene}`, `cues/{cue}`. Which of these belong in a *driver*, and
  which in a show controller built on it?
- **Fixture profiles.** Import GDTF, the Open Fixture Library, or our own?
- **The hot path.** Universe output streamed at up to 44 Hz per universe:
  protobuf on gRPC, or a FlatBuffers mirror as bacnet-rs does for COV?
- **MCP shape.** Which tools write, how the safety policy appears in their
  descriptions, and what a model may never do (strobe, full intensity).
- **Sharing with bacnet-rs.** Common packages (network, traffic, server
  status) copied or imported from a shared module.

## Later

- RDMnet (ANSI E1.33) and LLRP for RDM over IP.
- Art-Net 4 RDM proxying and ArtTrigger/ArtTimeCode.
- KiNET and other vendor transports, as profiles if they fit, crates if not.
- Pixel mapping and effects: a show-controller concern, kept out of the
  driver unless the boundary moves deliberately.
