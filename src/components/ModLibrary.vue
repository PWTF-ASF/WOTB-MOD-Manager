<template>
  <div class="mod-library" @dragover.prevent="onDragOver" @dragleave="onDragLeave" @drop.prevent="onDrop">
    <!-- 拖拽导入覆盖层 -->
    <Teleport to="body">
      <div v-if="isDraggingOver" class="drop-overlay">
        <div class="drop-zone">
          <n-icon :component="AddOutline" size="48" />
          <span>释放文件以导入模组</span>
        </div>
      </div>
    </Teleport>

    <!-- ===== 顶部栏：新拟态凸起面板 ===== -->
    <header class="top-deck">
      <div class="search-container">
        <NeumorphicSearchBox
          v-model="searchQuery"
          placeholder="搜索模组..."
        />
      </div>

      <div class="category-wrapper">
        <nav ref="navRef" class="category-nav" @wheel="handleWheel" @mousedown="handleMouseDown"
          @mousemove="handleMouseMove" @mouseup="handleMouseUp" @mouseleave="handleMouseUp">
          <button
            v-for="cat in categories" :key="cat.type"
            class="neu-pill"
            :class="{ active: currentCategory === cat.type }"
            @click="selectCategory(cat.type)"
          >
            {{ cat.name }}
          </button>
        </nav>
      </div>

      <div class="right-group">
        <button class="neu-icon-btn" @click="toggleLayout"
          :title="isGridLayout ? '切换为列表布局' : '切换为网格布局'">
          <n-icon :component="isGridLayout ? ListOutline : GridOutline" size="20" />
        </button>
      </div>
    </header>

    <!-- ===== Mod 卡片网格 ===== -->
    <main class="modules-grid">
      <!-- 网格布局 -->
      <div v-if="isGridLayout" class="grid-container">
        <div v-for="mod in filtermodlist" :key="mod.id" class="neu-card"
          :class="{ active: mod.active }">
          <!-- 顶栏 -->
          <div class="neu-card-header">
            <n-checkbox v-model:checked="mod.selected" size="small" />
            <div class="neu-card-actions">
              <button class="neu-icon-btn-xs" @click.stop="handleEditMod(mod)" title="重命名">
                <n-icon size="13"><PencilOutline /></n-icon>
              </button>
              <button class="neu-icon-btn-xs" @click.stop="handleDeleteMod(mod.id)" title="删除">
                <n-icon size="13"><CloseOutline /></n-icon>
              </button>
            </div>
          </div>

          <!-- 卡片主体 -->
          <div class="neu-card-body">
            <div class="neu-icon-box" @click.stop="openIconModal(mod)">
              <img v-if="getModIconUrl(mod)" :src="getModIconUrl(mod)" alt="icon" class="neu-icon-img"
                @error="onIconLoadError(mod)" />
              <n-icon v-else size="36" :depth="2">
                <DocumentOutline />
              </n-icon>
            </div>
            <div class="neu-card-info">
              <span class="neu-mod-name">{{ mod.displayName }}</span>
              <n-tag size="tiny" :bordered="false" class="neu-tag"
                @click.stop="handleEditCategory(mod)">
                {{ formatType(mod.type) }}
              </n-tag>
            </div>
            <div class="neu-card-meta">
              <span>{{ formatDate(mod.installDate) }}</span>
            </div>
          </div>

          <!-- 卡底 -->
          <div class="neu-card-footer">
            <div class="neu-toggle-row">
              <span class="neu-toggle-label">已启用</span>
              <n-switch v-model:value="mod.active" size="small" />
            </div>
            <div class="neu-status-dot" :class="{ on: mod.active }" />
          </div>
        </div>
      </div>

      <!-- 列表布局 -->
      <div v-else class="list-container">
        <div v-for="mod in filtermodlist" :key="mod.id" class="neu-list-item"
          :class="{ active: mod.active }">
          <n-checkbox v-model:checked="mod.selected" size="small" />
          <div class="neu-list-icon" @click.stop="openIconModal(mod)">
            <img v-if="getModIconUrl(mod)" :src="getModIconUrl(mod)" alt="icon" class="neu-list-img"
              @error="onIconLoadError(mod)" />
            <n-icon v-else size="20" :depth="2"><DocumentOutline /></n-icon>
          </div>
          <div class="neu-list-info">
            <span class="neu-list-name">{{ mod.displayName }}</span>
            <span class="neu-list-desc">{{ formatType(mod.type) }} · {{ formatDate(mod.installDate) }}</span>
          </div>
          <div class="neu-list-actions">
            <n-switch v-model:value="mod.active" size="small" />
            <div class="neu-status-dot" :class="{ on: mod.active }" />
          </div>
        </div>
      </div>
    </main>

    <!-- ===== 底部控制台：新拟态凸起面板 ===== -->
    <footer class="control-deck">
      <div class="deck-left">
        <div class="neu-btn-group">
          <button class="neu-action-btn" @click="selectAll">
            <n-icon :component="CheckboxOutline" size="16" />
            <span>{{ isAllSelected ? '取消全选' : '全选' }}</span>
          </button>
          <button class="neu-action-btn" @click="handleBatchToggle(true)">
            <n-icon :component="CheckmarkOutline" size="16" />
            <span>启用</span>
          </button>
          <button class="neu-action-btn" @click="handleBatchToggle(false)">
            <n-icon :component="CloseOutline" size="16" />
            <span>禁用</span>
          </button>
        </div>
        <div class="neu-btn-group">
          <button class="neu-action-btn accent" @click="handleDeployMods">
            <n-icon :component="RocketOutline" size="16" />
            <span>部署</span>
          </button>
          <button class="neu-action-btn" @click="handleAddMod">
            <n-icon :component="AddOutline" size="16" />
            <span>添加</span>
          </button>
          <button class="neu-action-btn danger" @click="handleBatchDelete">
            <n-icon :component="TrashOutline" size="16" />
            <span>删除</span>
          </button>
        </div>
      </div>

      <div class="deck-right">
        <button class="neu-launch-btn" :disabled="isLaunching" @click="handleLaunchGame">
          <span class="launch-title">{{ isLaunching ? '正在启动...' : '启动游戏' }}</span>
          <span class="launch-sub">READY TO LAUNCH</span>
        </button>
      </div>
    </footer>

    <!-- ===== 图标预览模态框 ===== -->
    <n-modal v-model:show="showIconModal" preset="card"
      :title="currentMod ? currentMod.displayName : 'Mod 图标'"
      style="width: 400px" :mask-closable="false">
      <div class="icon-modal-content">
        <div class="neu-preview-area">
          <div v-if="loadingIcon" class="preview-loading">
            <n-spin size="medium" />
          </div>
          <img v-if="currentMod && currentIconPreviewUrl" :src="currentIconPreviewUrl"
            alt="预览图标" class="preview-img"
            @load="onIconLoadSuccess" @error="onIconPreviewError" v-show="!loadingIcon" />
          <n-icon v-else size="80" :depth="2">
            <DocumentOutline />
          </n-icon>
        </div>
        <div class="modal-actions">
          <n-button type="primary" @click="uploadIconForCurrentMod" :loading="uploadingIcon">
            <template #icon><n-icon :component="AddOutline" /></template>
            更换图标
          </n-button>
          <n-button v-if="currentMod && currentMod.iconPath"
            @click="clearCurrentModIcon" :loading="uploadingIcon">
            移除图标
          </n-button>
        </div>
      </div>
    </n-modal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { open, ask } from '@tauri-apps/plugin-dialog'
