CREATE TABLE game (
  id TEXT NOT NULL PRIMARY KEY,
  password TEXT,
  description TEXT,
  created_by INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  world_blob BLOB NOT NULL,
  FOREIGN KEY (created_by) REFERENCES user (id) ON DELETE CASCADE ON UPDATE CASCADE
)
