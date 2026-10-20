# Copyright 2026 RegaliumOS™.
# SPDX-License-Identifier: Apache-2.0

# The commands CI runs, so that "it passes locally" and "it passes in CI" are
# the same claim. Anything CI does that is not a recipe here is a way for the
# two to drift apart.

default:
    @just --list

# Architecture, size, purity, dependency and licence gates. Run this before you
# think you are done.
[group('conformance')]
[doc('Architecture, size, purity, dependency and licence gates')]
gates *gate:
    cargo run -q -p xtask -- {{gate}}

# The gates, plus the negative tests that prove each one fails when it should.
# A gate exercised only against a correct tree reports success both when it
# works and when it does nothing.
[group('conformance')]
[doc('The gates, plus the negative tests proving each one bites')]
conformance: gates
    cargo test -q -p xtask

[group('test')]
test:
    cargo test --workspace --all-targets

# The phase gate: every committed corpus decodes and re-encodes byte for byte.
# The first corpus lands in Phase 2 with the DMX512 frame codec. Until then
# this recipe says so rather than reporting a pass it has not earned; once a
# corpus exists, a corpus nothing reads is a failure.
[group('test')]
[doc('The phase gate: every committed corpus round-trips byte for byte')]
fixtures:
    #!/usr/bin/env bash
    set -euo pipefail
    corpora=$(find crates -name '*.hex' 2>/dev/null | wc -l | tr -d ' ')
    if [ "$corpora" -eq 0 ]; then
        echo "no fixture corpus yet; the first lands in Phase 2 with the frame codec"
        exit 0
    fi
    out=$(cargo test --workspace -- fixture 2>&1) || { echo "$out"; exit 1; }
    # A filter that matches no test passes having checked nothing.
    ran=$(echo "$out" | awk '/^test result:/ {n += $4} END {print n + 0}')
    if [ "$ran" -eq 0 ]; then
        echo "no fixture test ran over $corpora corpora; the walker was renamed or removed"
        exit 1
    fi
    echo "$ran fixture test(s) passed"

[group('lint')]
lint:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings

[group('lint')]
fmt:
    cargo fmt --all

# Licences and advisories across the whole resolved graph.
[group('lint')]
deny:
    cargo deny check

# Every target CI builds. The driver is checked apart from the applications,
# which alone may carry the telemetry SDK's audited C.
[group('build')]
[doc('Check every target CI builds')]
cross:
    #!/usr/bin/env bash
    set -euo pipefail
    for target in armv7-unknown-linux-gnueabihf aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
        echo "check $target"
        cargo check -q --workspace --target "$target" \
            --exclude regalium-dmx-probe --exclude regalium-dmx-telemetry-otel \
            --exclude regalium-dmx-sim --exclude regalium-dmx-server
    done

# Everything above, in CI's order.
[group('conformance')]
ci: conformance lint test fixtures deny cross
