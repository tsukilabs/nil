CREATE TABLE user (
  id INTEGER NOT NULL PRIMARY KEY,
  player_id TEXT NOT NULL UNIQUE CHECK (length(player_id) > 0),
  password TEXT NOT NULL CHECK (length(password) > 0),
  created_at TEXT NOT NULL CHECK (length(created_at) > 0),
  updated_at TEXT NOT NULL CHECK (length(updated_at) > 0)
)
