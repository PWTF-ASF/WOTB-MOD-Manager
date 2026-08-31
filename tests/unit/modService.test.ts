import { beforeEach, describe, expect, test, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { modService } from '@/services/tauri/modService'

const mockedInvoke = vi.mocked(invoke)

describe('modService', () => {
  beforeEach(() => {
    mockedInvoke.mockReset()
  })

  test('maps Rust payloads into frontend domain items', async () => {
    mockedInvoke.mockResolvedValueOnce([
      {
        name: 'voice-pack.zip',
        display_name: '中文语音包',
        applied: true,
        conflicts: ['other.zip'],
        install_date: '2026-08-31T10:00:00+08:00',
        category: 'voice',
        icon_path: null,
      },
      {
        name: 'mystery.zip',
        display_name: '未知内容',
        applied: false,
        conflicts: [],
        install_date: null,
        category: 'unexpected-value',
        icon_path: 'C:/icons/mystery.png',
      },
    ])

    await expect(modService.list()).resolves.toEqual([
      {
        id: 'voice-pack.zip',
        filename: 'voice-pack.zip',
        displayName: '中文语音包',
        category: 'voice',
        iconPath: null,
        installDate: '2026-08-31T10:00:00+08:00',
        conflicts: ['other.zip'],
        deployed: true,
        desiredEnabled: true,
      },
      {
        id: 'mystery.zip',
        filename: 'mystery.zip',
        displayName: '未知内容',
        category: 'unknown',
        iconPath: 'C:/icons/mystery.png',
        installDate: null,
        conflicts: [],
        deployed: false,
        desiredEnabled: false,
      },
    ])
  })

  test('returns a complete summary when only some imports succeed', async () => {
    mockedInvoke
      .mockResolvedValueOnce(undefined)
      .mockRejectedValueOnce('压缩包损坏')
      .mockResolvedValueOnce(undefined)

    const result = await modService.importFiles(['one.zip', 'broken.zip', 'two.zip'])

    expect(result).toMatchObject({ total: 3, succeeded: 2, failed: 1 })
    expect(result.results[1]).toEqual({ path: 'broken.zip', success: false, error: '压缩包损坏' })
    expect(mockedInvoke).toHaveBeenNthCalledWith(1, 'copy_mod_file', { src: 'one.zip' })
  })

  test('wraps command failures with operation context', async () => {
    mockedInvoke.mockRejectedValueOnce('无法读取目录')

    await expect(modService.list()).rejects.toMatchObject({
      name: 'TauriServiceError',
      operation: '读取 Mod 列表',
      message: '无法读取目录',
    })
  })

  test('passes the image using the argument name expected by the Rust command', async () => {
    mockedInvoke.mockResolvedValueOnce('C:/icons/voice-pack.png')

    await modService.setIcon('voice-pack.zip', 'C:/images/custom.png')

    expect(mockedInvoke).toHaveBeenCalledWith('set_mod_icon', {
      modName: 'voice-pack.zip',
      imagePath: 'C:/images/custom.png',
    })
  })
})
