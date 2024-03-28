# syntax=docker/dockerfile:1
FROM rust:1.77.0 as build
WORKDIR /app
COPY . /app
RUN env OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/ OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-gnu/openssl/ OPENSSL_STATIC=yes \
    cargo install --locked --path .

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y git curl && rm -rf /var/lib/apt/lists/*
RUN curl -LOJR https://download.docker.com/linux/debian/dists/bookworm/pool/stable/amd64/docker-ce-cli_25.0.5-1~debian.12~bookworm_amd64.deb && \
    dpkg -i docker-ce-cli_25.0.5-1~debian.12~bookworm_amd64.deb && \
    rm docker-ce-cli_25.0.5-1~debian.12~bookworm_amd64.deb
COPY --from=build /usr/local/cargo/bin/hooked /usr/local/bin/hooked
