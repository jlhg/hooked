# syntax=docker/dockerfile:1
FROM rust:1.77.0 as build
WORKDIR /app
COPY . /app
RUN env OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/ OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-gnu/openssl/ OPENSSL_STATIC=yes \
    cargo install --locked --path .

FROM debian:bookworm-slim
WORKDIR /app
# RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists/*
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
COPY --from=build /usr/local/cargo/bin/hooked /usr/local/bin/hooked
