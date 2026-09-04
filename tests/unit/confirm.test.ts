import { beforeEach, describe, expect, test } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useConfirmStore } from '@/stores/confirm'

describe('confirm store', () => {
  beforeEach(() => setActivePinia(createPinia()))

  test('resolves custom confirmation choices', async () => {
    const store = useConfirmStore()
    const result = store.request({
      title: '确认删除',
      message: '确定删除这个 Mod 吗？',
      tone: 'danger',
      confirmLabel: '删除',
    })

    expect(store.current).toMatchObject({
      title: '确认删除',
      message: '确定删除这个 Mod 吗？',
      confirmLabel: '删除',
      cancelLabel: '取消',
      showCancel: true,
      tone: 'danger',
    })

    store.confirm()
    await expect(result).resolves.toBe(true)
    expect(store.current).toBeNull()
  })

  test('supports acknowledgement dialogs without a cancel action', async () => {
    const store = useConfirmStore()
    const result = store.request({ message: '发现冲突', confirmLabel: '知道了', showCancel: false })

    expect(store.current?.showCancel).toBe(false)
    store.confirm()
    await expect(result).resolves.toBe(true)
  })

  test('queues a second request until the active dialog is handled', async () => {
    const store = useConfirmStore()
    const first = store.request({ message: '第一项' })
    const second = store.request({ message: '第二项' })

    expect(store.current?.message).toBe('第一项')
    store.cancel()
    await expect(first).resolves.toBe(false)
    expect(store.current?.message).toBe('第二项')
    store.confirm()
    await expect(second).resolves.toBe(true)
  })
})
