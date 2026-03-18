CREATE TABLE game (
  id TEXT NOT NULL PRIMARY KEY,
  password TEXT,
  description TEXT,
  round_duration TEXT,
  server_version TEXT NOT NULL,
  created_by INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  world_blob BLOB NOT NULL,
  FOREIGN KEY (created_by) REFERENCES user (id) ON DELETE CASCADE ON UPDATE CASCADE
)
