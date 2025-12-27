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

    <!-- 中间：新增分类导航 (嵌入式) -->
    <nav class="category-nav">
      <button
        v-for="cat in categories"
        :key="cat.type"
        class="nav-item"
        :class="{ active: currentCategory === cat.type }"
        @click="currentCategory = cat.type"
      >
        {{ cat.name }}
      </button>
      <!-- 滑动的光标背景 (可选高级效果) -->
      <div class="nav-glider"></div>
    </nav>

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
      <button class="launch-btn">
        <div class="launch-content">
          <span class="launch-title">启动游戏</span>
          <span class="launch-sub">READY TO LAUNCH</span>
        </div>
        <div class="launch-effect"></div>
      </button>
    </div>
  </footer>
</template>

<script setup>
import { ref, computed } from 'vue'

// ================= 响应式数据 =================
const isGridLayout = ref(true)
const currentCategory = ref('all')
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

// ================= 常量 =================
const categories = [
  { type: 'all', name: '全部' },
  { type: 'model', name: '3d改模' },
  { type: 'voice', name: '语音包' },
  { type: 'ui', name: 'UI' },
  { type: 'lightIcon', name: '点亮' },
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
</script>

<style scoped>
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  border: none;
  animation: fadeIn 0.3s ease-out;
}

/* --- 顶部栏 --- */
.top-deck {
  /* 保持原有高度 */
  height: 80px;
  padding: 0 40px; /* 原有 padding */
  display: flex;
  align-items: center;
  /* 关键：改为两端对齐，中间部分靠 margin 或 flex 撑开 */
  justify-content: space-between;
  gap: 20px;
}

/* 搜索框和布局切换按钮容器 */
.search-container {
  display: flex;
  align-items: center;
}

.search-module {
  position: relative;
  width: 300px;
  display: block;
}

.search-icon {
  position: absolute;
  left: 15px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-dim);
  font-size: 18px;
  margin-left: 5px;
  z-index: 2;
}

.search-icon img {
  width: 18px;
  height: 18px;
}

