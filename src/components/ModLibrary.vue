<template>
  <!-- 顶部信息栏 -->
  <header class="top-deck">
    <!-- 左侧：搜索 -->
    <div class="search-container">
      <div class="search-module">
        <span class="search-icon"><img src="@/assets/vue.svg" /></span>
        <input type="text" />
      </div>
    </div>

    <!-- 中间：响应式横向滚动导航 -->
    <div class="category-wrapper">
      <nav
        class="category-nav"
        ref="navRef"
        @wheel="handleWheel"
        @mousedown="handleMouseDown"
        @mousemove="handleMouseMove"
        @mouseup="handleMouseUp"
        @mouseleave="handleMouseUp"
      >
        <button
          v-for="cat in categories"
          :key="cat.type"
          class="nav-item"
          :class="{ active: currentCategory === cat.type }"
          @click="selectCategory(cat.type)"
        >
          {{ cat.name }}
        </button>
      </nav>
    </div>

    <!-- 右侧：原有工具组 -->
    <div class="right-group">
      <!-- 布局切换按钮 -->
      <button class="layout-toggle" @click="toggleLayout" :title="isGridLayout ? '切换为列表布局' : '切换为网格布局'">
        <span class="layout-icon">{{ isGridLayout ? '☰' : '□' }}</span>
      </button>

      <div class="stats-module">
        <div class="stat-item">
          <span class="stat-num">{{ totalmods }}</span>
          <span class="stat-label">已安装</span>
        </div>
        <div class="stat-item active-stat">
          <span class="stat-num">{{ activemods }}</span>
          <span class="stat-label">运行中</span>
        </div>
      </div>
    </div>
  </header>

  <!-- 中间列表：卡片式流 -->
  <section class="modules-grid" :class="{ 'grid-layout': isGridLayout, 'list-layout': !isGridLayout }">
    <div class="mod-card" v-for="mods in filtermodlist" :key="mods.id" :class="{ 'active-card': mods.active }">
      <div class="status-indicator" :class="{ inactive: mods.active }"></div>
      <div class="card-content">
        <div class="mod-header">
          <span class="mod-title">通用前置库 Lib_{{ mods.modName }}</span>
          <span class="tag">{{ formatType(mods.type) }}</span>
        </div>
        <div class="mod-desc">必要的前置依赖文件</div>
      </div>
      <div class="card-action">
        <label class="switch">
          <input type="checkbox" v-model="mods.active" />
          <span class="slider"></span>
        </label>
      </div>
    </div>
  </section>

  <!-- 底部控制台 -->
  <footer class="control-deck">
    <div class="deck-left">
      <button class="deck-btn danger">卸载选中</button>
      <button class="deck-btn">加载mod</button>
    </div>

    <div class="deck-right">
      <button class="launch-btn" @click="handleLaunchGame" :disabled="isLaunching">
        <div class="launch-content">
          <span class="launch-title">{{ isLaunching ? '正在启动...' : '启动游戏' }}</span>
          <span class="launch-sub">READY TO LAUNCH</span>
        </div>
        <div class="launch-effect"></div>
      </button>
    </div>
  </footer>
</template>

<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog' // 引入选择框插件

// ================= 响应式数据 =================
const isGridLayout = ref(true)
const currentCategory = ref('all')
const navRef = ref(null)
const modlist = ref([
  {
    id: 1,
    active: true,
    modName: 'a1',
    type: 'model',
  },
  {
    id: 2,
    active: false,
    modName: 'a2',
    type: 'voice',
  },
  {
    id: 3,
    active: false,
    modName: 'a3',
    type: 'ui',
  },
  {
    id: 4,
    active: false,
    modName: 'a4',
    type: 'ui',
  },
])
// 游戏启动状态
const isLaunching = ref(false)

//鼠标拖拽逻辑
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
  { type: 'script', name: '扩展脚本' }, // 增加几个测试滚动
  { type: 'map', name: '地图纹理' },
]

