import { computed, ref, watch } from 'vue'
import { defineStore } from 'pinia'
import { convertFileSrc } from '@tauri-apps/api/core'
import { backgroundService } from '@/services/tauri/backgroundService'
import type { BackgroundMode, ThemeMode } from '@/types/settings'

type Platform = 'windows' | 'linux' | 'macos' | 'unknown'

const DEFAULTS = {
  themeMode: 'system' as ThemeMode,
  backgroundMask: true,
  maskOpacity: 20,
  enableBackgroundBlur: true,
  backgroundBlurAmount: 10,
  backgroundMode: 'cover' as BackgroundMode,
}

const STORAGE_KEYS = {
  themeMode: 'app-theme-mode',
  resolvedTheme: 'app-theme',
  backgroundMask: 'app-background-mask',
  maskOpacity: 'app-mask-opacity',
  enableBlur: 'app-enable-blur',
  blurAmount: 'app-blur-amount',
  backgroundMode: 'app-background-mode',
}

function readBoolean(key: string, fallback: boolean): boolean {
  if (typeof localStorage === 'undefined') return fallback
  const value = localStorage.getItem(key)
  if (value === null) return fallback
  try {
    return JSON.parse(value) === true
  } catch {
    return fallback
  }
}

function readNumber(key: string, fallback: number, min: number, max: number): number {
  if (typeof localStorage === 'undefined') return fallback
  const parsed = Number(localStorage.getItem(key))
  return Number.isFinite(parsed) ? Math.min(max, Math.max(min, parsed)) : fallback
}

function detectPlatform(): Platform {
  if (typeof navigator === 'undefined') return 'unknown'
  const agent = navigator.userAgent.toLowerCase()
  if (agent.includes('win')) return 'windows'
  if (agent.includes('linux')) return 'linux'
  if (agent.includes('mac') || agent.includes('apple')) return 'macos'
  return 'unknown'
}

