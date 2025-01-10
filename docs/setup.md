# Setup

## Requirements

### Mastodon

You can create an application and get an access token for your account by:

1. Go to your Mastodon server's settings.
2. Clicking on the **"Development"** tab.
3. Clicking on the **"New application"** button.
4. Fill out a name for the application.
5. Set the **Redirect URI** to `http://localhost:3000/auth/mastodon/authorized`.
6. Check the following scopes:
    * `read:statuses`
    * `profile`
7. Click the **"Submit"** button at the bottom.
8. Copy the **Client key** and **Client secret** values generated.

### BlueSky - App Password

* You can generate an app password [here](https://bsky.app/settings/app-passwords).
  * The **Allow access to your direct messages** checkbox **is not needed**.

### Token encryption keys

> [!WARNING]
> Changing these keys will invalidate all existing tokens.

You can generate a public/private key-pair a few different ways. The options provided below will output the keys in a `.env` format in the terminal. You can then copy and paste the values into your `.env` file or pipe them directly into the `.env` file.

#### `fediproto-sync`

```bash
fediproto-sync generate-token-encryption-keys
```

#### Shell

You can either run a provided shell script:

```bash
./scripts/generateTokenKeys.sh
```

Or run the following commands in your terminal:

```bash
PRIVATE_KEY_PATH="${TMPDIR}fediproto-private_key.pem"
PUBLIC_KEY_PATH="${TMPDIR}fediproto-public_key.pem"

openssl genrsa -out $PRIVATE_KEY_PATH 4096 > /dev/null 2>&1
BASE64_PRIVATE_KEY=$(cat $PRIVATE_KEY_PATH | base64 -w 0)

openssl rsa -in $PRIVATE_KEY_PATH -pubout -out $PUBLIC_KEY_PATH > /dev/null 2>&1
BASE64_PUBLIC_KEY=$(cat $PUBLIC_KEY_PATH | base64 -w 0)

rm -f $PRIVATE_KEY_PATH
rm -f $PUBLIC_KEY_PATH

echo "TOKEN_ENCRYPTION_PRIVATE_KEY=\"${BASE64_PRIVATE_KEY}\""
echo ""
echo "TOKEN_ENCRYPTION_PUBLIC_KEY=\"${BASE64_PUBLIC_KEY}\""
```

## Configuration

The following environment variables are required to run the daemon:

| Name | Description |
| --- | --- |
| `FEDIPROTO_SYNC_MODE` * | The mode to run the daemon in. **Valid values:** `normal`, `auth` |
| `AUTH_SERVER_ADDRESS` | The address to bind the auth server to. **Default:** `localhost` |
| `AUTH_SERVER_PORT` | The port to bind the auth server to. **Default:** `3000` |
| `DATABASE_TYPE` * | The type of database to use. **Valid values:** `SQLite`, `Postgres` |
| `DATABASE_URL` * | The URL to the PostgreSQL database to use. |
| `TOKEN_ENCRYPTION_PRIVATE_KEY` * | The private key to use for encrypting tokens in a base64 format. |
| `TOKEN_ENCRYPTION_PUBLIC_KEY` * | The public key to use for decrypting tokens in a base64 format. |
| `USER_AGENT` | The user agent string to use for HTTP requests. |
| `MASTODON_SERVER` * | The hostname of your Mastodon server. |
| `MASTODON_CLIENT_ID` * | The client ID for the Mastodon application. |
| `MASTODON_CLIENT_SECRET` * | The client secret for the Mastodon application. |
| `MASTODON_REDIRECT_URI` * | The redirect URI for the Mastodon application. |
| `BLUESKY_PDS_SERVER` * | The hostname of your BlueSky PDS. If you don't host your own PDS, it's typically `bsky.social`. |
| `BLUESKY_HANDLE` * | Your handle on BlueSky without the `@`. |
| `BLUESKY_APP_PASSWORD` * | The app password for your account on BlueSky. [See more](#bluesky---app-password) |
| `BLUESKY_VIDEO_ALWAYS_FALLBACK` | Whether to always fallback to the video URL. **Default:** `false` |
| `SYNC_INTERVAL_SECONDS` | The interval in seconds to sync posts. Defaults to `300` seconds. |

**Note:** The `*` icon indicates that the environment variable is required.

You can:

1. Create a `production.env` file, with these values defined, in the directory you run the daemon in.
2. Define these environment variables in your shell before running the daemon.
3. Define the environment variables with Docker or Docker Compose.
4. Define the environment variables with Kubernetes using a mix of `ConfigMap` and `Secret` resources.

## Initial setup

> [!WARNING]
> All of the required environment variables [in the previous section](#configuration) must be defined before running the daemon.

1. Set the `FEDIPROTO_SYNC_MODE` environment variable to `auth`.
2. Run the daemon.
3. Navigate to [`http://localhost:3000/auth/mastodon/login`](http://localhost:3000/auth/mastodon/login) in your browser and follow the instructions for logging in with Mastodon.
4. Once you've logged on, stop the daemon.