// ================= 方法 =================
const toggleLayout = () => {
  isGridLayout.value = !isGridLayout.value
}
const filtermodlist = computed(() => {
  if (currentCategory.value === 'all') {
    return modlist.value
  } else {
    return modlist.value.filter(mods => mods.type === currentCategory.value)
  }
})
const totalmods = computed(() => {
  return modlist.value.length
})
const activemods = computed(() => {
  return modlist.value.filter(mods => mods.active).length
})
const TYPE_MAP = {
  all: '全部',
  model: '3d模型',
  voice: '语音包',
  ui: 'UI',
  lightIcon: '点亮图标',
}
const formatType = type => {
  return TYPE_MAP[type] || '未知类型'
}
//滚轮重定向 (纵向转横向)
const handleWheel = e => {
  if (e.deltaY !== 0) {
    e.preventDefault()
    navRef.value.scrollLeft += e.deltaY
  }
}

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
  const walk = (x - startX) * 1.5 // 1.5是滚动速度
  navRef.value.scrollLeft = scrollLeft - walk
}

const handleMouseUp = () => {
  isDragging = false
  navRef.value.classList.remove('grabbing')
}

const selectCategory = type => {
  currentCategory.value = type
}
const handleLaunchGame = async () => {
  if (isLaunching.value) return
  isLaunching.value = true

  try {
    // 1. 判断是否获取了游戏启动路径
    let gamePath = await invoke('get_game_path')

    // 2. 如果路径为空，弹窗让用户指定目录
    if (!gamePath) {
      const selected = await open({
        directory: true, // 只能选择文件夹
        multiple: false, // 不允许多选
        title: '请选择《坦克世界闪击战》安装目录 (包含 wotblitz.exe 的文件夹)',
      })

      if (selected) {
        // 用户选择了路径，保存到后端
        gamePath = selected
        await invoke('set_game_path', { path: gamePath })
      } else {
        // 用户取消了选择
        console.log('用户取消了路径选择')
        return
      }
    }

    // 3. 调用后端启动程序
    await invoke('launch_game')
    console.log('游戏指令已发送')
  } catch (error) {
    // 捕获 Rust 返回的 Result::Err
    alert(`操作失败: ${error}`)
  } finally {
    isLaunching.value = false
  }
}
</script>

