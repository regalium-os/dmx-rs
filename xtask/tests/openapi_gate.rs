// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// A test asserts by panicking; see tree_gates.rs.
#![allow(clippy::expect_used, clippy::panic)]

//! The `OpenAPI` summary step and its gate, against specs that are wrong.

mod common;

use common::Tree;
use xtask::openapi::{self, SPEC};

const BARE: &str = "\
paths:
    /v1/devices:
        get:
            tags:
                - DeviceService
            description: |-
                Lists devices.
                summary: not a key, only a line of the description
            operationId: DeviceService_ListDevices
            parameters:
                - name: pageSize
        post:
            operationId: DeviceService_CreateDevice
";

#[test]
fn a_summary_is_the_method_split_into_words() {
    let cases = [
        ("DeviceService_GetDevice", "Get Device"),
        ("PropertyService_BatchGetProperties", "Batch Get Properties"),
        ("ObjectService_ReadCOVValue", "Read COV Value"),
        ("ServerService_GetServerStatus", "Get Server Status"),
    ];
    for (id, want) in cases {
        assert_eq!(openapi::summary(id).expect(id), want);
    }
}

#[test]
fn an_operation_id_that_is_not_an_identifier_is_refused() {
    let error = openapi::summary("Device Service: Get").expect_err("a space is not an id");
    assert!(error.contains("operationId"), "{error}");
}

#[test]
fn every_operation_gets_one_summary_at_its_own_indent_and_a_second_pass_adds_none() {
    let once = openapi::summarise(BARE).expect("summarised");
    assert!(once.contains("            summary: List Devices\n            operationId"));
    assert!(once.contains("            summary: Create Device\n"));
    assert_eq!(once.matches("summary: List Devices").count(), 1);
    let twice = openapi::summarise(&once).expect("summarised again");
    assert_eq!(once, twice);
}

#[test]
fn a_spec_with_no_operations_is_refused_rather_than_passed() {
    let error = openapi::summarise("openapi: 3.0.3\npaths: {}\n").expect_err("nothing to do");
    assert!(error.contains("no operationId"), "{error}");
}

#[test]
fn the_gate_names_an_operation_without_a_summary() {
    let tree = Tree::new("openapi-bare");
    tree.file(SPEC, BARE);
    let error = openapi::run(tree.root()).expect_err("no summaries must fail the gate");
    assert!(error.contains("2 of 2"), "{error}");
    assert!(error.contains("DeviceService_ListDevices"), "{error}");
}

#[test]
fn the_gate_passes_once_the_step_has_run() {
    let tree = Tree::new("openapi-written");
    tree.file(SPEC, BARE);
    openapi::write(tree.root()).expect("written");
    let summary = openapi::run(tree.root()).expect("every operation summarised");
    assert!(summary.contains("2 operations"), "{summary}");
}

#[test]
fn the_gate_fails_when_the_spec_has_no_operations_or_is_missing() {
    let tree = Tree::new("openapi-empty");
    let missing = openapi::run(tree.root()).expect_err("no file must fail the gate");
    assert!(missing.contains(SPEC), "{missing}");
    tree.file(SPEC, "openapi: 3.0.3\npaths: {}\n");
    let empty = openapi::run(tree.root()).expect_err("no operations must fail the gate");
    assert!(empty.contains("examined nothing"), "{empty}");
}
