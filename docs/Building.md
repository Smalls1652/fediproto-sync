# 🏗️ Building from source

## Prerequisites

### Core

* [Rust](https://www.rust-lang.org/tools/install)
  * The `nightly` toolchain is **preferred**. `stable` *might* work, but is not guaranteed.

### Linux

You will need the following packages installed on your system (Names vary by distribution) or you can build them from source:

* `gcc`
* `g++`
* `make`
* `libc6`
* `cmake`
* `pkg-config`
* `libssl`
* `libpq`
* `libsqlite3`
* `git`

### macOS

> [!NOTE]
> Ensure that you have the latest Xcode Command Line Tools installed by running the following in your terminal:
> 
> ```bash
> xcode-select --install
> ```

> [!NOTE]
> You will need to install [Homebrew](https://brew.sh/) to install the following packages.

* `gcc`
* `g++`
* `make`
* `cmake`
* `pkg-config`
* `perl`
* `git`

## Building

1. Clone the repository to your local machine.
2. Navigate to where you cloned the repository in your terminal.
3. Run the following command to build the project:

```bash
cargo build --package fediproto-sync --release
```

The compiled binary will be located at `target/release/fediproto-sync`.
