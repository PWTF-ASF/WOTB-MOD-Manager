<template>
  <div class="mod-library" @dragover.prevent="onDragOver" @dragleave="onDragLeave" @drop.prevent="onDrop">
    <!-- 拖拽导入覆盖层 -->
    <Teleport to="body">
      <div v-if="isDraggingOver" class="drop-overlay">
        <div class="drop-zone">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" aria-hidden="true"><path d="M12 4v16M4 12h16" /></svg>
          <span>释放文件以导入模组</span>
        </div>
      </div>
    </Teleport>

    <DeployProgressDialog v-model="showDeployOverlay" :deploying="isDeploying" :current="deployCurrent" :total="deployTotal" :current-name="deployCurrentName" :progress="deployProgressPercent" :results="deployResults" :errors="deployErrors" />
    <ModToolbar v-if="modlist.length" v-model:search="searchQuery" v-model:category="currentCategory" :categories="categories" :is-grid="isGridLayout" :total-count="modlist.length" :visible-count="filtermodlist.length" @toggle-layout="toggleLayout" />

    <!-- ===== Mod 卡片网格 ===== -->
    <main class="modules-grid">
      <!-- 加载状态 -->
      <div v-if="isLoading" class="state-container">
        <UiSpinner :size="28" />
        <span class="state-text">正在加载 Mod 列表...</span>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="loadError" class="state-container error">
        <svg class="state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m9 9 6 6M15 9l-6 6"/></svg>
        <span class="state-text">{{ loadError }}</span>
        <UiButton size="small" @click="refreshModList">重试</UiButton>
      </div>

      <WelcomeMenu v-else-if="modlist.length === 0" @mods-added="refreshModList" />

      <!-- 网格布局 -->
      <div v-if="isGridLayout && !isLoading && !loadError && modlist.length > 0" class="grid-container">
        <ModCard v-for="mod in filtermodlist" :key="mod.id" :item="mod" :icon-url="getModIconUrl(mod)" :category-label="formatType(mod.category)" @update:selected="mod.selected=$event" @update:enabled="mod.desiredEnabled=$event" @edit="handleEditMod(mod)" @delete="handleDeleteMod(mod.id)" @category="handleEditCategory(mod)" @icon="openIconModal(mod)" @icon-load="onIconLoad(mod)" @icon-error="onIconLoadError(mod)" />
      </div>

      <!-- 列表布局 -->
      <div v-if="!isGridLayout && !isLoading && !loadError && modlist.length > 0" class="list-container">
        <ModListItem v-for="mod in filtermodlist" :key="mod.id" :item="mod" :icon-url="getModIconUrl(mod)" :category-label="formatType(mod.category)" :date-label="formatDate(mod.installDate)" @update:selected="mod.selected=$event" @update:enabled="mod.desiredEnabled=$event" @edit="handleEditMod(mod)" @delete="handleDeleteMod(mod.id)" @icon="openIconModal(mod)" @icon-load="onIconLoad(mod)" @icon-error="onIconLoadError(mod)" />
      </div>

      <!-- 无匹配结果（搜索/筛选后为空） -->
      <div
        v-if="!isLoading && !loadError && modlist.length > 0 && filtermodlist.length === 0"
        class="state-container empty"
      >
        <svg class="state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><circle cx="11" cy="11" r="7"/><path d="m20 20-4-4"/></svg>
        <span class="state-text">没有找到匹配的模组</span>
        <span class="state-hint">试试其他关键词，或切换分类查看</span>
      </div>
    </main>

    <ModSelectionBar v-if="modlist.length" :selected-count="library.selectedItems.length" :pending-count="library.pendingChanges.length" :all-selected="isAllSelected" :deploying="isDeploying" :launching="isLaunching" @select-all="selectAll" @enable="handleBatchToggle(true)" @disable="handleBatchToggle(false)" @delete="handleBatchDelete" @add="handleAddMod" @deploy="handleDeployMods" @launch="handleLaunchGame" />
    <ModIconDialog v-model="showIconModal" :title="currentMod?.displayName ?? 'Mod 图标'" :image-url="currentIconPreviewUrl" :loading="loadingIcon" :uploading="uploadingIcon" :has-icon="Boolean(currentMod?.iconPath)" @upload="uploadIconForCurrentMod" @remove="clearCurrentModIcon" @loaded="onIconLoadSuccess" @error="onIconPreviewError" />
    <ModEditorDialog v-model="showEditModal" v-model:value="editValue" :mode="editType" :loading="editLoading" @confirm="confirmEdit" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, onActivated, onDeactivated, inject } from 'vue'
