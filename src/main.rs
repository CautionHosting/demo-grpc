use std::net::SocketAddr;

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("caution.hello.v1");
}

const LISTEN_PORT: u16 = 8083;
const MAX_NAME_BYTES: usize = 128;

#[derive(Default)]
struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        if name.is_empty() {
            return Err(Status::invalid_argument("name is required"));
        }
        if name.len() > MAX_NAME_BYTES {
            return Err(Status::invalid_argument("name must be at most 128 bytes"));
        }
        if name.chars().any(char::is_control) {
            return Err(Status::invalid_argument(
                "name must not contain control characters",
            ));
        }

        Ok(Response::new(HelloReply {
            message: format!("Hello, {name}! Caution gRPC is working."),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = SocketAddr::from(([0, 0, 0, 0], LISTEN_PORT));
    println!("gRPC server listening on {address}");

    Server::builder()
        .add_service(GreeterServer::new(GreeterService))
        .serve(address)
        .await?;

    Ok(())
}
