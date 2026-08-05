.DEFAULT_GOAL := check

GRPC_PORT ?= 443
TLS_DOMAIN ?= chelupa.caution.dev

export ENCLAVE_IP
export GRPC_PORT
export TLS_DOMAIN

.PHONY: check test

check:
	cargo test --locked

test:
	@if [ -z "$${ENCLAVE_IP:-}" ]; then \
		echo "ENCLAVE_IP is required; usage: make test ENCLAVE_IP=203.0.113.10" >&2; \
		exit 2; \
	fi
	cargo run --locked --quiet --bin grpc-hello-client
