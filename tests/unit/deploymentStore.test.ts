import { beforeEach, describe, expect, test, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useDeploymentStore } from '@/stores/deployment'
import type { DeployProgressPayload } from '@/types/deployment'

const mockedInvoke = vi.mocked(invoke)
const mockedListen = vi.mocked(listen)

describe('deployment store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockedInvoke.mockReset()
    mockedListen.mockReset()
  })

  test('tracks per-item progress and preserves partial failure results', async () => {
    let progressHandler: ((event: { payload: DeployProgressPayload }) => void) | undefined
    const unlisten = vi.fn()
    mockedListen.mockImplementation(async (_event, handler) => {
      progressHandler = handler as typeof progressHandler
      return unlisten
    })
    mockedInvoke.mockImplementation(async command => {
      if (command !== 'deploy_mods') return null
      progressHandler?.({ payload: { current: 1, total: 2, mod_name: 'one.zip', status: 'installing' } })
      progressHandler?.({ payload: { current: 1, total: 2, mod_name: 'one.zip', status: 'done' } })
      progressHandler?.({ payload: { current: 2, total: 2, mod_name: 'two.zip', status: 'error' } })
      throw new Error('1 个成功, 1 个失败')
    })

    const store = useDeploymentStore()
    const summary = await store.deploy(['one.zip', 'two.zip'])

    expect(summary).toMatchObject({ succeeded: 1, failed: 1 })
    expect(store.results).toEqual([
      { name: 'one.zip', status: 'success' },
      { name: 'two.zip', status: 'error' },
    ])
    expect(store.errors).toEqual(['two.zip: 安装失败'])
    expect(store.isDeploying).toBe(false)
    expect(unlisten).toHaveBeenCalledOnce()
  })

  test('reports listener setup failures without leaving deployment locked', async () => {
    mockedListen.mockRejectedValueOnce(new Error('事件系统不可用'))
    const store = useDeploymentStore()

    const summary = await store.deploy(['one.zip'])

    expect(summary.commandError).toContain('事件系统不可用')
    expect(store.isDeploying).toBe(false)
    expect(store.errors[0]).toContain('事件系统不可用')
  })
})
