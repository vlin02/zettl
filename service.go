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
	"golang.design/x/hotkey"
)

type Service struct {
	ctx        context.Context
	window     application.Window
	db         *sql.DB
	hk         *hotkey.Hotkey
	hkStop     chan struct{}
	app        *application.App
	readyCh    chan struct{}
	readyOnce  sync.Once
	logFile    *os.File
	systemTray *application.SystemTray
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
			Appearance:         application.DefaultAppearance,
			DisableShadow:      true,
			Backdrop:           application.MacBackdropTranslucent,
			NonactivatingPanel: true,
			WindowLevel:        application.MacWindowLevelFloating,
		},
		URL: "/",
	})
	s.window = w

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
	windowPtr := s.window.NativeWindow()
	pkg.ShowPanel(windowPtr)
}

func (s *Service) FrontendReady() {
	s.readyOnce.Do(func() {
		close(s.readyCh)
		windowPtr := s.window.NativeWindow()
		pkg.SetupPanelNotifications(windowPtr)
		s.show()
	})
}

func (s *Service) FindSnippets(q string, before int64, limit int) []pkg.Snippet {
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

func (s *Service) SetToggleHotkey(ev pkg.KeyboardEvent) {
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

func (s *Service) GetSnippetDetail(id int64) pkg.Snippet {
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

func (s *Service) ShowQuickLaunch() {
	if s.window != nil {
		s.show()
	}
}

func (s *Service) ToggleQuickLaunch() {
	if s.window != nil {
		if s.window.IsVisible() {
			s.window.Hide()
		} else {
			s.show()
		}
	}
}
