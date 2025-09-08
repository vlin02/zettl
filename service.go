package main

import (
	"context"
	"database/sql"
	"fmt"
	"time"
	"zettl/pkg"

	"github.com/alecthomas/chroma/v2/styles"
	"github.com/go-vgo/robotgo"
	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
	"golang.design/x/hotkey"
)

type Service struct {
	ctx    context.Context
	window application.Window
	db     *sql.DB
	hk     *hotkey.Hotkey
	hkStop chan struct{}
}

func (g *Service) ServiceStartup(ctx context.Context, _ application.ServiceOptions) error {
	fmt.Println("ServiceStartup called")
	g.ctx = ctx

	env := pkg.GlobalEnv
	dataDir := pkg.GetDataDir(env)
	dbPath := fmt.Sprintf("%s/zettl.db", dataDir)
	g.db = pkg.OpenDB(dbPath)

	app := application.Get()
	w := app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:       "Window 1",
		Frameless:   true,
		AlwaysOnTop: true,
		Mac: application.MacWindow{
			InvisibleTitleBarHeight: 50,
			Backdrop:                application.MacBackdropTranslucent,
			TitleBar:                application.MacTitleBarHiddenInset,
			Panel:                   true,
		},
		URL: "/",
	})
	g.window = w

	app.Event.OnApplicationEvent(events.Mac.ApplicationDidBecomeActive, func(_ *application.ApplicationEvent) { show(w) })
	app.Event.OnApplicationEvent(events.Mac.ApplicationDidResignActive, func(_ *application.ApplicationEvent) {
		if w.IsVisible() {
			w.Hide()
		}
	})

	go func() {
		g.registerHotkeys()
	}()

	return nil
}

func (s *Service) ServiceName() string {
	return "Zettl"
}

func show(win application.Window) {
	mx, my := robotgo.Location()
	n := robotgo.DisplaysNum()
	did := 0
	for i := range n {
		x, y, w, h := robotgo.GetDisplayBounds(i)
		if mx >= x && mx < x+w && my >= y && my < y+h {
			did = i
			break
		}
	}
	r := robotgo.GetScreenRect(did)
	y := r.Y
	h := r.H
	if did == 0 {
		y += 25
		h -= 25
	}
	w, _ := win.Size()
	win.SetSize(w, h)
	win.SetPosition(r.X, y)
	win.Show()
}

func (s *Service) FindSnippets(q string, before int64, limit int) []pkg.SnippetPreview {
	return pkg.FindSnippets(s.db, q, before, limit)
}

func (s *Service) GetUISettings() pkg.UISettings {
	return pkg.GetUISettings(s.db)
}

func (s *Service) SetSyntaxStyle(style string) {
	pkg.SetSyntaxStyle(s.db, style)
}

func (s *Service) SetUITheme(theme string) {
	pkg.SetUITheme(s.db, theme)
}

func (s *Service) SetRetentionDays(days int) {
	pkg.SetRetentionDays(s.db, days)
}

func (s *Service) SetToggleHotkey(ev pkg.HotkeyEvent) {
	pkg.SetToggleHotkey(s.db, ev)
	s.registerHotkeys()
}

func (s *Service) SetBgColors(lightColor, darkColor string) {
	pkg.SetBgColors(s.db, lightColor, darkColor)
}

func (s *Service) SetFontSize(size int) {
	pkg.SetFontSize(s.db, size)
}

func (s *Service) ListStyles() []string {
	return styles.Names()
}

func (s *Service) AddSnippet(content string, language string) int64 {
	return pkg.AddSnippet(s.db, content, language, time.Now().Unix())
}

func (s *Service) GetSnippetDetail(id int64) pkg.SnippetDetail {
	return pkg.GetSnippetDetail(s.db, id)
}

func (s *Service) PurgeExpired() {
	pkg.PurgeExpiredSnippets(s.db)
}

func (s *Service) registerHotkeys() {
	if s.db == nil || s.ctx == nil {
		return
	}
	if s.hk != nil {
		s.hk.Unregister()
	}
	if s.hkStop != nil {
		close(s.hkStop)
	}
	settings := pkg.GetSettings(s.db)
	hk := settings.ToggleHotkey
	if hk == nil {
		return
	}
	if err := hk.Register(); err != nil {
		fmt.Println("hotkey register error:", err)
		return
	}
	s.hk = hk
	s.hkStop = make(chan struct{})
	w := s.window
	go func() {
		for {
			select {
			case <-s.ctx.Done():
				hk.Unregister()
				return
			case <-s.hkStop:
				return
			case <-hk.Keydown():
				if w == nil {
					continue
				}
				if w.IsVisible() {
					w.Hide()
				} else {
					show(w)
				}
			}
		}
	}()
}
