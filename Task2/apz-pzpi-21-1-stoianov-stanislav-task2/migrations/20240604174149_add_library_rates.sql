-- Modify "libraries" table
ALTER TABLE "public"."libraries"
ADD CONSTRAINT "libraries_currency_check" CHECK ((currency)::text = ANY ((ARRAY['UAH'::character varying, 'USD'::character varying, 'EUR'::character varying])::text[])),
ADD COLUMN "daily_rate" numeric(10,2),
ADD COLUMN "overdue_rate" numeric(10,5),
ADD COLUMN "currency" character varying(3);

UPDATE "public"."libraries"
SET ("daily_rate", "overdue_rate", "currency")
  = (0, 1, 'USD');

ALTER TABLE "public"."libraries"
ALTER COLUMN "daily_rate" SET NOT NULL,
ALTER COLUMN "overdue_rate" SET NOT NULL,
ALTER COLUMN "currency" SET NOT NULL;
