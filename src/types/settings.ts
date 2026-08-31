export type ThemeMode = 'light' | 'dark' | 'system'
export type BackgroundMode = 'cover' | 'contain' | 'fill' | 'tile'

export interface VisualPreferences {
  themeMode: ThemeMode
  backgroundMask: boolean
  maskOpacity: number
  enableBackgroundBlur: boolean
  backgroundBlurAmount: number
  backgroundMode: BackgroundMode
}

export interface AppPaths {
  gamePath: string | null
  modRepositoryPath: string | null
}