import {
  NButton,
  NCheckbox,
  NSwitch,
  NTag,
  NIcon,
  NModal,
  NSpin
} from 'naive-ui'
import NeumorphicSearchBox from './NeumorphicSearchBox.vue'
import { useNotify, useConfirm } from '@/composables/useNotification'
import {
  DocumentOutline,
  ListOutline,
  GridOutline,
  CloseOutline,
  CheckboxOutline,
  CheckmarkOutline,
  RocketOutline,
  AddOutline,
  TrashOutline,
  PencilOutline,
} from '@vicons/ionicons5'

// ================= 响应式数据 =================
const notify = useNotify()
const { confirm } = useConfirm()

const isGridLayout = ref(true)
const currentCategory = ref('all')
const navRef = ref(null)
const modlist = ref([])
const isLaunching = ref(false)
const searchQuery = ref('')
const loadingIcon = ref(false)      // 预览图片加载状态
const isDraggingOver = ref(false)   // 拖拽导入状态
let dragCounter = 0                 // 拖拽进出计数

// 图标预览模态框相关
const showIconModal = ref(false)      // 控制模态框显示
const currentMod = ref(null)          // 当前选中的 Mod
const uploadingIcon = ref(false)      // 上传/移除图标时的加载状态

// 鼠标拖拽逻辑
let isDragging = false
let startX = 0
let scrollLeft = 0

