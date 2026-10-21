<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Roadmap

Everything dmx-rs is to be, in the order it comes in. Each crate's `lib.rs`
layer contract is the authority on what it owns; the costs of this plan are
in [drawbacks.md](drawbacks.md).

**Built** means merged, tested and run in CI; what each built phase
delivered is in [built.md](built.md). Everything else is designed, not
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
| 4 | Art-Net 4: ArtDmx, ArtPoll/Reply, ArtSync, ArtAddress; first live output; the receive and merge loop | `artnet`, `runtime` | **built** |
| 5 | sACN: data, sync, universe discovery, priority, source loss | `sacn`, `core`, `runtime` | **built** |
| 6 | the physical line: ENTTEC DMX USB Pro, RS-485 break generation; port auto-discovery | `serial` | **built** |
| 6b | RDM: codec, discovery, GET/SET, over the widget and Art-Net (sACN has none; E1.33 later) | `core`, `runtime`, transports | **built** |
| 7 | **the schema tree** — designed together, see below | `schema`, `protobuf/` | next: to discuss |
| 8 | the field probe: TUI and CLI | `dmx-probe` | |
| 9 | the server: gRPC, REST, MCP generated from the schema | `dmx-server` | |
| — | the rig simulator, growing with each transport | `dmx-sim` | alongside 4–6b |
| — | fuzz targets for every decoder | `fuzz/` | `frame`, `artnet`, `sacn`, `enttec`, `rdm` **built**; one per codec after |
| — | Grafana dashboard and observability stack, as `telemetry.just` | `tools/` | after 8 |

RDM is 6b rather than 2 on purpose: it is the hardest part of the protocol
(discovery collisions, turnaround timing, vendor NACKs) and it needs a working
transport to test against real fixtures.

## What each built phase delivered

Phase by phase, with what was deferred and why, in [built.md](built.md).

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
