-- Add migration script here

PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS new_expenses (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  username TEXT NOT NULL,
  price REAL NOT NULL,
  comments TEXT,
  date TEXT NOT NULL
);

INSERT INTO new_expenses (user_id, username, price, comments, date)
SELECT user_id, username, price, comments, date FROM expenses;

DROP TABLE expenses;

ALTER TABLE new_expenses RENAME TO expenses;

COMMIT;

PRAGMA foreign_keys=on;