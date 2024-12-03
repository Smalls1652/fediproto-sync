# FediProto Sync

> [!WARNING]
> This project is still in development and is **not ready for use**. It is very rough around the edges and a lot of things are rapidly changing.

FediProto Sync is a daemon to sync posts from your Mastodon account to your BlueSky account. It runs in the background and will automatically sync posts on an interval.

## 🤔 Why make this?

I'm making this because I wanted to have a way to sync any posts I make on my Mastodon server to BlueSky. There are multiple alternatives to this, but I wanted to make my own and tweak it to my liking. There are two alternatives that I know of:

- [**Bridgy Fed**](https://github.com/snarfed/bridgy-fed)
  - **Bridgy Fed** is the simplest way to create a bridge between Mastodon and BlueSky. It's a hosted service that creates a *"proxy"* account on either platform and syncs posts/interactions/reposts/replies between the two. It has two big downsides for me though: **You can't control the proxied account it creates** and **it requires people on both sides to opt-in to the service**.
- [**Skymoth**](https://github.com/thilobillerbeck/skymoth)
  - **Skymoth** is an open-source hosted service that syncs posts from your Mastodon account to your BlueSky account. This is the closest alternative to what I'm making, but it's *technically* a hosted service. It's actually what I have been using in the meantime while I worked on this. It's a great service and I recommend it if you don't want to jump through the hoops of setting this up.

## 📋 What it can do

**Legend:**

| Icon | Description |
| --- | --- |
| 🛑 | Currently unsupported on BlueSky. |
| ❌ | No plans to implement. |
| 🏗️ | Not yet implemented, but will be. |
| 🚧 | Not supported on BlueSky, but plan to workaround. |

- [x] Sync posts from your Mastodon account to your BlueSky account.
  - [x] Maintain the original post's creation date.
    - *This will only be applicable if the post is synced well after it was created on Mastodon. If the post is synced within a few minutes of being created, the creation date will be the time it was synced.*
  - [x] Maintain thread structure.
  - [x] Sync image attachments.
    - [x] With any alternative text.
  - [ ] Sync video attachments. 🏗️
  - [x] Truncate posts that are too long for BlueSky's 300 character limit.
    - *Truncated posts will have a link to the original post on Mastodon.*
  - [ ] Sync content warnings. 🏗️
  - [x] Enrich links in posts.
  - [x] Enrich tags/hashtags in posts.
    - *If a post was truncated, any tags/hashtags that were in the truncated will be added to the end of the post.*
  - [ ] Sync polls. 🛑
  - [ ] Sync boosts/reblogs. 🚧
    - *There are two methods I'm looking into:*
        1. *If the original account and post are bridged to BlueSky with Bridgy Fed, the bridged post will be reposted.*
        2. *Otherwise, a link to the original post will be added to the post.*
  - [ ] Sync replies. ❌
  - [ ] Backfill older posts on Mastodon. ❌
    - *This isn't within the scope of what I want this to do. The goal is to sync posts as they are made on Mastodon.*
    - *In addition I've heard there was some jank for people, who did something similar with their Twitter posts, and how others saw the posts flooding their discover feeds on BlueSky.*
- [ ] Sync posts from your BlueSky account to your Mastodon account. 🏗 ️

## ▶️ Running

> [!NOTE]
> No pre-built binaries are available at this time. You will need to [build from source](#️-building-from-source).

### Requirements

### Mastodon - Access Token

You can create an application and get an access token for your account by:

1. Going to your Mastodon server's settings.
2. Clicking on the **"Development"** tab.
3. Clicking on the **"New application"** button.
4. Fill out a name for the application.
5. Check the `read:statuses` scope.
6. Click the **"Submit"** button at the bottom.
7. Copying the access token that is generated.

### BlueSky - App Password

- You can generate an app password [here](https://bsky.app/settings/app-passwords).
  - The **Allow access to your direct messages** checkbox **is not needed**.

### Configuration

The following environment variables are required to run the daemon:

| Name | Description |
| --- | --- |
| `DATABASE_URL` | The file path to the SQLite database. |
| `MASTODON_SERVER` | The hostname of your Mastodon server. |
| `MASTODON_ACCESS_TOKEN` | The access token for the application on Mastodon. [See more](#mastodon---access-token) |
| `BLUESKY_PDS_SERVER` | The hostname of your BlueSky PDS. If you don't host your own PDS, it's typically `bsky.social`. |
| `BLUESKY_HANDLE` | Your handle on BlueSky without the `@`. |
| `BLUESKY_APP_PASSWORD` | The app password for your account on BlueSky. [See more](#bluesky---app-password) |
| `SYNC_INTERVAL_SECONDS` | The interval in seconds to sync posts. Defaults to `30` seconds. |

You can:

1. Create a `production.env` file, with these values defined, in the directory you run the daemon in.
2. Define these environment variables in your shell before running the daemon.
3. Define the environment variables with Docker or Docker Compose.
4. Define the environment variables with Kubernetes using a mix of `ConfigMap` and `Secret` resources.

### Running the daemon

You can run the daemon in the following ways:

1. Directly in your terminal.
    - Doing this will require you to keep the terminal open to keep the daemon running.
    - Not recommended for long-term use.
2. With Docker/Docker Compose.
3. With Kubernetes.
4. With `systemd` or another service manager.

## 🏗️ Building from source

### 🧰 Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)
  - The `nightly` toolchain is preferred, but `stable` should work as well.
  - Make sure to have the toolchains for the [target platforms](https://doc.rust-lang.org/nightly/rustc/platform-support.html) you want to build for.

### 🧱 Building

#### Command-line

1. Navigate to the project directory in your terminal.
2. Run the following command to build the project:

```bash
cargo build --package fediproto-sync --release --target <TARGET>
```

Replace `<TARGET>` with the desired [target platform](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

## 🗂️ Dependencies used

- [`chrono`](https://crates.io/crates/chrono)
- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)
- [`reqwest`](https://crates.io/crates/reqwest)
- [`tokio`](https://crates.io/crates/tokio)
- [`tokio-util`](https://crates.io/crates/tokio-util)
- [`megalodon`](https://crates.io/crates/megalodon)
- [`dotenvy`](https://crates.io/crates/dotenvy)
- [`dom_query`](https://crates.io/crates/dom_query)
- [`tendril`](https://crates.io/crates/tendril)
- [`diesel`](https://crates.io/crates/diesel)
- [`tracing`](https://crates.io/crates/tracing)
- [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)
- [`atprotolib-rs`](https://github.com/Smalls1652/atprotolib-rs)

## 🤝 License

The source code for this project is licensed with the [MIT License](LICENSE).
