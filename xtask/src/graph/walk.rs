// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Walking the resolved graph: members, declared features, closures.

use super::{Graph, Node};
use std::collections::BTreeSet;

impl Graph {
    /// Every workspace member's name.
    #[must_use]
    pub fn member_names(&self) -> Vec<String> {
        self.members
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|n| n.name.clone())
            .collect()
    }

    /// Whether the crate named `name` declares `feature`.
    #[must_use]
    pub fn declares(&self, name: &str, feature: &str) -> bool {
        self.nodes
            .values()
            .any(|n| n.name == name && n.declared.iter().any(|f| f == feature))
    }

    /// Everything `member` pulls in for normal and build use, itself
    /// excluded, sorted by name so a report reads the same everywhere.
    #[must_use]
    pub fn closure(&self, member: &str) -> Vec<&Node> {
        self.closure_avoiding(member, None)
    }

    /// As [`Graph::closure`], but never walking into the crate named
    /// `avoid`: what `member` pulls in other than through it.
    #[must_use]
    pub fn closure_avoiding(&self, member: &str, avoid: Option<&str>) -> Vec<&Node> {
        let Some(start) = self
            .members
            .iter()
            .find(|id| self.nodes.get(*id).is_some_and(|n| n.name == member))
        else {
            return Vec::new();
        };
        let mut seen = BTreeSet::new();
        let mut stack = vec![start.clone()];
        while let Some(id) = stack.pop() {
            if !seen.insert(id.clone()) {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                if &id != start && avoid == Some(node.name.as_str()) {
                    seen.remove(&id);
                    continue;
                }
                stack.extend(node.deps.iter().cloned());
            }
        }
        seen.remove(start);
        let mut found: Vec<&Node> = seen.iter().filter_map(|id| self.nodes.get(id)).collect();
        found.sort_by(|a, b| a.name.cmp(&b.name));
        found
    }
}
