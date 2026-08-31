import { beforeEach, describe, expect, test, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { settingsService } from '@/services/tauri/settingsService'

const mockedInvoke = vi.mocked(invoke)

describe('settingsService', () => {
  beforeEach(() => {
    mockedInvoke.mockReset()
    mockedInvoke.mockResolvedValue(undefined)
  })

  test('opens the configured repository through the dedicated backend command', async () => {
    await settingsService.openModRepository()

    expect(mockedInvoke).toHaveBeenCalledWith('open_mod_repo')
  })
})
