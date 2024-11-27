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

mod builder;
mod data_export;
mod engine;
mod state;

#[cfg(test)]
mod tests;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use anyhow::Context;
use anyhow::Result;
pub use builder::PshEngineBuilder;
use chrono::DateTime;
use chrono::Utc;
use data_export::DataExportCtx;
pub use engine::PshEngine;
pub use state::PshState;

use crate::services::rpc::RpcClient;

pub struct Task {
    pub wasm_component: Vec<u8>,
    pub wasm_component_args: Vec<String>,
    pub end_time: DateTime<Utc>,
}

pub struct TaskRuntime {
    tx: Sender<Task>,
    rx: Option<Receiver<Task>>,
    len: Arc<AtomicUsize>,
}

impl TaskRuntime {
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();

        Ok(Self {
            tx,
            rx: Some(rx),
            len: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn schedule(&mut self, task: Task) -> Result<()> {
        self.len.fetch_add(1, Ordering::Release);
        self.tx.send(task)?;
        Ok(())
    }

    pub fn is_idle(&self) -> bool {
        let len = self.len.load(Ordering::Acquire);
        len == 0
    }

    pub fn spawn(&mut self, rpc_client: Option<RpcClient>) -> Result<JoinHandle<()>> {
        let rx = match self.rx.take() {
            Some(rx) => rx,
            None => panic!("twice spawned"),
        };

        let envs: Vec<(String, String)> = std::env::vars().collect();
        let data_export_ctx = DataExportCtx { rpc_client };

        let len = self.len.clone();
        let handle = thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                let mut envs = envs.clone();
                let task_time_slice = {
                    let delta = task.end_time.timestamp_millis() - Utc::now().timestamp_millis();
                    delta.max(0) as u64
                };
                envs.push(("TASK_TIME_SLICE".to_string(), task_time_slice.to_string()));
                let engine = PshEngineBuilder::new()
                    .wasi_inherit_stdio()
                    .wasi_envs(&envs)
                    .wasi_args(&task.wasm_component_args)
                    .allow_perf_op(true)
                    .allow_system_op(true)
                    .allow_data_export_op(Some(data_export_ctx.clone()))
                    .build()
                    .context("Failed to build PshEngine.");

                match engine {
                    Ok(o) => {
                        let _ = o.run(&task.wasm_component, task_time_slice);
                    }
                    Err(e) => eprintln!("{}", e),
                };

                len.fetch_sub(1, Ordering::Release);
            }
        });

        Ok(handle)
    }
}
