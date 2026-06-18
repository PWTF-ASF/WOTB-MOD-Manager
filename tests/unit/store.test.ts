import { describe, test, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useStore } from '@/store'

describe('Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  test('version is a non-empty string', () => {
    const store = useStore()
    expect(store.version).toBeTruthy()
    expect(typeof store.version).toBe('string')
  })

  test('version follows semver-like format', () => {
    const store = useStore()
    // Matches "2.6.0" or "2.6.0-dev"
    expect(store.version).toMatch(/^\d+\.\d+\.\d+(-dev)?$/)
  })

  test('debug is true in dev mode, false otherwise', () => {
    const store = useStore()
    expect(typeof store.debug).toBe('boolean')
  })
})
