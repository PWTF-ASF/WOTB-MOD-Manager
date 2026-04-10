<template>
  <div class="mod-library">
    <!-- 顶部栏（重构后） -->
    <header class="top-deck">
      <div class="search-container">
        <n-input v-model:value="searchQuery" size="tiny" clearable placeholder="搜索模组...">
          <template #suffix>
            <n-icon :component="SearchOutline" />
          </template>
        </n-input>
      </div>

      <div class="category-wrapper">
        <nav ref="navRef" class="category-nav" @wheel="handleWheel" @mousedown="handleMouseDown"
          @mousemove="handleMouseMove" @mouseup="handleMouseUp" @mouseleave="handleMouseUp">
          <n-button v-for="cat in categories" :key="cat.type"
            :type="currentCategory === cat.type ? 'primary' : 'default'" size="small" class="nav-item"
            @click="selectCategory(cat.type)">
            {{ cat.name }}
          </n-button>
        </nav>
      </div>

      <div class="right-group">
        <n-button class="layout-toggle" @click="toggleLayout" :title="isGridLayout ? '切换为列表布局' : '切换为网格布局'"
          size="large">
          <template #icon>
            <n-icon :component="isGridLayout ? ListOutline : GridOutline" />
          </template>
        </n-button>

        <!-- 统计模块：使用 n-card + n-space + n-divider，确保垂直居中 -->
        <n-card :bordered="true" size="small" class="stats-module" content-style="padding: 0;">
          <n-space align="center" :size="16" justify="center" style="height: 100%; flex-wrap: nowrap;">
            <n-statistic label="已添加" :value="totalmods" />
            <!-- <n-divider vertical style="height: 30px;" /> -->
            <n-statistic label="已安装" :value="activemods" class="installed-stat" />
          </n-space>
        </n-card>
      </div>
    </header>

    <!-- 中间卡片列表区域（保持不变） -->
    <main class="modules-grid" :class="{ 'grid-layout': isGridLayout, 'list-layout': !isGridLayout }">
      <!-- 网格布局 -->
      <div v-if="isGridLayout" class="grid-container">
        <n-card v-for="mod in filtermodlist" :key="mod.id" class="mod-card grid-card"
          :class="{ 'active-card': mod.active }" size="small" hoverable>
          <template #header>
            <n-checkbox v-model:checked="mod.selected" size="small" />
          </template>
          <template #header-extra>
            <n-button text class="edit-btn" @click.stop="handleEditMod(mod)">
              <n-icon size="14">
                <PencilOutline />
              </n-icon>
            </n-button>
            <n-button text class="win-close-btn" @click.stop="handleDeleteMod(mod.id)">
              <n-icon size="14">
                <CloseOutline />
              </n-icon>
            </n-button>
          </template>

          <div class="grid-card-content">
            <!-- 图标区域：可点击 -->
            <div class="mod-icon clickable-icon" @click.stop="openIconModal(mod)">
              <img v-if="getModIconUrl(mod)" :src="getModIconUrl(mod)" alt="icon" class="mod-icon-img"
                @error="onIconLoadError(mod)" />
              <n-icon v-else size="40" :depth="2">
                <DocumentOutline />
              </n-icon>
            </div>
            <div class="mod-header">
              <span class="mod-title">{{ mod.displayName }}</span>
              <n-tag size="small" :bordered="false" @click.stop="handleEditCategory(mod)">
                {{ formatType(mod.type) }}
              </n-tag>
            </div>
            <div class="mod-meta">
              <span>2.1 MB</span>
              <span class="mod-date">{{ formatDate(mod.installDate) }}</span>
            </div>
            <div class="mod-desc">必要的前置依赖文件</div>
          </div>
          <template #action>
            <div class="card-action">
              <n-switch v-model:value="mod.active" size="small" />
              <div class="status-indicator" :class="{ inactive: mod.active }" />
            </div>
          </template>
        </n-card>
      </div>

      <!-- 列表布局（类似改动）... -->
      <div v-else class="list-container">
        <n-card v-for="mod in filtermodlist" :key="mod.id" class="mod-card list-card"
          :class="{ 'active-card': mod.active }" size="small" hoverable>
          <!-- 头部省略... -->
          <div class="list-layout-content">
            <div class="mod-icon clickable-icon" @click.stop="openIconModal(mod)">
              <img v-if="getModIconUrl(mod)" :src="getModIconUrl(mod)" alt="icon" class="mod-icon-img-small"
                @error="onIconLoadError(mod)" />
              <n-icon v-else size="24" :depth="2">
                <DocumentOutline />
              </n-icon>
            </div>
            <div class="mod-info">
              <div class="mod-header">
                <span class="mod-title">{{ mod.displayName }}</span>
                <n-tag size="small" :bordered="false" @click.stop="handleEditCategory(mod)">
                  {{ formatType(mod.type) }}
                </n-tag>
              </div>
              <div class="mod-desc">必要的前置依赖文件 · 2.1 MB · {{ formatDate(mod.installDate) }}</div>
            </div>
            <div class="card-action">
              <n-switch v-model:value="mod.active" size="small" />
              <div class="status-indicator" :class="{ inactive: mod.active }" />
            </div>
          </div>
        </n-card>
      </div>
    </main>

    <!-- 底部控制台（保持不变） -->
    <footer class="control-deck">
      <div class="deck-left">
        <n-space vertical :size="12">
          <n-space :size="8">
            <n-button @click="selectAll" size="small" secondary>
              <template #icon><n-icon :component="CheckboxOutline" /></template>
              {{ isAllSelected ? '取消全选' : '全选' }}
            </n-button>
            <n-button type="success" @click="handleBatchToggle(true)" size="small" secondary>
              <template #icon><n-icon :component="CheckmarkOutline" /></template>
              启用
            </n-button>
            <n-button type="warning" @click="handleBatchToggle(false)" size="small" secondary>
              <template #icon><n-icon :component="CloseOutline" /></template>
              禁用
            </n-button>
          </n-space>
          <n-space :size="8">
            <n-button type="primary" @click="handleDeployMods" size="small" secondary>
              <template #icon><n-icon :component="RocketOutline" /></template>
              部署
            </n-button>
            <n-button type="info" @click="handleAddMod" size="small" secondary>
              <template #icon><n-icon :component="AddOutline" /></template>
              添加
            </n-button>
            <n-button type="error" @click="handleBatchDelete" size="small" secondary>
              <template #icon><n-icon :component="TrashOutline" /></template>
              删除
            </n-button>
          </n-space>
        </n-space>
      </div>

      <div class="deck-right">
        <n-button class="launch-btn" :loading="isLaunching" type="info" @click="handleLaunchGame" size="large"
          :disabled="isLaunching">
          <div class="launch-text">
            <span class="launch-title">{{ isLaunching ? '正在启动...' : '启动游戏' }}</span>
            <span class="launch-sub">READY TO LAUNCH</span>
          </div>
        </n-button>
      </div>
    </footer>

    <!-- 图标预览/自定义模态框 -->
    <!-- 图标预览/自定义模态框 -->
    <n-modal v-model:show="showIconModal" preset="card" :title="currentMod ? currentMod.displayName : 'Mod 图标'"
      style="width: 400px">
      <div class="icon-modal-content">
        <div class="preview-area" style="position: relative;">
          <!-- 加载指示器 -->
          <div v-if="loadingIcon" class="preview-loading">
            <n-spin size="medium" />
          </div>
          <!-- 图片元素：有图标 URL 时渲染，用 v-show 控制显示 -->
          <img v-if="currentMod && currentIconPreviewUrl" :src="currentIconPreviewUrl" alt="预览图标" class="preview-img"
            @load="onIconLoadSuccess" @error="onIconPreviewError" v-show="!loadingIcon" />
          <!-- 无图标时显示默认图标 -->
          <n-icon v-else size="80" :depth="2">
            <DocumentOutline />
          </n-icon>
        </div>
        <div class="modal-actions">
          <n-button type="primary" @click="uploadIconForCurrentMod" :loading="uploadingIcon">
            <template #icon><n-icon :component="AddOutline" /></template>
            更换图标
          </n-button>
          <n-button v-if="currentMod && currentMod.iconPath" @click="clearCurrentModIcon" :loading="uploadingIcon">
            移除图标
          </n-button>
        </div>
      </div>
    </n-modal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { convertFileSrc } from '@tauri-apps/api/core'
