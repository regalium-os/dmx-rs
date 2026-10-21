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
| 7 | **the schema tree**, decided below | `schema`, `protobuf/` | **built** |
| 8 | the field probe: TUI and CLI | `dmx-probe` | next |
| 9 | the server: gRPC, REST, MCP generated from the schema | `dmx-server` | |
| — | the rig simulator, growing with each transport | `dmx-sim` | alongside 4–6b |
| — | fuzz targets for every decoder | `fuzz/` | `frame`, `artnet`, `sacn`, `enttec`, `rdm` **built**; one per codec after |
| — | Grafana dashboard and observability stack, as `telemetry.just` | `tools/` | after 8 |

RDM is 6b rather than 2 on purpose: it is the hardest part of the protocol
(discovery collisions, turnaround timing, vendor NACKs) and it needs a working
transport to test against real fixtures.

## What each built phase delivered

Phase by phase, with what was deferred and why, in [built.md](built.md).

## Phase 7: the schema, decided

Designed as bacnet-rs's is: resources first, standard methods over them,
custom methods for what is neither, and Google's AIP linter, strict, as the
gate. The answers to the questions this section used to ask:

- **Resources.** `nodes`, `universes` with their `levels`, RDM `devices` and
  their `parameters` under a universe, and the `policy` and `serverStatus`
  singletons. Fixtures, profiles, scenes and cues belong to a show
  controller built on this driver, not to the driver.
- **Fixture profiles.** None here, for the same reason.
- **The hot path.** The levels stream is protobuf on the API, and mirrored to
  FlatBuffers and Cap'n Proto through `buffers`, as COV is in bacnet-rs.
- **MCP shape.** Every write carries an elicitation saying what moves; every
  write goes through `policy` (blackout, hold, master); MCP writes are
  refused without `--allow-writes`, which `policy` reports and no call sets.
- **Sharing with bacnet-rs.** Conventions copied, not a shared module: the
  two schemas version independently.

## Later

- RDMnet (ANSI E1.33) and LLRP for RDM over IP.
- Art-Net 4 RDM proxying and ArtTrigger/ArtTimeCode.
- KiNET and other vendor transports, as profiles if they fit, crates if not.
- Pixel mapping and effects: a show-controller concern, kept out of the
  driver unless the boundary moves deliberately.
