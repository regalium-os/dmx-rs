# CLAUDE.md

Conventions for working in this repository. These are enforced by `xtask` and
by CI, not left to reviewer memory.

`dmx-rs` is a pure-Rust implementation of DMX512-A (ANSI E1.11), RDM (ANSI
E1.20), Art-Net 4 and Streaming ACN (ANSI E1.31). It is a **library and a
runtime**, not an application: it is imported by lighting control software
that must keep a rig lit for years. Its sibling is `bacnet-rs`, and where this
document is silent, that repository's CLAUDE.md is the precedent.

## Non-negotiables

Each of these fails CI. Run `just conformance` before you think you are done.

| Rule | Enforced by |
| --- | --- |
| No hand-written file exceeds 200 lines, **documentation included** | `xtask size` |
| **No direct OpenTelemetry dependency**, outside the adapter | `xtask deps` |
| **Nothing beneath the driver links a C library** | `xtask deps` + CI `cross` |
| The pure core performs no I/O and reads no clock | `xtask purity` |
| Dependencies only ever point inward | `xtask arch` |
| Every hand-written file carries the Apache-2.0 notice | `xtask licence` |
| Every gate fails when it examined nothing | `xtask::scan::Findings::verdict` |
| Every gate has a test proving it bites | `xtask/tests/` |
| Clean under `clippy -D warnings` and `cargo fmt --check` | CI `lint` |
| Licences and advisories across the whole graph | `deny.toml`, CI `deny` |

## The 200-line limit

A file that fits in two screens can be held in the head while reading it.
When one approaches the ceiling, split it along a seam that already exists:
encode from decode, Art-Net's poll from its data, the discovery state machine
from its transition table. **Never reclaim lines by deleting documentation.**
Generated code under `**/generated/` is exempt. Test files are not.

## Licensing

Apache-2.0. Every hand-written file opens with the notice and its SPDX
identifier, in that file kind's comment syntax, followed by a blank line:

```rust
// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0
```

## Layering

The table in `xtask/src/arch.rs` is the authority. Change it there,
deliberately, and say why in the commit.

| Crate | Role | May import |
| --- | --- | --- |
| `regalium-dmx-telemetry` | the seam: tracer, meter, logger traits | nothing |
| `regalium-dmx-core` | **pure** DMX512, RDM, merging, transport trait | the seam |
| `regalium-dmx-artnet` | Art-Net 4 over UDP | core |
| `regalium-dmx-sacn` | E1.31 over UDP multicast | core |
| `regalium-dmx-serial` | the physical line: USB-DMX widgets, RS-485 | core |
| `regalium-dmx-schema` | protobuf model generated from `protobuf/` | core |
| `regalium-dmx-telemetry-otel` | the only crate naming OpenTelemetry | the seam |
| `regalium-dmx-runtime` | refresh loops, sources, RDM transactions | core, transports, schema |
| `regalium-dmx` | the facade: `pub use`, never a newtype | everything beneath |
| `dmx-probe`, `dmx-sim`, `dmx-server` | applications at the edge | facade, schema, adapter |

The seam is importable everywhere. A transport never reimplements a DMX frame
or an RDM message: Art-Net and sACN carry the core's types.

## Documentation

Every crate's `lib.rs` opens with a `//!` **layer contract**: what it owns,
what it must not do, which crates it may import, which spans it opens.

Every public item has a doc comment. **Cite the standard** (`/// Per ANSI
E1.20-2010 §6.2.11, the checksum is the 16-bit sum of every preceding slot.`),
say why rather than what, state whether a decode borrows or copies, and
document concurrency on every public type. A vendor quirk points at the
fixture that proves it.

## Library API

- Required arguments positionally, optional ones as builder methods.
- Typed errors carrying the offending bytes, never matched on a message.
- Accept traits, return concrete types.
- **Never panic.** No indexing, no `unwrap`, no `expect` outside tests. A
  node that aborts on a bad length blacks out the stage.

## Runtime API

- `new` constructs and **never spawns**; `run` blocks until cancelled and does
  not return before its tasks stop; `close` is idempotent.
- Time is injected. The core returns a frame and a deadline; the runtime
  waits. Refresh rate, RDM timeouts and sACN source loss are table tests.
- Backpressure is a design decision: a slow consumer of the event stream never
  delays a universe's refresh.
- A node going silent is an event, not an error.

## Output safety

DMX is write-first: almost every useful call moves a light. Every surface
that writes — CLI, TUI, gRPC, REST, MCP — goes through one policy in the
runtime (blackout, hold, rate and strobe limits). No surface implements its
own. MCP writes are refused unless the server is started with
`--allow-writes`.

## Telemetry

**Never depend on `opentelemetry`.** Observability goes through the
the-protobuf-project telemetry SDK via the seam, and only
`regalium-dmx-telemetry-otel` may name it. Spans are named
`dmx.<layer>.<operation>`. Attributes are declared with `trace_attrs!`; an
undeclared field cannot be recorded. Never record slot values per frame at
44 Hz: that is metrics, aggregated, not spans.

## Schemas

`protobuf/` will be the source of truth, resource-oriented per Google AIP.
Its design is open; see `docs/roadmap.md`, Phase 7.

## Tests and phase gates

Every protocol phase ends with a fixture corpus: hex-dumped traffic decoded
byte for byte, and re-encoded losslessly, malformed records included. **Do not
start a phase before the previous corpus passes.** A test that cannot see what
it is meant to check must fail, not pass. Test names are sentences.

## Commands

```sh
just conformance  # the gates, and the tests that prove they bite -- run this
just test         # cargo test --workspace --all-targets
just fixtures     # the phase gate
just lint         # fmt and clippy, as CI runs them
just deny         # licences and advisories
just cross        # every target CI builds
just ci           # everything above, in CI's order
```

## Do not

- Add a dependency to a pure crate.
- Depend on OpenTelemetry.
- Weaken a test or a gate to make it pass. Change the rule in `xtask`
  deliberately and say why in the commit.
- Let any surface write output except through the runtime's safety policy.
- Commit or push unless asked.
