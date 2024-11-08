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

use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tonic::{
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request,
};

use crate::{
    runtime::{Task, TaskRuntime},
    services::{
        host_info::RawInfo,
        pb::{psh_service_client::PshServiceClient, HostInfoRequest},
    },
};

use super::config::RpcConfig;

#[derive(Clone)]
pub struct RpcClient {
    token: String,
    client: PshServiceClient<Channel>,
    raw_info: RawInfo,
    duration: Duration,
    instance_id_file: String,
    task_runtime: Arc<Mutex<TaskRuntime>>,
}

impl RpcClient {
    pub async fn new(
        config: RpcConfig,
        token: String,
        task_runtime: Arc<Mutex<TaskRuntime>>,
    ) -> Result<Self> {
        let ep = Endpoint::from_shared(config.addr)?
            .tls_config(ClientTlsConfig::new().with_native_roots())?;
        let client: PshServiceClient<Channel> = PshServiceClient::connect(ep).await?;
        let raw_info = RawInfo::new(&config.instance_id_file);
        Ok(Self {
            duration: Duration::from_secs(config.duration),
            token,
            client,
            raw_info,
            instance_id_file: config.instance_id_file,
            task_runtime,
        })
    }

    pub async fn send_info(&mut self) -> Result<()> {
        let req: Request<HostInfoRequest> = {
            let req: HostInfoRequest = (&self.raw_info).into();
            let mut req = Request::new(req);
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        let resp = self.client.send_host_info(req).await?;

        let resp = resp.get_ref();
        if let Some(id) = &resp.instance_id {
            self.raw_info
                .set_instance_id(id.clone(), &self.instance_id_file);
        };

        tracing::trace!("{:?}", resp);

        Ok(())
    }

    pub async fn heartbeat(&mut self) -> Result<()> {
        let req: Request<HostInfoRequest> = {
            let raw_info = self.raw_info.to_heartbeat();
            let mut req: HostInfoRequest = raw_info.into();
            req.idle = self.task_runtime.lock().unwrap().is_idle();
            let mut req = Request::new(req);
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        let resp = self.client.send_host_info(req).await?;
        let resp = resp.into_inner();
        tracing::trace!("{:?}", &resp);

        if let Some(id) = &resp.instance_id {
            self.raw_info
                .set_instance_id(id.clone(), &self.instance_id_file);
        }
        if let Some(wasm_component) = resp.wasm_component {
            self.task_runtime.lock().unwrap().schedule(Task {
                wasm_component,
                wasm_component_args: vec![],
            })?
        }

        Ok(())
    }

    pub async fn rpc_tasks(&mut self) -> Result<()> {
        self.send_info().await?;
        loop {
            if let Err(e) = self.heartbeat().await {
                tracing::error!("heartbeat: {e}");
            }
            tokio::time::sleep(self.duration).await;
        }
    }
}

#[cfg(test)]
mod rpc_tests {
    use std::{future::Future, net::Ipv4Addr};

    use tokio::sync::oneshot;
    use tonic::transport::{Channel, Error, Server};

    use self::psh_service_client::PshServiceClient;
    use crate::{
        infra::{option::WrapOption, result::WrapResult},
        services::{
            host_info::RawInfo,
            pb::{
                psh_service_server::{PshService, PshServiceServer},
                *,
            },
        },
    };

    static ADDR: &str = "[::1]:50051";
    static ADDR_INFO: &str = "[::1]:50052";

    // For testing purpose, implement a simple heartbeat RPC at server side.
    #[derive(Debug, Default)]
    pub struct MyPshService {}

    #[tonic::async_trait]
    impl PshService for MyPshService {
        async fn send_host_info(
            &self,
            request: tonic::Request<HostInfoRequest>,
        ) -> std::result::Result<tonic::Response<HostInfoResponse>, tonic::Status> {
            let addr = request.remote_addr().unwrap();
            dbg!(addr.ip());
            let resp = HostInfoResponse {
                errno: None,
                message: "ok".to_owned().wrap_some(),
                instance_id: None,
            };
            tonic::Response::new(resp).wrap_ok()
        }
    }

    async fn server_shutdown(tx: oneshot::Sender<()>, last_test: impl Future<Output = ()>) {
        last_test.await;
        // Send a signal to trigger shutdown
        let _ = tx.send(());
    }

    async fn server_setup(
        rx: oneshot::Receiver<()>,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = addr.parse()?;
        let heartbeat = MyPshService::default();

        Server::builder()
            .add_service(PshServiceServer::new(heartbeat))
            .serve_with_shutdown(addr, async {
                let _ = rx.await;
                println!("Received shutdown signal, shutting down gracefully...");
            })
            .await?;
        Ok(())
    }

    type ClientChannelResult = Result<PshServiceClient<Channel>, Error>;
    async fn test_heartbeat(client: impl Future<Output = ClientChannelResult>) {
        let info: HostInfoRequest = RawInfo::new(String::new()).into();
        let resp = client.await.unwrap().send_host_info(info).await.unwrap();

        assert!(resp.get_ref().errno.is_none())
    }

    async fn test_send_info(
        client: impl Future<Output = ClientChannelResult>,
        req: impl tonic::IntoRequest<HostInfoRequest>,
    ) {
        let resp = client.await.unwrap().send_host_info(req).await.unwrap();

        assert_eq!(resp.get_ref().message.as_ref().unwrap(), "ok");
    }

    async fn server_shutdown_info(tx: oneshot::Sender<()>, last_test: impl Future<Output = ()>) {
        last_test.await;
        // Send a signal to trigger shutdown
        let _ = tx.send(());
    }

    #[tokio::test]
    async fn test_psh_send_info() {
        let (tx, rx) = oneshot::channel();
        let server = server_setup(rx, ADDR_INFO);
        let client = PshServiceClient::connect(format!("http://{}", ADDR_INFO));
        let info_req = HostInfoRequest {
            os: "Linux".to_owned().wrap_some(),
            hostname: "Host".to_owned().wrap_some(),
            architecture: "x86_64".to_owned().wrap_some(),
            kernel_version: "6.10.2".to_owned().wrap_some(),
            local_ipv4_addr: Some(Ipv4Addr::new(127, 0, 0, 1).to_bits()),
            local_ipv6_addr: None,
            instance_id: None,
        };

        let heartbeat = test_send_info(client, info_req);
        let shutdown = server_shutdown_info(tx, heartbeat);

        let (ser_status, _) = tokio::join!(server, shutdown);
        assert!(ser_status.is_ok());
    }

    #[tokio::test]
    async fn test_psh_rpc_heartbeat() {
        // Create a oneshot channel to signal shutdown
        let (tx, rx) = oneshot::channel();
        let server = server_setup(rx, ADDR);
        let client = PshServiceClient::connect(format!("http://{}", ADDR));
        let heartbeat = test_heartbeat(client);
        let shutdown = server_shutdown(tx, heartbeat);

        let (ser_status, _) = tokio::join!(server, shutdown);
        assert!(ser_status.is_ok());
    }
}