.search-module input {
  width: 100%;
  height: 44px;
  background: var(--def-col-fltr);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-radius: 25px;
  padding-left: 45px;
  color: #2c3e50;
  font-family: inherit;
  font-size: 16px;
  transition: all 0.3s;
  box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.search-module input:hover {
  background: var(--def-col-fltr-hover);
  backdrop-filter: blur(12px);
}

.search-module input:focus {
  border-radius: 5px;
  background: rgb(242, 243, 244);
  border-color: var(--accent);
  box-shadow: 0 0 12px var(--accent-glow);
}

.right-group {
  display: flex;
  align-items: center;
  gap: 15px;
}

/* 3. 中间分类导航样式 (核心) */
.category-nav {
  display: flex;
  background: var(--def-col-fltr);
  padding: 4px;
  border-radius: 8px; /* 稍微圆角 */
  border: 1px solid rgba(255, 255, 255, 0.05);
  position: relative;
  /* 玻璃拟态 */
  backdrop-filter: blur(12px);
  box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.nav-item {
  background: transparent;
  border: none;
  color: var(--text-dim);
  padding: 6px 20px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.3s ease;
  z-index: 1; /* 确保在滑块之上 */
}

.nav-item:hover {
  color: var(--text-main);
}

.nav-item.active {
  color: #fff;
  background: var(--accent); /* 选中背景色 */
  box-shadow: 0 0 15px var(--accent-glow); /* 选中发光 */
  text-shadow: 0 0 5px rgba(255, 255, 255, 0.5);
}

/* 如果屏幕变窄，隐藏文字只留搜索和右侧，或者变成滚动 */
@media (max-width: 1100px) {
  .search-module {
    width: 200px;
  }
  .nav-item {
    padding: 6px 12px;
  }
}

/* 布局切换按钮样式 */
.layout-toggle {
  height: 44px;
  padding: 0 16px;
  background: var(--def-col-fltr);
  backdrop-filter: blur(12px);
  border-radius: 25px;
  color: var(--text-main);
  font-family: inherit;
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.3s;
  box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.layout-toggle:hover {
  background: var(--def-col-fltr-hover);
  backdrop-filter: blur(12px);
}

.layout-toggle:active {
  transform: scale(0.96);
}

.layout-icon {
  font-size: 18px;
}

.layout-text {
  font-weight: 500;
}

/* 模组状态统计信息 */
.stats-module {
  width: 120px;
  height: 44px;
  display: flex;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(12px);
  border-radius: 25px;
  justify-content: space-around;
  position: relative;
  overflow: visible;
  box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.stat-num {
  font-size: 24px;
  font-weight: 700;
  line-height: 1;
}

.stat-label {
  font-size: 12px;
  color: var(--text-dim);
  text-transform: uppercase;
}

.active-stat .stat-num {
  color: var(--accent);
  text-shadow: 0 0 10px rgba(61, 90, 254, 0.5);
}

/* --- Mod 卡片列表 --- */
.modules-grid {
  flex: 1;
  padding: 10px 40px 0;
  overflow-y: auto;
  grid-auto-rows: max-content;
  gap: 15px;
  padding-bottom: 110px;
}

/* 网格布局（两列） */
.modules-grid.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
}

/* 列表布局（一列） */
.modules-grid.list-layout {
  display: grid;
  grid-template-columns: 1fr;
}

.modules-grid::-webkit-scrollbar {
  width: 6px;
}

.modules-grid::-webkit-scrollbar-track {
  background: transparent;
}

.modules-grid::-webkit-scrollbar-thumb {
  background: #333;
  border-radius: 3px;
}

.mod-card {
  border-radius: 6px;
  padding: 15px;
  display: flex;
  align-items: center;
  position: relative;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  overflow: hidden;
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.25),
    /* 主阴影 */ 0 2px 6px rgba(0, 0, 0, 0.15);
  /* 次阴影 */
}

/* 列表布局下的卡片样式优化 */
.list-layout .mod-card {
  padding: 20px;
}

.mod-card:hover {
  transform: translateY(-2px);
  box-shadow:
    0 4px 15px rgba(0, 0, 0, 0.25),
    /* 主阴影 */ 0 2px 15px rgba(0, 0, 0, 0.15);
}

.active-card {
  border-left: 2px solid var(--accent);
  box-shadow:
    -3px 0 8px -1px rgba(61, 90, 254, 0.5),
    0 0 0 0 transparent;
  transition:
    box-shadow 0.3s ease,
    transform 0.2s ease;
}

.active-card:hover {
  box-shadow:
    -4px 0 12px -1px var(--accent-glow),
    0 2px 15px rgba(61, 90, 254, 0.15);
  transform: translateY(-2px);
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #333;
  margin-right: 15px;
}

.status-indicator.inactive {
  background: var(--accent);
  box-shadow: 0 0 8px var(--accent);
}

.card-content {
  flex: 1;
  min-width: 0;
}

/* 列表布局下的内容样式优化 */
.list-layout .card-content {
  display: flex;
  align-items: center;
  gap: 20px;
}

.list-layout .mod-header {
  margin-bottom: 0;
}

.list-layout .mod-desc {
  white-space: normal;
  line-height: 1.4;
  max-width: 600px;
}

.mod-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.mod-title {
  font-weight: 700;
  font-size: 16px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tag {
  font-size: 10px;
  padding: 2px 6px;
  background: #333;
  border-radius: 4px;
  color: #aaa;
}

.mod-desc {
  font-size: 12px;
  color: var(--text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 开关样式统一 */
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

/* 滑块轨道背景 */
.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--switch-track);
  border: 1px solid var(--switch-border);
  transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 2px;
}

/* 滑块方块 */
.slider:before {
  position: absolute;
  content: '';
  height: 10px;
  width: 10px;
  left: 3px;
  bottom: 3px;
  background-color: var(--switch-thumb);
  transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 1px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

/* 选中状态：轨道 */
input:checked + .slider {
  background-color: var(--switch-track-checked);
  border-color: var(--switch-border-checked);
}

/* 选中状态：滑块 */
input:checked + .slider:before {
  transform: translateX(22px);
  background-color: var(--switch-thumb-checked);
}

/* 悬停效果 */
.switch:hover .slider {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent-glow);
}

input:checked:hover + .slider {
  border-color: var(--accent);
  box-shadow: 0 0 10px var(--accent-glow);
}

/* ================= 底部控制台 (Deck) ================= */
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
}

.deck-btn {
  height: 40px;
  padding: 0 24px;
  background: transparent;
  border: 1px solid #444;
  color: #ccc;
  font-family: inherit;
  font-weight: 600;
  cursor: pointer;
  margin-right: 15px;
  transition: all 0.2s;
  text-transform: uppercase;
  text-align: center;
  line-height: 40px;
  font-size: 12px;
  letter-spacing: 1px;
}

.deck-btn:hover {
  border-color: var(--accent);
  color: var(--text-main);
  background: var(--hover-bg);
}

.deck-btn.danger {
  border-color: #662222;
  color: #ff5555;
}

.deck-btn.danger:hover {
  background: rgba(255, 85, 85, 0.1);
}

.launch-btn {
  height: 60px;
  padding: 0 50px;
  background: linear-gradient(90deg, #3d5afe, #536dfe);
  border: none;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  clip-path: polygon(10px 0, 100% 0, 100% 100%, 0 100%, 0 10px);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 0 20px var(--accent-glow);
  transition: transform 0.1s;
}

.launch-btn:hover {
  transform: scale(1.02);
  filter: brightness(1.1);
}

.launch-btn:active {
  transform: scale(0.98);
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
  opacity: 0.7;
  letter-spacing: 1px;
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

@keyframes shimmer {
  0% {
    left: -100%;
  }

  20% {
    left: 100%;
  }

  100% {
    left: 100%;
  }
}

/* 响应式调整 */
@media (max-width: 768px) {
  .search-container {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .search-module {
    width: 100%;
  }

  .layout-toggle {
    width: 100%;
    justify-content: center;
  }

  .modules-grid.grid-layout {
    grid-template-columns: 1fr;
  }
}
</style>
