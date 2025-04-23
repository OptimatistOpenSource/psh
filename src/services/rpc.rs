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

use anyhow::{Result, bail};
use chrono::{TimeZone, Utc, offset::LocalResult};
use psh_proto::{
    ExportDataReq, GetTaskReq, HeartbeatReq, TaskDoneReq, Unit,
    psh_service_client::PshServiceClient,
};
use std::time::Duration;
use tokio::time::sleep;
use tonic::Code;
use tonic::{
    Request,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

use crate::{config::RpcConfig, runtime::Task, services::host_info::new_info_req};

#[derive(Clone)]
pub struct RpcClient {
    token: String,
    client: PshServiceClient<Channel>,
    max_retries: u32,
    base_delay: Duration,
}

fn into_req<T>(message: T, token: &str) -> Result<Request<T>> {
    let mut req = Request::new(message);
    req.metadata_mut()
        .insert("authorization", format!("Bearer {}", token).parse()?);
    Ok(req)
}

async fn retry_with_backoff<F, T>(
    max_retries: u32,
    base_delay: Duration,
    mut operation: F,
) -> Result<T, tonic::Status>
where
    F: AsyncFnMut() -> Result<T, tonic::Status>,
{
    let mut attempts = 0;
    loop {
        match operation().await {
            Ok(resp) => break Ok(resp),
            Err(status) => {
                attempts += 1;
                if attempts >= max_retries {
                    tracing::error!("RpcClient max retries reached after {} attempts", attempts);
                    break Err(status);
                }

                let retry_delay = base_delay * (2_u32.pow(attempts - 1));

                if status.code() == Code::Unknown && status.message().contains("transport error") {
                    tracing::warn!(
                        "RpcClient transport error detected (attempt {}/{}), retrying in {:?}...",
                        attempts,
                        max_retries,
                        retry_delay
                    );
                    sleep(retry_delay).await;
                    continue;
                }
                break Err(status);
            }
        }
    }
}

impl RpcClient {
    pub async fn new(config: &RpcConfig, token: String) -> Result<Self> {
        let ep = Endpoint::from_shared(config.addr.clone())?
            // 连接相关设置
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(30))
            // TCP 相关设置
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .tcp_nodelay(true)
            // HTTP/2相关设置
            .http2_keep_alive_interval(Duration::from_secs(30))
            .keep_alive_while_idle(true)
            // 并发和限流
            .concurrency_limit(256)
            .rate_limit(5, Duration::from_secs(1))
            // TLS 配置
            .tls_config(ClientTlsConfig::new().with_native_roots())?;

        let client: PshServiceClient<Channel> = PshServiceClient::connect(ep).await?;

        Ok(Self {
            token,
            client,
            max_retries: config.max_retries.unwrap_or(3),
            base_delay: config.base_delay.unwrap_or(Duration::from_secs(1)),
        })
    }

    pub async fn send_host_info(&mut self, instance_id: String) -> Result<()> {
        let req = into_req(new_info_req(instance_id), &self.token)?;
        let resp = self.client.send_host_info(req).await?;
        tracing::trace!("{:?}", resp.get_ref());
        Ok(())
    }

    pub async fn export_data(&mut self, message: ExportDataReq) -> Result<()> {
        let req = into_req(message, &self.token)?;
        self.client.export_data(req).await?;
        Ok(())
    }
    pub async fn heartbeat(&mut self, message: HeartbeatReq) -> Result<()> {
        let token = &self.token;

        retry_with_backoff(self.max_retries, self.base_delay, async || {
            let req = into_req(message.clone(), token)
                .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
            self.client.heartbeat(req).await
        })
        .await?;
        Ok(())
    }

    pub async fn get_task(&mut self, instance_id: String) -> Result<Option<Task>> {
        let get_task_req = GetTaskReq { instance_id };
        let token = &self.token;

        let response = retry_with_backoff(self.max_retries, self.base_delay, async || {
            let req = into_req(get_task_req.clone(), token)
                .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
            self.client.get_task(req).await
        })
        .await?;
        let task = match response.into_inner().task {
            Some(task) => task,
            None => return Ok(None),
        };

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

        Ok(Some(task))
    }

    pub async fn task_done(&mut self, task_id: String) -> Result<()> {
        let req = into_req(TaskDoneReq { task_id }, &self.token)?;
        self.client.task_done(req).await?;
        Ok(())
    }

    pub async fn new_instance_id(&mut self) -> Result<String> {
        let req = into_req(Unit {}, &self.token)?;
        let resp = self.client.new_instance_id(req).await?;
        Ok(resp.into_inner().instance_id)
    }
}
