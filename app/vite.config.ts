import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    fs: {
      allow: ['../../../../'],
    },
  },
  optimizeDeps: {
    exclude: ['@crate/backend']
  },
})
