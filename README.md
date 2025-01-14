# FediProto Sync [![Build status](https://github.com/Smalls1652/fediproto-sync/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/Smalls1652/fediproto-sync/actions/workflows/build.yml) [![MIT license](https://badgen.net/static/License/MIT/blue)](./LICENSE)

![FediProto Sync Logo](.images/fediproto-sync_logo_social.jpg)

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
  - [x] Sync video attachments.
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

### Setup

For detailed setup instructions, see the [setup guide](docs/setup.md).

### Running the daemon

You can run the daemon in the following ways:

1. Directly in your terminal.
    - Doing this will require you to keep the terminal open to keep the daemon running.
    - Not recommended for long-term use.
2. With Docker/Docker Compose.
3. With Kubernetes.
4. With `systemd` or another service manager.

## 🏗️ Building from source

For detailed building instructions, see the [building guide](docs/Building.md).

## 🗂️ Dependencies used

- [`anyhow`](https://crates.io/crates/anyhow)
- [`async-session`](https://crates.io/crates/async-session)
- [`atrium-api`](https://crates.io/crates/atrium-api)
- [`atrium-xrpc-client`](https://crates.io/crates/atrium-xrpc-client)
- [`axum`](https://crates.io/crates/axum)
- [`axum-extra`](https://crates.io/crates/axum-extra)
- [`bytes`](https://crates.io/crates/bytes)
- [`chrono`](https://crates.io/crates/chrono)
- [`diesel`](https://crates.io/crates/diesel)
- [`diesel_migrations`](https://crates.io/crates/diesel_migrations)
- [`dom_query`](https://crates.io/crates/dom_query)
- [`dotenvy`](https://crates.io/crates/dotenvy)
- [`git-version`](https://crates.io/crates/git-version)
- [`image`](https://crates.io/crates/image)
- [`ipld-core`](https://crates.io/crates/ipld-core)
- [`libipld`](https://crates.io/crates/libipld)
- [`megalodon`](https://crates.io/crates/megalodon)
- [`oauth2`](https://crates.io/crates/oauth2)
- [`openssl`](https://crates.io/crates/openssl)
- [`rand`](https://crates.io/crates/rand)
- [`reqwest`](https://crates.io/crates/reqwest)
- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)
- [`thiserror`](https://crates.io/crates/thiserror)
- [`tokio`](https://crates.io/crates/tokio)
- [`toml_edit`](https://crates.io/crates/toml_edit)
- [`tracing`](https://crates.io/crates/tracing)
- [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)
- [`uuid`](https://crates.io/crates/uuid)

## 🤝 License

The source code for this project is licensed with the [MIT License](LICENSE).
