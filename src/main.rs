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
mod daemon;
mod infra;
mod log;
mod otlp;
mod resources;
mod runtime;
mod security;
mod services;
mod utils;

use std::{process::exit, sync::Arc};

use anyhow::Context;
use args::Args;
use clap::Parser;
use config::PshConfig;
use log::log_init;
use opentelemetry_otlp::ExportConfig;
use runtime::PshEngineBuilder;
use utils::check_root_privilege;

fn main() -> anyhow::Result<()> {
    log_init();

    if !check_root_privilege() {
        tracing::error!("Insufficient privileges. Please run psh with root permissions.");
        exit(1);
    }

    let mut args = Args::parse();
    let mut psh_config = PshConfig::read_config(PshConfig::DEFAULT_PATH).unwrap_or_else(|e| {
        tracing::warn!("{e}, use default Psh config.");
        PshConfig::default()
    });

    // When running as a daemon, it ignores all other cli arguments
    let component_args = if args.systemd() || args.daemon() {
        psh_config.get_component_args()
    } else {
        args.get_component_args()
    };
    let component_envs: Vec<(String, String)> = std::env::vars().collect();

    if args.daemon() {
        daemon::Daemon::new(psh_config.daemon().clone()).daemon()?;
    }

    let otlp_conf = psh_config.otlp_conf();
    let mut otlp_th = None;

    let runtime = Arc::new(tokio::runtime::Runtime::new()?);

    let luoxiao = psh_config.luoxiao();
    let addr = luoxiao.addr().to_owned();
    let token = luoxiao.token().to_owned();
    runtime.spawn(async move {
        match services::rpc::Luoxiao::new(&addr).await {
            Ok(mut lx) => {
                if let Err(e) = lx.send_info(token).await {
                    tracing::error!("send info to luoxiao: {}", e)
                };
            }
            Err(e) => tracing::error!("connect to luoxiao: {}", e),
        }
    });

    if otlp_conf.enable() {
        let rt = Arc::clone(&runtime);
        let export_conf: ExportConfig = otlp_conf.into();
        let hd = std::thread::spawn(move || rt.block_on(otlp::otlp_tasks(export_conf)));
        otlp_th = Some(hd);
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
