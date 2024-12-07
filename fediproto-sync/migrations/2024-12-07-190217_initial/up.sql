-- Your SQL goes here
CREATE TABLE "cached_files"(
	"id" UUID NOT NULL PRIMARY KEY,
	"file_path" VARCHAR NOT NULL
);

CREATE TABLE "synced_posts"(
	"id" UUID NOT NULL PRIMARY KEY,
	"mastodon_post_id" VARCHAR NOT NULL,
	"bsky_post_cid" VARCHAR NOT NULL,
	"bsky_post_uri" VARCHAR NOT NULL
);

CREATE TABLE "mastodon_posts"(
	"id" UUID NOT NULL PRIMARY KEY,
	"account_id" VARCHAR NOT NULL,
	"post_id" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"is_thread_post" BOOL NOT NULL,
	"previous_post_id" VARCHAR,
	"bsky_post_id" VARCHAR,
	"root_mastodon_post_id" VARCHAR
);

