FROM rustlang/rust:nightly-slim AS builder
WORKDIR /usr/src/glorp-rust

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/glorp-rust/target/release/glorp /usr/local/bin/

CMD ["glorp"]