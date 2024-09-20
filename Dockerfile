# https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c

FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src
COPY src src
RUN cargo build --release

RUN strip target/release/file-server-rust

FROM gcr.io/distroless/cc-debian12 AS release
WORKDIR /app
COPY --from=builder /app/target/release/file-server-rust .

EXPOSE 8080

CMD ["/app/file-server-rust"]