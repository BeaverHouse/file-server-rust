# https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c
# Host is Mac mini, so this script is customized for Mac architecture

FROM rust:latest AS builder
WORKDIR /app

# Cross-compilation : https://kerkour.com/rust-cross-compilation
COPY Cargo.toml .
RUN mkdir src
COPY src src
RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
RUN cargo build --release --target aarch64-unknown-linux-gnu


FROM gcr.io/distroless/cc-debian12:latest-arm64 AS release
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/file-server-rust .

EXPOSE 8080

CMD ["./file-server-rust"]
