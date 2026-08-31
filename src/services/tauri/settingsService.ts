import { invoke } from '@tauri-apps/api/core'
import type { AppPaths } from '@/types/settings'
import { runTauriOperation } from './errors'

export const settingsService = {
  async getPaths(): Promise<AppPaths> {
    return runTauriOperation('读取应用路径', async () => {
      const [gamePath, modRepositoryPath] = await Promise.all([
        invoke<string | null>('get_game_path'),
        invoke<string | null>('get_mod_repo_path'),
      ])
      return { gamePath, modRepositoryPath }
    })
  },

  getModRepositoryPath(): Promise<string | null> {
    return runTauriOperation('读取 Mod 仓库路径', () => invoke<string | null>('get_mod_repo_path'))
  },

  openModRepository(): Promise<void> {
    return runTauriOperation('打开 Mod 仓库', () => invoke('open_mod_repo'))
  },

  setGamePath(path: string): Promise<void> {
    return runTauriOperation('保存游戏路径', () => invoke('set_game_path', { path }))
  },

  setModRepositoryPath(path: string): Promise<void> {
    return runTauriOperation('保存 Mod 仓库路径', () => invoke('set_mod_repo_path', { path }))
  },

  migrateModRepository(newPath: string): Promise<void> {
    return runTauriOperation('迁移 Mod 仓库', () => invoke('migrate_mod_repo', { newPath }))
  },
}
