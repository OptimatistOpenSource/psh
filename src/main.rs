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

use std::process::exit;
use std::result::Result::Ok;
use std::time::Duration;
use std::{fs, thread};

use anyhow::{bail, Error, Result};
use args::Args;
use chrono::offset::LocalResult;
use chrono::{TimeZone, Utc};
use clap::Parser;
use config::{PshConfig, RemoteConfig};
use log::log_init;
use opentelemetry_otlp::ExportConfig;
use runtime::{Task, TaskRuntime};
use services::rpc::RpcClient;
use tokio::try_join;
use utils::check_root_privilege;

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
            id: None,
            wasm_component: fs::read(&args[0])?,
            wasm_component_args: args,
            end_time: Utc.with_ymd_and_hms(3000, 1, 1, 1, 1, 1).unwrap(),
        };
        task_rt.schedule(task)?;
    };

    thread::spawn(move || -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let tasks = async_tasks(psh_config.remote.clone(), psh_config.take_token(), task_rt);
        rt.block_on(tasks)?;
        Ok(())
    })
    .join()
    .expect("The async tasks thread has panicked")?;

    Ok(())
}

async fn async_tasks(
    remote_cfg: RemoteConfig,
    token: String,
    mut task_rt: TaskRuntime,
) -> Result<()> {
    let token_ = token.clone();

    let rpc_task = async move {
        if !remote_cfg.enable {
            let handle = task_rt.spawn(None)?;
            handle.join().expect("TaskRuntime has panicked");
            return Ok(());
        }

        let duration = Duration::from_secs(remote_cfg.rpc.duration);
        let mut client = RpcClient::new(remote_cfg.rpc, token_).await?;
        task_rt.spawn(Some(client.clone()))?;
        client.send_info().await?;
        loop {
            let finished_task_id = task_rt.finished_task_id();
            if let Some(task) = client
                .heartbeat(task_rt.is_idle(), finished_task_id)
                .await?
            {
                let end_time = match Utc.timestamp_millis_opt(task.end_time as _) {
                    LocalResult::Single(t) => t,
                    _ => bail!("Invalid task end time"),
                };
                let task = Task {
                    id: Some(task.id),
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
        if !remote_cfg.enable {
            return Ok(());
        }

        let export_conf: ExportConfig = remote_cfg.otlp_conf.into();
        otlp::otlp_tasks(export_conf, token).await?;
        Ok::<(), Error>(())
    };

    try_join!(rpc_task, otlp_task)?;

    Ok(())
}
