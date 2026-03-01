-- Add migration script here
ALTER TABLE users
ADD COLUMN group_id UUID;

ALTER TABLE users
ADD CONSTRAINT fk_user_group
FOREIGN KEY (group_id)
REFERENCES groups (id);
