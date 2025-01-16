# Install the latest nightly toolchain for 
# 'x86_64-unknown-linux-gnu' and 'aarch64-unknown-linux-gnu'.
FROM --platform=${BUILDPLATFORM} docker.io/library/rust:1-bookworm AS base

RUN rustup default nightly \
    && rustup target add --toolchain "nightly" "x86_64-unknown-linux-gnu" \
    && rustup target add --toolchain "nightly" "aarch64-unknown-linux-gnu" \
    && cargo install --force cargo-make

# Install OS dependencies.
FROM --platform=${BUILDPLATFORM} base AS dependencies

ARG DEBIAN_FRONTEND="noninteractive"
ARG BUILD_ENVIRONMENT="CONTAINER"
ARG TARGETPLATFORM
ARG TARGETARCH

WORKDIR /app

ADD --keep-git-dir . .

RUN cargo make install-linux-dependencies-container

# Build the app.
FROM --platform=${BUILDPLATFORM} dependencies AS build

ARG DEBIAN_FRONTEND="noninteractive"
ARG BUILD_ENVIRONMENT="CONTAINER"
ARG TARGETPLATFORM
ARG TARGETARCH

WORKDIR /app

RUN cargo make build-fediproto-sync-container

# Final layer.
FROM --platform=${TARGETARCH:-$BUILDPLATFORM} docker.io/library/debian:bookworm-slim

ARG DEBIAN_FRONTEND="noninteractive"
ARG TARGETARCH

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-0 \
        libpq5 \
        openssl \
        ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /tmp/fediproto-sync/linux_${TARGETARCH}/fediproto-sync /usr/local/bin/fediproto-sync

WORKDIR /app

CMD ["fediproto-sync"]
