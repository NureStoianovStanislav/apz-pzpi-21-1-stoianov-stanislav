-- Create "libraries" table
CREATE TABLE "public"."libraries" (
  "id" bigserial NOT NULL,
  "name" character varying(50) NOT NULL,
  "address" character varying(100) NOT NULL,
  "owner_id" bigint NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "libraries_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