// ================= 常量 =================
const categories = [
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

// ================= 计算属性 =================
const filtermodlist = computed(() => {
  let result = modlist.value
  if (currentCategory.value !== 'all') {
    result = result.filter(mod => mod.type === currentCategory.value)
  }
  const query = searchQuery.value.trim().toLowerCase()
  if (query) {
    // 改为基于 displayName 搜索
    result = result.filter(mod => mod.displayName.toLowerCase().includes(query))
  }
  return result
})

const isAllSelected = computed(() => {
  return modlist.value.length > 0 && modlist.value.every(mod => mod.selected)
})

const currentIconPreviewUrl = computed(() => {
  if (!currentMod.value) return null
  return getModIconUrl(currentMod.value)
})

// ================= 方法 =================
const toggleLayout = () => {
  isGridLayout.value = !isGridLayout.value
}

const formatType = type => TYPE_MAP[type] || '未知类型'

// 横向滚轮
const handleWheel = e => {
  if (e.deltaY !== 0) {
    e.preventDefault()
    navRef.value.scrollLeft += e.deltaY
  }
}

// 拖拽滚动
const handleMouseDown = e => {
  isDragging = true
  navRef.value.classList.add('grabbing')
  startX = e.pageX - navRef.value.offsetLeft
  scrollLeft = navRef.value.scrollLeft
}

const handleMouseMove = e => {
  if (!isDragging) return
  e.preventDefault()
  const x = e.pageX - navRef.value.offsetLeft
  const walk = (x - startX) * 1.5
  navRef.value.scrollLeft = scrollLeft - walk
}

const handleMouseUp = () => {
  isDragging = false
  navRef.value.classList.remove('grabbing')
}

const selectCategory = type => {
  currentCategory.value = type
}

// 全选/取消全选
const selectAll = () => {
  const allSelected = modlist.value.every(mod => mod.selected)
  modlist.value.forEach(mod => {
    mod.selected = !allSelected
  })
}

// 批量启用/禁用
const handleBatchToggle = enable => {
  modlist.value
    .filter(mod => mod.selected)
    .forEach(mod => {
      mod.active = enable
    })
}

// 批量删除
const handleBatchDelete = async () => {
  const selectedMods = modlist.value.filter(mod => mod.selected)
  if (selectedMods.length === 0) {
    notify.warning('请先选择要删除的Mod')
    return
  }
  const confirmDelete = await ask(`确定要删除选中的 ${selectedMods.length} 个Mod吗？`, {
    title: '确认删除',
    kind: 'warning',
    okLabel: '删除',
    cancelLabel: '取消',
  })
  if (!confirmDelete) return
  try {
    for (const mod of selectedMods) {
      await invoke('delete_mod_file', { modName: mod.name })
    }
    await refreshModList()
    notify.success(`已成功删除 ${selectedMods.length} 个Mod`)
  } catch (error) {
    console.error('批量删除失败:', error)
    notify.error(`批量删除失败: ${error}`)
  }
}

// 导入文件列表
const importFiles = async (paths) => {
  let successCount = 0
  let failCount = 0

  for (const path of paths) {
    try {
      await invoke('copy_mod_file', { src: path })
      successCount++
    } catch (err) {
      console.error(`导入失败: ${path}`, err)
      failCount++
    }
  }

  await refreshModList()

  if (failCount === 0) {
    notify.success(successCount === 1 ? '已添加 1 个模组' : `已添加 ${successCount} 个模组`)
  } else {
    notify.warning(`已添加 ${successCount} 个模组，${failCount} 个导入失败`)
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

// 修改 refreshModList 以包含 iconPath
const refreshModList = async () => {
  try {
    const modStatuses = await invoke('get_mods_with_status')
    modlist.value = modStatuses.map((info, index) => ({
      id: index + 1,
      name: info.name,
      displayName: info.display_name,
      active: info.applied,
      type: info.category || 'unknown',
      selected: false,
      installDate: info.install_date,
      iconPath: info.icon_path || null,   // 新增
    }))
  } catch (error) {
    console.error('刷新mod列表失败:', error)
  }
}

// 删除单个 Mod
const handleDeleteMod = async id => {
  try {
    const modIndex = modlist.value.findIndex(mod => mod.id === id)
    if (modIndex === -1) return
    const mod = modlist.value[modIndex]
    const confirmDelete = await ask(`确定要删除 "${mod.displayName}" 吗？`, {
      title: '确认删除',
      kind: 'warning',
      okLabel: '删除',
      cancelLabel: '取消',
    })
    if (!confirmDelete) return
    await invoke('delete_mod_file', { modName: mod.name })
    modlist.value.splice(modIndex, 1)
    notify.success(`"${mod.displayName}" 已成功删除！`)
  } catch (error) {
    console.error('删除失败:', error)
    notify.error(`删除失败: ${error}`)
  }
}

// 部署 Mod
const handleDeployMods = async () => {
  try {
    const activeModNames = modlist.value.filter(mod => mod.active).map(mod => mod.name)
    if (activeModNames.length === 0) {
      const proceed = await confirm('当前未启用任何Mod，是否继续？', { kind: 'warning' })
      if (!proceed) return
    }
    await invoke('deploy_mods', { modNames: activeModNames })
    notify.success(`部署成功！已应用 ${activeModNames.length} 个项目。`)
    await refreshModList()
  } catch (error) {
    console.error('部署失败:', error)
    notify.error(`部署失败: ${error}`)
  }
}

// 启动游戏
const handleLaunchGame = async () => {
  if (isLaunching.value) return
  isLaunching.value = true
  try {
    let gamePath = await invoke('get_game_path')
    if (!gamePath) {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '请选择《坦克世界闪击战》安装目录 (包含 wotblitz.exe 的文件夹)',
      })
      if (selected) {
        gamePath = selected
        await invoke('set_game_path', { path: gamePath })
      } else {
        return
      }
    }
    await invoke('launch_game')
  } catch (error) {
    notify.error(`操作失败: ${error}`)
  } finally {
    isLaunching.value = false
  }
}

// 重命名处理函数（带乐观更新）
const handleEditMod = async (mod) => {
  const newName = prompt('请输入新的显示名称', mod.displayName)
  if (!newName || newName === mod.displayName) return

  // 保存旧名称以便回滚
  const oldDisplayName = mod.displayName

  // 立即更新本地显示（乐观更新）
  mod.displayName = newName

  try {
    // 调用后端重命名
    await invoke('rename_mod', {
      originalFilename: mod.name,
      newDisplayName: newName
    })
    // 后端成功后，可选重新从后端获取全量数据以保证一致性
    await refreshModList()
  } catch (err) {
    // 后端失败，回滚显示名称
    mod.displayName = oldDisplayName
    alert(`重命名失败: ${err}`)
  }
}

//日期格式化函数
const formatDate = (dateStr) => {
  if (!dateStr) return '未安装'
  const date = new Date(dateStr)
  return date.toLocaleDateString()
}

//类别修改函数
const handleEditCategory = async (mod) => {
  const newCategory = prompt(
    `请输入类别代码 (model/voice/ui/lightIcon/script/map) 或留空表示未知`,
    mod.type === 'unknown' ? '' : mod.type
  )
  if (newCategory === null) return

  const category = newCategory.trim() === '' ? null : newCategory.trim()
  if (category === mod.type) return

  const oldCategory = mod.type
  // 乐观更新
  mod.type = category || 'unknown'
  try {
    await invoke('update_mod_category', {
      originalFilename: mod.name,
      category: category
    })
    await refreshModList() // 可选，确保与后端一致
  } catch (err) {
    mod.type = oldCategory
    alert(`修改类别失败: ${err}`)
  }
}


// 获取 Mod 图标的完整访问 URL
const getModIconUrl = (mod) => {
  if (!mod.iconPath) return null
  try {
    const url = convertFileSrc(mod.iconPath)
    return url
  } catch (err) {
    console.error('转换图标路径失败:', err)
    return null
  }
}

// 图标加载失败时清除路径
const onIconLoadError = (mod) => {
  console.warn('卡片图标加载失败，路径:', mod.iconPath)
  // 只记录错误，不清除路径，避免丢失用户上传的图标路径
  // mod.iconPath = null
}

// 打开图标预览模态框
const openIconModal = (mod) => {
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

    await invoke('set_mod_icon', {
      modName: currentMod.value.name,
      imagePath: selected
    })

    // 刷新列表获取最新数据
    await refreshModList()

    // 重新获取当前 Mod 的最新引用
    const updatedMod = modlist.value.find(m => m.name === currentMod.value.name)
    if (updatedMod) {
      currentMod.value = updatedMod
    }

    // 设置加载状态，让 @load/@error 事件负责结束
    loadingIcon.value = true

    notify.success('图标已更新')
  } catch (err) {
    console.error('上传图标失败:', err)
    notify.error(`上传图标失败: ${err}`)
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
    await invoke('clear_mod_icon', { modName: currentMod.value.name })

    // 刷新列表获取最新数据
    await refreshModList()

    // 重新获取当前 Mod 的最新引用
    const updatedMod = modlist.value.find(m => m.name === currentMod.value.name)
    if (updatedMod) {
      currentMod.value = updatedMod
    }

    // 无图标，直接显示默认图标
    loadingIcon.value = false

    notify.success('图标已移除')
  } catch (err) {
    console.error('移除图标失败:', err)
    notify.error(`移除图标失败: ${err}`)
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
  // 可选：标记加载失败，以便模板显示默认图标
  // 目前模板中 v-else-if 仍为 true，可能会显示破碎图标，因此可增加一个标志
  console.warn('预览图片加载失败，路径:', currentMod.value?.iconPath)
}

let unlistenDragDrop = null

onMounted(async () => {
  await refreshModList()

  // 注册 Tauri 拖拽导入事件
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
})

onUnmounted(() => {
  if (unlistenDragDrop) unlistenDragDrop()
})
</script>

<style scoped>
/* ===========================
   新拟态 (Neumorphism) 暗黑模式 3.0
   =========================== */

/* ---- 主容器 ---- */
.mod-library {
  height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
}

/* ---- 顶部栏：凸起面板 ---- */
.top-deck {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  height: 80px;
  background: var(--neu-raised);
  box-shadow:
    -8px -4px 16px var(--neu-shadow-light),
    8px 4px 16px var(--neu-shadow-dark),
    0 2px 8px rgba(0, 0, 0, 0.2);
  position: sticky;
  top: 0;
  z-index: 10;
  flex-shrink: 0;
  border-radius: 0 0 var(--neu-radius) var(--neu-radius);
  margin: 0 4px;
}

/* ---- 搜索框容器 ---- */
.search-container {
  position: relative;
  height: 44px;
  display: flex;
  align-items: center;
  width: 220px;
  flex-shrink: 0;
}

/* ---- 分类导航 ---- */
.category-wrapper {
  flex: 1;
  min-width: 0;
  height: 44px;
  display: flex;
  align-items: center;
  margin: 0 16px;
  mask-image: linear-gradient(to right, transparent 0%, black 8%, black 92%, transparent 100%);
  -webkit-mask-image: linear-gradient(to right, transparent 0%, black 8%, black 92%, transparent 100%);
}

.category-nav {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  scrollbar-width: none;
  cursor: grab;
  align-items: center;
  height: 100%;
  padding: 4px 0;
}

.category-nav::-webkit-scrollbar { display: none; }
.category-nav.grabbing { cursor: grabbing; }

/* 新拟态分类按钮 */
.neu-pill {
  flex-shrink: 0;
  height: 36px;
  padding: 0 20px;
  border: none;
  border-radius: 18px;
  background: var(--neu-raised);
  color: var(--text-dim);
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
  letter-spacing: 0.3px;
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
}

.neu-pill:hover {
  color: var(--text-main);
  box-shadow:
    -5px -5px 10px var(--neu-shadow-light),
    5px 5px 10px var(--neu-shadow-dark);
  transform: translateY(-1px);
}

.neu-pill:active {
  transform: translateY(0);
}

.neu-pill.active {
  color: var(--accent);
  background: var(--neu-inset);
  box-shadow:
    inset 2px 2px 6px var(--neu-shadow-dark),
    inset -2px -2px 6px var(--neu-shadow-light);
}

/* ---- 布局切换按钮 ---- */
.right-group {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
}

.neu-icon-btn {
  width: 44px;
  height: 44px;
  border: none;
  border-radius: var(--neu-radius-sm);
  background: var(--neu-raised);
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.neu-icon-btn:hover {
  color: var(--accent);
  box-shadow:
    -5px -5px 10px var(--neu-shadow-light),
    5px 5px 10px var(--neu-shadow-dark);
  transform: translateY(-1px);
}

.neu-icon-btn:active {
  box-shadow:
    inset 2px 2px 6px var(--neu-shadow-dark),
    inset -2px -2px 6px var(--neu-shadow-light);
  color: var(--accent);
  transform: translateY(0);
}

/* ---- 卡片网格区域 ---- */
.modules-grid {
  flex: 1;
  overflow-y: auto;
  padding: 20px 24px;
  margin-bottom: 90px;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 20px;
}

/* ---- 新拟态卡片 ---- */
.neu-card {
  background: var(--neu-raised);
  border-radius: var(--neu-radius);
  box-shadow:
    -8px -8px 16px var(--neu-shadow-light),
    8px 8px 16px var(--neu-shadow-dark);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  padding: 0;
}

.neu-card:hover {
  box-shadow:
    -10px -10px 20px var(--neu-shadow-light),
    10px 10px 20px var(--neu-shadow-dark);
  transform: translateY(-2px);
}

.neu-card.active {
  box-shadow:
    -8px -8px 16px var(--neu-shadow-light),
    8px 8px 16px var(--neu-shadow-dark),
    0 0 0 1px var(--neu-shadow-accent);
}

/* 卡片顶栏 */
.neu-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px 4px;
}

.neu-card-actions {
  display: flex;
  gap: 4px;
}

.neu-icon-btn-xs {
  width: 26px;
  height: 26px;
  border: none;
  border-radius: 8px;
  background: var(--neu-raised);
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow:
    -2px -2px 4px var(--neu-shadow-light),
    2px 2px 4px var(--neu-shadow-dark);
  transition: all 0.2s ease;
}

.neu-icon-btn-xs:hover {
  color: var(--accent);
  box-shadow:
    -3px -3px 6px var(--neu-shadow-light),
    3px 3px 6px var(--neu-shadow-dark);
}

.neu-icon-btn-xs:active {
  box-shadow:
    inset 1px 1px 3px var(--neu-shadow-dark),
    inset -1px -1px 3px var(--neu-shadow-light);
}

/* 卡片主体 */
.neu-card-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 16px 8px;
  gap: 8px;
}

