-- Your SQL goes here



CREATE TABLE "cached_service_tokens"(
	"id" UUID NOT NULL PRIMARY KEY,
	"service_name" TEXT NOT NULL,
	"access_token" TEXT NOT NULL,
	"refresh_token" TEXT,
	"expires_in" INTEGER,
	"scopes" TEXT
);

