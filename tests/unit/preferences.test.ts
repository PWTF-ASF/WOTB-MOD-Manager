import { beforeEach, describe, expect, test, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePreferencesStore } from '@/stores/preferences'

const mockedInvoke = vi.mocked(invoke)

function createStorage(initial: Record<string, string> = {}) {
  const values = new Map(Object.entries(initial))
  return {
    getItem: vi.fn((key: string) => values.get(key) ?? null),
    setItem: vi.fn((key: string, value: string) => values.set(key, value)),
    removeItem: vi.fn((key: string) => values.delete(key)),
    clear: vi.fn(() => values.clear()),
    key: vi.fn((index: number) => [...values.keys()][index] ?? null),
    get length() { return values.size },
  }
}

describe('preferences store', () => {
  beforeEach(() => {
    vi.useRealTimers()
    setActivePinia(createPinia())
    mockedInvoke.mockReset()
    mockedInvoke.mockResolvedValue(null)
  })

  test('hydrates valid saved preferences and clamps numeric values', async () => {
    const storage = createStorage({
      'app-theme-mode': 'dark',
      'app-background-mask': 'false',
      'app-mask-opacity': '180',
      'app-enable-blur': 'true',
      'app-blur-amount': '-5',
      'app-background-mode': 'contain',
    })
    vi.stubGlobal('localStorage', storage)

    const store = usePreferencesStore()
    await store.initialize()

    expect(store.themeMode).toBe('dark')
    expect(store.isDark).toBe(true)
    expect(store.backgroundMask).toBe(false)
    expect(store.maskOpacity).toBe(100)
    expect(store.backgroundBlurAmount).toBe(0)
    expect(store.backgroundMode).toBe('contain')
    store.dispose()
    vi.unstubAllGlobals()
  })

  test('persists changes after initialization', async () => {
    const storage = createStorage()
    vi.stubGlobal('localStorage', storage)

    const store = usePreferencesStore()
    await store.initialize()
    store.maskOpacity = 36
    store.backgroundMode = 'tile'
    await nextTick()

    expect(storage.setItem).toHaveBeenCalledWith('app-mask-opacity', '36')
    expect(storage.setItem).toHaveBeenCalledWith('app-background-mode', 'tile')
    store.dispose()
    vi.unstubAllGlobals()
  })

  test('updates background state only after the backend saves the image', async () => {
    const storage = createStorage()
    vi.stubGlobal('localStorage', storage)
    mockedInvoke
      .mockResolvedValueOnce(null)
      .mockResolvedValueOnce('C:/saved/background.webp')

    const store = usePreferencesStore()
    await store.initialize()
    await store.setBackgroundImage('C:/source/background.webp')

    expect(mockedInvoke).toHaveBeenNthCalledWith(2, 'set_background_image', {
      imagePath: 'C:/source/background.webp',
    })
    expect(store.backgroundImagePath).toBe('C:/saved/background.webp')
    expect(store.backgroundImageUrl).toContain('C:/saved/background.webp')
    store.dispose()
    vi.unstubAllGlobals()
  })
})