/* 图标容器 */
.neu-icon-box {
  width: 64px;
  height: 64px;
  border-radius: 18px;
  background: var(--neu-inset);
  box-shadow:
    inset 2px 2px 6px var(--neu-shadow-dark),
    inset -2px -2px 6px var(--neu-shadow-light);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.25s ease;
  overflow: hidden;
}

.neu-icon-box:hover {
  box-shadow:
    inset 2px 2px 8px var(--neu-shadow-dark),
    inset -2px -2px 8px var(--neu-shadow-light),
    0 0 0 2px var(--neu-shadow-accent);
}

.neu-icon-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 18px;
}

.neu-card-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  width: 100%;
}

.neu-mod-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-main);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.neu-tag {
  cursor: pointer;
  font-size: 11px;
}

.neu-card-meta {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--text-dim);
}

.neu-card-meta .dot {
  opacity: 0.4;
}

/* 卡片底栏 */
.neu-card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px 12px;
}

.neu-toggle-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.neu-toggle-label {
  font-size: 12px;
  color: var(--text-dim);
}

/* 状态指示点 */
.neu-status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--neu-inset);
  box-shadow:
    inset 1px 1px 3px var(--neu-shadow-dark),
    inset -1px -1px 3px var(--neu-shadow-light);
  transition: all 0.3s ease;
}

