<!--
Copyright 2026 RegaliumOS™.
SPDX-License-Identifier: Apache-2.0
-->

# Built: the API

Phase 7 onward of the [roadmap](roadmap.md): the schema, and what serves
it. The foundations are in [built.md](built.md).

## Phase 7, in detail

- **19 protos in four packages** under `protobuf/dmx/`, laid out as
  bacnet-rs's: a resource, its messages, its service, its enums.
  `dmx.node.v1` (nodes, found by discovery), `dmx.universe.v1` (universes,
  created by patching or listening; their levels; blackout, write and
  stream), `dmx.rdm.v1` (devices and raw parameters, found by RDM
  discovery), `dmx.server.v1` (the policy, the status). 25 operations.
- **Lint**: `buf lint` (STANDARD less the two rules AIP overrides, as in
  bacnet-rs) and the AIP linter, strict, comment disables ignored: zero
  findings. `just api-lint` locally; `.github/workflows/api-lint.yml` in CI.
  Three findings fixed on the way, each in AIP's favour: a custom method
  returns its own resource; `update_mask` is optional; `uid` is AIP-148's
  UUID, so an RDM UID is `rdm_uid`.
- **Generated, committed, reproducible** (`just gen-all`, `just gen-check`):
  prost types with protobuf's canonical JSON in `regalium-dmx-schema`; the
  OpenAPI spec and typed schemas, with summaries the `openapi` gate checks;
  FlatBuffers and Cap'n Proto mirrors of the levels stream, with their
  ordinal ledger.
- **Conversions** (`schema::convert`): UIDs and PIDs to resource IDs and
  back, `DEVICE_INFO` to a device, a universe to its levels, merge modes.
- **Not yet**: serving any of it (Phase 9); traffic dissection as a service,
  as bacnet-rs has; sensors and sub-devices as resources.