import { open, ask } from '@tauri-apps/plugin-dialog'
import {
  NInput,
  NButton,
  NCheckbox,
  NSwitch,
  NCard,
  NStatistic,
  NSpace,
  NTag,
  NIcon,
  NModal,
  NSpin
} from 'naive-ui'
import {
  DocumentOutline,
  SearchOutline,
  ListOutline,
  GridOutline,
  CloseOutline,
  CheckboxOutline,
  CheckmarkOutline,
  RocketOutline,
  AddOutline,
  TrashOutline,
  PencilOutline
} from '@vicons/ionicons5'

// ================= 响应式数据 =================
const isGridLayout = ref(true)
const currentCategory = ref('all')
const navRef = ref(null)
const modlist = ref([])
const isLaunching = ref(false)
const searchQuery = ref('')
const loadingIcon = ref(false)      // 预览图片加载状态
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

const totalmods = computed(() => modlist.value.length)

const activemods = computed(() => modlist.value.filter(m => m.active).length)

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
    window.alert('请先选择要删除的Mod')
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
    window.alert(`已成功删除 ${selectedMods.length} 个Mod`)
  } catch (error) {
    console.error('批量删除失败:', error)
    window.alert(`批量删除失败: ${error}`)
  }
}

// 添加 Mod
const handleAddMod = async () => {
  try {
    const selected = await open({
      title: '请选择mod文件',
      multiple: false,
      directory: false,
    })
    if (selected) {
      await invoke('copy_mod_file', { src: selected })
      await refreshModList()
      window.alert('添加成功')
    }
  } catch (error) {
    console.error('出错了:', error)
    window.alert(`添加Mod时出错: ${error}`)
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
    window.alert(`"${mod.modName}" 已成功删除！`)
  } catch (error) {
    console.error('删除失败:', error)
    window.alert(`删除失败: ${error}`)
  }
}