<style scoped>
/* ================= 顶部栏 ================= */
.top-deck {
  height: 80px;
  padding: 0 30px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  background: var(--glass-effect);
  backdrop-filter: blur(25px) saturate(200%);
  -webkit-backdrop-filter: blur(25px) saturate(200%);
  border-bottom: 1px solid var(--border);
  animation: slide-up-fade 0.5s var(--animation-timing) 0.1s backwards;
  z-index: 20;
  position: sticky;
  top: 0;
  transition: all var(--animation-duration) var(--animation-timing);
  box-shadow: 
    0 2px 12px rgba(0, 0, 0, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.top-deck:hover {
  background: var(--glass-effect-hover);
  box-shadow: 
    0 4px 24px rgba(0, 0, 0, 0.12),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

/* 搜索容器 - 修复暗色模式割裂问题 */
.search-container {
  flex: 0 1 180px;
  min-width: 140px;
  display: flex;
  align-items: center;
  position: relative;
  animation: fade-scale-in 0.4s var(--animation-timing) 0.15s backwards;
}

.search-module {
  position: relative;
  width: 100%;
  display: block;
  isolation: isolate;
}

.search-icon {
  position: absolute;
  left: 16px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-dim);
  font-size: 18px;
  z-index: 2;
  transition: all var(--animation-duration);
  pointer-events: none;
}

.search-icon img {
  width: 18px;
  height: 18px;
  filter: brightness(var(--icon-brightness, 0.8));
  transition: 
    filter var(--animation-duration),
    transform var(--animation-duration);
}

.search-module input {
  width: 100%;
  height: 46px;
  background: var(--bg-input);
  backdrop-filter: blur(12px) saturate(180%);
  -webkit-backdrop-filter: blur(12px) saturate(180%);
  border-radius: 12px;
  padding: 0 48px 0 48px;
  color: var(--text-main);
  font-family: inherit;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.35s var(--animation-timing);
  box-shadow: 
    0 2px 8px rgba(0, 0, 0, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  border: 1.5px solid var(--border);
  outline: none;
}

/* 修复暗色模式搜索框背景 */
:global(.dark-mode) .search-module input {
  background: var(--bg-input);
  border-color: rgba(255, 255, 255, 0.1);
}

.search-module input:hover {
  background: var(--bg-input-focus);
  border-color: var(--border);
  box-shadow: 
    0 4px 16px rgba(0, 0, 0, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
  transform: translateY(-1px);
}

:global(.dark-mode) .search-module input:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
}

.search-module input:focus {
  background: var(--bg-input-focus);
  border-color: var(--accent);
  box-shadow: 
    0 0 0 3px var(--accent-glow),
    0 8px 24px rgba(0, 0, 0, 0.12),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  transform: translateY(-1px) scale(1.02);
}

:global(.dark-mode) .search-module input:focus {
  background: rgba(255, 255, 255, 0.12);
  border-color: var(--accent);
}

.search-module input:focus ~ .search-icon {
  color: var(--accent);
}

.search-module input:focus ~ .search-icon img {
  filter: brightness(1.2);
  transform: scale(1.1);
}

.search-module input::placeholder {
  color: var(--text-dim);
  opacity: 0.7;
  font-weight: 400;
}

.search-module input:focus::placeholder {
  opacity: 0.5;
}

/* 分类导航包装器 */
.category-wrapper {
  flex: 1;
  min-width: 0;
  position: relative;
  mask-image: linear-gradient(
    to right,
    transparent 0%,
    black 15%,
    black 85%,
    transparent 100%
  );
  -webkit-mask-image: linear-gradient(
    to right,
    transparent 0%,
    black 15%,
    black 85%,
    transparent 100%
  );
  animation: slide-up-fade 0.5s var(--animation-timing) 0.2s backwards;
}

.category-nav {
  display: flex;
  gap: 6px;
  overflow-x: auto;
  overflow-y: hidden;
  padding: 12px 30px;
  scrollbar-width: none;
  cursor: grab;
  scroll-behavior: smooth;
  user-select: none;
  position: relative;
  align-items: center;
}

.category-nav::-webkit-scrollbar {
  display: none;
}

.category-nav.grabbing {
  cursor: grabbing;
  scroll-behavior: auto;
}

/* 导航项 - 优化暗色模式文字可读性 */
.nav-item {
  flex-shrink: 0;
  background: var(--glass-effect);
  border: 1.5px solid transparent;
  color: var(--text-dim);
  padding: 10px 24px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  border-radius: 10px;
  transition: all 0.35s cubic-bezier(0.34, 1.56, 0.64, 1);
  white-space: nowrap;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  isolation: isolate;
}

/* 提高暗色模式未选中状态文字对比度 */
:global(.dark-mode) .nav-item {
  color: rgba(255, 255, 255, 0.85);
}

.nav-item::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(
    135deg,
    rgba(var(--accent-rgb, 61, 90, 254), 0.1) 0%,
    rgba(var(--accent-rgb, 61, 90, 254), 0.05) 50%,
    transparent 100%
  );
  opacity: 0;
  transition: opacity var(--animation-duration);
  z-index: -1;
}

.nav-item::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%) scaleX(0);
  width: 40%;
  height: 2px;
  background: var(--accent);
  border-radius: 1px;
  transition: transform 0.4s var(--animation-timing);
}

.nav-item:hover {
  background: var(--glass-effect-hover);
  color: var(--text-main);
  transform: translateY(-2px);
  border-color: rgba(255, 255, 255, 0.1);
  box-shadow: 
    0 4px 16px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

:global(.dark-mode) .nav-item:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
}

.nav-item:hover::before {
  opacity: 1;
}

.nav-item.active {
  background: var(--glass-effect-focus);
  color: var(--accent);
  border-color: var(--accent);
  box-shadow: 
    0 4px 20px var(--accent-glow),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  transform: translateY(-2px);
}

.nav-item.active::before {
  opacity: 1;
  background: linear-gradient(
    135deg,
    rgba(var(--accent-rgb, 61, 90, 254), 0.15) 0%,
    rgba(var(--accent-rgb, 61, 90, 254), 0.08) 100%
  );
}

