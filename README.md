# Hello World Enclave

This test repository deploys a small Rust gRPC service in a Caution enclave. The
`SayHello` RPC is served as cleartext HTTP/2 inside the enclave, while enclave
Caddy terminates public TLS and proxies to the service with h2c.

The authoritative deployment configuration is [`caution.hcl`](./caution.hcl).
Its `upstream_protocol = "h2c"` setting requires a Caution CLI/API build that
includes platform pull request 407.

## Test a deployment locally

After deploying and pointing `chelupa.caution.dev` at the deployment, run the
client from this repository with the enclave's public IP:

```sh
make test ENCLAVE_IP=203.0.113.10
```

The client connects directly to that IP on port 443, sends
`caution.hello.v1.Greeter/SayHello`, and verifies the response. It still uses
`chelupa.caution.dev` for TLS SNI and certificate verification, so this tests the
full public TLS-to-h2c path without relying on local DNS.

Override the defaults when testing another deployment:

```sh
make test ENCLAVE_IP=203.0.113.10 TLS_DOMAIN=grpc.example.com GRPC_PORT=443
```

Run `make check` to compile the service and client and execute the local unit
tests without contacting a deployment.
