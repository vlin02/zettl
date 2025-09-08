package pkg

import (
	"database/sql"
	"encoding/json"

	hotkey "golang.design/x/hotkey"
)

func EnsureSettings(db *sql.DB) {
	if db == nil {
		return
	}
	obj := struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}{Mods: []uint32{uint32(hotkey.ModCmd), uint32(hotkey.ModShift)}, Key: uint32(hotkey.KeyA)}
	b, err := json.Marshal(obj)
	if err != nil {
		panic(err)
	}

	if _, err := db.Exec("INSERT OR IGNORE INTO settings (retention_days, theme, style, toggle_hotkey, light_bg_color, dark_bg_color, font_size) VALUES (?, ?, ?, ?, ?, ?, ?)", 30, "dark", "onedark", string(b), "#ffffff", "#0a0a0a", 14); err != nil {
		panic(err)
	}
}

// Internal/native settings for backend usage
type Settings struct {
	Theme         string
	Style         string
	CSS           string
	RetentionDays int
	ToggleHotkey  *hotkey.Hotkey
}

func GetSettings(db *sql.DB) Settings {
	if db == nil {
		return Settings{RetentionDays: 30, Theme: "dark", Style: "onedark"}
	}
	row := db.QueryRow("SELECT retention_days, theme, style, toggle_hotkey FROM settings LIMIT 1")
	var days int
	var theme, style, toggle string
	if err := row.Scan(&days, &theme, &style, &toggle); err != nil {
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
	return Settings{Theme: theme, Style: style, CSS: ChromaCSSForStyle(style), RetentionDays: days, ToggleHotkey: thk}
}

func GetUISettings(db *sql.DB) UISettings {
	row := db.QueryRow("SELECT retention_days, theme, style, toggle_hotkey, light_bg_color, dark_bg_color, font_size FROM settings LIMIT 1")
	var days, fontSize int
	var theme, style, toggle, lightBg, darkBg string
	if err := row.Scan(&days, &theme, &style, &toggle, &lightBg, &darkBg, &fontSize); err != nil {
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
	out.Theme = theme
	out.Style.CSS = ChromaCSSForStyle(style)
	out.Style.Name = style
	hotkey := HotkeyToEvent(mods, hotkey.Key(obj.Key))
	if hotkey != nil {
		out.ToggleHotkey = *hotkey
	}
	out.RetentionDays = days
	out.LightBgColor = lightBg
	out.DarkBgColor = darkBg
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
	Theme         string      `json:"theme"`
	Style         Style       `json:"style"`
	ToggleHotkey  HotkeyEvent `json:"toggle_hotkey"`
	RetentionDays int         `json:"retention_days"`
	LightBgColor  string      `json:"light_bg_color"`
	DarkBgColor   string      `json:"dark_bg_color"`
	FontSize      int         `json:"font_size"`
}

func SetUITheme(db *sql.DB, theme string) {
	if db == nil {
		return
	}
	if _, err := db.Exec("UPDATE settings SET theme = ?", theme); err != nil {
		panic(err)
	}
}

func SetSyntaxStyle(db *sql.DB, style string) {
	if db == nil {
		return
	}
	if _, err := db.Exec("UPDATE settings SET style = ?", style); err != nil {
		panic(err)
	}
}

func SetRetentionDays(db *sql.DB, days int) {
	if db == nil || days <= 0 {
		return
	}
	if _, err := db.Exec("UPDATE settings SET retention_days = ?", days); err != nil {
		panic(err)
	}
}

func SetToggleHotkey(db *sql.DB, event HotkeyEvent) {
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

func SetBgColors(db *sql.DB, lightColor, darkColor string) {
	if db == nil {
		return
	}
	if _, err := db.Exec("UPDATE settings SET light_bg_color = ?, dark_bg_color = ?", lightColor, darkColor); err != nil {
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
