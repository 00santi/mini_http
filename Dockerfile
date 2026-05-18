FROM rust:1.89 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/mini_http /app/mini_http
EXPOSE 7878
CMD ["./mini_http"]