.nav-item.active::after {
  transform: translateX(-50%) scaleX(1);
}

.nav-item:active {
  transform: translateY(0) scale(0.98);
}

/* 右侧工具组 */
.right-group {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-shrink: 0;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.25s backwards;
}

/* 布局切换按钮 */
.layout-toggle {
  width: 46px;
  height: 46px;
  background: var(--glass-effect);
  backdrop-filter: blur(12px) saturate(180%);
  -webkit-backdrop-filter: blur(12px) saturate(180%);
  border-radius: 12px;
  color: var(--text-main);
  font-family: inherit;
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.35s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 
    0 2px 8px rgba(0, 0, 0, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  border: 1.5px solid transparent;
  position: relative;
  overflow: hidden;
}

:global(.dark-mode) .layout-toggle {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.1);
}

.layout-toggle::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(
    135deg,
    rgba(var(--accent-rgb, 61, 90, 254), 0.1) 0%,
    transparent 100%
  );
  opacity: 0;
  transition: opacity var(--animation-duration);
}

.layout-toggle:hover {
  background: var(--glass-effect-hover);
  border-color: var(--border);
  color: var(--accent);
  transform: translateY(-2px) rotate(5deg);
  box-shadow: 
    0 6px 20px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

:global(.dark-mode) .layout-toggle:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.15);
}

.layout-toggle:hover::before {
  opacity: 1;
}

.layout-toggle:active {
  transform: translateY(0) scale(0.95);
}

.layout-icon {
  font-size: 20px;
  font-weight: 600;
  transition: transform 0.3s ease;
}

.layout-toggle:hover .layout-icon {
  transform: rotate(15deg);
}

/* 统计模块 - 修复黑色边框突兀问题 */
.stats-module {
  height: 46px;
  display: flex;
  align-items: center;
  background: var(--glass-effect);
  backdrop-filter: blur(12px) saturate(180%);
  -webkit-backdrop-filter: blur(12px) saturate(180%);
  border-radius: 12px;
  padding: 0 18px;
  gap: 16px;
  box-shadow: 
    0 2px 8px rgba(0, 0, 0, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  transition: all 0.35s var(--animation-timing);
  isolation: isolate;
  /* 修复边框突兀问题 */
  border: 1.5px solid transparent;
}

:global(.dark-mode) .stats-module {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.08);
}

.stats-module:hover {
  border-color: var(--accent);
  background: var(--glass-effect-hover);
  box-shadow: 
    0 6px 20px var(--accent-glow),
    inset 0 1px 0 rgba(255, 255, 255, 0.08);
  transform: translateY(-1px);
}

:global(.dark-mode) .stats-module:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: var(--accent);
}

.stats-module::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(
    135deg,
    rgba(var(--accent-rgb, 61, 90, 254), 0.05) 0%,
    transparent 100%
  );
  opacity: 0;
  transition: opacity var(--animation-duration);
  z-index: -1;
}

.stats-module:hover::before {
  opacity: 1;
}

.stat-item {
  display: flex;
  align-items: baseline;
  gap: 8px;
  position: relative;
  animation: fade-scale-in 0.4s var(--animation-timing) backwards;
  padding: 4px 0;
}

.stat-item:nth-child(1) {
  animation-delay: 0.3s;
}
.stat-item:nth-child(2) {
  animation-delay: 0.35s;
}

.stat-item:not(:last-child)::after {
  content: '';
  width: 1px;
  height: 20px;
  background: rgba(255, 255, 255, 0.15);
  margin-left: 12px;
  align-self: center;
  opacity: 0.5;
}

:global(.light-mode) .stat-item:not(:last-child)::after {
  background: rgba(0, 0, 0, 0.15);
}

