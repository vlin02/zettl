web_port := "1420"

start-web:
  cd web; pnpm vite --port {{web_port}}

start-core:
  cd core; pnpm tauri dev --port {{web_port}}

build:
  cd web; pnpm vite build
  cd core; pnpm tauri build --bundles app