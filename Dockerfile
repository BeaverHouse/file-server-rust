# https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c
# Host is Mac mini, so this script is customized for Mac architecture

FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src
COPY src src
RUN apt-get update && apt-get install gcc-multilib gcc-aarch64-linux-gnu -y
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --target aarch64-unknown-linux-gnu

RUN strip target/aarch64-unknown-linux-gnu/release/file-server-rust



FROM gcr.io/distroless/cc-debian12:latest-arm64 AS release
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/file-server-rust .

EXPOSE 8080

CMD ["./file-server-rust"]