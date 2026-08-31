import { invoke } from '@tauri-apps/api/core'
import type { ImportModsSummary, ModCategory, ModItem, RawModInfo } from '@/types/mod'
import { MOD_CATEGORIES } from '@/types/mod'
import { formatTauriError, runTauriOperation } from './errors'

function normalizeCategory(category: string | null): ModCategory {
  return MOD_CATEGORIES.includes(category as ModCategory) ? (category as ModCategory) : 'unknown'
}

function toModItem(info: RawModInfo): ModItem {
  return {
    id: info.name,
    filename: info.name,
    displayName: info.display_name,
    category: normalizeCategory(info.category),
    iconPath: info.icon_path,
    installDate: info.install_date,
    conflicts: info.conflicts ?? [],
    deployed: info.applied,
    desiredEnabled: info.applied,
  }
}

export const modService = {
  async list(): Promise<ModItem[]> {
    return runTauriOperation('读取 Mod 列表', async () => {
      const items = await invoke<RawModInfo[]>('get_mods_with_status')
      return items.map(toModItem)
    })
  },

  async importFiles(paths: string[]): Promise<ImportModsSummary> {
    const results = []

    for (const path of paths) {
      try {
        await invoke('copy_mod_file', { src: path })
        results.push({ path, success: true as const })
      } catch (error) {
        results.push({ path, success: false as const, error: formatTauriError(error) })
      }
    }

    const succeeded = results.filter(result => result.success).length
    return {
      total: paths.length,
      succeeded,
      failed: paths.length - succeeded,
      results,
    }
  },

  remove(filename: string): Promise<void> {
    return runTauriOperation('删除 Mod', () => invoke('delete_mod_file', { modName: filename }))
  },

  rename(filename: string, displayName: string): Promise<void> {
    return runTauriOperation('重命名 Mod', () =>
      invoke('rename_mod', { originalFilename: filename, newDisplayName: displayName }),
    )
  },

  updateCategory(filename: string, category: ModCategory | null): Promise<void> {
    return runTauriOperation('修改 Mod 分类', () =>
      invoke('update_mod_category', { originalFilename: filename, category }),
    )
  },

  setIcon(filename: string, iconPath: string): Promise<string> {
    return runTauriOperation('设置 Mod 图标', () =>
      invoke<string>('set_mod_icon', { modName: filename, imagePath: iconPath }),
    )
  },

  clearIcon(filename: string): Promise<void> {
    return runTauriOperation('移除 Mod 图标', () => invoke('clear_mod_icon', { modName: filename }))
  },

  deploy(filenames: string[]): Promise<void> {
    return runTauriOperation('部署 Mod', () => invoke('deploy_mods', { modNames: filenames }))
  },
}
