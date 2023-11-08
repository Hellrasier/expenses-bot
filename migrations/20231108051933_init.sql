-- Add migration script here

CREATE TABLE IF NOT EXISTS new_expenses (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  username TEXT NOT NULL,
  price REAL NOT NULL,
  comments TEXT,
  date TEXT NOT NULL
);