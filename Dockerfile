FROM rustlang/rust:nightly-slim
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/webserver"]