-- Your SQL goes here




CREATE TABLE `mastodon_post_retry_queue`(
	`id` BIGINT NOT NULL PRIMARY KEY,
	`failure_reason` TEXT NOT NULL,
	`last_retried_at` TIMESTAMP NOT NULL,
	`retry_count` INTEGER NOT NULL
);

