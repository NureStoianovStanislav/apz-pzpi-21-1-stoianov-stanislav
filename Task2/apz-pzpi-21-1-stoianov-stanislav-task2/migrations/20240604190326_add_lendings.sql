-- Create "lendings" table
CREATE TABLE "public"."lendings" (
  "id" bigserial NOT NULL,
  "book_id" bigint NOT NULL,
  "lendee_id" bigint NOT NULL,
  "lent_on" date NOT NULL,
  "due" date NOT NULL,
  "returned_on" date NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "lendings_book_id_fkey" FOREIGN KEY ("book_id") REFERENCES "public"."books" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "lendings_lendee_id_fkey" FOREIGN KEY ("lendee_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
