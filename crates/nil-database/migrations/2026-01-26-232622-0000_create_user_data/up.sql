CREATE TABLE user_data (
  id INTEGER NOT NULL PRIMARY KEY,
  user TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
)
