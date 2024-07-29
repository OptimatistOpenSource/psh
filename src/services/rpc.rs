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

use crate::services::{
    host_info::RawInfo,
    pb::{psh_service_client::PshServiceClient, HostInfoRequest},
};

#[derive(Clone, Debug)]
pub struct Luoxiao {
    client: PshServiceClient<tonic::transport::Channel>,
}

impl Luoxiao {
    pub async fn new(addr: &str) -> Result<Self> {
        let client: PshServiceClient<tonic::transport::Channel> =
            PshServiceClient::connect(format!("https://{}", addr)).await?;
        Ok(Self { client })
    }

    pub async fn send_info(&mut self, token: String) -> Result<()> {
        let info: HostInfoRequest = RawInfo::new(token).into();

        let resp = self.client.send_host_info(info).await?;

        let resp = resp.get_ref();

        tracing::info!("{:?}", resp.message);

        Ok(())
    }
}

#[cfg(test)]
mod rpc_tests {
    use std::future::Future;

    use sysinfo::System;
    use tokio::sync::oneshot;
    use tonic::{transport::Server, Request, Response, Status};

    use self::psh_service_client::PshServiceClient;
    use crate::{
        infra::{option::WrapOption, result::WrapResult},
        services::{
            pb::{
                psh_service_server::{PshService, PshServiceServer},
                *,
            },
            rpc::Luoxiao,
        },
    };

    static ADDR: &str = "[::1]:50051";
    static ADDR_INFO: &str = "[::1]:50052";

    #[allow(dead_code)]
    const ADDR_LUOXIAO: &str = "[::1]:7878";

    #[ignore]
    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        let mut lx = Luoxiao::new(ADDR_LUOXIAO).await?;
        lx.send_info("psh token".to_owned()).await?;

        Ok(())
    }

    // For testing purpose, implement a simple heartbeat RPC at server side.
    #[derive(Debug, Default)]
    pub struct MyPshService {}

    #[tonic::async_trait]
    impl PshService for MyPshService {
        async fn heartbeat(&self, request: Request<()>) -> Result<Response<PshResponse>, Status> {
            println!("host: {}", request.remote_addr().unwrap());
            let resp = PshResponse {
                resp: "beep".to_string(),
            };

            Ok(Response::new(resp))
        }

        async fn send_host_info(
            &self,
            request: tonic::Request<HostInfoRequest>,
        ) -> std::result::Result<tonic::Response<HostInfoResponse>, tonic::Status> {
            let req = request.into_inner();
            if let Some(ip) = req.ip_addr {
                match ip {
                    host_info_request::IpAddr::Ipv4(v4) => {
                        let v4 = std::net::Ipv4Addr::from_bits(v4);
                        println!("{}", v4);
                    }
                    host_info_request::IpAddr::Ipv6(_v6) => {
                        println!("v6");
                    }
                }
            }
            let resp = HostInfoResponse {
                errno: 0,
                message: "ok".to_owned().wrap_some(),
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

    type ClientChannelResult =
        Result<PshServiceClient<tonic::transport::Channel>, tonic::transport::Error>;
    async fn test_heartbeat(client: impl Future<Output = ClientChannelResult>) {
        let resp = client.await.unwrap().heartbeat(()).await.unwrap();

        assert_eq!(resp.get_ref().resp, "beep");
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
            token: "token".to_owned(),
            ip_addr: host_info_request::IpAddr::Ipv4(0xFF00FF00).wrap_some(),
            os: System::name(),
            hostname: "MTS-MILA".to_owned().wrap_some(),
            architecture: System::cpu_arch(),
            kernel_version: System::kernel_version(),
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
