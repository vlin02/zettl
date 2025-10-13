package pkg

import (
	"database/sql"

	hotkey "golang.design/x/hotkey"
)

func BootstrapDB(db *sql.DB) {
	hotkeyStr := marshalHotkey([]hotkey.Modifier{hotkey.ModCmd, hotkey.ModShift}, hotkey.KeyC)
	if _, err := db.Exec("INSERT OR IGNORE INTO settings (retention_days, style, toggle_hotkey, font_size) VALUES (?, ?, ?, ?)", 30, "onedark", hotkeyStr, 14); err != nil {
		panic(err)
	}
}

type Settings struct {
	Style         string
	CSS           string
	RetentionDays int
	ToggleHotkey  *hotkey.Hotkey
}

func GetSettings(db *sql.DB) Settings {
	row := db.QueryRow("SELECT COALESCE(retention_days, 0), style, toggle_hotkey FROM settings LIMIT 1")
	var days int
	var style, toggle string
	if err := row.Scan(&days, &style, &toggle); err != nil {
		panic(err)
	}
	mods, key := unmarshalHotkey(toggle)
	return Settings{Style: style, CSS: ChromaCSSForStyle(style), RetentionDays: days, ToggleHotkey: hotkey.New(mods, key)}
}

func GetUISettings(db *sql.DB) UISettings {
	row := db.QueryRow("SELECT COALESCE(retention_days, 0), style, toggle_hotkey, font_size FROM settings LIMIT 1")
	var days, fontSize int
	var style, toggle string
	if err := row.Scan(&days, &style, &toggle, &fontSize); err != nil {
		panic(err)
	}
	mods, key := unmarshalHotkey(toggle)

	var out UISettings
	out.Style.CSS = ChromaCSSForStyle(style)
	out.Style.Name = style
	event := HotkeyToEvent(mods, key)
	if event != nil {
		out.ToggleHotkey = *event
	}
	out.RetentionDays = days
	out.FontSize = fontSize
	return out
}

type Style struct {
	CSS  string `json:"css"`
	Name string `json:"name"`
}

type UISettings struct {
	Style         Style         `json:"style"`
	ToggleHotkey  KeyboardEvent `json:"toggle_hotkey"`
	RetentionDays int           `json:"retention_days"`
	FontSize      int           `json:"font_size"`
}

func SetSyntaxStyle(db *sql.DB, style string) {
	if _, err := db.Exec("UPDATE settings SET style = ?", style); err != nil {
		panic(err)
	}
}

func SetRetentionDays(db *sql.DB, days int) {
	if days <= 0 {
		if _, err := db.Exec("UPDATE settings SET retention_days = 0"); err != nil {
			panic(err)
		}
		return
	}
	if _, err := db.Exec("UPDATE settings SET retention_days = ?", days); err != nil {
		panic(err)
	}
}

func SetToggleHotkey(db *sql.DB, event KeyboardEvent) {
	mods, key := EventToHotkey(&event)
	hotkeyStr := marshalHotkey(mods, key)
	if _, err := db.Exec("UPDATE settings SET toggle_hotkey = ?", hotkeyStr); err != nil {
		panic(err)
	}
}

func SetFontSize(db *sql.DB, size int) {
	if db == nil || size < 8 || size > 32 {
		return
	}
	if _, err := db.Exec("UPDATE settings SET font_size = ?", size); err != nil {
		panic(err)
	}
}
