package main

import (
	"embed"
	_ "embed"
	"log"
	"log/slog"

	"github.com/wailsapp/wails/v3/pkg/application"
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

	// Create system tray
	systemTray := app.SystemTray.New()

	// Set icon for macOS
	systemTray.SetTemplateIcon(customTrayIcon)

	// Set tooltip
	systemTray.SetTooltip("Zettl - Quick Snippet Manager")

	// Store systemTray reference in service
	svc.systemTray = systemTray

	// Create menu
	menu := app.NewMenu()
	menu.Add("Show").OnClick(func(ctx *application.Context) {
		if svc.window != nil {
			svc.ShowQuickLaunch()
		}
	})
	menu.Add("Quit").OnClick(func(ctx *application.Context) {
		app.Quit()
	})

	systemTray.SetMenu(menu)

	// Handle left click on tray icon - toggle quick launch
	systemTray.OnClick(func() {
		if svc.window != nil {
			svc.ToggleQuickLaunch()
		}
	})

	err := app.Run()

	if err != nil {
		log.Fatal(err)
	}
}
