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
mod log;
mod otlp;
mod runtime;
mod services;

use std::time::Duration;
use std::{fs, thread};

use anyhow::{bail, Error, Result};
use args::Args;
use chrono::{TimeZone, Utc};
use clap::Parser;
use config::RemoteConfig;
use daemon::{get_daemon_wasm_args, spawn_daemon};
use log::log_init;
use nix::unistd::geteuid;
use opentelemetry_otlp::ExportConfig;
use runtime::{Task, TaskRuntime};
use services::pb::HeartbeatReq;
use services::rpc::RpcClient;
use tokio::try_join;

fn main() -> Result<()> {
    log_init();

    if !geteuid().is_root() {
        bail!("Insufficient privileges. Please run psh with root permissions.");
    }

    let args = Args::parse();
    let cfg = config::read_or_gen(args.config.clone())?;

    let wasm_with_args = match args {
        Args {
            daemon: true,
            wasm_with_args: None,
            ..
        } => {
            spawn_daemon(cfg.daemon.clone())?;
            get_daemon_wasm_args(cfg.daemon.wasm.clone())
        }
        Args {
            daemon: true,
            wasm_with_args: Some(_),
            ..
        } => {
            bail!("Invalid argument, WASM can only be configured in the config file in daemon mode")
        }
        Args {
            daemon: false,
            wasm_from_daemon_config,
            wasm_with_args,
            ..
        } => {
            if wasm_from_daemon_config {
                get_daemon_wasm_args(cfg.daemon.wasm.clone())
            } else {
                wasm_with_args
            }
        }
    };

    let task_rt = TaskRuntime::new()?;

    if let Some(args) = wasm_with_args {
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
        let tasks = async_tasks(cfg.remote, task_rt);
        rt.block_on(tasks)?;
        Ok(())
    })
    .join()
    .expect("The async tasks thread has panicked")?;

    Ok(())
}

#[expect(clippy::significant_drop_tightening)]
async fn async_tasks(remote_cfg: RemoteConfig, mut task_rt: TaskRuntime) -> Result<()> {
    let token_cloned = remote_cfg.token.clone();
    let rpc_task = async move {
        if !remote_cfg.rpc.enable {
            let handle = task_rt.spawn(
                None,
                remote_cfg.rpc.data_export.buf_size,
                "unknown".to_string(),
            )?;
            drop(task_rt);
            handle.join().expect("TaskRuntime has panicked");
            return Ok(());
        }

        let duration = Duration::from_secs(remote_cfg.rpc.heartbeat_interval);
        let mut client = RpcClient::new(&remote_cfg.rpc, token_cloned).await?;

        let instance_id = match fs::read_to_string(&remote_cfg.rpc.instance_id_file).ok() {
            Some(s) => s,
            None => {
                let instance_id = client.new_instance_id().await?;
                fs::write(&remote_cfg.rpc.instance_id_file, &instance_id)?;
                instance_id
            }
        };

        task_rt.spawn(
            Some(client.clone()),
            remote_cfg.rpc.data_export.buf_size,
            instance_id.clone(),
        )?;
        client.send_host_info(instance_id.clone()).await?;
        loop {
            let idle = task_rt.is_idle();
            if idle {
                if let Some(mut task) = client.get_task(instance_id.clone()).await? {
                    let task_id = task
                        .id
                        .as_ref()
                        .map(|it| it.to_string())
                        .expect("No task id provided");
                    task.wasm_component_args.insert(0, task_id);
                    task_rt.schedule(task)?
                }
            }

            client
                .heartbeat(HeartbeatReq {
                    instance_id: instance_id.clone(),
                    idle,
                })
                .await?;

            task_rt.finished_task_id().map(|it| client.task_done(it));

            tokio::time::sleep(duration).await;
        }
        #[allow(unreachable_code)]
        Ok::<(), Error>(())
    };

    let otlp_task = async {
        if !remote_cfg.otlp.enable {
            return Ok(());
        }

        let export_conf = ExportConfig {
            endpoint: Some(remote_cfg.otlp.addr),
            ..Default::default()
        };
        otlp::otlp_tasks(export_conf, remote_cfg.token).await?;
        Ok::<(), Error>(())
    };

    try_join!(rpc_task, otlp_task)?;

    Ok(())
}
