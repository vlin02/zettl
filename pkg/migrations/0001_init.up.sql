CREATE TABLE IF NOT EXISTS snippets (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  content TEXT NOT NULL,
  copied_at INTEGER NOT NULL,
  language TEXT,
  hash TEXT NOT NULL UNIQUE,
  html_lines TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_snippets_copied_at_desc ON snippets(copied_at DESC);
CREATE VIRTUAL TABLE IF NOT EXISTS snippets_fts USING fts5(
  content,
  content='snippets',
  content_rowid='id',
  tokenize='trigram'
);
CREATE TRIGGER IF NOT EXISTS snippets_ai AFTER INSERT ON snippets BEGIN
  INSERT INTO snippets_fts(rowid, content) VALUES (new.id, new.content);
END;
CREATE TRIGGER IF NOT EXISTS snippets_ad AFTER DELETE ON snippets BEGIN
  INSERT INTO snippets_fts(snippets_fts, rowid, content) VALUES('delete', old.id, old.content);
END;
CREATE TRIGGER IF NOT EXISTS snippets_au AFTER UPDATE ON snippets BEGIN
  INSERT INTO snippets_fts(snippets_fts, rowid, content) VALUES('delete', old.id, old.content);
  INSERT INTO snippets_fts(rowid, content) VALUES (new.id, new.content);
END;
DROP TABLE IF EXISTS settings;
CREATE TABLE settings (
  retention_days INTEGER NOT NULL,
  style TEXT NOT NULL,
  toggle_hotkey TEXT NOT NULL,
  font_size INTEGER NOT NULL DEFAULT 14
);