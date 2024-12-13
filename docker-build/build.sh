#!/bin/sh

RUST_TARGET=""
case "${TARGETPLATFORM}" in
    "linux/amd64")
        RUST_TARGET="x86_64-unknown-linux-gnu"
        export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-linux-gnu-gcc"
        export CC_x86_64_unknown_linux_gnu="x86_64-linux-gnu-gcc"
        export CXX_x86_64_unknown_linux_gnu="x86_64-linux-gnu-g++"

        #export X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu/"
        #export X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR=""

        dpkg --add-architecture amd64
        apt-get update
        apt-get install -y libpq-dev:amd64 libpq5:amd64 libsqlite3-dev:amd64 libsqlite3-0:amd64 libssl-dev:amd64
        apt-get clean
        rm -rf /var/lib/apt/lists/*
        ;;
    "linux/arm64")
        RUST_TARGET="aarch64-unknown-linux-gnu"
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-gcc"
        export CC_aarch64_unknown_linux_gnu="aarch64-linux-gnu-gcc"
        export CXX_aarch64_unknown_linux_gnu="aarch64-linux-gnu-g++"

        dpkg --add-architecture arm64
        apt-get update
        apt-get install -y libpq-dev:arm64 libpq5:arm64 libsqlite3-dev:arm64 libsqlite3-0:arm64 libssl-dev:arm64
        apt-get clean
        rm -rf /var/lib/apt/lists/*
        ;;
    *)
        echo "Unsupported platform: ${TARGETPLATFORM}"
        exit 1
        ;;
esac

export FEDIPROTOSYNC_INCLUDE_COMMIT_HASH="true"
export FEDIPROTOSYNC_UPDATE_MANIFEST_VERSION="true"
export PKG_CONFIG_SYSROOT_DIR=/

rustup default nightly
rustup target add --toolchain "nightly" "${RUST_TARGET}"
cargo build --package "fediproto-sync" --release --target "${RUST_TARGET}"
cp "./target/${RUST_TARGET}/release/fediproto-sync" "/tmp/fediproto-sync"
