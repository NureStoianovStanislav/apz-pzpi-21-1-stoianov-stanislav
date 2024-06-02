-- Modify "users" table
ALTER TABLE "public"."users"
ADD CONSTRAINT "users_role_check" CHECK ((role)::text = ANY ((ARRAY['administrator'::character varying, 'client'::character varying])::text[])),
ADD COLUMN "role" character varying(32) NULL;

UPDATE "public"."users"          
SET "role" = 'client';           

ALTER TABLE "public"."users"     
ALTER COLUMN "role" SET NOT NULL;