// 部署 Mod
const handleDeployMods = async () => {
  try {
    const activeModNames = modlist.value.filter(mod => mod.active).map(mod => mod.name)
    if (activeModNames.length === 0) {
      if (!window.confirm('当前未启用任何Mod，是否继续？')) return
    }
    await invoke('deploy_mods', { modNames: activeModNames })
    window.alert(`✅ 部署成功！已应用 ${activeModNames.length} 个项目。`)
    await refreshModList()
  } catch (error) {
    console.error('部署失败:', error)
    window.alert(`部署失败: ${error}`)
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
    window.alert(`操作失败: ${error}`)
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
    console.log('生成的 asset URL:', url)
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
  console.log('打开图标模态框，当前Mod:', mod.displayName, '图标URL:', iconUrl)
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
    console.log('开始上传图标，文件路径:', selected)

    const newIconPath = await invoke('set_mod_icon', {
      modName: currentMod.value.name,
      imagePath: selected
    })
    console.log('后端返回的图标路径:', newIconPath)

    // 强制刷新当前 Mod 图标路径
    currentMod.value.iconPath = newIconPath
    const targetMod = modlist.value.find(m => m.id === currentMod.value.id)
    if (targetMod) targetMod.iconPath = newIconPath

    // 强制刷新整个列表，确保所有数据一致（可选）
    await refreshModList()

    // 如果模态框还开着，手动重置加载状态并强制重绘图片
    loadingIcon.value = true
    // 延迟一点，让 Vue 响应式更新完成
    setTimeout(() => {
      loadingIcon.value = false
    }, 100)

    window.alert('图标已更新')
  } catch (err) {
    console.error('上传图标失败:', err)
    window.alert(`上传图标失败: ${err}`)
  } finally {
    // 上传成功后...
    await refreshModList()
    // 重新获取当前 Mod 的最新引用（refreshModList 后列表已更新）
    const updatedMod = modlist.value.find(m => m.name === currentMod.value.name)
    if (updatedMod) {
      currentMod.value = updatedMod
    }
    // 重置加载状态
    loadingIcon.value = true   // 触发图片重新加载
  }
}