.neu-status-dot.on {
  background: var(--accent);
  box-shadow:
    0 0 8px var(--accent-glow),
    inset 1px 1px 3px rgba(255, 255, 255, 0.3),
    inset -1px -1px 3px rgba(0, 0, 0, 0.2);
}

/* ---- 列表布局 ---- */
.list-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.neu-list-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 20px;
  background: var(--neu-raised);
  border-radius: var(--neu-radius-sm);
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark);
  transition: all 0.25s ease;
}

.neu-list-item:hover {
  box-shadow:
    -6px -6px 12px var(--neu-shadow-light),
    6px 6px 12px var(--neu-shadow-dark);
  transform: translateX(2px);
}

.neu-list-item.active {
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark),
    0 0 0 1px var(--neu-shadow-accent);
}

.neu-list-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--neu-radius-sm);
  background: var(--neu-inset);
  box-shadow:
    inset 2px 2px 4px var(--neu-shadow-dark),
    inset -2px -2px 4px var(--neu-shadow-light);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  cursor: pointer;
  overflow: hidden;
}

.neu-list-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.neu-list-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.neu-list-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.neu-list-desc {
  font-size: 12px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.neu-list-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

/* ---- 底部控制台 ---- */
.control-deck {
  height: 90px;
  background: var(--neu-raised);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  position: fixed;
  bottom: 0;
  left: 90px;
  right: 0;
  z-index: 30;
  box-shadow:
    -4px -8px 16px var(--neu-shadow-light),
    4px -8px 16px var(--neu-shadow-dark);
  border-radius: var(--neu-radius) var(--neu-radius) 0 0;
  margin: 0 4px;
}

.deck-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.neu-btn-group {
  display: flex;
  gap: 8px;
}

/* 新拟态操作按钮 */
.neu-action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 32px;
  padding: 0 14px;
  border: none;
  border-radius: 16px;
  background: var(--neu-raised);
  color: var(--text-dim);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
  box-shadow:
    -3px -3px 6px var(--neu-shadow-light),
    3px 3px 6px var(--neu-shadow-dark);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.neu-action-btn:hover {
  color: var(--text-main);
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark);
  transform: translateY(-1px);
}

