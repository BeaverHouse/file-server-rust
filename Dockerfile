# https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c

FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src
COPY src src
RUN cargo build --release

RUN strip target/release/file-server-rust

FROM alpine:latest AS release
WORKDIR /app
COPY --from=builder /app/target/release/file-server-rust .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

CMD ["./file-server-rust"]