.stat-num {
  font-size: 22px;
  font-weight: 800;
  font-family: 'JetBrains Mono', monospace, 'Rajdhani', sans-serif;
  line-height: 1;
  color: var(--text-main);
  transition: 
    color var(--animation-duration),
    text-shadow var(--animation-duration);
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.stat-label {
  font-size: 11px;
  color: var(--text-dim);
  white-space: nowrap;
  letter-spacing: 0.5px;
  font-weight: 600;
  text-transform: uppercase;
  transition: color var(--animation-duration);
}

.active-stat .stat-num {
  color: var(--accent);
  text-shadow: 
    0 2px 8px var(--accent-glow),
    0 0 20px rgba(var(--accent-rgb, 61, 90, 254), 0.3);
}

.active-stat .stat-label {
  color: var(--text-accent);
  font-weight: 700;
}

/* 响应式设计 - 顶栏部分 */
@media (max-width: 1200px) {
  .top-deck {
    padding: 0 20px;
    gap: 15px;
  }
  
  .search-container {
    flex: 0 1 160px;
  }
  
  .category-nav {
    padding: 10px 20px;
  }
  
  .nav-item {
    padding: 8px 20px;
    font-size: 12px;
  }
  
  .layout-toggle {
    width: 42px;
    height: 42px;
  }
  
  .stats-module {
    padding: 0 15px;
    gap: 12px;
  }
  
  .stat-num {
    font-size: 20px;
  }
}

@media (max-width: 768px) {
  .top-deck {
    flex-wrap: wrap;
    height: auto;
    padding: 15px;
    gap: 12px;
  }
  
  .search-container {
    order: 1;
    flex: 1 0 100%;
    margin-bottom: 12px;
  }
  
  .category-wrapper {
    order: 2;
    flex: 1;
    min-width: 0;
  }
  
  .right-group {
    order: 3;
    flex: 0 0 auto;
    gap: 12px;
  }
  
  .layout-toggle {
    width: 40px;
    height: 40px;
  }
  
  .stats-module {
    height: 40px;
    padding: 0 12px;
  }
  
  .stat-label {
    display: none;
  }
  
  .stat-item:not(:last-child)::after {
    height: 16px;
  }
}

@media (max-width: 480px) {
  .category-nav {
    padding: 8px 15px;
    gap: 4px;
  }
  
  .nav-item {
    padding: 6px 16px;
    font-size: 11px;
    border-radius: 8px;
  }
  
  .search-module input {
    height: 42px;
    font-size: 13px;
  }
  
  .right-group {
    gap: 10px;
  }
}

/* 明暗模式特定的调整 */
:global(.dark-mode) {
  --icon-brightness: 0.85;
  
  /* 搜索框暗模式优化 */
  .search-module input {
    background: var(--bg-input);
  }
  
  .search-module input:hover {
    background: rgba(255, 255, 255, 0.08);
  }
  
  .search-module input:focus {
    background: rgba(255, 255, 255, 0.12);
  }
  
  /* 导航项暗模式优化 - 提高文字对比度 */
  .nav-item {
    color: rgba(255, 255, 255, 0.85);
  }
  
  .nav-item:hover {
    background: rgba(255, 255, 255, 0.08);
  }
  
  /* 统计模块暗模式优化 */
  .stats-module {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.08);
  }
  
  .stats-module:hover {
    background: rgba(255, 255, 255, 0.08);
  }
}

:global(.light-mode) {
  --icon-brightness: 1;
  
  /* 搜索框亮模式优化 */
  .search-module input {
    background: var(--bg-input);
  }
  
  .search-module input:hover {
    background: var(--bg-input-focus);
  }
  
  .search-module input:focus {
    background: var(--bg-input-focus);
  }
  
  /* 导航项亮模式优化 */
  .nav-item {
    background: var(--glass-effect);
  }
  
  .nav-item:hover {
    background: var(--glass-effect-hover);
  }
  
  /* 统计模块亮模式优化 */
  .stats-module {
    background: var(--glass-effect);
  }
  
  /* 亮模式下为选中状态添加轻微阴影 */
  .nav-item.active {
    box-shadow: 
      0 4px 20px rgba(61, 90, 254, 0.15),
      inset 0 1px 0 rgba(255, 255, 255, 0.5);
  }
}

/* 性能优化 */
.top-deck,
.search-module,
.category-nav,
.nav-item,
.layout-toggle,
.stats-module {
  will-change: transform, opacity, background-color;
  contain: layout style;
}

