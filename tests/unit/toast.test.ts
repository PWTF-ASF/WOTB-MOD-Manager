import { beforeEach, describe, expect, test, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useToastStore } from '@/stores/toast'

describe('toast store', () => {
  beforeEach(() => {
    vi.useFakeTimers()
    setActivePinia(createPinia())
  })

  test('adds and automatically removes a toast', () => {
    const store = useToastStore()
    store.push('success', '导入完成', { duration: 1000 })

    expect(store.messages).toHaveLength(1)
    expect(store.messages[0]).toMatchObject({ title: '操作成功', message: '导入完成' })

    vi.advanceTimersByTime(1000)
    expect(store.messages).toHaveLength(0)
  })

  test('deduplicates identical messages created in quick succession', () => {
    const store = useToastStore()
    const firstId = store.push('error', '路径无效')
    const secondId = store.push('error', '路径无效')

    expect(secondId).toBe(firstId)
    expect(store.messages).toHaveLength(1)
  })

  test('keeps a paused toast visible until resumed', () => {
    const store = useToastStore()
    const id = store.push('info', '处理中', { duration: 500 })
    store.pause(id)

    vi.advanceTimersByTime(1000)
    expect(store.messages).toHaveLength(1)

    store.resume(id)
    vi.advanceTimersByTime(500)
    expect(store.messages).toHaveLength(0)
  })
})
