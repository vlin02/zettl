import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: { alias: { '@': new URL('./src', import.meta.url).pathname } },
  build: {
    rollupOptions: {
      onwarn(warning, warn) {
        if (warning.message && warning.message.includes('no babel-plugin-flow-react-proptypes'))
          return
        warn(warning)
      },
    },
  },
})
