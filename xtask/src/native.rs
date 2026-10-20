// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Native code in each member's closure: refused, but for the audited
//! libraries beneath the telemetry adapter and the opt-in features below.

use crate::deps::{AUDITED_NATIVE, LINK_MARKERS, NATIVE_PERMITTED};
use crate::graph::{Graph, Node};
use crate::scan::Findings;
use std::collections::BTreeSet;

/// A native library one crate's opt-in feature brings: accepted beneath
/// that crate only, in the graph resolved with the feature on, and charged
/// to any member that reaches it some other way.
pub struct FeatureNative {
    /// The crate whose feature it is.
    pub owner: &'static str,
    /// The feature.
    pub feature: &'static str,
    /// The native crate.
    pub krate: &'static str,
    /// Its `links` value, up to the version the crate appends.
    pub links_prefix: &'static str,
    /// Why it is accepted.
    pub why: &'static str,
    /// The features, the owner's or one forwarding to it, by which an
    /// application in `NATIVE_PERMITTED` may turn it on by default.
    pub asked_by: &'static [&'static str],
}

/// The opt-in native libraries. Add one only after reading its build script.
///
/// Empty: no transport needs C yet. The first candidate is USB-DMX on Linux,
/// where the usual serial crates reach libudev for port enumeration; the
/// serial transport is to enumerate through sysfs instead, so this stays
/// empty. If that fails, the entry goes here, scoped to one feature of
/// `regalium-dmx-serial`, with the reason.
pub const FEATURE_NATIVE: &[FeatureNative] = &[];

/// Checks every member of `graph`; `exception` is the feature it was
/// resolved with, when it was.
pub fn check_native(
    graph: &Graph,
    exception: Option<&FeatureNative>,
    findings: &mut Findings,
    out: &mut BTreeSet<String>,
) {
    for member in graph.member_names() {
        let permitted = NATIVE_PERMITTED.contains(&member.as_str());
        // What the member reaches other than through the feature's owner.
        let around: BTreeSet<String> = exception
            .map(|e| graph.closure_avoiding(&member, Some(e.owner)))
            .unwrap_or_default()
            .iter()
            .map(|n| n.name.clone())
            .collect();
        let excused = |node: &Node| {
            exception.is_some_and(|e| {
                node.name == e.krate
                    && node
                        .links
                        .as_deref()
                        .is_some_and(|l| l.starts_with(e.links_prefix))
                    && (member == e.owner || !around.contains(&node.name))
            })
        };
        let closure = graph.closure(&member);
        let audited = |node: &Node| AUDITED_NATIVE.iter().any(|(name, _, _)| *name == node.name);
        // cc is there for some crate's build script; it is excused only when
        // every such crate is.
        let cc_excused = closure
            .iter()
            .filter(|n| n.build_deps.iter().any(|d| d == "cc"))
            .all(|n| excused(n) || (permitted && audited(n)));
        for node in &closure {
            findings.checked();
            if excused(node) {
                continue;
            }
            if let Some(library) = node.links.as_deref().filter(|l| !LINK_MARKERS.contains(l)) {
                let accepted = permitted
                    && AUDITED_NATIVE
                        .iter()
                        .any(|(n, l, _)| *n == node.name && *l == library);
                if !accepted {
                    out.insert(native(&member, &node.name, library, permitted));
                }
            }
            if node.name == "cc" && !permitted && !(exception.is_some() && cc_excused) {
                out.insert(format!(
                    "{member} pulls in cc, so some dependency compiles C at build time; find \
                     it with `cargo tree -p {member} -i cc`. Lighting nodes are ARM, and a C \
                     toolchain is what turns a one-flag cross build into a cross-compilation \
                     project"
                ));
            }
            if permitted && !audited(node) && node.build_deps.iter().any(|d| d == "cc") {
                out.insert(format!(
                    "{} compiles C with cc beneath {member} and is not an audited native \
                     library; read its build script, then add it to AUDITED_NATIVE with the \
                     reason, or replace the dependency",
                    node.name
                ));
            }
        }
    }
}

fn native(member: &str, krate: &str, library: &str, permitted: bool) -> String {
    let rule = if permitted {
        "it is not in AUDITED_NATIVE; read its build script before adding it there".to_owned()
    } else {
        format!(
            "native code is accepted only beneath {NATIVE_PERMITTED:?}, and an opt-in \
             feature's only beneath the crate whose feature it is, so the driver itself \
             stays a one-flag cross build"
        )
    };
    format!(
        "{member} pulls in {krate}, which links the native library {library}; {rule}. Find \
         the path with `cargo tree -p {member} -i {krate}`"
    )
}