import type { Ref } from 'vue'
import { storeToRefs } from 'pinia'
import { MOD_CATEGORIES, type ModCategory } from '@/types/mod'
import type { ModLibraryCategory, ModLibraryItem } from '@/stores/modLibrary'
import { useModLibraryStore } from '@/stores/modLibrary'
import { useDeploymentStore } from '@/stores/deployment'
import { gameService } from '@/services/tauri/gameService'
import { formatTauriError } from '@/services/tauri/errors'

// 缓存失效信号（来自 HomePage 的 provide）
const modListVersion = inject<Ref<number>>('modListVersion', ref(0))
let lastSeenVersion = 0
import { convertFileSrc } from '@tauri-apps/api/core'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { open } from '@tauri-apps/plugin-dialog'
import WelcomeMenu from './WelcomeMenu.vue'
import { useToast } from '@/composables/useToast'
import { useConfirmDialog } from '@/composables/useConfirmDialog'
import UiButton from '@/ui/button/UiButton.vue'
import UiSpinner from '@/ui/feedback/UiSpinner.vue'
import ModToolbar from '@/features/mods/components/ModToolbar.vue'
import ModCard from '@/features/mods/components/ModCard.vue'
import ModListItem from '@/features/mods/components/ModListItem.vue'
import ModSelectionBar from '@/features/mods/components/ModSelectionBar.vue'
import DeployProgressDialog from '@/features/mods/components/DeployProgressDialog.vue'
import ModEditorDialog from '@/features/mods/components/ModEditorDialog.vue'
import ModIconDialog from '@/features/mods/components/ModIconDialog.vue'

// ================= 响应式数据 =================
const notify = useToast()
const { confirm } = useConfirmDialog()
const library = useModLibraryStore()
const deployment = useDeploymentStore()
const {
  items: modlist,
  isGridLayout,
  currentCategory,
  filteredItems: filtermodlist,
  isAllVisibleSelected: isAllSelected,
  isLoading,
  loadError,
  searchQuery,
  desiredEnabledFilenames,
} = storeToRefs(library)
const {
  isDeploying,
  showOverlay: showDeployOverlay,
  current: deployCurrent,
  total: deployTotal,
  currentName: deployCurrentName,
  results: deployResults,
  errors: deployErrors,
  progressPercent: deployProgressPercent,
} = storeToRefs(deployment)

const isLaunching = ref(false)
const loadingIcon = ref(false)      // 预览图片加载状态
const isDraggingOver = ref(false)   // 拖拽导入状态
const brokenIconPaths = ref<Set<string>>(new Set()) // 加载失败的图标路径
let dragCounter = 0                 // 拖拽进出计数

// 图标预览模态框相关
const showIconModal = ref(false)      // 控制模态框显示
const currentMod = ref<ModLibraryItem | null>(null)          // 当前选中的 Mod
const uploadingIcon = ref(false)      // 上传/移除图标时的加载状态

// 编辑模态框相关（重命名 / 修改分类）
const showEditModal = ref(false)
const editingMod = ref<ModLibraryItem | null>(null)
const editValue = ref('')
const editType = ref<'rename' | 'category'>('rename')
const editLoading = ref(false)

// ================= 常量 =================
const categories: Array<{ type: ModLibraryCategory; name: string }> = [
  { type: 'all', name: '全部' },
  { type: 'model', name: '3d改模' },
  { type: 'voice', name: '语音包' },
  { type: 'ui', name: 'UI' },
  { type: 'lightIcon', name: '点亮' },
  { type: 'script', name: '扩展脚本' },
  { type: 'map', name: '地图纹理' },
]

const TYPE_MAP = {
  all: '全部',
  model: '3d模型',
  voice: '语音包',
  ui: 'UI',
  lightIcon: '点亮图标',
  script: '扩展脚本',
  map: '地图纹理',
  unknown: '未知'
}

const currentIconPreviewUrl = computed(() => {
  if (!currentMod.value) return null
  return getModIconUrl(currentMod.value)
})

// ================= 方法 =================
const toggleLayout = () => {
  library.toggleLayout()
}

const formatType = (type: string) => TYPE_MAP[type as keyof typeof TYPE_MAP] || '未知类型'

// 全选/取消全选
const selectAll = () => {
  library.toggleSelectAllVisible()
}

