package pkg

import (
	"crypto/sha256"
	"database/sql"
	"encoding/json"
	"strings"
	"unicode/utf8"

	"fmt"
	"time"

	"github.com/alecthomas/chroma/v2/lexers"
)

const (
	MaxSyntaxHighlightChars = 100000 // 100KB
)

type Snippet struct {
	ID       int64  `json:"id"`
	Content  string `json:"content"`
	CopiedAt int64  `json:"copied_at"`
	Language string `json:"language"`
	HTML     string `json:"html"`
}

func AddSnippet(db *sql.DB, content string, language string, copiedAt int64) int64 {
	l := lexers.Get(language)
	if l == nil {
		panic(language)
	}

	used := l.Config().Name
	h := sha256.Sum256([]byte(content))
	hash := fmt.Sprintf("%x", h[:])

	contentLength := len(content)

	var htmlLinesJSON sql.NullString
	if contentLength <= MaxSyntaxHighlightChars {
		lines, err := HighlightLinesWithClasses(content, l)
		if err != nil {
			panic(err)
		}
		b, err := json.Marshal(lines)
		if err != nil {
			panic(err)
		}
		htmlLinesJSON = sql.NullString{String: string(b), Valid: true}
	}

	_, err := db.Exec("DELETE FROM snippets WHERE hash = ?", hash)
	if err != nil {
		panic(err)
	}
	res, err := db.Exec(`
		INSERT INTO snippets(content, copied_at, language, hash, html_lines)
		VALUES(?, ?, ?, ?, ?)
	`, content, copiedAt, used, hash, htmlLinesJSON)
	if err != nil {
		panic(err)
	}
	id, err := res.LastInsertId()
	if err != nil {
		panic(err)
	}
	if id%10 == 0 {
		PurgeExpiredSnippets(db)
	}
	return id
}

func PurgeExpiredSnippets(db *sql.DB) {
	days := GetSettings(db).RetentionDays
	if days <= 0 {
		return
	}
	cutoff := time.Now().Add(-time.Duration(days) * 24 * time.Hour).Unix()
	rows, err := db.Query("SELECT id FROM snippets WHERE copied_at < ?", cutoff)
	if err != nil {
		panic(err)
	}
	defer rows.Close()
	var ids []int64
	for rows.Next() {
		var id int64
		if err := rows.Scan(&id); err != nil {
			panic(err)
		}
		ids = append(ids, id)
	}
	if len(ids) == 0 {
		return
	}
	ph := make([]string, len(ids))
	args := make([]any, len(ids))
	for i, id := range ids {
		ph[i] = "?"
		args[i] = id
	}
	_, err = db.Exec("DELETE FROM snippets WHERE id IN ("+strings.Join(ph, ",")+")", args...)
	if err != nil {
		panic(err)
	}
}

func FindSnippets(db *sql.DB, q string, before int64, limit int) []Snippet {
	if limit <= 0 {
		limit = 100
	}
	var (
		query string
		args  []any
	)

	if q != "" {
		if utf8.RuneCountInString(q) >= 3 {
			lit := "\"" + strings.ReplaceAll(q, "\"", "\"\"") + "\""
			query = "SELECT s.id, s.content, s.copied_at, s.language, s.html_lines FROM snippets_fts f JOIN snippets s ON f.rowid = s.id WHERE snippets_fts MATCH ?"
			args = append(args, lit)
		} else {
			query = "SELECT s.id, s.content, s.copied_at, s.language, s.html_lines FROM snippets s WHERE instr(lower(s.content), lower(?)) > 0"
			args = append(args, q)
		}
		if before > 0 {
			query += " AND s.id < ?"
			args = append(args, before)
		}
		query += " ORDER BY s.id DESC LIMIT ?"
		args = append(args, limit)
	} else {
		query = "SELECT id, content, copied_at, language, html_lines FROM snippets"
		if before > 0 {
			query += " WHERE id < ?"
			args = append(args, before)
		}
		query += " ORDER BY id DESC LIMIT ?"
		args = append(args, limit)
	}
	rows, err := db.Query(query, args...)
	if err != nil {
		panic(err)
	}
	defer rows.Close()
	var out []Snippet
	for rows.Next() {
		var id int64
		var content string
		var copiedAt int64
		var language string
		var htmlLinesJSON sql.NullString
		if err := rows.Scan(&id, &content, &copiedAt, &language, &htmlLinesJSON); err != nil {
			panic(err)
		}

		var previewHTML string
		if htmlLinesJSON.Valid {
			var full []string
			if err := json.Unmarshal([]byte(htmlLinesJSON.String), &full); err != nil {
				panic(err)
			}

			if len(full) > 0 {
				lineOffset := 0
				if q != "" {
					lc := strings.ToLower(content)
					lq := strings.ToLower(q)
					if idx := strings.Index(lc, lq); idx >= 0 {
						lineOffset = strings.Count(content[:idx], "\n")
					}
				}
				si := min(max(0, lineOffset), len(full))
				ei := min(si+5, len(full))
				previewHTML = strings.Join(full[si:ei], "")
			}
		}
		out = append(out, Snippet{
			ID:       id,
			Content:  content,
			CopiedAt: copiedAt,
			Language: language,
			HTML:     previewHTML,
		})
	}
	if err := rows.Err(); err != nil {
		panic(err)
	}
	return out
}

func GetSnippetDetail(db *sql.DB, id int64) Snippet {
	if db == nil {
		return Snippet{}
	}
	var r Snippet
	var htmlLinesJSON sql.NullString
	err := db.QueryRow(`SELECT id, content, copied_at, language, html_lines
		FROM snippets WHERE id = ?`, id).
		Scan(&r.ID, &r.Content, &r.CopiedAt, &r.Language, &htmlLinesJSON)
	if err != nil {
		panic(err)
	}

	if htmlLinesJSON.Valid && htmlLinesJSON.String != "" {
		var lines []string
		if err := json.Unmarshal([]byte(htmlLinesJSON.String), &lines); err != nil {
			panic(err)
		}
		r.HTML = strings.Join(lines, "")
	}
	return r
}
