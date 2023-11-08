-- Add migration script here

CREATE TABLE IF NOT EXISTS expenses (
  id INTEGER PRIMARY KEY,
  user_id INTEGER NOT NULL,
  username TEXT NOT NULL,
  price REAL NOT NULL,
  comments TEXT,
  date TEXT NOT NULL
);