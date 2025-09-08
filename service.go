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
	app    *application.App
}

func (s *Service) ServiceStartup(ctx context.Context, _ application.ServiceOptions) error {
	fmt.Println("ServiceStartup called")
	s.ctx = ctx

	env := pkg.GlobalEnv
	dataDir := pkg.GetDataDir(env)
	dbPath := fmt.Sprintf("%s/zettl.db", dataDir)
	s.db = pkg.OpenDB(dbPath)

	app := application.Get()
	s.app = app
	w := app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:            "Zettl",
		Frameless:        true,
		AlwaysOnTop:      true,
		DisableResize:    true,
		BackgroundType:   application.BackgroundTypeTransparent,
		BackgroundColour: application.RGBA{Red: 0, Green: 0, Blue: 0, Alpha: 0},
		Mac: application.MacWindow{
			Backdrop: application.MacBackdropTransparent,
			TitleBar: application.MacTitleBarHiddenInset,
			Panel:    true,
			LiquidGlass: application.MacLiquidGlass{
				CornerRadius: 0,
			},
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

func (s *Service) show() {
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
		y += 40
		h -= 40
	}
	curW, _ := s.window.Size()
	s.window.SetSize(curW, h)
	s.app.Event.Emit("windowHeight", h)
	s.window.SetPosition(r.X, y)
	s.window.Show()

	// Simulate a click inside the window to allow typing without manual click
	go func(x, y int) {
		// brief delay to ensure window is ready
		time.Sleep(100 * time.Millisecond)
		ox, oy := robotgo.Location()
		tx := x + 40
		ty := y + 40
		robotgo.Move(tx, ty)
		robotgo.Click("left", false)
		robotgo.Move(ox, oy)
	}(r.X, y)
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
					s.show()
				}
			}
		}
	}()
}
