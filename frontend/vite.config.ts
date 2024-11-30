import { resolve } from "path"
import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true
  },
  build: {
    rollupOptions: {
      input: {
        popover: resolve(__dirname, "popover.html"),
        settings: resolve(__dirname, "settings.html")
      }
    }
  }
})
