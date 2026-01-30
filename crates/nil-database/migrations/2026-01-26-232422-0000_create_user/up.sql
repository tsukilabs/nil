CREATE TABLE user (
  id INTEGER NOT NULL PRIMARY KEY,
  player_id TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
)
