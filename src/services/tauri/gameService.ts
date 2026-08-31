import { invoke } from '@tauri-apps/api/core'
import { runTauriOperation } from './errors'

export const gameService = {
  getPath(): Promise<string | null> {
    return runTauriOperation('读取游戏路径', () => invoke<string | null>('get_game_path'))
  },

  setPath(path: string): Promise<void> {
    return runTauriOperation('保存游戏路径', () => invoke('set_game_path', { path }))
  },

  launch(): Promise<void> {
    return runTauriOperation('启动游戏', () => invoke('launch_game'))
  },
}
