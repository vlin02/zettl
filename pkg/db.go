package pkg

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"

	_ "github.com/mattn/go-sqlite3"
)

func OpenDB(dbPath string) *sql.DB {
	_ = os.MkdirAll(filepath.Dir(dbPath), 0o755)
	
	dsn := fmt.Sprintf("file:%s?_busy_timeout=5000&_journal_mode=WAL&_synchronous=NORMAL", dbPath)
	db, err := sql.Open("sqlite3", dsn)
	if err != nil {
		panic(err)
	}
	
	db.SetMaxOpenConns(1)
	return db
}

func Init(db *sql.DB) {
	_, err := db.Exec(`
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
	`)
	if err != nil {
		panic(err)
	}
}
