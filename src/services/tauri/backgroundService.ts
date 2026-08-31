import { invoke } from '@tauri-apps/api/core'
import { runTauriOperation } from './errors'

function imageMimeType(path: string): string {
  const extension = path.toLowerCase().split('.').pop()
  if (extension === 'png') return 'image/png'
  if (extension === 'webp') return 'image/webp'
  return 'image/jpeg'
}

export const backgroundService = {
  getPath(): Promise<string | null> {
    return runTauriOperation('读取背景图片', () => invoke<string | null>('get_background_image'))
  },

  set(imagePath: string): Promise<string> {
    return runTauriOperation('保存背景图片', () =>
      invoke<string>('set_background_image', { imagePath }),
    )
  },

  remove(): Promise<void> {
    return runTauriOperation('移除背景图片', () => invoke('remove_background_image'))
  },

  async readAsDataUrl(path: string): Promise<string> {
    return runTauriOperation('读取背景图片数据', async () => {
      const base64 = await invoke<string>('read_image_base64', { path })
      return `data:${imageMimeType(path)};base64,${base64}`
    })
  },
}
