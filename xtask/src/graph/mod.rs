// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! The resolved dependency graph, walked edge by edge.
//!
//! `metadata` answers "which crates are in the build"; this answers "which
//! crates does *this* member pull in". The difference matters as soon as a
//! rule is scoped to a layer: native code beneath the telemetry adapter is a
//! decision, the same native code beneath the core is a defect, and only the
//! edges tell the two apart.

mod walk;

use crate::features::activated;
use crate::metadata;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// One crate in the resolved graph.
pub struct Node {
    /// The crate name.
    pub name: String,
    /// The native library it links, when it declares one.
    pub links: Option<String>,
    /// Names of the crates it build-depends on.
    pub build_deps: Vec<String>,
    /// The features it declares, enabled or not.
    pub declared: Vec<String>,
    /// Ids of what it depends on for normal and build use, never dev.
    deps: Vec<String>,
}

/// The whole graph, and which nodes are workspace members.
pub struct Graph {
    nodes: HashMap<String, Node>,
    members: Vec<String>,
}

impl Graph {
    /// Asks cargo for the graph as resolved for one target triple.
    ///
    /// Per target, because a dependency is often platform-specific: a crate
    /// that compiles C only when building for Haiku compiles nothing here,
    /// and a gate that cannot tell the difference cries wolf until ignored.
    ///
    /// # Errors
    ///
    /// When cargo cannot be run, or answers with something unparseable.
    pub fn resolve(root: &Path, target: &str) -> Result<Self, String> {
        Self::resolve_with(root, target, &[])
    }

    /// As [`Graph::resolve`], with `features` (`crate/feature`) enabled as
    /// well as every default: how an opt-in feature's graph is examined.
    ///
    /// # Errors
    ///
    /// As [`Graph::resolve`], and when a feature does not exist.
    pub fn resolve_with(root: &Path, target: &str, features: &[&str]) -> Result<Self, String> {
        let joined = features.join(",");
        let mut args = vec![
            "metadata",
            "--format-version",
            "1",
            "--filter-platform",
            target,
        ];
        if !features.is_empty() {
            args.extend(["--features", joined.as_str()]);
        }
        let json = metadata::run_cargo(root, &args)?;
        let packages = json
            .get("packages")
            .and_then(Value::as_array)
            .ok_or("cargo metadata produced no packages array")?;
        let resolved = json
            .pointer("/resolve/nodes")
            .and_then(Value::as_array)
            .ok_or("cargo metadata produced no resolve graph")?;
        let names: HashMap<&str, &Value> = packages
            .iter()
            .filter_map(|p| Some((p.get("id")?.as_str()?, p)))
            .collect();
        let mut nodes = HashMap::new();
        for node in resolved {
            let Some(id) = node.get("id").and_then(Value::as_str) else {
                continue;
            };
            let package = names.get(id).copied();
            let text = |field| package.and_then(|p| p.get(field)).and_then(Value::as_str);
            let enabled: Vec<&str> = node
                .get("features")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            let mut deps = Vec::new();
            let mut build_deps = Vec::new();
            for dep in node
                .get("deps")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let kinds = dep.get("dep_kinds").and_then(Value::as_array);
                let kind_of = |k: &Value| k.get("kind").and_then(Value::as_str).map(str::to_owned);
                let kinds: Vec<Option<String>> = kinds.into_iter().flatten().map(kind_of).collect();
                let Some(pkg) = dep.get("pkg").and_then(Value::as_str) else {
                    continue;
                };
                let extern_name = dep.get("name").and_then(Value::as_str).unwrap_or_default();
                if !activated(package, &enabled, extern_name) {
                    continue;
                }
                if kinds.iter().any(|k| k.as_deref() != Some("dev")) {
                    deps.push(pkg.to_owned());
                }
                if kinds.iter().any(|k| k.as_deref() == Some("build"))
                    && let Some(name) = dep.get("name").and_then(Value::as_str)
                {
                    build_deps.push(name.replace('_', "-"));
                }
            }
            let name = text("name").unwrap_or(id).to_owned();
            let links = text("links").map(str::to_owned);
            let declared = package
                .and_then(|p| p.get("features"))
                .and_then(Value::as_object)
                .map(|f| f.keys().cloned().collect())
                .unwrap_or_default();
            nodes.insert(
                id.to_owned(),
                Node {
                    name,
                    links,
                    build_deps,
                    declared,
                    deps,
                },
            );
        }
        let members = json
            .get("workspace_members")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|m| m.as_str().map(str::to_owned))
            .collect();
        Ok(Self { nodes, members })
    }
}
