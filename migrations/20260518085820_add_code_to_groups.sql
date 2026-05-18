-- Add migration script here
ALTER TABLE groups ADD COLUMN code TEXT UNIQUE;
UPDATE groups SET code = gen_random_uuid()::TEXT WHERE code IS NULL;
ALTER TABLE groups ALTER COLUMN code SET NOT NULL;
