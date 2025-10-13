package main

import (
	"crypto/sha256"
	"database/sql"
	"embed"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	pkg "zettl/pkg"

	_ "github.com/mattn/go-sqlite3"
)

//go:embed realistic_snippets.json
var realisticSnippetsFS embed.FS

type RealisticSnippet struct {
	Language string `json:"language"`
	Content  string `json:"content"`
}

func encodeJSON(v any) error {
	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	return enc.Encode(v)
}

func loadRealisticSnippets() ([]RealisticSnippet, error) {
	data, err := realisticSnippetsFS.ReadFile("realistic_snippets.json")
	if err != nil {
		return nil, err
	}
	var snippets []RealisticSnippet
	if err := json.Unmarshal(data, &snippets); err != nil {
		return nil, err
	}
	return snippets, nil
}

func SeedRealistic(db *sql.DB, n int) error {
	snippets, err := loadRealisticSnippets()
	if err != nil {
		return err
	}

	now := time.Now().Unix()
	if n <= 0 {
		n = len(snippets)
	}

	for i := 0; i < n; i++ {
		snippet := snippets[i%len(snippets)]
		timestamp := now - int64(n-i)*3600
		pkg.AddSnippet(db, snippet.Content, snippet.Language, timestamp)
	}
	return nil
}

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
	return encodeJSON(out)
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
	if err := pkg.MigrateUp(db); err != nil {
		return err
	}
	pkg.BootstrapDB(db)
	return nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: db <command> [args]\nCommands: settings | reset | seed [realistic] [n] | dump | search | migrate | delete")
		os.Exit(1)
	}

	prod := flag.Bool("prod", false, "Use production environment")
	flag.Parse()

	cmd := flag.Arg(0)
	args := flag.Args()[1:]

	db := setupDB(*prod)
	defer db.Close()

	switch cmd {
	case "settings":
		if err := encodeJSON(pkg.GetUISettings(db)); err != nil {
			panic(err)
		}
	case "reset":
		if err := Reset(db); err != nil {
			panic(err)
		}
	case "seed":
		variant := ""
		n := 0
		if len(args) > 0 {
			if args[0] == "realistic" {
				variant = "realistic"
				if len(args) > 1 {
					fmt.Sscanf(args[1], "%d", &n)
				}
			} else {
				fmt.Sscanf(args[0], "%d", &n)
			}
		}

		if variant == "realistic" {
			if err := SeedRealistic(db, n); err != nil {
				panic(err)
			}
		} else {
			if err := Seed(db, n); err != nil {
				panic(err)
			}
		}
	case "dump":
		n := 10
		if len(args) > 0 {
			fmt.Sscanf(args[0], "%d", &n)
		}
		if err := Dump(db, n); err != nil {
			panic(err)
		}
	case "search":
		if err := encodeJSON(pkg.FindSnippets(db, strings.Join(args, " "), 0, 50)); err != nil {
			panic(err)
		}
	case "migrate":
		if err := pkg.MigrateUp(db); err != nil {
			panic(err)
		}
	case "delete":
		db.Close()
		dataDir := getDataDir(*prod)
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

func setupDB(prod bool) *sql.DB {
	dataDir := getDataDir(prod)
	os.MkdirAll(dataDir, 0755)
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
