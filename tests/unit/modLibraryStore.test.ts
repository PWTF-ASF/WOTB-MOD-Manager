import { beforeEach, describe, expect, test, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useModLibraryStore } from '@/stores/modLibrary'

const mockedInvoke = vi.mocked(invoke)

const rawMods = [
  {
    name: 'voice.zip',
    display_name: '中文语音包',
    applied: true,
    conflicts: [],
    install_date: '2026-08-31T10:00:00+08:00',
    category: 'voice',
    icon_path: null,
  },
  {
    name: 'model.zip',
    display_name: '高清坦克模型',
    applied: false,
    conflicts: [],
    install_date: null,
    category: 'model',
    icon_path: null,
  },
]

describe('mod library store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    mockedInvoke.mockReset()
  })

  test('loads domain items and combines category with text filters', async () => {
    mockedInvoke.mockResolvedValueOnce(rawMods)
    const store = useModLibraryStore()
    await store.refresh()

    expect(store.items).toHaveLength(2)
    store.selectCategory('voice')
    expect(store.filteredItems.map(item => item.filename)).toEqual(['voice.zip'])

    store.searchQuery = '不存在'
    expect(store.filteredItems).toHaveLength(0)
  })

  test('selects visible items and updates every selected item across filters', async () => {
    mockedInvoke.mockResolvedValueOnce(rawMods)
    const store = useModLibraryStore()
    await store.refresh()
    store.selectCategory('model')
    store.toggleSelectAllVisible()
    store.selectCategory('voice')
    store.toggleSelectAllVisible()
    store.selectCategory('model')
    store.setSelectedEnabled(false)

    expect(store.items.find(item => item.filename === 'model.zip')).toMatchObject({
      selected: true,
      desiredEnabled: false,
    })
    expect(store.items.find(item => item.filename === 'voice.zip')).toMatchObject({
      selected: true,
      desiredEnabled: false,
    })
    expect(store.pendingChanges.map(item => item.filename)).toEqual(['voice.zip'])
  })

  test('preserves draft changes on ordinary refresh and discards them after deployment', async () => {
    mockedInvoke.mockResolvedValue(rawMods)
    const store = useModLibraryStore()
    await store.refresh()
    const model = store.items.find(item => item.filename === 'model.zip')!
    model.desiredEnabled = true
    model.selected = true

    await store.refresh()
    expect(store.items.find(item => item.filename === 'model.zip')).toMatchObject({
      desiredEnabled: true,
      selected: true,
    })

    await store.refresh({ discardDrafts: true })
    expect(store.items.find(item => item.filename === 'model.zip')).toMatchObject({
      desiredEnabled: false,
      selected: false,
    })
  })
})