// 清除图标
const clearCurrentModIcon = async () => {
  if (!currentMod.value) return
  try {
    uploadingIcon.value = true
    await invoke('clear_mod_icon', { modName: currentMod.value.name })
    currentMod.value.iconPath = null
    const targetMod = modlist.value.find(m => m.id === currentMod.value.id)
    if (targetMod) targetMod.iconPath = null
    window.alert('图标已移除')
  } catch (err) {
    console.error('移除图标失败:', err)
    window.alert(`移除图标失败: ${err}`)
  } finally {
    // 清除成功后...
    await refreshModList()
    const updatedMod = modlist.value.find(m => m.name === currentMod.value.name)
    if (updatedMod) {
      currentMod.value = updatedMod
    }
    loadingIcon.value = false  // 无图标，直接显示默认图标
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

onMounted(async () => {
  await refreshModList()
})
</script>

<style scoped>
/* 新增图标样式 */
.clickable-icon {
  cursor: pointer;
  transition: transform 0.1s ease;
}

.clickable-icon:hover {
  transform: scale(1.02);
}

.mod-icon-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 16px;
}

.mod-icon-img-small {
  width: 24px;
  height: 24px;
  object-fit: cover;
  border-radius: 4px;
}

.icon-modal-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 16px 0;
}

.preview-area {
  position: relative;
  width: 200px;
  height: 200px;
  background: var(--glass-effect);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border);
  overflow: hidden;
}

.preview-loading {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 16px;
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

.mod-library {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: transparent;
  position: relative;
}

/* 顶部栏（sticky） */
.top-deck {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  height: 80px;
  background: var(--glass-effect);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  z-index: 10;
  flex-shrink: 0;
}

.category-wrapper {
  flex: 1;
  min-width: 0;
  mask-image: linear-gradient(to right, transparent 0%, black 15%, black 85%, transparent 100%);
  -webkit-mask-image: linear-gradient(to right, transparent 0%, black 15%, black 85%, transparent 100%);
}

.category-nav {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  scrollbar-width: none;
  cursor: grab;
  padding: 12px 0;
  align-items: center;
}

.category-nav::-webkit-scrollbar {
  display: none;
}

.category-nav.grabbing {
  cursor: grabbing;
}

.nav-item {
  flex-shrink: 0;
}

.right-group {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
}

/* 统计模块样式 - 确保完美垂直居中 */
.stats-module {
  height: 60px;
  background: var(--glass-effect);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  overflow: hidden;
  padding: 5px;
}

.stats-module:hover {
  border-color: var(--accent);
  box-shadow: 0 4px 12px var(--accent-glow);
  background: var(--glass-effect-hover);
  transform: translateY(-1px);
}

/* 卡片内容区域填满高度并垂直居中 */
.stats-module .n-card__content {
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 !important;
}

.stats-module .n-space {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-wrap: nowrap;
  /* 防止换行 */
}

/* 统计项内部水平居中，并统一行高 */
.stats-module .n-statistic {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100%;
  line-height: 1.2;
  /* 统一行高 */
}

.stats-module .n-statistic .n-statistic-label,
.stats-module .n-statistic .n-statistic-value {
  line-height: 1.2;
  padding: 0;
}

.stats-module .n-statistic .n-statistic-label {
  font-size: 12px;
  margin-bottom: 2px;
  color: var(--text-dim);
}

.stats-module .n-statistic .n-statistic-value {
  font-size: 18px;
}

.stats-module :deep(.installed-stat .n-statistic-value) {
  --n-value-text-color: var(--accent);
  /* 增加优先级 */
}

/* 卡片区域 - 占据剩余高度，底部留出footer空间 */
.modules-grid {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  margin-bottom: 90px;
  /* 为fixed footer留出空间 */
}

/* 网格容器：纯 CSS Grid 响应式 */
.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

/* 网格卡片样式 - 增加信息密度 */
.grid-card .grid-card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 6px;
  padding: 12px 8px;
}

