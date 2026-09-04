import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vitest/config'
import { version as pkgVersion } from './package.json'

process.env.VITE_APP_VERSION = pkgVersion
if (process.env.NODE_ENV === 'production') {
  process.env.VITE_APP_BUILD_EPOCH = new Date().getTime().toString()
}

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  test: {
    globals: true,
    include: ['tests/unit/**/*.test.ts', 'src/**/*.spec.ts'],
    root: fileURLToPath(new URL('./', import.meta.url)),
    setupFiles: ['./tests/setup/mocks.ts', './tests/setup/testglobals.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'json-summary'],
      include: ['!src/main.ts', 'src/**/*.ts', 'src/**/*.vue'],
      thresholds: {
        lines: 10,
        functions: 0,
        branches: 10,
        statements: 10,
      },
    },
  },
})
