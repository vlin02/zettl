package main

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"sync"
	"time"
	"zettl/pkg"

	"github.com/alecthomas/chroma/v2/styles"
	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
	"golang.design/x/hotkey"
)

type Service struct {
	ctx       context.Context
	window    application.Window
	db        *sql.DB
	hk        *hotkey.Hotkey
	hkStop    chan struct{}
	app       *application.App
	readyCh   chan struct{}
	readyOnce sync.Once
	logFile   *os.File
}

func (s *Service) ServiceStartup(ctx context.Context, _ application.ServiceOptions) error {
	s.ctx = ctx
	s.readyCh = make(chan struct{})

	env := pkg.GlobalEnv
	dataDir := pkg.GetDataDir(env)
	dbPath := fmt.Sprintf("%s/zettl.db", dataDir)

	s.db = pkg.OpenDB(dbPath)
	pkg.MigrateUp(s.db, "migrations")
	pkg.BootstrapDB(s.db)

	h, _ := os.UserHomeDir()
	d := h + "/zettl"
	os.MkdirAll(d, 0755)
	lf, _ := os.OpenFile(d+"/log.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	s.logFile = lf

	app := application.Get()
	s.app = app
	w := app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:         "Zettl",
		Frameless:     true,
		AlwaysOnTop:   true,
		DisableResize: true,
		Hidden:        true,
		Mac: application.MacWindow{
			Appearance:    application.DefaultAppearance,
			DisableShadow: true,
			Backdrop:      application.MacBackdropTranslucent,
			Panel:         true,
		},
		URL: "/",
	})
	s.window = w

	app.Event.OnApplicationEvent(events.Mac.ApplicationDidBecomeActive, func(_ *application.ApplicationEvent) {
		s.show()
	})

	go func() {
		s.registerHotkeys()
	}()

	return nil
}

func (s *Service) ServiceName() string {
	return "Zettl"
}

func (s *Service) AppendLog(msg string) {
	if s.logFile != nil {
		s.logFile.WriteString(msg + "\n")
	}
}

func (s *Service) show() {
	<-s.readyCh
	mx, my := pkg.Location()
	n := pkg.DisplaysNum()
	did := 0
	for i := range n {
		x, y, w, h := pkg.GetDisplayBounds(i)
		if mx >= x && mx < x+w && my >= y && my < y+h {
			did = i
			break
		}
	}
	r := pkg.GetScreenRect(did)
	y := r.Y
	h := r.H
	if shift := pkg.MenuBarShiftPhysical(did); shift > 0 {
		y += shift
		h -= shift
	}

	w, _ := s.window.Size()

	go func() {
		s.window.SetSize(w, h)
		s.window.SetPosition(-10000, -10000)
		s.window.SetPosition(r.X, y)
		s.window.Show().Focus()
	}()
}

func (s *Service) FrontendReady() {
	s.readyOnce.Do(func() { close(s.readyCh) })
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

func (s *Service) SetWidth(w int) {
	_, h := s.window.Size()
	s.window.SetSize(w, h)
}

func (s *Service) SetRetentionDays(days int) {
	pkg.SetRetentionDays(s.db, days)
}

func (s *Service) SetToggleHotkey(ev pkg.Shortcut) {
	pkg.SetToggleHotkey(s.db, ev)
	s.registerHotkeys()
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

func (s *Service) Paste() {
	pkg.Paste()
}

func (s *Service) GetSnippetDetail(id int64) pkg.SnippetDetail {
	return pkg.GetSnippetDetail(s.db, id)
}

func (s *Service) registerHotkeys() {
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
		panic(err)
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
					s.show()
				}
			}
		}
	}()
}
