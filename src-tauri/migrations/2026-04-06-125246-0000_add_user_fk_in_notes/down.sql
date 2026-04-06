PRAGMA foreign_keys = OFF;

ALTER TABLE notes ADD COLUMN user_id_new INTEGER NOT NULL;
UPDATE notes SET user_id_new = user_id;
ALTER TABLE notes DROP COLUMN user_id;
ALTER TABLE notes RENAME COLUMN user_id_new TO user_id;
