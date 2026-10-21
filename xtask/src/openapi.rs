// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Every operation in the generated `OpenAPI` description has a `summary`.
//!
//! gnostic's `protoc-gen-openapi` writes an RPC's comment as the operation's
//! `description` and never sets a `summary`; its only way to set one is a
//! `gnostic.openapi.v3.operation` annotation on every RPC, which would put an
//! OpenAPI-only dependency into a schema the Rust, MCP and mirror generators
//! also read. So `just gen-openapi` runs [`summarise`] after it, deriving
//! each summary from the `operationId`: `DeviceService_GetDevice` becomes
//! `Get Device`, as shokki-engineering/freebusy's merge tool does.
//!
//! The edit is textual — one line inserted beside each `operationId` — rather
//! than a YAML round trip, which would reformat the whole generated file and
//! need a YAML crate for one key. The [`run`] gate fails when a regeneration
//! skipped the step.

use std::path::Path;

/// The generated description, relative to the repository root.
pub const SPEC: &str = "protobuf/generated/openapi/openapi.yaml";

const ID_KEY: &str = "operationId: ";
const SUMMARY_KEY: &str = "summary: ";

/// `text` with a `summary` beside every `operationId` that lacks one.
/// Idempotent: a second pass changes nothing.
///
/// # Errors
///
/// When there is no operation at all — a spec with nothing to summarise is
/// the wrong file, not a finished one — or an `operationId` is not a plain
/// identifier, so its summary could not be written as a bare YAML scalar.
pub fn summarise(text: &str) -> Result<String, String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = String::with_capacity(text.len() + lines.len() * 4);
    let mut operations = 0;
    for (at, line) in lines.iter().enumerate() {
        if let Some((indent, id)) = operation_id(line) {
            operations += 1;
            let summary = summary(id)?;
            if !has_summary(&lines, at, indent) {
                out.push_str(&" ".repeat(indent));
                out.push_str(SUMMARY_KEY);
                out.push_str(&summary);
                out.push('\n');
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    if operations == 0 {
        return Err("no operationId found: nothing to summarise".to_owned());
    }
    Ok(out)
}

/// Rewrites [`SPEC`] under `root` through [`summarise`]: the step
/// `just gen-openapi` runs after the generator. Returns what it did.
///
/// # Errors
///
/// When the file cannot be read or written, or [`summarise`] refuses it.
pub fn write(root: &Path) -> Result<String, String> {
    let path = root.join(SPEC);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{SPEC}: {e}"))?;
    let summarised = summarise(&text)?;
    std::fs::write(&path, &summarised).map_err(|e| format!("{SPEC}: {e}"))?;
    let added = summarised
        .lines()
        .count()
        .saturating_sub(text.lines().count());
    Ok(format!("{SPEC}: {added} summaries added"))
}

/// The gate: every operation in [`SPEC`] under `root` has a summary.
///
/// # Errors
///
/// When the file is missing, has no operations, or an operation lacks a
/// summary — run `just gen-openapi`, which adds them.
pub fn run(root: &Path) -> Result<String, String> {
    let path = root.join(SPEC);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{SPEC}: {e}"))?;
    let lines: Vec<&str> = text.lines().collect();
    let mut operations = 0;
    let mut missing = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        if let Some((indent, id)) = operation_id(line) {
            operations += 1;
            if !has_summary(&lines, at, indent) {
                missing.push(id);
            }
        }
    }
    if operations == 0 {
        return Err(format!(
            "openapi: examined nothing; {SPEC} has no operationId"
        ));
    }
    if !missing.is_empty() {
        return Err(format!(
            "openapi: {} of {operations} operations in {SPEC} have no summary ({}); \
             run `just gen-openapi`",
            missing.len(),
            missing.join(", ")
        ));
    }
    Ok(format!(
        "openapi: {operations} operations, each with a summary"
    ))
}

/// The indent and value of an `operationId` line.
fn operation_id(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_start();
    let id = trimmed.strip_prefix(ID_KEY)?;
    Some((line.len() - trimmed.len(), id.trim()))
}

/// Whether the operation whose `operationId` is on line `at` already has a
/// `summary`: a key at the same indent, in the run of lines at that indent
/// or deeper on either side of it.
fn has_summary(lines: &[&str], at: usize, indent: usize) -> bool {
    let in_operation = |line: &&&str| line.trim().is_empty() || depth(line) >= indent;
    let is_summary =
        |line: &&str| depth(line) == indent && line.trim_start().starts_with(SUMMARY_KEY);
    let before = lines.get(..at).unwrap_or_default();
    let after = lines.get(at..).unwrap_or_default();
    before.iter().rev().take_while(in_operation).any(is_summary)
        || after.iter().take_while(in_operation).any(is_summary)
}

fn depth(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// `DeviceService_GetDevice` → `Get Device`: the method after the last
/// underscore, split into words at each capital that starts one, so an
/// acronym stays whole (`ReadCOVValue` → `Read COV Value`).
///
/// # Errors
///
/// When the method is empty or holds anything but ASCII letters and digits.
pub fn summary(operation_id: &str) -> Result<String, String> {
    let method = operation_id.rsplit('_').next().unwrap_or_default();
    if method.is_empty() || !method.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!(
            "{operation_id:?}: not a Service_Method operationId"
        ));
    }
    let chars: Vec<char> = method.chars().collect();
    let mut words = String::with_capacity(method.len() + 8);
    for (i, c) in chars.iter().enumerate() {
        let prev = i.checked_sub(1).and_then(|p| chars.get(p));
        let next = chars.get(i + 1);
        let starts_word = c.is_ascii_uppercase()
            && match prev {
                None => false,
                Some(p) if p.is_ascii_uppercase() => next.is_some_and(char::is_ascii_lowercase),
                Some(_) => true,
            };
        if starts_word {
            words.push(' ');
        }
        words.push(*c);
    }
    Ok(words)
}
