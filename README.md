<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# dmx-rs

Pure-Rust DMX512-A, RDM, Art-Net 4 and Streaming ACN: a driver and runtime for
lighting control, with a field probe (TUI), a rig simulator, and a server that
exposes it over gRPC, REST and MCP.

The sibling of [bacnet-rs](https://github.com/regalium-os/bacnet-rs), built in
the same shape: a pure core on an injected clock, transports around it, one
telemetry seam, and conformance gates that CI enforces.

**State:** Phase 0. The workspace, layer contracts and gates are built; no
protocol code yet. See the [roadmap](docs/roadmap.md) and its
[drawbacks](docs/drawbacks.md).

## Layout

```
crates/
  regalium-dmx-telemetry       the seam every layer records through
  regalium-dmx-core            pure DMX512 + RDM: frames, universes, merge
  regalium-dmx-artnet          Art-Net 4 over UDP
  regalium-dmx-sacn            sACN (E1.31) over multicast
  regalium-dmx-serial          USB-DMX widgets and RS-485
  regalium-dmx-schema          protobuf model (Phase 7)
  regalium-dmx-telemetry-otel  the only OpenTelemetry binding
  regalium-dmx-runtime         refresh loops, sources, RDM transactions
  regalium-dmx                 the facade
tools/
  dmx-probe                    TUI and CLI
  dmx-sim                      simulated Art-Net/sACN nodes and RDM fixtures
  dmx-server                   gRPC, REST and MCP
xtask/                         the conformance gates
```

## Working here

```sh
just conformance   # gates, and the tests proving each one bites
just ci            # everything CI runs
```

Conventions are in [CLAUDE.md](CLAUDE.md).

## Licence

Apache-2.0.
