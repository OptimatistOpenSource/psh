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

use std::{
    fs,
    process::exit,
    result::Result::Ok,
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::{bail, Error, Result};
use args::Args;
use chrono::{offset::LocalResult, TimeZone, Utc};
use clap::Parser;
use config::PshConfig;
use log::log_init;
use opentelemetry_otlp::ExportConfig;
use runtime::{Task, TaskRuntime};
use utils::check_root_privilege;

use otlp::config::OtlpConfig;
use services::{config::RpcConfig, rpc::RpcClient};

fn main() -> Result<()> {
    log_init();

    if !check_root_privilege() {
        tracing::error!("Insufficient privileges. Please run psh with root permissions.");
        exit(1);
    }

    let mut args = Args::parse();
    let mut psh_config = PshConfig::read_config(&args.config).unwrap_or_else(|e| {
        tracing::warn!("{e}, use default Psh config.");
        PshConfig::default()
    });
    let rpc_conf = psh_config.rpc();
    let otlp_conf = psh_config.otlp_conf();

    // When running as a daemon, it ignores all other cli arguments
    let component_args = if args.systemd() || args.daemon() {
        psh_config.get_component_args()
    } else {
        args.get_component_args()
    };

    if args.daemon() {
        daemon::Daemon::new(psh_config.daemon().clone()).daemon()?;
    }

    let mut task_rt = TaskRuntime::new()?;

    if let Some(args) = component_args {
        let task = Task {
            wasm_component: fs::read(&args[0])?,
            wasm_component_args: args,
            end_time: Utc.with_ymd_and_hms(3000, 1, 1, 1, 1, 1).unwrap(),
        };
        task_rt.schedule(task)?;
    };
    let task_handle = task_rt.spawn()?;

    psh_config
        .remote
        .enable
        .then(|| async_tasks(rpc_conf, otlp_conf, psh_config.take_token(), task_rt))
        .map(|it| it.join());

    let _ = task_handle.join();

    Ok(())
}

fn async_tasks(
    rpc_conf: RpcConfig,
    otlp_conf: OtlpConfig,
    token: String,
    mut task_rt: TaskRuntime,
) -> JoinHandle<Result<()>> {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let token_ = token.clone();
            let rpc_task = async move {
                let duration = Duration::from_secs(rpc_conf.duration);
                let mut client = RpcClient::new(rpc_conf, token_).await?;
                client.send_info().await?;
                loop {
                    if let Some(task) = client.heartbeat(task_rt.is_idle()).await? {
                        let end_time = match Utc.timestamp_millis_opt(task.end_time as _) {
                            LocalResult::Single(t) => t,
                            _ => bail!("Invalid task end time"),
                        };
                        let task = Task {
                            wasm_component: task.wasm,
                            wasm_component_args: task.wasm_args,
                            end_time,
                        };
                        task_rt.schedule(task)?
                    }
                    tokio::time::sleep(duration).await;
                }
                #[allow(unreachable_code)]
                Ok::<(), Error>(())
            };

            let otlp_task = async {
                let export_conf: ExportConfig = otlp_conf.into();
                otlp::otlp_tasks(export_conf, token).await?;
                Ok::<(), Error>(())
            };

            if let Err(e) = tokio::try_join!(rpc_task, otlp_task) {
                tracing::error!("Some async tasks failed:\n{e}");
            }
            Ok::<(), Error>(())
        })?;

        Ok::<(), Error>(())
    })
}
