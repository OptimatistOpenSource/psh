// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

mod args;
mod config;
mod infra;
mod otlp;
mod resources;
mod runtime;
mod security;
mod services;
mod utils;

use std::process::exit;

use anyhow::Context;
use clap::Parser;

use args::Args;
use config::PshConfig;
use opentelemetry_otlp::ExportConfig;
use otlp::config::OtlpConfig;
use runtime::PshEngineBuilder;
use utils::check_root_privilege;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    if !check_root_privilege() {
        tracing::error!("Insufficient privileges. Please run psh with root permissions.");
        exit(1);
    }

    let args = Args::parse();

    // when running as a daemon, it ignores the other arguments from the cli
    let component_args = if args.daemon() {
        let psh_config = PshConfig::read_config(PshConfig::DEFAULT_PATH).unwrap_or_else(|e| {
            tracing::warn!("Error: {e}, use default Psh config.");
            PshConfig::default()
        });
        if !psh_config.check_vaild() {
            tracing::error!("The configuration must specify WASM path.");
            exit(1);
        }
        psh_config.into_component_args()
    } else {
        let mut component_args: Vec<String> = vec![args.psh_wasm_component];
        component_args.extend(args.extra_args);
        component_args
    };
    let component_envs: Vec<(String, String)> = std::env::vars().collect();

    let otlp_conf = OtlpConfig::read_config(OtlpConfig::DEFAULT_PATH).unwrap_or_else(|e| {
        tracing::warn!("Error: {e}, use default OpenTelemetry config.");
        OtlpConfig::default()
    });
    let mut otlp_th = None;

    if otlp_conf.enable() {
        let export_conf: ExportConfig = otlp_conf.into();
        let th = std::thread::spawn(|| {
            tokio::runtime::Runtime::new()?.block_on(otlp::otlp_tasks(export_conf))
        });
        otlp_th = Some(th);
    }

    let mut engine = PshEngineBuilder::new()
        .wasi_inherit_stdio()
        .wasi_envs(&component_envs)
        .wasi_args(&component_args)
        .allow_perf_op(true)
        .allow_system_op(true)
        .build()
        .context("Failed to build PshEngine.")?;

    engine.run(&component_args[0])?;

    if let Some(th) = otlp_th {
        let _ = th.join();
    }

    Ok(())
}
