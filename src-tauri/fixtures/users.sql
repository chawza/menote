PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
INSERT INTO users VALUES(1,'admin@asd.com','Admin','10:03:19');
INSERT INTO users VALUES(2,'bob@asd.com','Bob','10:03:42');
INSERT INTO users VALUES(3,'sandra@asd.com','Sandra','10:04:48');
COMMIT;
