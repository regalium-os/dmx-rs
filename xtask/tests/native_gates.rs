// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

// A test asserts by panicking, so the workspace's never-panic lints are not
// the rule here: an expect that cannot fire is how a test states what it
// requires, and one that fires is the test doing its job.
#![allow(clippy::expect_used, clippy::panic)]

//! The one exception to "nothing links C", proved narrow.
//!
//! An audited native library is accepted beneath the telemetry adapter and
//! nowhere else, and an unaudited one is refused even there. Each test plants
//! a stub `-sys` crate with a `links` key and a build script, then wires it
//! beneath exactly one layer.

mod common;

use common::{Tree, legal_workspace, rewrite_crate};
use xtask::deps;

/// A stub crate that declares it links `library`, as a real `-sys` crate does.
fn native_crate(tree: &Tree, name: &str, library: &str) {
    common::stub_crate(tree, name);
    tree.file(
        &format!("crates/{name}/Cargo.toml"),
        &format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n\
             edition = \"2024\"\nlinks = \"{library}\"\nbuild = \"build.rs\"\n"
        ),
    );
    tree.file(
        &format!("crates/{name}/build.rs"),
        &format!("{}fn main() {{}}\n", common::NOTICE),
    );
}

#[test]
fn an_audited_native_library_is_accepted_beneath_the_telemetry_adapter() {
    let tree = legal_workspace("native-adapter");
    native_crate(&tree, "zstd-sys", "zstd");
    rewrite_crate(
        &tree,
        "regalium-dmx-telemetry-otel",
        &["regalium-dmx-telemetry", "zstd-sys"],
    );

    deps::run(tree.root()).expect("zstd is audited, and the adapter may hold it");
}

#[test]
fn an_audited_native_library_is_refused_beneath_a_transport() {
    let tree = legal_workspace("native-transport");
    native_crate(&tree, "zstd-sys", "zstd");
    rewrite_crate(
        &tree,
        "regalium-dmx-artnet",
        &["regalium-dmx-core", "zstd-sys"],
    );

    let error = deps::run(tree.root()).expect_err("a transport must stay a one-flag cross build");
    assert!(
        error.contains("regalium-dmx-artnet pulls in zstd-sys"),
        "{error}"
    );
    assert!(error.contains("accepted only beneath"), "{error}");
}

#[test]
fn an_unaudited_native_library_is_refused_even_beneath_the_adapter() {
    let tree = legal_workspace("native-unaudited");
    native_crate(&tree, "openssl-sys", "ssl");
    rewrite_crate(
        &tree,
        "regalium-dmx-telemetry-otel",
        &["regalium-dmx-telemetry", "openssl-sys"],
    );

    let error = deps::run(tree.root()).expect_err("only audited libraries are accepted");
    assert!(error.contains("links the native library ssl"), "{error}");
    assert!(error.contains("not in AUDITED_NATIVE"), "{error}");
}

#[test]
fn an_audited_name_with_a_different_library_is_not_the_audited_crate() {
    let tree = legal_workspace("native-impostor");
    native_crate(&tree, "zstd-sys", "ssl");
    rewrite_crate(
        &tree,
        "regalium-dmx-telemetry-otel",
        &["regalium-dmx-telemetry", "zstd-sys"],
    );

    let error = deps::run(tree.root()).expect_err("the audit covers a crate and its library");
    assert!(error.contains("links the native library ssl"), "{error}");
}

/// The Art-Net transport with `openssl-sys` as an optional dependency, enabled by
/// its `tls` feature, and `default` set as given.
fn optional_native(tree: &Tree, default: &str) {
    native_crate(tree, "openssl-sys", "ssl");
    tree.file(
        "crates/regalium-dmx-artnet/Cargo.toml",
        &format!(
            "[package]\nname = \"regalium-dmx-artnet\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
             [dependencies]\nregalium-dmx-core = {{ path = \"../regalium-dmx-core\" }}\n\
             openssl-sys = {{ path = \"../openssl-sys\", optional = true }}\n\n\
             [features]\ndefault = [{default}]\ntls = [\"dep:openssl-sys\"]\n"
        ),
    );
}

#[test]
fn a_native_library_behind_a_disabled_optional_dependency_is_not_charged() {
    let tree = legal_workspace("native-optional-off");
    optional_native(&tree, "");

    deps::run(tree.root()).expect("nothing enables the dependency, so nothing builds it");
}

#[test]
fn a_native_library_behind_an_enabled_optional_dependency_is_charged() {
    let tree = legal_workspace("native-optional-on");
    optional_native(&tree, "\"tls\"");

    let error = deps::run(tree.root()).expect_err("the default feature builds it");
    assert!(
        error.contains("regalium-dmx-artnet pulls in openssl-sys"),
        "{error}"
    );
}

#[test]
fn libudev_is_refused_beneath_the_serial_transport() {
    let tree = legal_workspace("native-serial-udev");
    native_crate(&tree, "libudev-sys", "udev");
    rewrite_crate(
        &tree,
        "regalium-dmx-serial",
        &["regalium-dmx-core", "libudev-sys"],
    );

    let error = deps::run(tree.root()).expect_err("USB-DMX enumerates through sysfs, not C");
    assert!(error.contains("links the native library udev"), "{error}");
}