// 批量启用/禁用
const handleBatchToggle = (enable: boolean) => {
  library.setSelectedVisibleEnabled(enable)
}

// 批量删除
const handleBatchDelete = async () => {
  const selectedMods = [...library.selectedVisibleItems]
  if (selectedMods.length === 0) {
    notify.warning('请先选择要删除的Mod')
    return
  }
  const confirmDelete = await confirm({
    title: '确认删除',
    message: `确定要删除选中的 ${selectedMods.length} 个 Mod 吗？`,
    tone: 'danger',
    confirmLabel: '删除',
  })
  if (!confirmDelete) return
  const summary = await library.removeSelectedVisible()
  if (summary.failed.length === 0) {
    notify.success(`已成功删除 ${summary.succeeded.length} 个Mod`)
  } else if (summary.succeeded.length === 0) {
    notify.error(`批量删除失败: ${summary.failed[0].error}`)
  } else {
    notify.warning(`已删除 ${summary.succeeded.length} 个，${summary.failed.length} 个删除失败`)
  }
}

// 导入文件列表
const importFiles = async (paths: string[]) => {
  const summary = await library.importFiles(paths)
  if (summary.failed === 0) {
    notify.success(summary.succeeded === 1 ? '已添加 1 个模组' : `已添加 ${summary.succeeded} 个模组`)
  } else if (summary.succeeded === 0) {
    notify.error(`导入失败: ${summary.results.find(result => !result.success)?.error ?? '未知错误'}`)
  } else {
    notify.warning(`已添加 ${summary.succeeded} 个模组，${summary.failed} 个导入失败`)
  }
}

// 添加 Mod（支持批量选择）
const handleAddMod = async () => {
  try {
    const selected = await open({
      title: '请选择mod文件（可多选）',
      multiple: true,
      directory: false,
    })
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      await importFiles(paths)
    }
  } catch (error) {
    console.error('出错了:', error)
    notify.error(`添加Mod时出错: ${error}`)
  }
}

// 拖拽导入
const onDragOver = () => {
  isDraggingOver.value = true
}

const onDragLeave = (e) => {
  if (e.currentTarget === e.target) {
    isDraggingOver.value = false
  }
}

const onDrop = async (e) => {
  isDraggingOver.value = false
  const files = e.dataTransfer?.files
  if (!files || files.length === 0) return

  const paths = []
  for (const file of files) {
    if (file.path) {
      paths.push(file.path)
    }
  }

  if (paths.length > 0) {
    await importFiles(paths)
  }
}

const refreshModList = () => library.refresh()

// 删除单个 Mod
const handleDeleteMod = async (id: string) => {
  try {
    const mod = modlist.value.find(candidate => candidate.id === id)
    if (!mod) return
    const confirmDelete = await confirm({
      title: '确认删除',
      message: `确定要删除“${mod.displayName}”吗？`,
      tone: 'danger',
      confirmLabel: '删除',
    })
    if (!confirmDelete) return
    await library.remove(mod.filename)
    notify.success(`"${mod.displayName}" 已成功删除！`)
  } catch (error) {
    console.error('删除失败:', error)
    notify.error(`删除失败: ${formatTauriError(error)}`)
  }
}

// 部署 Mod
const handleDeployMods = async () => {
  if (isDeploying.value) return
  const activeModNames = desiredEnabledFilenames.value
  if (activeModNames.length === 0) {
    const proceed = await confirm({
      title: '确认部署',
      message: '当前未启用任何 Mod，是否继续部署？',
      confirmLabel: '继续部署',
    })
    if (!proceed) return
  }

  const summary = await deployment.deploy(activeModNames)
  if (summary.failed > 0) {
    notify.warning(`部署完成: ${summary.succeeded} 成功, ${summary.failed} 失败`)
  } else if (summary.commandError) {
    notify.error(`部署失败: ${summary.commandError}`)
  } else {
    notify.success(`部署成功！已应用 ${activeModNames.length} 个项目。`)
  }
  await library.refresh({ discardDrafts: true })
}

// 启动游戏
const handleLaunchGame = async () => {
  if (isLaunching.value) return
  isLaunching.value = true
  try {
    let gamePath = await gameService.getPath()
    if (!gamePath) {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '请选择《坦克世界闪击战》安装目录 (包含 wotblitz.exe 的文件夹)',
      })
      if (selected) {
        gamePath = selected
        await gameService.setPath(gamePath)
      } else {
        return
      }
    }
    await gameService.launch()
  } catch (error) {
    notify.error(`操作失败: ${formatTauriError(error)}`)
  } finally {
    isLaunching.value = false
  }
}

