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
