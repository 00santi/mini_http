FROM rust:1.89

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 7878

CMD ["./target/release/mini_http"]