.neu-action-btn:active {
  box-shadow:
    inset 2px 2px 5px var(--neu-shadow-dark),
    inset -2px -2px 5px var(--neu-shadow-light);
  color: var(--text-dim);
  transform: translateY(0);
}

.neu-action-btn.accent {
  color: var(--accent);
}

.neu-action-btn.accent:hover {
  color: var(--accent);
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark),
    0 0 12px var(--neu-shadow-accent);
}

.neu-action-btn.danger {
  color: #f43f5e;
}

.neu-action-btn.danger:hover {
  color: #f43f5e;
}

/* 启动按钮 */
.neu-launch-btn {
  height: 60px;
  padding: 0 36px;
  border: none;
  border-radius: 30px;
  background: linear-gradient(135deg, var(--accent) 0%, #536dfe 100%);
  color: white;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  line-height: 1.2;
  gap: 1px;
  box-shadow:
    -6px -6px 12px var(--neu-shadow-light),
    6px 6px 12px var(--neu-shadow-dark),
    0 4px 16px rgba(61, 90, 254, 0.3);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
}

.neu-launch-btn::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 30px;
  background: linear-gradient(135deg, rgba(255,255,255,0.1) 0%, transparent 60%);
  pointer-events: none;
}

.neu-launch-btn:hover:not(:disabled) {
  transform: translateY(-2px) scale(1.02);
  box-shadow:
    -8px -8px 16px var(--neu-shadow-light),
    8px 8px 16px var(--neu-shadow-dark),
    0 8px 24px rgba(61, 90, 254, 0.4);
}

