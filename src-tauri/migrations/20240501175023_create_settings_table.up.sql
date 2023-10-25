CREATE TABLE IF NOT EXISTS settings (
  id INTEGER PRIMARY KEY NOT NULL,
  notifications BOOLEAN NULL DEFAULT 0,
  sound BOOLEAN NULL DEFAULT 0,
  preview BOOLEAN NULL DEFAULT 0
);

INSERT INTO settings (notifications, sound, preview) values (false, false, false);