// 打开编辑模态框
const openEditModal = (mod: ModLibraryItem, type: string) => {
  editingMod.value = mod
  editType.value = type
  if (type === 'rename') {
    editValue.value = mod.displayName
  } else {
    editValue.value = mod.category === 'unknown' ? '' : mod.category
  }
  showEditModal.value = true
}

// 确认编辑
const confirmEdit = async () => {
  const mod = editingMod.value
  if (!mod || !editValue.value.trim()) return

  editLoading.value = true
  if (editType.value === 'rename') {
    const newName = editValue.value.trim()
    if (newName === mod.displayName) { showEditModal.value = false; editLoading.value = false; return }
    try {
      await library.rename(mod.filename, newName)
      showEditModal.value = false
    } catch (err) {
      notify.error(`重命名失败: ${formatTauriError(err)}`)
    }
  } else {
    const value = editValue.value.trim()
    const category = value || null
    if (category && !MOD_CATEGORIES.includes(category as ModCategory)) {
      notify.warning('请输入有效分类：model、voice、ui、lightIcon、script 或 map')
      editLoading.value = false
      return
    }
    try {
      await library.updateCategory(mod.filename, category as ModCategory | null)
      showEditModal.value = false
    } catch (err) {
      notify.error(`修改分类失败: ${formatTauriError(err)}`)
    }
  }
  editLoading.value = false
}

// 重命名处理函数 — 打开模态框
const handleEditMod = (mod: ModLibraryItem) => openEditModal(mod, 'rename')

//日期格式化函数
const formatDate = (dateStr: string | null) => {
  if (!dateStr) return '未安装'
  const date = new Date(dateStr)
  return date.toLocaleDateString()
}

//类别修改函数 — 打开模态框
const handleEditCategory = (mod: ModLibraryItem) => openEditModal(mod, 'category')


// 获取 Mod 图标的完整访问 URL
const getModIconUrl = (mod: ModLibraryItem) => {
  if (!mod.iconPath) return null
  if (brokenIconPaths.value.has(mod.iconPath)) return null
  try {
    const url = convertFileSrc(mod.iconPath)
    return url
  } catch (err) {
    console.error('转换图标路径失败:', err)
    return null
  }
}

// 图标加载成功时清除失效记录
const onIconLoad = (mod: ModLibraryItem) => {
  if (mod.iconPath) {
    brokenIconPaths.value.delete(mod.iconPath)
  }
}

// 图标加载失败时清除路径
const onIconLoadError = (mod: ModLibraryItem) => {
  console.warn('卡片图标加载失败，路径:', mod.iconPath)
  if (mod.iconPath) {
    brokenIconPaths.value.add(mod.iconPath)
  }
}

// 打开图标预览模态框
const openIconModal = (mod: ModLibraryItem) => {
  currentMod.value = mod
  const iconUrl = getModIconUrl(mod)
  if (iconUrl) {
    loadingIcon.value = true   // 有图标才显示加载状态
  } else {
    loadingIcon.value = false  // 无图标直接显示默认图标
  }
  showIconModal.value = true
}

// 上传新图标
const uploadIconForCurrentMod = async () => {
  if (!currentMod.value) return
  try {
    const selected = await open({
      title: '选择图片作为 Mod 图标',
      multiple: false,
      filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'] }]
    })
    if (!selected) return

    uploadingIcon.value = true

    const filename = currentMod.value.filename
    await library.setIcon(filename, selected)

    // 重新获取当前 Mod 的最新引用
    const updatedMod = modlist.value.find(m => m.filename === filename)
    if (updatedMod) {
      currentMod.value = updatedMod
    }

    // 设置加载状态，让 @load/@error 事件负责结束
    loadingIcon.value = true

    notify.success('图标已更新')
  } catch (err) {
    console.error('上传图标失败:', err)
    notify.error(`上传图标失败: ${formatTauriError(err)}`)
    loadingIcon.value = false
  } finally {
    uploadingIcon.value = false
  }
}

