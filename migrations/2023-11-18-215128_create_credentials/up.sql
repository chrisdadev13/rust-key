-- Your SQL goes here
CREATE TABLE credentials (
  id INTEGER PRIMARY KEY NOT NULL,
  url TEXT,
  account_name TEXT NOT NULL,
  password TEXT NOT NULL,
  category TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
