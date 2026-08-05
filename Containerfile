FROM stagex/pallet-rust@sha256:9c38bf1066dd9ad1b6a6b584974dd798c2bf798985bf82e58024fbe0515592ca AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY proto ./proto
COPY src ./src

RUN RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --locked --release --target x86_64-unknown-linux-musl --bin grpc-hello-server

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/grpc-hello-server /usr/local/bin/grpc-hello-server

EXPOSE 8083

ENTRYPOINT ["/usr/local/bin/grpc-hello-server"]
