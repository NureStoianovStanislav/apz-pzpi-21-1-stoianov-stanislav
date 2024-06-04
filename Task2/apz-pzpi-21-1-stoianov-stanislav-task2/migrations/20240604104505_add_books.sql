-- Create "books" table
CREATE TABLE "public"."books" (
  "id" bigserial NOT NULL,
  "year" smallint NOT NULL,
  "name" character varying(50) NOT NULL,
  "genre" character varying(50) NOT NULL,
  "author" character varying(50) NOT NULL,
  "library_id" bigint NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "books_library_id_fkey" FOREIGN KEY ("library_id") REFERENCES "public"."libraries" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
