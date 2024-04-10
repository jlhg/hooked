# syntax=docker/dockerfile:1
FROM rust:1.77.2 as build
WORKDIR /app
COPY . /app
RUN env OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/ OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-gnu/openssl/ OPENSSL_STATIC=yes \
    cargo install --locked --path .

FROM debian:bookworm-slim
ARG USERNAME=service
ARG USER_UID=1000
ARG USER_GID=1000
WORKDIR /app
RUN apt-get update && apt-get install -y git curl sudo && rm -rf /var/lib/apt/lists/*
RUN curl -LOJR https://download.docker.com/linux/debian/dists/bookworm/pool/stable/amd64/docker-ce-cli_26.0.0-1~debian.12~bookworm_amd64.deb && \
    dpkg -i docker-ce-cli_26.0.0-1~debian.12~bookworm_amd64.deb && \
    rm docker-ce-cli_26.0.0-1~debian.12~bookworm_amd64.deb && \
    curl -LOJR https://download.docker.com/linux/debian/dists/bookworm/pool/stable/amd64/docker-compose-plugin_2.25.0-1~debian.12~bookworm_amd64.deb && \
    dpkg -i docker-compose-plugin_2.25.0-1~debian.12~bookworm_amd64.deb && \
    rm docker-compose-plugin_2.25.0-1~debian.12~bookworm_amd64.deb
COPY --from=build /usr/local/cargo/bin/hooked /usr/local/bin/hooked
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
RUN groupadd --system docker && \
    usermod -aG docker $USERNAME
USER $USERNAME
