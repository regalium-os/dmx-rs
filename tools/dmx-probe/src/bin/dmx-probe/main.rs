// Copyright 2026 RegaliumOS™.
// SPDX-License-Identifier: Apache-2.0

//! `dmx-probe`: discover, watch and drive Art-Net, sACN and USB-DMX, in an
//! interactive TUI.
//!
//! `--demo` runs the probe over a rig in memory — consoles, nodes and RDM
//! fixtures on a simulated bus — so it can be tried, and shown, without a
//! lighting network.

mod tui;

use clap::Parser;
use dmx_probe::engine::Engine;
use dmx_probe::sim;
use std::process::ExitCode;

/// Discover, watch and drive DMX512 over Art-Net, sACN and USB-DMX.
#[derive(Debug, Parser)]
#[command(name = "dmx-probe", version)]
struct Cli {
    /// Run over a simulated rig in memory rather than a network.
    #[arg(long)]
    demo: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if !cli.demo {
        eprintln!(
            "dmx-probe: the live rig is the next step of Phase 8; run `dmx-probe --demo` for now"
        );
        return ExitCode::FAILURE;
    }
    let engine = match sim::demo().and_then(Engine::start) {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    match tui::run(engine) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
