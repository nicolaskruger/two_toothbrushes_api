-- Add migration script here
ALTER TABLE users
ALTER COLUMN group_id SET NOT NULL;
