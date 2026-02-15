FROM rustlang/rust:nightly-slim AS builder
WORKDIR /usr/src/glorp-rust

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/glorp-rust/target/release/rust-ci /usr/local/bin/

CMD ["rust-ci"]