import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// In CI the workflow sets VITE_BASE_URL to "/<repo-name>/".
// Locally it falls back to "./" so `npm run dev` still works.
const base = process.env.VITE_BASE_URL ?? './'

export default defineConfig({
  plugins: [vue()],
  base,
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
  },
  server: {
    fs: {
      allow: ['..'],
    },
  },
})
