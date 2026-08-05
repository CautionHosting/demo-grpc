use std::env;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use hello::greeter_client::GreeterClient;
use hello::HelloRequest;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tonic::transport::{ClientTlsConfig, Endpoint, Uri};
use tonic::Request;
use tower::service_fn;

pub mod hello {
    tonic::include_proto!("caution.hello.v1");
}

const DEFAULT_GRPC_PORT: u16 = 443;
const DEFAULT_TLS_DOMAIN: &str = "chelupa.caution.dev";
const TEST_NAME: &str = "local client";
const EXPECTED_MESSAGE: &str = "Hello, local client! Caution gRPC is working.";

fn required_enclave_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    let value = env::var("ENCLAVE_IP").map_err(|_| "ENCLAVE_IP is required")?;
    value
        .parse()
        .map_err(|_| format!("ENCLAVE_IP must be an IPv4 or IPv6 address: {value:?}").into())
}

fn grpc_port() -> Result<u16, Box<dyn std::error::Error>> {
    let value = env::var("GRPC_PORT").unwrap_or_else(|_| DEFAULT_GRPC_PORT.to_string());
    let port: u16 = value
        .parse()
        .map_err(|_| format!("GRPC_PORT must be an integer from 1 to 65535: {value:?}"))?;
    if port == 0 {
        return Err("GRPC_PORT must be an integer from 1 to 65535".into());
    }
    Ok(port)
}

fn tls_domain() -> Result<String, Box<dyn std::error::Error>> {
    let domain = env::var("TLS_DOMAIN").unwrap_or_else(|_| DEFAULT_TLS_DOMAIN.to_string());
    let valid = domain.len() <= 253
        && domain.contains('.')
        && domain.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        });
    if !valid {
        return Err(format!("TLS_DOMAIN must be a fully qualified hostname: {domain:?}").into());
    }
    Ok(domain)
}

fn endpoint_uri(tls_domain: &str, port: u16) -> String {
    format!("https://{tls_domain}:{port}")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = required_enclave_ip()?;
    let port = grpc_port()?;
    let tls_domain = tls_domain()?;
    let uri = endpoint_uri(&tls_domain, port);
    let socket_address = SocketAddr::new(ip, port);

    let endpoint = Endpoint::from_shared(uri)?
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_config(
            ClientTlsConfig::new()
                .domain_name(tls_domain.clone())
                .with_webpki_roots(),
        )?;
    let channel = endpoint
        .connect_with_connector(service_fn(move |_: Uri| async move {
            TcpStream::connect(socket_address).await.map(TokioIo::new)
        }))
        .await?;
    let mut client = GreeterClient::new(channel);
    let response = client
        .say_hello(Request::new(HelloRequest {
            name: TEST_NAME.to_string(),
        }))
        .await?
        .into_inner();

    if response.message != EXPECTED_MESSAGE {
        return Err(format!(
            "unexpected SayHello response: expected {EXPECTED_MESSAGE:?}, got {:?}",
            response.message
        )
        .into());
    }

    println!("gRPC test passed for {ip}:{port} (TLS domain {tls_domain})");
    println!("{}", response.message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_uri_uses_the_tls_domain_as_authority() {
        assert_eq!(
            endpoint_uri("grpc.example.com", 8443),
            "https://grpc.example.com:8443"
        );
    }
}
