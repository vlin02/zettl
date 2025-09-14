package main

import (
	"crypto/sha256"
	"database/sql"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	pkg "zettl/pkg"

	_ "github.com/mattn/go-sqlite3"
)

func Seed(db *sql.DB, n int) error {
	type sample struct{ s, lang string }
	samples := []sample{
		{"package main\nfunc main() { println(\"hi\") }", "go"},
		{"def add(a,b):\n    return a+b\nprint(add(2,3))", "python"},
		{"function add(a,b){return a+b;}\nconsole.log(add(2,3));", "javascript"},
		{"<!doctype html>\n<title>x</title>\n<p>Hello</p>", "html"},
		{"SELECT 1;\nCREATE TABLE t(id INT);", "sql"},
		{"#!/usr/bin/env bash\necho hi\nexit 0", "bash"},
		{"{\n  \"name\": \"demo\",\n  \"ok\": true\n}", "json"},
	}
	now := time.Now().Unix()
	if n <= 0 {
		n = len(samples)
	}
	for i := 0; i < n; i++ {
		x := samples[i%len(samples)]
		idsum := sha256.Sum256([]byte(fmt.Sprintf("%d-%d", time.Now().UnixNano(), i)))
		id := fmt.Sprintf("%x", idsum[:])[:8]
		var s string
		switch x.lang {
		case "go", "javascript":
			s = x.s + "\n// seed:" + id
		case "python", "bash":
			s = x.s + "\n# seed:" + id
		case "sql":
			s = x.s + "\n-- seed:" + id
		case "html":
			s = x.s + "\n<!-- seed:" + id + " -->"
		case "json":
			var m map[string]any
			if err := json.Unmarshal([]byte(x.s), &m); err == nil {
				m["_seed"] = id
				if b, e := json.MarshalIndent(m, "", "  "); e == nil {
					s = string(b)
				}
			}
			if s == "" {
				s = x.s + "\n"
			}
		default:
			s = x.s + "\n" + id
		}
		pkg.AddSnippet(db, s, x.lang, now)
	}
	return nil
}

func Dump(db *sql.DB, n int) error {
	rows, err := db.Query("SELECT id, content FROM snippets ORDER BY copied_at DESC LIMIT ?", n)
	if err != nil {
		return err
	}
	defer rows.Close()
	type row struct {
		ID      int64  `json:"id"`
		Content string `json:"content"`
	}
	out := make([]row, 0)
	for rows.Next() {
		var r row
		if err := rows.Scan(&r.ID, &r.Content); err != nil {
			return err
		}
		out = append(out, r)
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	return enc.Encode(out)
}

func Reset(db *sql.DB) error {
	if _, err := db.Exec(`
        PRAGMA writable_schema = 1;
        DELETE FROM sqlite_master WHERE type IN ('table','index','trigger','view');
        PRAGMA writable_schema = 0;
        VACUUM;
    `); err != nil {
		return err
	}
	pkg.MigrateUp(db, "migrations")
	pkg.BootstrapDB(db)
	return nil
}

func main() {
	prod := flag.Bool("prod", false, "Use production environment")
	flag.Parse()
	args := flag.Args()
	if len(args) == 0 {
		fmt.Println("Usage: db [--prod] <command> [args]\nCommands: settings | reset | seed [n] | dump [n] | search <query> | migrate | delete")
		os.Exit(1)
	}
	cmd := args[0]
	dataDir := getDataDir(*prod)
	os.MkdirAll(dataDir, 0755)

	db := openDB(dataDir)
	defer db.Close()

	switch cmd {
	case "settings":
		s := pkg.GetUISettings(db)
		enc := json.NewEncoder(os.Stdout)
		enc.SetEscapeHTML(false)
		if err := enc.Encode(s); err != nil {
			panic(err)
		}
	case "reset":
		if err := Reset(db); err != nil {
			panic(err)
		}
	case "seed":
		n := 0
		if len(args) > 1 {
			if x, e := strconv.Atoi(args[1]); e == nil {
				n = x
			}
		}
		if err := Seed(db, n); err != nil {
			panic(err)
		}
	case "dump":
		n := 10
		if len(args) > 1 {
			if x, e := strconv.Atoi(args[1]); e == nil {
				n = x
			}
		}
		if err := Dump(db, n); err != nil {
			panic(err)
		}
	case "search":
		if len(args) < 2 {
			fmt.Println("search requires a query")
			os.Exit(1)
		}
		q := strings.Join(args[1:], " ")
		res := pkg.FindSnippets(db, q, 0, 50)
		enc := json.NewEncoder(os.Stdout)
		enc.SetEscapeHTML(false)
		if err := enc.Encode(res); err != nil {
			panic(err)
		}
	case "migrate":
		pkg.MigrateUp(db, "migrations")
	case "delete":
		db.Close()
		dbPath := filepath.Join(dataDir, "zettl.db")
		if err := os.Remove(dbPath); err != nil {
			panic(err)
		}
		if err := os.Remove(dbPath + "-shm"); err != nil && !os.IsNotExist(err) {
			panic(err)
		}
		if err := os.Remove(dbPath + "-wal"); err != nil && !os.IsNotExist(err) {
			panic(err)
		}
		fmt.Println("Database deleted.")
	default:
		fmt.Println("Unknown command:", cmd)
		os.Exit(1)
	}
}

func openDB(dataDir string) *sql.DB {
	dbPath := filepath.Join(dataDir, "zettl.db")
	return pkg.OpenDB(dbPath)
}

func getDataDir(prod bool) string {
	if prod {
		home, _ := os.UserHomeDir()
		return filepath.Join(home, "Library", "Containers", "dev.zettl.app", "Data", "zettl")
	}
	return "data"
}
