package main

import (
	"embed"
	"log"
	"log/slog"

	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/events"
)

//go:embed all:frontend/dist
var assets embed.FS

//go:embed assets/tray-icon.svg
var customTrayIcon []byte

func main() {

	svc := &Service{}
	app := application.New(application.Options{
		LogLevel: slog.LevelError,
		Name:     "Zettl",
		Services: []application.Service{
			application.NewService(svc),
		},
		Assets: application.AssetOptions{
			Handler: application.AssetFileServerFS(assets),
		},
		Mac: application.MacOptions{
			ApplicationShouldTerminateAfterLastWindowClosed: false,
			ActivationPolicy: application.ActivationPolicyAccessory,
		},
		Windows: application.WindowsOptions{
			DisableQuitOnLastWindowClosed: true,
		},
	})

	systemTray := app.SystemTray.New()

	systemTray.SetTemplateIcon(customTrayIcon)

	systemTray.SetTooltip("Zettl - Quick Snippet Manager")

	svc.systemTray = systemTray

	menu := app.NewMenu()
	menu.Add("Show").OnClick(func(ctx *application.Context) {
		svc.ShowQuickLaunch()
	})
	menu.Add("Quit").OnClick(func(ctx *application.Context) {
		app.Quit()
	})

	systemTray.SetMenu(menu)

	systemTray.OnClick(func() {
		svc.ToggleQuickLaunch()
	})

	app.Event.OnApplicationEvent(events.Common.ApplicationStarted, func(event *application.ApplicationEvent) {
		svc.ShowQuickLaunch()
	})

	err := app.Run()

	if err != nil {
		log.Fatal(err)
	}
}
