FROM --platform=${TARGETPLATFORM} docker.io/library/rust:1-bullseye AS build

WORKDIR /app

COPY . .

RUN apt-get update \
    && apt-get install -y libsqlite3-dev libsqlite3-0 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
    
RUN cargo build --package "fediproto-sync" --release

FROM --platform=${TARGETPLATFORM} docker.io/library/debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y libsqlite3-0 ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/fediproto-sync /usr/local/bin/fediproto-sync

WORKDIR /app

ENV DATABASE_URL="/app/fediproto-sync.db"

CMD ["fediproto-sync"]
