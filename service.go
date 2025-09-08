package main

import (
	"context"
	"fmt"

	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
	"golang.design/x/hotkey"
)

type Service struct{ window application.Window }

// removed hotkeyOnce (unused)

func (g *Service) ServiceStartup(ctx context.Context, _ application.ServiceOptions) error {
	fmt.Println("ServiceStartup called")
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
		BackgroundColour: application.NewRGB(27, 38, 54),
		URL:              "/",
	})
	g.window = w

	w.OnWindowEvent(events.Common.WindowLostFocus, func(_ *application.WindowEvent) { w.Hide() })

	app.Event.OnApplicationEvent(events.Mac.ApplicationDidBecomeActive, func(_ *application.ApplicationEvent) {
		w.Show()
	})
	app.Event.OnApplicationEvent(events.Mac.ApplicationDidResignActive, func(_ *application.ApplicationEvent) {
		if w.IsVisible() {
			w.Hide()
		}
	})

	go func() {
		hk := hotkey.New([]hotkey.Modifier{hotkey.ModCmd, hotkey.ModShift}, hotkey.KeyZ)
		if err := hk.Register(); err != nil {
			fmt.Println("hotkey register error:", err)
		}
		for {
			select {
			case <-ctx.Done():
				hk.Unregister()
				return
			case <-hk.Keydown():
				if w.IsVisible() {
					w.Hide()
				} else {
					w.Show()
				}
			}
		}
	}()
	return nil
}

func (g *Service) Greet(name string) string { return "Hello " + name + "!" }