// 清除图标
const clearCurrentModIcon = async () => {
  if (!currentMod.value) return
  try {
    uploadingIcon.value = true
    const filename = currentMod.value.filename
    await library.clearIcon(filename)

    // 重新获取当前 Mod 的最新引用
    const updatedMod = modlist.value.find(m => m.filename === filename)
    if (updatedMod) {
      currentMod.value = updatedMod
    }

    // 无图标，直接显示默认图标
    loadingIcon.value = false

    notify.success('图标已移除')
  } catch (err) {
    console.error('移除图标失败:', err)
    notify.error(`移除图标失败: ${formatTauriError(err)}`)
    loadingIcon.value = false
  } finally {
    uploadingIcon.value = false
  }
}

// 图片加载成功时
const onIconLoadSuccess = () => {
  loadingIcon.value = false
}

// 图片加载失败时
const onIconPreviewError = () => {
  loadingIcon.value = false
  if (currentMod.value?.iconPath) brokenIconPaths.value.add(currentMod.value.iconPath)
  console.warn('预览图片加载失败，路径:', currentMod.value?.iconPath)
}

let unlistenDragDrop: (() => void) | null = null

const setupDragListener = async () => {
  try {
    unlistenDragDrop = await getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === 'drop' && event.payload.paths?.length > 0) {
        importFiles(event.payload.paths)
      }
      if (event.payload.type === 'leave') {
        isDraggingOver.value = false
      }
    })
  } catch {
    // 非 Tauri 环境静默失败
  }
}

const teardownDragListener = () => {
  if (unlistenDragDrop) {
    unlistenDragDrop()
    unlistenDragDrop = null
  }
}

// 首次挂载：加载数据 + 注册拖拽
onMounted(async () => {
  await refreshModList()
  lastSeenVersion = modListVersion.value
  setupDragListener()
})

// 每次切回该标签页：检查是否需要刷新 + 重新注册拖拽
onActivated(() => {
  if (modListVersion.value > lastSeenVersion) {
    refreshModList()
    lastSeenVersion = modListVersion.value
  }
  setupDragListener()
})

// 切走时：卸载拖拽监听（避免在设置页误触拖拽导入）
onDeactivated(() => {
  teardownDragListener()
})

// 应用关闭时最终清理
onUnmounted(() => {
  teardownDragListener()
  deployment.dispose()
})
</script>

<style scoped>
.mod-library {
  position: relative;
  display: flex;
  height: 100vh;
  min-width: 0;
  flex-direction: column;
  color: var(--ui-text-primary);
}

.modules-grid {
  display: flex;
  min-height: 0;
  flex: 1;
  flex-direction: column;
  overflow-y: auto;
  padding: var(--ui-space-5);
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--ui-space-4);
}

.list-container {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-2);
}

.drop-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: grid;
  place-items: center;
  color: var(--ui-accent);
  background: rgba(4, 8, 13, 0.72);
  backdrop-filter: blur(5px);
}

.drop-zone {
  display: flex;
  width: min(340px, calc(100vw - 48px));
  box-sizing: border-box;
  flex-direction: column;
  align-items: center;
  gap: var(--ui-space-4);
  padding: 48px 32px;
  border: 1px solid var(--ui-accent);
  border-radius: var(--ui-radius-xl);
  background: var(--ui-bg-elevated);
  box-shadow: var(--ui-shadow-md);
  font-size: 14px;
  font-weight: 650;
}

.drop-zone svg {
  width: 48px;
  height: 48px;
}

.state-container {
  display: flex;
  min-height: 300px;
  flex: 1;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-4);
  padding: var(--ui-space-8);
  color: var(--ui-text-secondary);
  text-align: center;
}

.state-container.error {
  color: var(--ui-danger);
}

.state-container.empty {
  color: var(--ui-text-muted);
}

.state-icon {
  width: 32px;
  height: 32px;
}

.state-text {
  max-width: 520px;
  font-size: 13px;
  line-height: 1.6;
}

.state-hint {
  margin-top: calc(var(--ui-space-2) * -1);
  color: var(--ui-text-muted);
  font-size: 12px;
}

.modules-grid::-webkit-scrollbar {
  width: 8px;
}

.modules-grid::-webkit-scrollbar-track {
  background: transparent;
}

.modules-grid::-webkit-scrollbar-thumb {
  border: 2px solid transparent;
  border-radius: 999px;
  background: var(--ui-border-strong);
  background-clip: padding-box;
}

@media (max-width: 680px) {
  .modules-grid {
    padding: var(--ui-space-3);
  }

  .grid-container {
    grid-template-columns: 1fr;
  }
}
</style>
