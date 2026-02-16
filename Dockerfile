FROM rustlang/rust:nightly-slim AS builder
WORKDIR /build
COPY ./ ./
RUN cargo build --release


FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app/glorp
COPY --from=builder /build /app/glorp/

CMD ["./target/release/glorp"]