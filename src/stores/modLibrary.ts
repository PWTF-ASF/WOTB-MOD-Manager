import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { modService } from '@/services/tauri/modService'
import { formatTauriError } from '@/services/tauri/errors'
import type { ImportModsSummary, ModCategory, ModItem } from '@/types/mod'

export type ModLibraryCategory = ModCategory | 'all'
export type ModLibraryItem = ModItem & { selected: boolean }

export interface RemoveModsSummary {
  succeeded: string[]
  failed: Array<{ filename: string; error: string }>
}

export const useModLibraryStore = defineStore('modLibrary', () => {
  const items = ref<ModLibraryItem[]>([])
  const isLoading = ref(false)
  const loadError = ref<string | null>(null)
  const searchQuery = ref('')
  const currentCategory = ref<ModLibraryCategory>('all')
  const isGridLayout = ref(true)

  const filteredItems = computed(() => {
    const query = searchQuery.value.trim().toLocaleLowerCase()
    return items.value.filter(item => {
      const matchesCategory = currentCategory.value === 'all' || item.category === currentCategory.value
      const matchesQuery = !query || item.displayName.toLocaleLowerCase().includes(query)
      return matchesCategory && matchesQuery
    })
  })

  const selectedItems = computed(() => items.value.filter(item => item.selected))
  const selectedVisibleItems = computed(() => filteredItems.value.filter(item => item.selected))
  const isAllVisibleSelected = computed(() =>
    filteredItems.value.length > 0 && filteredItems.value.every(item => item.selected),
  )
  const desiredEnabledFilenames = computed(() =>
    items.value.filter(item => item.desiredEnabled).map(item => item.filename),
  )
  const pendingChanges = computed(() =>
    items.value.filter(item => item.deployed !== item.desiredEnabled),
  )

  async function refresh(options: { discardDrafts?: boolean } = {}) {
    isLoading.value = true
    loadError.value = null
    try {
      const previous = new Map(items.value.map(item => [item.filename, item]))
      const loaded = await modService.list()
      items.value = loaded.map(item => {
        const old = previous.get(item.filename)
        const keepDraft = old && !options.discardDrafts
        return {
          ...item,
          desiredEnabled: keepDraft ? old.desiredEnabled : item.desiredEnabled,
          selected: keepDraft ? old.selected : false,
        }
      })
    } catch (error) {
      loadError.value = `获取 Mod 列表失败：${formatTauriError(error)}`
    } finally {
      isLoading.value = false
    }
  }

  function toggleLayout() {
    isGridLayout.value = !isGridLayout.value
  }

  function selectCategory(category: ModLibraryCategory) {
    currentCategory.value = category
  }

  function toggleSelectAllVisible() {
    const nextSelected = !isAllVisibleSelected.value
    filteredItems.value.forEach(item => { item.selected = nextSelected })
  }

  function setSelectedEnabled(enabled: boolean) {
    selectedItems.value.forEach(item => { item.desiredEnabled = enabled })
  }

  async function importFiles(paths: string[]): Promise<ImportModsSummary> {
    const summary = await modService.importFiles(paths)
    if (summary.succeeded > 0) await refresh()
    return summary
  }

  async function remove(filename: string) {
    await modService.remove(filename)
    items.value = items.value.filter(item => item.filename !== filename)
  }

  async function removeSelected(): Promise<RemoveModsSummary> {
    const targets = [...selectedItems.value]
    const summary: RemoveModsSummary = { succeeded: [], failed: [] }

    for (const item of targets) {
      try {
        await modService.remove(item.filename)
        summary.succeeded.push(item.filename)
      } catch (error) {
        summary.failed.push({ filename: item.filename, error: formatTauriError(error) })
      }
    }

    if (summary.succeeded.length > 0) {
      const removed = new Set(summary.succeeded)
      items.value = items.value.filter(item => !removed.has(item.filename))
    }
    return summary
  }

  async function rename(filename: string, displayName: string) {
    await modService.rename(filename, displayName)
    const item = items.value.find(candidate => candidate.filename === filename)
    if (item) item.displayName = displayName
  }

  async function updateCategory(filename: string, category: ModCategory | null) {
    await modService.updateCategory(filename, category)
    const item = items.value.find(candidate => candidate.filename === filename)
    if (item) item.category = category ?? 'unknown'
  }

  async function setIcon(filename: string, imagePath: string) {
    const savedPath = await modService.setIcon(filename, imagePath)
    const item = items.value.find(candidate => candidate.filename === filename)
    if (item) item.iconPath = savedPath
  }

  async function clearIcon(filename: string) {
    await modService.clearIcon(filename)
    const item = items.value.find(candidate => candidate.filename === filename)
    if (item) item.iconPath = null
  }

  return {
    items,
    isLoading,
    loadError,
    searchQuery,
    currentCategory,
    isGridLayout,
    filteredItems,
    selectedItems,
    selectedVisibleItems,
    isAllVisibleSelected,
    desiredEnabledFilenames,
    pendingChanges,
    refresh,
    toggleLayout,
    selectCategory,
    toggleSelectAllVisible,
    setSelectedEnabled,
    importFiles,
    remove,
    removeSelected,
    rename,
    updateCategory,
    setIcon,
    clearIcon,
  }
})
