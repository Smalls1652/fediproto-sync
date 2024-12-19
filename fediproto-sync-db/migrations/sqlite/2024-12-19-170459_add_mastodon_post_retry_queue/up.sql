-- Your SQL goes here




CREATE TABLE `mastodon_post_retry_queue`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`mastodon_post_id` TEXT NOT NULL,
	`failure_reason` TEXT NOT NULL
);