export const usePreferencesStore = defineStore('preferences', () => {
  const themeMode = ref<ThemeMode>(DEFAULTS.themeMode)
  const systemPrefersDark = ref(false)
  const platform = ref<Platform>('unknown')
  const backgroundMask = ref(DEFAULTS.backgroundMask)
  const maskOpacity = ref(DEFAULTS.maskOpacity)
  const enableBackgroundBlur = ref(DEFAULTS.enableBackgroundBlur)
  const backgroundBlurAmount = ref(DEFAULTS.backgroundBlurAmount)
  const backgroundMode = ref<BackgroundMode>(DEFAULTS.backgroundMode)
  const backgroundImagePath = ref<string | null>(null)
  const backgroundImageDataUrl = ref<string | null>(null)
  const initialized = ref(false)
  let mediaQuery: MediaQueryList | null = null

  const isLinux = computed(() => platform.value === 'linux')
  const isDark = computed(() =>
    themeMode.value === 'system' ? systemPrefersDark.value : themeMode.value === 'dark',
  )

  const backgroundImageUrl = computed(() => {
    if (!backgroundImagePath.value) return null
    if (isLinux.value) return backgroundImageDataUrl.value
    return convertFileSrc(backgroundImagePath.value)
  })

  const backgroundStyle = computed<Record<string, string>>(() => {
    let backgroundSize = 'cover'
    let backgroundRepeat = 'no-repeat'
    if (backgroundMode.value === 'contain') backgroundSize = 'contain'
    if (backgroundMode.value === 'fill') backgroundSize = '100% 100%'
    if (backgroundMode.value === 'tile') {
      backgroundSize = 'auto'
      backgroundRepeat = 'repeat'
    }

    const overlay = isDark.value
      ? `rgba(0, 0, 0, ${backgroundMask.value ? maskOpacity.value / 100 : 0})`
      : `rgba(255, 255, 255, ${backgroundMask.value ? (85 + maskOpacity.value * 0.15) / 100 : 0})`

    return {
      ...(backgroundImageUrl.value ? { backgroundImage: `url("${backgroundImageUrl.value}")` } : {}),
      backgroundSize,
      backgroundRepeat,
      backgroundPosition: 'center',
      backgroundAttachment: 'fixed',
      filter: enableBackgroundBlur.value && backgroundBlurAmount.value > 0
        ? `blur(${backgroundBlurAmount.value}px)`
        : 'none',
      '--app-bg-overlay': overlay,
    }
  })

  function hydrate() {
    if (typeof localStorage === 'undefined') return

    const savedTheme = localStorage.getItem(STORAGE_KEYS.themeMode)
    if (savedTheme === 'light' || savedTheme === 'dark' || savedTheme === 'system') {
      themeMode.value = savedTheme
    }
    backgroundMask.value = readBoolean(STORAGE_KEYS.backgroundMask, DEFAULTS.backgroundMask)
    maskOpacity.value = readNumber(STORAGE_KEYS.maskOpacity, DEFAULTS.maskOpacity, 0, 100)
    enableBackgroundBlur.value = readBoolean(STORAGE_KEYS.enableBlur, DEFAULTS.enableBackgroundBlur)
    backgroundBlurAmount.value = readNumber(STORAGE_KEYS.blurAmount, DEFAULTS.backgroundBlurAmount, 0, 20)

    const savedMode = localStorage.getItem(STORAGE_KEYS.backgroundMode)
    if (savedMode === 'cover' || savedMode === 'contain' || savedMode === 'fill' || savedMode === 'tile') {
      backgroundMode.value = savedMode
    }
  }

  function persist() {
    if (!initialized.value || typeof localStorage === 'undefined') return
    localStorage.setItem(STORAGE_KEYS.themeMode, themeMode.value)
    localStorage.setItem(STORAGE_KEYS.resolvedTheme, isDark.value ? 'dark' : 'light')
    localStorage.setItem(STORAGE_KEYS.backgroundMask, JSON.stringify(backgroundMask.value))
    localStorage.setItem(STORAGE_KEYS.maskOpacity, String(maskOpacity.value))
    localStorage.setItem(STORAGE_KEYS.enableBlur, JSON.stringify(enableBackgroundBlur.value))
    localStorage.setItem(STORAGE_KEYS.blurAmount, String(backgroundBlurAmount.value))
    localStorage.setItem(STORAGE_KEYS.backgroundMode, backgroundMode.value)
  }

  function applyThemeClass() {
    if (typeof document === 'undefined') return
    document.documentElement.classList.toggle('dark-mode', isDark.value)
    document.documentElement.classList.toggle('light-mode', !isDark.value)
    document.documentElement.dataset.theme = isDark.value ? 'dark' : 'light'
  }

  async function loadBackground() {
    backgroundImagePath.value = await backgroundService.getPath()
    if (isLinux.value && backgroundImagePath.value) {
      backgroundImageDataUrl.value = await backgroundService.readAsDataUrl(backgroundImagePath.value)
    }
  }

  async function setBackgroundImage(imagePath: string | null) {
    if (imagePath === null) {
      await backgroundService.remove()
      backgroundImagePath.value = null
      backgroundImageDataUrl.value = null
      return
    }

    const savedPath = await backgroundService.set(imagePath)
    backgroundImagePath.value = savedPath
    backgroundImageDataUrl.value = isLinux.value
      ? await backgroundService.readAsDataUrl(savedPath)
      : null
  }

  function resetVisualSettings() {
    backgroundMask.value = DEFAULTS.backgroundMask
    maskOpacity.value = DEFAULTS.maskOpacity
    enableBackgroundBlur.value = DEFAULTS.enableBackgroundBlur
    backgroundBlurAmount.value = DEFAULTS.backgroundBlurAmount
    backgroundMode.value = DEFAULTS.backgroundMode
  }

  async function initialize() {
    if (initialized.value) return
    platform.value = detectPlatform()
    hydrate()

    if (typeof window !== 'undefined' && window.matchMedia) {
      mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      systemPrefersDark.value = mediaQuery.matches
      mediaQuery.addEventListener('change', handleSystemThemeChange)
    }

    initialized.value = true
    applyThemeClass()
    persist()

    try {
      await loadBackground()
    } catch (error) {
      console.error('加载背景图片失败:', error)
    }
  }

  function handleSystemThemeChange(event: MediaQueryListEvent) {
    systemPrefersDark.value = event.matches
  }

  function dispose() {
    mediaQuery?.removeEventListener('change', handleSystemThemeChange)
    mediaQuery = null
    initialized.value = false
  }

  watch(
    [themeMode, isDark, backgroundMask, maskOpacity, enableBackgroundBlur, backgroundBlurAmount, backgroundMode],
    () => {
      applyThemeClass()
      persist()
    },
  )

  return {
    themeMode,
    platform,
    isLinux,
    isDark,
    backgroundMask,
    maskOpacity,
    enableBackgroundBlur,
    backgroundBlurAmount,
    backgroundMode,
    backgroundImagePath,
    backgroundImageDataUrl,
    backgroundImageUrl,
    backgroundStyle,
    initialized,
    initialize,
    dispose,
    setBackgroundImage,
    resetVisualSettings,
  }
})
