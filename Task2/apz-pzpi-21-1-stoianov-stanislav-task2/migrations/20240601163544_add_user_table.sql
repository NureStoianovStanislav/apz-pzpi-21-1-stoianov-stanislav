-- Create "users" table
CREATE TABLE "public"."users" (
  "id" bigserial NOT NULL,
  "email" character varying(50) NOT NULL,
  "password_hash" text NOT NULL,
  "refresh_token" uuid NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "users_email_key" UNIQUE ("email"),
  CONSTRAINT "users_refresh_token_key" UNIQUE ("refresh_token")
);
