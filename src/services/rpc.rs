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
use tonic::{
    Request,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

use crate::{config::RpcConfig, runtime::Task, services::host_info::new_info_req};

#[derive(Clone)]
pub struct RpcClient {
    token: String,
    client: PshServiceClient<Channel>,
}

fn into_req<T>(message: T, token: &str) -> Result<Request<T>> {
    let mut req = Request::new(message);
    req.metadata_mut()
        .insert("authorization", format!("Bearer {}", token).parse()?);
    Ok(req)
}

impl RpcClient {
    pub async fn new(config: &RpcConfig, token: String) -> Result<Self> {
        let ep = Endpoint::from_shared(config.addr.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?;
        let client: PshServiceClient<Channel> = PshServiceClient::connect(ep).await?;
        Ok(Self { token, client })
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
        let req = into_req(message, &self.token)?;
        self.client.heartbeat(req).await?;
        Ok(())
    }

    pub async fn get_task(&mut self, instance_id: String) -> Result<Option<Task>> {
        let req = into_req(GetTaskReq { instance_id }, &self.token)?;

        let Some(task) = self.client.get_task(req).await?.into_inner().task else {
            return Ok(None);
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
