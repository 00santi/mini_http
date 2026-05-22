FROM rust:1.95 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/mini_http /app/mini_http

EXPOSE 7878
CMD ["./mini_http"]
