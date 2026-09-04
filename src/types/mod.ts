export const MOD_CATEGORIES = ['model', 'voice', 'ui', 'lightIcon', 'script', 'map', 'unknown'] as const

export type ModCategory = (typeof MOD_CATEGORIES)[number]

export interface RawModInfo {
  name: string
  display_name: string
  applied: boolean
  conflicts: string[]
  install_date: string | null
  category: string | null
  icon_path: string | null
}

export interface ModItem {
  id: string
  filename: string
  displayName: string
  category: ModCategory
  iconPath: string | null
  installDate: string | null
  conflicts: string[]
  deployed: boolean
  desiredEnabled: boolean
}

export interface ImportModResult {
  path: string
  success: boolean
  error?: string
}

export interface ImportModsSummary {
  total: number
  succeeded: number
  failed: number
  results: ImportModResult[]
}

export interface ModConflict {
  first_mod: string
  second_mod: string
  paths: string[]
}
