
CREATE TABLE IF NOT EXISTS calendar_entries
(
  id          INTEGER NOT NULL PRIMARY KEY,
  title       TEXT NOT NULL,
  start_date  DATE NOT NULL,
  start_time  TIME,
  end_date    DATE,
  end_time    TIME
);
