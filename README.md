# gRPC on a Caution Enclave

This repository is a focused integration demo for serving native gRPC from a
[Caution](https://caution.co/) enclave. It verifies that Caution's public HTTPS
ingress can terminate TLS for an HTTP/2 gRPC connection and proxy the request to
an enclave application using cleartext HTTP/2 (`h2c`).

It is intentionally a small test fixture, not a production application or a
general-purpose Rust service template. The service exposes one unary RPC,
`caution.hello.v1.Greeter/SayHello`, implemented with
[`tonic`](https://github.com/hyperium/tonic).

## Request path

```text
gRPC client
    |
    | TLS + HTTP/2 on port 443
    v
Caution public ingress
    |
    | h2c on port 8083
    v
Rust gRPC service inside the enclave
```

The important deployment setting is in [`caution.hcl`](./caution.hcl):

```hcl
http {
  domain            = "chelupa.caution.dev"
  port              = 8083
  upstream_protocol = "h2c"
}
```

Without `upstream_protocol = "h2c"`, the ingress would not preserve the HTTP/2
transport required by gRPC on its connection to the service.

## Repository contents

- [`proto/hello.proto`](./proto/hello.proto) defines the demo API.
- [`src/main.rs`](./src/main.rs) implements the gRPC server listening on port
  `8083`.
- [`src/bin/grpc-hello-client.rs`](./src/bin/grpc-hello-client.rs) is an
  end-to-end probe for a deployed enclave.
- [`caution.hcl`](./caution.hcl) configures the enclave and its h2c ingress.
- [`Containerfile`](./Containerfile) produces a minimal image containing the
  statically linked server.

## Check the project locally

With Rust and Cargo installed, compile the server and client and run the unit
tests:

```sh
make check
```

This does not start an enclave or contact a deployment.

## Verify a deployed enclave

Deploy the repository through Caution and configure the domain in
[`caution.hcl`](./caution.hcl) to point at the deployment. Then run the included
probe with the enclave's public IP:

```sh
make test ENCLAVE_IP=203.0.113.10
```

The probe connects directly to that IP on port `443`, but uses
`chelupa.caution.dev` as the TLS server name and certificate identity. This
deliberately avoids relying on local DNS while still testing the complete public
TLS-to-h2c request path. It calls `SayHello` and fails unless the response has
the expected value.

For a deployment using a different domain or public port, override the client
defaults:

```sh
make test \
  ENCLAVE_IP=203.0.113.10 \
  TLS_DOMAIN=grpc.example.com \
  GRPC_PORT=443
```

`TLS_DOMAIN` must match both the deployed ingress domain and the certificate
presented by it.
