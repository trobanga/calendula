
CREATE TABLE IF NOT EXISTS calendar_entries
(
  id          INTEGER NOT NULL PRIMARY KEY,
  title       TEXT NOT NULL,
  start       TEXT NOT NULL,
  end         TEXT
);
