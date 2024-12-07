-- Your SQL goes here
CREATE TABLE "mastodon_posts"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"account_id" TEXT NOT NULL,
	"post_id" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"is_thread_post" BOOL NOT NULL,
	"previous_post_id" TEXT,
	"bsky_post_id" TEXT,
	"root_mastodon_post_id" TEXT
);

CREATE TABLE "synced_posts"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"mastodon_post_id" TEXT NOT NULL,
	"bsky_post_cid" TEXT NOT NULL,
	"bsky_post_uri" TEXT NOT NULL
);

CREATE TABLE "cached_files"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"file_path" TEXT NOT NULL
);

