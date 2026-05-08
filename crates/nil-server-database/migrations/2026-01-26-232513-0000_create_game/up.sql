CREATE TABLE game (
  id TEXT NOT NULL PRIMARY KEY CHECK (length(id) > 0),
  password TEXT CHECK (password IS NULL OR length(password) > 0),
  description TEXT,
  round_duration TEXT CHECK (round_duration IS NULL OR length(round_duration) > 0),
  server_version TEXT NOT NULL CHECK (length(server_version) > 0),
  created_by INTEGER NOT NULL,
  created_at TEXT NOT NULL CHECK (length(created_at) > 0),
  updated_at TEXT NOT NULL CHECK (length(updated_at) > 0),
  world_blob BLOB NOT NULL,
  FOREIGN KEY (created_by) REFERENCES user (id) ON DELETE CASCADE ON UPDATE CASCADE
)
