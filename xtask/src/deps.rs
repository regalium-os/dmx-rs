// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! What may never enter the dependency graph, and where the one exception is.

use crate::graph::Graph;
use crate::metadata;
use crate::native::{FEATURE_NATIVE, check_native};
use crate::scan::Findings;
use std::collections::BTreeSet;
use std::path::Path;

/// The crate permitted to name an OpenTelemetry type.
const OTEL_ADAPTER: &str = "regalium-dmx-telemetry-otel";

/// Crates whose graph may hold an [`AUDITED_NATIVE`] library: the telemetry
/// adapter, and the applications built on it — the probe, the rig simulator
/// and the data server, which export what they see through it.
///
/// Everything else -- the core, every transport, the runtime, the schema, the
/// seam and the facade -- stays pure Rust, so an integrator embedding the
/// driver in a lighting node keeps a one-flag cross build. Only a caller that
/// opts into exporting telemetry through the SDK takes on a C toolchain.
pub const NATIVE_PERMITTED: &[&str] = &[
    OTEL_ADAPTER,
    "regalium-dmx-probe",
    "regalium-dmx-sim",
    "regalium-dmx-server",
];

/// Native libraries read by hand and accepted, beneath [`NATIVE_PERMITTED`]
/// only: `(crate, links, why)`.
///
/// Both arrive through the telemetry SDK, which records MCAP for Foxglove
/// and compresses it. They compile vendored C with `cc`; nothing here links
/// a system library. Add an entry only after reading the crate's build script.
pub const AUDITED_NATIVE: &[(&str, &str, &str)] = &[
    (
        "zstd-sys",
        "zstd",
        "zstd for MCAP chunks, via telemetry -> mcap -> zstd",
    ),
    (
        "lz4-sys",
        "lz4",
        "lz4 for MCAP chunks, via telemetry -> mcap -> lz4",
    ),
];

/// Every target this repository builds: CI's cross matrix, and the release
/// platforms of the probe. The graph is checked as resolved for each.
pub const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "armv7-unknown-linux-gnueabihf",
    "x86_64-pc-windows-gnu",
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
];

/// `links` values that mark a version, not a native library.
///
/// The `links` key exists to let cargo refuse two crates claiming the same
/// native library. A few crates borrow it for an unrelated purpose -- one
/// copy of me per build -- and link no C at all. Each entry here is a crate
/// that was checked by hand; add one only after reading its build script.
///
/// `prettyplease02`: prettyplease 0.2's build script prints its version and
/// two `check-cfg`s, nothing more. It arrives as a build dependency of
/// pbjson-types (via prost-build), whose own build script generates the
/// well-known types from a bundled descriptor set and never runs protoc.
pub(crate) const LINK_MARKERS: &[&str] = &["wasm_bindgen", "prettyplease02"];

/// Dependency name prefixes nobody but [`OTEL_ADAPTER`] may declare.
///
/// Observability goes through the the-protobuf-project telemetry SDK, which
/// owns the exporter lifecycle and the attribute convention. Two things
/// deciding how a span is made is how instrumentation rots.
const OTEL_PREFIXES: &[&str] = &["opentelemetry", "tracing-opentelemetry"];

/// Checks the bans that keep the driver pure Rust and free of a second
/// telemetry stack.
///
/// # Errors
///
/// When a banned crate appears, or when nothing was examined.
pub fn run(root: &Path) -> Result<String, String> {
    let members = metadata::workspace_members(root)?;
    let mut findings = Findings::default();

    for member in &members {
        findings.checked();
        for dep in &member.direct_deps {
            if member.name != OTEL_ADAPTER && is_otel(&dep.name) {
                findings.violation(format!(
                    "{} depends on {}; only {OTEL_ADAPTER} may, and every other layer \
                     records through the telemetry seam instead",
                    member.name, dep.name
                ));
            }
            if dep.name == "cc" && dep.kind == "build" {
                findings.violation(format!(
                    "{} depends on {}, which compiles C. Lighting nodes are ARM, and what \
                     keeps `--target armv7-unknown-linux-gnueabihf` a one-flag build is \
                     that nothing in the graph needs a C toolchain",
                    member.name, dep.name
                ));
            }
        }
    }

    // A native library usually arrives transitively, which is the failure no
    // amount of reading our own manifests would reveal. The signal is cargo's
    // `links` key rather than a name ending in -sys: js-sys ends in -sys and
    // binds JavaScript, libc does not and binds a C library.
    // The same defect usually shows on every target; report it once.
    let mut native = BTreeSet::new();
    for target in TARGETS {
        let graph = Graph::resolve(root, target)?;
        // An application may turn an opt-in feature on by default, and
        // cargo unifies features across
        // the workspace, so the default resolve then holds it too. There it
        // is held to the same rule: reached only through the crate whose
        // feature it is. Nothing else turning it on by default is excused.
        check_native(
            &graph,
            asked_by_an_application(&members),
            &mut findings,
            &mut native,
        );
        // An opt-in feature's native code is invisible to the default
        // resolve, so each one is resolved again with it on.
        for exception in FEATURE_NATIVE {
            if graph.declares(exception.owner, exception.feature) {
                let feature = format!("{}/{}", exception.owner, exception.feature);
                let with = Graph::resolve_with(root, target, &[&feature])?;
                check_native(&with, Some(exception), &mut findings, &mut native);
            }
        }
    }
    for message in native {
        findings.violation(message);
    }

    findings.verdict("deps")
}

/// The opt-in native library an application in [`NATIVE_PERMITTED`] turns
/// on by default, if one does.
fn asked_by_an_application(
    members: &[metadata::Member],
) -> Option<&'static crate::native::FeatureNative> {
    FEATURE_NATIVE.iter().find(|e| {
        members
            .iter()
            .filter(|m| NATIVE_PERMITTED.contains(&m.name.as_str()))
            .any(|m| {
                m.direct_deps
                    .iter()
                    .any(|d| d.features.iter().any(|f| e.asked_by.contains(&f.as_str())))
            })
    })
}

/// Whether a dependency name is an OpenTelemetry crate.
fn is_otel(name: &str) -> bool {
    OTEL_PREFIXES.iter().any(|prefix| name.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::is_otel;

    #[test]
    fn otel_crates_are_recognised_by_prefix() {
        assert!(is_otel("opentelemetry"));
        assert!(is_otel("opentelemetry_sdk"));
        assert!(is_otel("opentelemetry-otlp"));
        assert!(!is_otel("telemetry"));
    }
}
