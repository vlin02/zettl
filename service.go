package main

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
	"golang.design/x/hotkey"
)

type Service struct {
	mu       sync.Mutex
	lastShow time.Time
}

var hotkeyOnce sync.Once

func (g *Service) ServiceStartup(ctx context.Context, _ application.ServiceOptions) error {
	fmt.Println("ServiceStartup called")
	app := application.Get()

	// attach focus/blur handlers (after window creation)
	go func() {
		time.Sleep(100 * time.Millisecond)
		w := app.Window.Current()
		if w == nil {
			ws := app.Window.GetAll()
			if len(ws) > 0 {
				w = ws[0]
			}
		}
		if w == nil { return }
		w.OnWindowEvent(events.Common.WindowLostFocus, func(_ *application.WindowEvent) {
			g.mu.Lock(); since := time.Since(g.lastShow); g.mu.Unlock()
			if since > 150*time.Millisecond { w.Hide() }
		})
	}()

	go func() {
		hk := hotkey.New([]hotkey.Modifier{hotkey.ModCmd, hotkey.ModShift}, hotkey.KeyZ)
		if err := hk.Register(); err != nil { fmt.Println("hotkey register error:", err) }
		for {
			select {
			case <-ctx.Done():
				hk.Unregister(); return
			case <-hk.Keydown():
				w := app.Window.Current()
				if w == nil {
					ws := app.Window.GetAll(); if len(ws) > 0 { w = ws[0] }
				}
				if w != nil {
					if w.IsVisible() { w.Hide() } else { w.Show(); g.mu.Lock(); g.lastShow = time.Now(); g.mu.Unlock() }
				}
			}
		}
	}()

	return nil
}

func (g *Service) Greet(name string) string { return "Hello " + name + "!" }
