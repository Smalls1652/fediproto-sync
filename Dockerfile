FROM --platform=${BUILDPLATFORM} docker.io/library/rust:1-bullseye AS build

ARG TARGETPLATFORM
ARG TARGETARCH

WORKDIR /app

COPY . .

RUN apt-get update \
    && apt-get install -y \
        g++-x86-64-linux-gnu \
        libc6-dev-amd64-cross \
        g++-aarch64-linux-gnu \
        libc6-dev-arm64-cross \
        pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
    
RUN chmod +x ./docker-build/build.sh \
    && ./docker-build/build.sh

FROM --platform=${TARGETARCH:-$BUILDPLATFORM} docker.io/library/debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y libsqlite3-0 ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /tmp/fediproto-sync /usr/local/bin/fediproto-sync

WORKDIR /app

ENV DATABASE_URL="/app/fediproto-sync.db"

CMD ["fediproto-sync"]
