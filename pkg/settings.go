package pkg

import (
	"database/sql"
	"encoding/json"

	hotkey "golang.design/x/hotkey"
)

func BootstrapDB(db *sql.DB) {
	obj := struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}{Mods: []uint32{uint32(hotkey.ModCmd), uint32(hotkey.ModShift)}, Key: uint32(hotkey.KeyA)}
	b, err := json.Marshal(obj)
	if err != nil {
		panic(err)
	}

	if _, err := db.Exec("INSERT OR IGNORE INTO settings (retention_days, style, toggle_hotkey, font_size) VALUES (?, ?, ?, ?)", 30, "onedark", string(b), 14); err != nil {
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
	var obj struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}
	if err := json.Unmarshal([]byte(toggle), &obj); err != nil {
		panic(err)
	}
	mods := make([]hotkey.Modifier, 0, len(obj.Mods))
	for _, m := range obj.Mods {
		mods = append(mods, hotkey.Modifier(m))
	}
	thk := hotkey.New(mods, hotkey.Key(obj.Key))
	return Settings{Style: style, CSS: ChromaCSSForStyle(style), RetentionDays: days, ToggleHotkey: thk}
}

func GetUISettings(db *sql.DB) UISettings {
	row := db.QueryRow("SELECT COALESCE(retention_days, 0), style, toggle_hotkey, font_size FROM settings LIMIT 1")
	var days, fontSize int
	var style, toggle string
	if err := row.Scan(&days, &style, &toggle, &fontSize); err != nil {
		panic(err)
	}
	var obj struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}
	if err := json.Unmarshal([]byte(toggle), &obj); err != nil {
		panic(err)
	}

	mods := make([]hotkey.Modifier, 0, len(obj.Mods))
	for _, m := range obj.Mods {
		mods = append(mods, hotkey.Modifier(m))
	}

	var out UISettings
	out.Style.CSS = ChromaCSSForStyle(style)
	out.Style.Name = style
	hk := HotkeyToEvent(mods, hotkey.Key(obj.Key))
	if hk != nil {
		out.ToggleHotkey = *hk
	}
	out.RetentionDays = days
	out.FontSize = fontSize
	return out
}

// Style represents syntax highlighting style configuration
type Style struct {
	CSS  string `json:"css"`
	Name string `json:"name"`
}

// UI-friendly settings for frontend: simplified shape
type UISettings struct {
	Style         Style    `json:"style"`
	ToggleHotkey  Shortcut `json:"toggle_hotkey"`
	RetentionDays int      `json:"retention_days"`
	FontSize      int      `json:"font_size"`
}

// Removed SetUITheme: theme support dropped

func SetSyntaxStyle(db *sql.DB, style string) {
	if db == nil {
		return
	}
	if _, err := db.Exec("UPDATE settings SET style = ?", style); err != nil {
		panic(err)
	}
}

func SetRetentionDays(db *sql.DB, days int) {
	if db == nil {
		return
	}
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

func SetToggleHotkey(db *sql.DB, event Shortcut) {
	if db == nil {
		return
	}
	mods, key := EventToHotkey(&event)
	uintMods := make([]uint32, len(mods))
	for i, m := range mods {
		uintMods[i] = uint32(m)
	}
	obj := struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}{Mods: uintMods, Key: uint32(key)}
	b, err := json.Marshal(obj)
	if err != nil {
		panic(err)
	}
	if _, err := db.Exec("UPDATE settings SET toggle_hotkey = ?", string(b)); err != nil {
		panic(err)
	}
}

// Removed SetBgColors: background color customization dropped

func SetFontSize(db *sql.DB, size int) {
	if db == nil || size < 8 || size > 32 {
		return
	}
	if _, err := db.Exec("UPDATE settings SET font_size = ?", size); err != nil {
		panic(err)
	}
}
