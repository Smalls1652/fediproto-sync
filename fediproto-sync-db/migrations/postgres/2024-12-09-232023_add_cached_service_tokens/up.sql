-- Your SQL goes here



CREATE TABLE "cached_service_tokens"(
	"id" UUID NOT NULL PRIMARY KEY,
	"service_name" VARCHAR NOT NULL,
	"access_token" VARCHAR NOT NULL,
	"refresh_token" VARCHAR,
	"expires_in" TIMESTAMP,
	"scopes" VARCHAR
);

