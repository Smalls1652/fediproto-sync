-- Your SQL goes here




CREATE TABLE "mastodon_post_retry_queue"(
	"id" UUID NOT NULL PRIMARY KEY,
	"mastodon_post_id" VARCHAR NOT NULL,
	"failure_reason" VARCHAR NOT NULL
);