.neu-launch-btn:active:not(:disabled) {
  transform: translateY(0) scale(0.98);
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark),
    0 2px 8px rgba(61, 90, 254, 0.3);
}

.neu-launch-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.neu-launch-btn .launch-title {
  font-size: 16px;
  font-weight: 700;
}

.neu-launch-btn .launch-sub {
  font-size: 9px;
  opacity: 0.8;
  letter-spacing: 0.5px;
}

/* ---- 图标预览模态框 ---- */
.icon-modal-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 16px 0;
}

.neu-preview-area {
  position: relative;
  width: 200px;
  height: 200px;
  border-radius: var(--neu-radius);
  background: var(--neu-inset);
  box-shadow:
    inset 4px 4px 12px var(--neu-shadow-dark),
    inset -4px -4px 12px var(--neu-shadow-light);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.preview-loading {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  border-radius: var(--neu-radius);
  z-index: 1;
}

.preview-img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: 8px;
  transition: opacity 0.2s;
}

.modal-actions {
  display: flex;
  gap: 12px;
}

/* ---- 拖拽导入覆盖层 ---- */
.drop-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(4px);
}

.drop-zone {
  width: 320px;
  padding: 48px 32px;
  border-radius: var(--neu-radius-lg);
  background: var(--neu-raised);
  box-shadow:
    -8px -8px 16px var(--neu-shadow-light),
    8px 8px 16px var(--neu-shadow-dark),
    0 0 0 2px var(--accent);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  color: var(--accent);
  font-size: 15px;
  font-weight: 600;
}
</style>