/* ================= Mod 卡片区域 ================= */
.modules-grid {
  flex: 1;
  padding: 20px 40px 0;
  overflow-y: auto;
  grid-auto-rows: max-content;
  gap: 15px;
  padding-bottom: 110px;
  animation: fade-scale-in 0.5s var(--animation-timing) 0.1s backwards;
}

/* 网格布局 */
.modules-grid.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
}

/* 列表布局 */
.modules-grid.list-layout {
  display: grid;
  grid-template-columns: 1fr;
}

/* 自定义滚动条 */
.modules-grid::-webkit-scrollbar {
  width: 6px;
}

.modules-grid::-webkit-scrollbar-track {
  background: transparent;
}

.modules-grid::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 3px;
  transition: background var(--animation-duration);
}

.modules-grid::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}

/* Mod 卡片 */
.mod-card {
  border-radius: 12px;
  padding: 18px;
  display: flex;
  align-items: center;
  position: relative;
  transition: all 0.3s var(--animation-timing);
  overflow: hidden;
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  background: var(--bg-card);
  border: 1px solid var(--border);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  animation: card-enter 0.5s var(--animation-timing) backwards;
}

@keyframes card-enter {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.mod-card:hover {
  transform: translateY(-4px);
  border-color: var(--accent);
  background: var(--bg-card-hover);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.mod-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0;
  transition: opacity 0.3s;
}

.mod-card:hover::before {
  opacity: 0.5;
}

/* 活动卡片 */
.mod-card.active-card {
  border-left: 4px solid var(--accent);
  background: rgba(var(--accent-rgb, 61, 90, 254), 0.05);
}

.mod-card.active-card:hover {
  box-shadow: 0 8px 32px var(--accent-glow);
}

/* 状态指示器 */
.status-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--text-dim);
  margin-right: 15px;
  position: relative;
  flex-shrink: 0;
  transition: all 0.3s;
}

.status-indicator::after {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 50%;
  border: 2px solid transparent;
  transition: border-color 0.3s;
}

.status-indicator.inactive {
  background: var(--accent);
  box-shadow: 0 0 10px var(--accent);
}

.status-indicator.inactive::after {
  border-color: var(--accent);
  animation: pulse-ring 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse-ring {
  0%,
  100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.5;
  }
}

/* 卡片内容 */
.card-content {
  flex: 1;
  min-width: 0;
}

.list-layout .card-content {
  display: flex;
  align-items: center;
  gap: 20px;
}

.mod-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.mod-title {
  font-weight: 700;
  font-size: 16px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-main);
}

.tag {
  font-size: 10px;
  padding: 3px 8px;
  background: var(--border);
  border-radius: 4px;
  color: var(--text-dim);
  font-weight: 600;
  letter-spacing: 0.5px;
}

.mod-desc {
  font-size: 12px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.4;
}

.list-layout .mod-desc {
  white-space: normal;
  max-width: 600px;
}

/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 18px;
  flex-shrink: 0;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--switch-track);
  border: 1px solid var(--switch-border);
  transition: 0.3s var(--animation-timing);
  border-radius: 12px;
}

.slider:before {
  position: absolute;
  content: '';
  height: 14px;
  width: 14px;
  left: 2px;
  bottom: 1px;
  background-color: var(--switch-thumb);
  transition: 0.3s var(--animation-timing);
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

input:checked + .slider {
  background-color: var(--switch-track-checked);
  border-color: var(--switch-border-checked);
}

input:checked + .slider:before {
  transform: translateX(20px);
  background-color: var(--switch-thumb-checked);
}

.switch:hover .slider {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent-glow);
}

input:checked:hover + .slider {
  box-shadow: 0 0 12px var(--accent-glow);
}

/* ================= 底部控制台 ================= */
.control-deck {
  height: 90px;
  background: var(--deck-bg);
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 40px;
  position: absolute;
  bottom: 0;
  width: 100%;
  z-index: 50;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.15s backwards;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all var(--animation-duration);
}

.deck-left {
  display: flex;
  gap: 10px;
}

