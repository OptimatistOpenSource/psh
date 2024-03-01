#[allow(unused_imports)]
use super::pb::psh_service_client::PshServiceClient;
#[allow(unused_imports)]
use super::pb::PshResponse;

// TODO Chengdong Li
// Define RPC methods here.

#[cfg(test)]
mod rpc_tests {
    use std::future::Future;

    use super::super::pb::psh_service_server::{PshService, PshServiceServer};
    use super::*;
    use tokio::sync::oneshot;
    use tonic::{transport::Server, Request, Response, Status};
    static ADDR: &str = "[::1]:50051";

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
    }

    async fn server_shutdown(tx: oneshot::Sender<()>, last_test: impl Future<Output = ()>) {
        last_test.await;
        // Send a signal to trigger shutdown
        let _ = tx.send(());
    }

    async fn server_setup(rx: oneshot::Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
        let addr = ADDR.parse()?;
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

    #[tokio::test]
    async fn test_psh_rpc_heartbeat() {
        // Create a oneshot channel to signal shutdown
        let (tx, rx) = oneshot::channel();
        let server = server_setup(rx);
        let client = PshServiceClient::connect(format!("http://{}", ADDR));
        let heartbeat = test_heartbeat(client);
        let shutdown = server_shutdown(tx, heartbeat);

        let (ser_status, _) = tokio::join!(server, shutdown);
        assert_eq!(ser_status.unwrap(), ());
    }
}
