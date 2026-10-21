<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Drawbacks and risks

What the [roadmap](roadmap.md) costs, and where it could go wrong. Each entry
says what we lose, why we accept it, and what would change the decision.

## Of copying bacnet-rs's shape

**Twelve crates for a small protocol.** DMX512 is a start code and 512 bytes;
BACnet is a thousand-page standard. The crate split, the gates and the
200-line limit were sized for BACnet's core (12.7k lines). Ours may be a
fifth of that, and the ceremony is a fixed cost paid regardless.
*Accepted because* the layering is what keeps the core pure and testable on a
fake clock, and a second repo in the same shape is cheaper to maintain than a
second shape. *Revisit if* a crate stays under ~300 lines after its phase:
fold it into its neighbour and update the layer table.

**Two copies of the infrastructure.** The telemetry seam, `xtask` and the
server scaffolding are copied, not shared. A fix in one repo must be
remembered in the other, and they will drift.
*Accepted because* extracting a shared crate before the second user exists
guesses at the wrong abstraction. *Revisit at* the third protocol (osdp-go is
already a sibling): that is when a `regalium-common` repo pays for itself.

**Pinned git dependencies.** The telemetry SDK and resourcename are not on
crates.io. Each upgrade is a rev bump in two repos, and an OpenTelemetry
release needs the SDK moved first (bacnet-rs broke main on 0.33 this way).

## Of the protocol choices

**RDM is late and hard.** Discovery collisions, turnaround timing under
E1.20's microsecond windows, and vendor-specific NACKs are where real
fixtures disagree with the standard. Placing it at 6b means the probe's most
useful feature arrives last. *Mitigation:* a fixture corpus per vendor quirk,
and the simulator gaining responders before real hardware is required.

**USB-DMX without C.** The common serial crates enumerate ports through
libudev on Linux, which the `deps` gate refuses. Enumerating through sysfs is
more code we own and test. ENTTEC Open DMX (FTDI bit-banging) needs precise
break timing that a userspace serial API may not give; it may simply be
unsupported. *Revisit if* sysfs enumeration proves unreliable: add libudev as
an opt-in feature of `serial`, audited in `xtask/src/native.rs`.

**Timing is the product.** BACnet tolerates a late answer; DMX does not. A
refresh loop on a general-purpose async runtime can jitter under load, and
flicker is visible. The runtime needs a dedicated timer per output path and
metrics on frame interval, not just counts. This is not a problem bacnet-rs
solved, so there is no precedent to copy.

**Hardware to test against.** Fixture corpora prove decoding, not that a
real dimmer accepts our break length. We need a bench: at least one Art-Net
node, one sACN receiver, one USB-DMX widget and a few RDM fixtures. Without
it, Phases 4–6b end on Wireshark captures only.

**One sACN process per host.** Receiving multicast on port 5568 alongside
another sACN application needs `SO_REUSEADDR` set before binding, which
`std` cannot do. The fix is `socket2`, a pure-Rust dependency on `libc`; it
waits until a rig needs a console and this driver on one machine.

**Corpora built from the specifications.** Phases 2, 4 and 5 decode
records laid out from the standards' tables, not traffic captured from
equipment. They prove the codecs agree with our reading of the standards;
a misreading would agree with itself. Wireshark captures from real nodes
are the second opinion still owed.

**Standards carry licence terms.** E1.11, E1.20 and E1.31 are free from
ESTA's TSP downloads, under licence terms; Art-Net is Artistic Licence's free spec.
Citing clauses in doc comments is fine; quoting tables at length is not.

## Of the API and MCP

**Write-first changes the MCP story.** bacnet-server is mostly reads, and
refuses writes by default. A DMX server that refuses writes does almost
nothing. Letting a model drive lights raises real safety concerns: a strobe
at the wrong rate is a photosensitive-epilepsy hazard, and a blackout during a
show is a safety incident. The safety policy (rate limits, strobe ceiling,
blackout and hold, an audit log) must exist before the first MCP write tool,
and it is new design, not a port.

**Protos before the probe.** Designing the schema in Phase 7, before anyone
has used the probe, risks modelling the wrong resources. bacnet-rs grew its
schema early (Phase 9 "built early") and kept it. The risk is lower if
Phase 7 is scoped to what the driver owns (nodes, ports, universes, RDM) and
leaves shows, cues and effects to a controller built on top.

**Hot-path payloads.** Up to 44 frames a second per universe, times dozens of
universes, does not belong in per-message protobuf on every subscriber.
Either a FlatBuffers mirror (and bacnet-rs's `buffers.lock` discipline) or
change-only streaming; both are more machinery.

## Of the schedule

bacnet-rs went from first commit (24 September) to a TUI, server and
simulator in about four weeks, on a much larger protocol.
DMX's protocol layer is smaller, but timing, RDM and the safety policy are
genuinely new. A fair estimate is Phases 1–6 in two to three weeks, with
RDM, the schema and the safety policy as the long poles.