/* 按钮样式 - 优化明暗模式 */
.deck-btn {
  height: 40px;
  padding: 0 24px;
  background: var(--glass-effect);
  border: 1px solid var(--border);
  color: var(--text-main);
  font-family: inherit;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s var(--animation-timing);
  text-transform: uppercase;
  text-align: center;
  line-height: 40px;
  font-size: 12px;
  letter-spacing: 1px;
  border-radius: 6px;
  position: relative;
  overflow: hidden;
}

.deck-btn::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: var(--accent);
  transform: translate(-50%, -50%);
  transition:
    width 0.6s,
    height 0.6s;
  opacity: 0.1;
}

.deck-btn:hover {
  border-color: var(--accent);
  color: var(--text-accent);
  background: var(--glass-effect-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px var(--accent-glow);
}

.deck-btn:hover::before {
  width: 200%;
  height: 200%;
}

.deck-btn:active {
  transform: scale(0.96);
}

.deck-btn.danger {
  border-color: #dc2626;
  color: #ef4444;
}

.deck-btn.danger:hover {
  border-color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.2);
}

/* 启动按钮 - 优化明暗模式 */
.launch-btn {
  height: 60px;
  padding: 0 50px;
  background: linear-gradient(135deg, var(--accent) 0%, #536dfe 100%);
  border: none;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  clip-path: polygon(12px 0, 100% 0, 100% 100%, 0 100%, 0 12px);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 32px var(--accent-glow);
  transition: all 0.3s var(--animation-timing);
}

.launch-btn:hover {
  transform: scale(1.02);
  filter: brightness(1.15);
  box-shadow: 0 12px 40px var(--accent-glow);
}

.launch-btn:active {
  transform: scale(0.98);
}

.launch-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  filter: none;
}

.launch-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  z-index: 2;
  color: white;
}

.launch-title {
  font-size: 20px;
  font-weight: 800;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.launch-sub {
  font-size: 10px;
  opacity: 0.9;
  letter-spacing: 1px;
  font-weight: 500;
}

.launch-effect {
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  animation: shimmer 3s infinite;
}

/* ================= 响应式设计 ================= */
@media (max-width: 1200px) {
  .top-deck {
    padding: 0 20px;
  }

  .modules-grid {
    padding: 20px 20px 0;
  }

  .control-deck {
    padding: 0 20px;
  }
}

@media (max-width: 768px) {
  .top-deck {
    flex-wrap: wrap;
    height: auto;
    padding: 15px;
  }

  .search-container {
    order: 1;
    flex: 1 0 100%;
    margin-bottom: 15px;
  }

  .category-wrapper {
    order: 2;
    flex: 1;
  }

  .right-group {
    order: 3;
  }

  .modules-grid.grid-layout {
    grid-template-columns: 1fr;
  }

  .layout-toggle {
    padding: 0 12px;
  }

  .stat-label {
    display: none;
  }

  .launch-btn {
    padding: 0 30px;
  }

  .launch-title {
    font-size: 16px;
  }
}

@media (max-width: 480px) {
  .category-nav {
    padding: 10px 20px;
  }

  .nav-item {
    padding: 6px 16px;
    font-size: 12px;
  }

  .deck-btn {
    padding: 0 16px;
    font-size: 11px;
  }

  .launch-btn {
    padding: 0 20px;
    height: 50px;
  }
}

/* ================= 明暗模式特定的调整 ================= */
:global(.dark-mode) {
  --icon-brightness: 0.8;
}

:global(.light-mode) {
  --icon-brightness: 1;
}

:global(.light-mode) .search-module input {
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

:global(.light-mode) .search-module input:hover {
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.08);
}

:global(.light-mode) .search-module input:focus {
  box-shadow: 0 0 20px rgba(61, 90, 254, 0.15);
}

:global(.light-mode) .launch-btn {
  box-shadow: 0 8px 32px rgba(61, 90, 254, 0.3);
}

:global(.light-mode) .launch-btn:hover {
  box-shadow: 0 12px 40px rgba(61, 90, 254, 0.4);
}

:global(.light-mode) .deck-btn.danger:hover {
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.15);
}

/* ================= 性能优化 ================= */
.top-deck,
.category-nav,
.mod-card,
.deck-btn,
.launch-btn,
.layout-toggle {
  will-change: transform, box-shadow, border-color;
}
</style>
