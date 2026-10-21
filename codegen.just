# Copyright 2026 RegaliumOS™.
# SPDX-License-Identifier: Apache-2.0

# Everything generated from protobuf/, imported by the justfile. The output of
# each recipe is committed, so a consumer builds without buf or a plugin.

# Regenerate the schema crate's Rust types from protobuf/.
[group('codegen')]
[doc('Regenerate the schema crate from protobuf/')]
gen:
    buf generate

# Regenerate the OpenAPI 3 description of the REST surface: one merged spec,
# protobuf/generated/openapi/openapi.yaml, and typed schemas per package under
# types/. Needs protoc-gen-openapi (gnostic) and protoc-gen-openapi-solo, both
# in Homebrew. gnostic sets no operation summary, so xtask derives one from
# each operationId, and the `openapi` gate fails if a regeneration skipped it.
[group('codegen')]
[doc('Regenerate the OpenAPI spec from protobuf/')]
gen-openapi:
    buf generate --template buf.gen.openapi.yaml
    cargo run -q -p xtask -- summarise-openapi

# Regenerate the FlatBuffers and Cap'n Proto mirrors of the hot path, and the
# ordinal ledger, from a descriptor set narrowed to the levels stream. Needs
# buffers >= 0.2.2.
[group('codegen')]
[doc('Regenerate the hot-path mirrors and the ordinal ledger')]
gen-mirror:
    buf build --type dmx.universe.v1.StreamLevelsResponse \
        -o protobuf/generated/payloads.descriptor.binpb
    buffers generate
    buffers verify

# Everything above, in order.
[group('codegen')]
[doc('Regenerate everything from protobuf/')]
gen-all: gen gen-openapi gen-mirror

# Fails when the committed output is not what protobuf/ generates: a .proto
# edited without `just gen-all` ships a crate that disagrees with its schema.
[group('codegen')]
[doc('Fail if anything generated is stale')]
gen-check: gen-all
    git diff --exit-code -- crates/regalium-dmx-schema/src/generated protobuf/generated buffers.lock

# Google AIP lint over the schema, as CI runs it: strict, and a finding
# cannot be suppressed from inside the file it is in. Dependencies are
# exported from buf into target/, where the linter finds them.
[group('codegen')]
[doc('Lint protobuf/ against Google AIP, strictly')]
api-lint:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -rf target/protodeps
    buf export . -o target/protodeps
    cd protobuf
    api-linter --set-exit-status --ignore-comment-disables --output-format summary \
        -I . -I ../target/protodeps $(find dmx -name '*.proto' | sort)
