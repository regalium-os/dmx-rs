// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! Imports only ever point inward.

use crate::metadata;
use crate::scan::Findings;
use std::path::Path;

/// The hexagon, stated once.
///
/// Each entry is a crate, the workspace siblings it may depend on, and whether
/// it is pure. `regalium-dmx-telemetry` is importable everywhere and so is
/// never listed: the seam is what every layer records through, and a rule
/// forbidding it would only be worked around.
///
/// This table is the authority. Change it here, deliberately, and say why in
/// the commit — never by routing an import around it.
///
/// One layer per line, against rustfmt's preference. The value of this
/// declaration is that it reads as a table: a reader checking whether the
/// runtime may reach a transport scans one column.
#[rustfmt::skip]
pub const LAYERS: &[Layer] = &[
    Layer { krate: "regalium-dmx-telemetry", allowed: &[], pure: true },
    Layer { krate: "regalium-dmx-core", allowed: &[], pure: true },
    Layer { krate: "regalium-dmx-artnet", allowed: &[CORE], pure: false },
    Layer { krate: "regalium-dmx-sacn", allowed: &[CORE], pure: false },
    Layer { krate: "regalium-dmx-serial", allowed: &[CORE], pure: false },
    Layer { krate: "regalium-dmx-schema", allowed: &[CORE], pure: false },
    Layer { krate: "regalium-dmx-telemetry-otel", allowed: &[], pure: false },
    Layer { krate: "regalium-dmx-runtime", allowed: TRANSPORTS, pure: false },
    Layer { krate: "regalium-dmx", allowed: EVERYTHING, pure: false },
    Layer { krate: "regalium-dmx-probe", allowed: PROBE, pure: false },
    Layer { krate: "regalium-dmx-sim", allowed: SIM, pure: false },
    Layer { krate: "regalium-dmx-server", allowed: SERVER, pure: false },
];

/// The core, which the transports frame and the schema converts from. The
/// schema reaches it because its layer contract owns the conversions between
/// the protocol types and the generated model; the reverse import stays
/// forbidden, so prost never enters the core's graph.
const CORE: &str = "regalium-dmx-core";

/// What the runtime may reach: every transport, the core, and the schema.
const TRANSPORTS: &[&str] = &[
    CORE,
    "regalium-dmx-artnet",
    "regalium-dmx-sacn",
    "regalium-dmx-serial",
    "regalium-dmx-schema",
];

/// The field probe, an application at the edge: it drives universes and
/// RDM through the driver — the facade — records through the seam and
/// exports through the adapter. It reaches the schema because it serves the
/// API the schema declares, as bacnet-rs's probe does.
const PROBE: &[&str] = &[
    "regalium-dmx",
    "regalium-dmx-telemetry-otel",
    "regalium-dmx-schema",
];

/// The rig simulator, an application at the edge like the probe: Art-Net
/// and sACN nodes and RDM responders served through the driver and nothing
/// else, so a simulated rig exercises exactly the code a real client talks
/// to. It reaches the schema because a rig is protobuf, read and written as
/// its canonical JSON.
const SIM: &[&str] = &[
    "regalium-dmx",
    "regalium-dmx-telemetry-otel",
    "regalium-dmx-schema",
];

/// The data server, an application at the edge like the probe: the probe's
/// engine served over gRPC, REST and MCP. It reaches the probe — the one
/// application-to-application edge — because the engine is the probe's
/// library and the server is its second front; a second copy of the engine
/// would be the first thing to drift. It reaches the simulator for its tests.
const SERVER: &[&str] = &[
    "regalium-dmx",
    "regalium-dmx-probe",
    "regalium-dmx-telemetry-otel",
    "regalium-dmx-schema",
    "regalium-dmx-sim",
];

/// The facade re-exports, so it reaches everything beneath the edge.
const EVERYTHING: &[&str] = &[
    CORE,
    "regalium-dmx-artnet",
    "regalium-dmx-sacn",
    "regalium-dmx-serial",
    "regalium-dmx-schema",
    "regalium-dmx-runtime",
    "regalium-dmx-telemetry-otel",
];

/// One crate's position in the hexagon.
pub struct Layer {
    /// The crate this rule governs.
    pub krate: &'static str,
    /// Workspace siblings it may depend on, besides the telemetry seam.
    pub allowed: &'static [&'static str],
    /// Whether it must perform no I/O and read no clock.
    pub pure: bool,
}

/// The seam every layer may reach.
pub const SEAM: &str = "regalium-dmx-telemetry";

/// Checks that every declared layer exists and depends only inward.
///
/// # Errors
///
/// When a layer is missing, an undeclared crate appears in the workspace, or a
/// dependency inverts the hexagon.
pub fn run(root: &Path) -> Result<String, String> {
    let members = metadata::workspace_members(root)?;
    let mut findings = Findings::default();

    for layer in LAYERS {
        findings.checked();
        let Some(member) = members.iter().find(|m| m.name == layer.krate) else {
            findings.violation(format!(
                "{} is declared in the layer table but absent from the workspace; a \
                 table describing a tree that is not there checks nothing",
                layer.krate
            ));
            continue;
        };
        for dep in &member.workspace_deps {
            if dep == SEAM || layer.allowed.contains(&dep.as_str()) {
                continue;
            }
            findings.violation(format!(
                "{} depends on {dep}, which it may not: a dependency in this direction \
                 inverts the hexagon. Permitted here: {:?} (plus {SEAM}, always)",
                layer.krate, layer.allowed
            ));
        }
    }

    for member in &members {
        if member.name == "xtask" || LAYERS.iter().any(|l| l.krate == member.name) {
            continue;
        }
        findings.violation(format!(
            "{} is in the workspace but not in the layer table; add it there with the \
             imports it may make, rather than leaving it ungoverned",
            member.name
        ));
    }

    findings.verdict("arch")
}
