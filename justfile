# Copyright 2026 RegaliumOS™.
# SPDX-License-Identifier: Apache-2.0

# The commands CI runs, so that "it passes locally" and "it passes in CI" are
# the same claim. Anything CI does that is not a recipe here is a way for the
# two to drift apart.

import 'codegen.just'

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
# Once a corpus exists this is a hard gate: a corpus no walker names is a file
# nothing reads, and a filter matching no test passes having checked nothing.
[group('test')]
[doc('The phase gate: every committed corpus round-trips byte for byte')]
fixtures:
    #!/usr/bin/env bash
    set -euo pipefail
    corpora=$(find crates -name '*.hex' 2>/dev/null | wc -l | tr -d ' ')
    if [ "$corpora" -eq 0 ]; then
        echo "no fixture corpus found; the frame corpus landed in Phase 2, so this is a failure"
        exit 1
    fi
    # The walkers compile their corpora in, so a file they do not name is a
    # file nothing decodes. A corpus lives at crates/<crate>/fixtures/...
    for f in $(find crates -name '*.hex'); do
        crate=$(echo "$f" | cut -d/ -f1-2)
        if ! grep -rqs "$(basename "$f")" "$crate/tests"; then
            echo "$f is not named by any fixture walker in $crate/tests, so nothing decodes it"
            exit 1
        fi
    done
    echo "walking $corpora corpora"
    out=$(cargo test --workspace -- fixture 2>&1) || { echo "$out"; exit 1; }
    ran=$(echo "$out" | awk '/^test result:/ {n += $4} END {print n + 0}')
    if [ "$ran" -eq 0 ]; then
        echo "no fixture test ran over $corpora corpora; the walker was renamed or removed"
        exit 1
    fi
    echo "$ran fixture test(s) passed"

[group('test')]
[doc('Fuzz every decoder for DURATION seconds each, on nightly')]
fuzz duration='60':
    #!/usr/bin/env bash
    set -euo pipefail
    # libFuzzer needs nightly; put its rustc ahead of any other on PATH.
    export PATH="$(dirname "$(rustup which --toolchain nightly rustc)"):$PATH"
    cd fuzz
    for target in $(cargo fuzz list); do
        echo "fuzz $target for {{duration}} s"
        cargo fuzz run "$target" -- -max_total_time={{duration}}
    done

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