.grid-card .mod-icon {
  width: 64px;
  height: 64px;
  background: var(--glass-effect);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.grid-card .mod-meta {
  font-size: 11px;
  color: var(--text-dim);
  display: flex;
  gap: 8px;
}

.grid-card .mod-date {
  opacity: 0.8;
}

.grid-card .mod-desc {
  font-size: 11px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

/* 列表卡片样式 */
.list-card .list-layout-content {
  display: flex;
  align-items: center;
  gap: 16px;
  width: 100%;
}

.list-card .mod-icon {
  flex-shrink: 0;
}

.list-card .mod-info {
  flex: 1;
  min-width: 0;
}

.list-card .mod-desc {
  font-size: 12px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.list-card .card-action {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 通用卡片样式 - 优化标题与标签布局 */
.mod-card {
  transition: all 0.2s;
}

.mod-card.active-card {
  border-left: 4px solid var(--accent);
}

.mod-card :deep(.n-card-header) {
  padding: 8px 12px;
}

.mod-card :deep(.n-card__action) {
  padding: 8px 12px;
}

.mod-header {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.mod-title {
  font-weight: 600;
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.mod-header :deep(.n-tag) {
  flex-shrink: 0;
}

.card-action {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-dim);
}

.status-indicator.inactive {
  background: var(--accent);
  box-shadow: 0 0 8px var(--accent-glow);
}

/* 底部控制台（fixed，适配侧边栏宽度） */
.control-deck {
  height: 90px;
  background: var(--deck-bg);
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  position: fixed;
  bottom: 0;
  left: 90px;
  /* 与侧边栏宽度一致 */
  right: 0;
  z-index: 30;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.deck-left .n-space {
  flex-wrap: nowrap;
}

.launch-btn {
  height: 60px;
  padding: 0 40px;
  background: linear-gradient(135deg, var(--accent) 0%, #536dfe 100%);
  border: none;
  color: white;
  clip-path: polygon(12px 0, 100% 0, 100% 100%, 0 100%, 0 12px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.launch-text {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1.2;
  gap: 2px;
}

.launch-title {
  font-size: 18px;
  font-weight: 700;
}

.launch-sub {
  font-size: 10px;
  opacity: 0.8;
  letter-spacing: 0.5px;
}

/* 新增：启动游戏按钮交互效果 */
.launch-btn:not(:disabled):hover {
  filter: brightness(1.1);
  transform: scale(1.02);
  transition: all 0.2s ease;
}

.launch-btn:not(:disabled):active {
  transform: scale(0.98);
  filter: brightness(0.9);
  transition: all 0.1s ease;
}

.launch-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 适配明暗模式变量 */
:deep(.dark-mode) {
  --border: rgba(255, 255, 255, 0.12);
  --text-dim: rgba(255, 255, 255, 0.6);
  --glass-effect: rgba(255, 255, 255, 0.05);
  --glass-effect-hover: rgba(255, 255, 255, 0.08);
  --deck-bg: rgba(15, 17, 21, 0.95);
}

:deep(.light-mode) {
  --border: rgba(0, 0, 0, 0.12);
  --text-dim: rgba(0, 0, 0, 0.6);
  --glass-effect: rgba(255, 255, 255, 0.8);
  --glass-effect-hover: rgba(255, 255, 255, 0.9);
  --deck-bg: rgba(255, 255, 255, 0.95);
}
</style>