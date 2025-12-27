<template>
  <div class="command-center">
    <!-- 背景图：使用 CSS 变量控制，浅色模式下可降低透明度或更换 -->
    <div class="background" :style="{ '--global-blur': EnableBlur ? '10px' : '0px' }"></div>

    <!-- 1. 左侧导航栏 -->
    <aside class="sidebar">
      <div class="logo-box">
        <div class="logo-icon"><img src="@/assets/WOTBIcon.png" /></div>
        <span class="logo-text">MOD<br />Manager</span>
      </div>

      <nav class="nav-links">
        <div class="nav-item" :class="{ active: NavLinksId === 1 }" @click="NavLinksId = 1">
          <span class="icon">
            <!-- 图标逻辑保持不变，确保图标颜色与文字匹配 -->
            <img :src="DarkMode ? HomeDarkIcon : (NavLinksId === 1 ? HomeActiveIcon : HomeIcon)" alt="模组库" />
          </span>
          <span class="label">模组库</span>
        </div>
        <div class="nav-item" :class="{ active: NavLinksId === 2 }" @click="NavLinksId = 2">
          <span class="icon">
            <img :src="DarkMode ? SettingDarkIcon : (NavLinksId === 2 ? SettingActiveIcon : SettingIcon)" alt="设置" />
          </span>
          <span class="label">系统设置</span>
        </div>
      </nav>

      <div class="sidebar-footer">
        <!-- 添加一个临时开关方便调试，实际项目中这通常在 Settings 组件里控制 -->
        <div class="debug-toggle" @click="DarkMode = !DarkMode" title="点击切换主题演示">
          {{ DarkMode ? 'Dark' : 'Light' }}
        </div>
        <div class="version">v1.2.0</div>
      </div>
    </aside>

    <!-- 主视口容器 -->
    <main class="main-viewport">
      <!-- 这里将 DarkMode 传递给子组件，或者使用 Pinia 全局管理 -->
      <ModLibrary v-if="NavLinksId === 1" />
      <Settings v-else-if="NavLinksId === 2" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, provide } from 'vue'

// ================= 图标资源 (假设路径正确) =================
import SettingIcon from '@/assets/设置.svg'
import SettingActiveIcon from '@/assets/设置_HL.svg'
import SettingDarkIcon from '@/assets/设置Dark.svg'
import HomeIcon from '@/assets/首页.svg'
import HomeActiveIcon from '@/assets/首页_HL.svg'
import HomeDarkIcon from '@/assets/首页Dark.svg'

// ================= 组件 =================
import ModLibrary from '@/components/ModLibrary.vue'
import Settings from '@/components/Settings.vue'

// ================= 响应式数据 =================
const NavLinksId = ref(1)
const DarkMode = ref(true)
const EnableBlur = ref(true)

// ================= 提供数据 (Provide) =================
// 我们把 DarkMode 这个 ref 对象直接提供出去，名字叫 'GlobalTheme'
// 这样子组件拿到后，修改它的 .value，父组件也会同步变
provide('GlobalTheme', DarkMode)
provide('GlobalBlur', EnableBlur)

// ================= 主题切换逻辑 =================
// 监听 DarkMode 变化，动态修改 HTML 根节点的 class
watch(DarkMode, (isDark) => {
  const root = document.documentElement;
  if (isDark) {
    root.classList.remove('light-mode');
    root.classList.add('dark-mode');
  } else {
    root.classList.remove('dark-mode');
    root.classList.add('light-mode');
  }
  localStorage.setItem('app-theme', isDark ? 'dark' : 'light')
}, { immediate: true });

</script>

<style>
/* ================= 全局变量定义 ================= */
@import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@500;700&display=swap');

:root {
  /* --- 基础配置 (深色模式/默认) --- */
  --aside-bg: #0f1115;
  /* 侧边栏背景：深黑 */
  --app-bg-overlay: rgba(0, 0, 0, 0.2);
  /* 背景图遮罩 */

  --deck-bg: #0f1115;
  /* 底部控制台背景 */

  --text-main: #ffffff;
  /* 主文字：纯白 */
  --text-dim: #9ca3af;
  /* 次要文字：灰 */

  --border: #3a3d47;
  /* 边框色 */
  --accent: rgb(61, 90, 254);
  /* 强调色 */
  --accent-glow: rgba(61, 90, 254, 0.5);

  --hover-bg: rgba(255, 255, 255, 0.05);
  /* 侧边栏悬停背景 */

    /* --- 开关相关变量 --- */
  --switch-track: rgba(255, 255, 255, 0.1);  /* 未选中状态轨道 */
  --switch-track-checked: rgba(61, 90, 254, 0.3);  /* 选中状态轨道 */
  --switch-thumb: #ffffff;  /* 滑块颜色 */
  --switch-thumb-checked: #ffffff;  /* 选中滑块颜色 */
  --switch-border: rgba(255, 255, 255, 0.2);  /* 边框 */
  --switch-border-checked: rgba(61, 90, 254, 0.8);  /* 选中边框 */
  
  --def-col-fltr: rgba(255, 255, 255, 0.05);
  --def-col-fltr-hover: rgba(255, 255, 255, 0.08);
  --scroll-track: rgba(255, 255, 255, 0.05);
}

/* --- 浅色模式覆盖 (Light Mode) --- */
:root.light-mode {
  --aside-bg: #ffffff;
  /* 侧边栏背景：纯白 */
  --app-bg-overlay: rgba(255, 255, 255, 0.85);
  /* 强力遮罩，淡化背景图，确保文字清晰 */

  --deck-bg: #FFFFFF;
  /* 底部控制台背景：纯白 */

  /* 关键修改：使用深蓝灰代替纯黑，视觉更舒适 */
  --text-main: #2c3e50;
  --text-dim: #64748b;

  --border: #e2e8f0;
  /* 浅灰边框 */
  --accent: #3d5afe;
  /* 保持品牌色不变 */
  --accent-glow: rgba(61, 90, 254, 0.15);

  --hover-bg: rgba(61, 90, 254, 0.05);
  /* 浅色模式下的悬停是淡淡的品牌色 */

    /* --- 浅色模式开关变量 --- */
  --switch-track: rgba(0, 0, 0, 0.1);
  --switch-track-checked: rgba(61, 90, 254, 0.2);
  --switch-thumb: #ffffff;
  --switch-thumb-checked: #ffffff;
  --switch-border: rgba(0, 0, 0, 0.2);
  --switch-border-checked: rgba(61, 90, 254, 0.8);
  
  --def-col-fltr: rgba(255, 255, 255, 0.8);
  --def-col-fltr-hover: rgba(255, 255, 255, 0.9);
  --scroll-track: rgba(0, 0, 0, 0.05);
}

html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  /* 基础背景色 */
  background-color: var(--aside-bg);
  color: var(--text-main);
  font-family: 'Rajdhani', 'Segoe UI', sans-serif;
  overflow: hidden;
  transition: color 0.3s ease, background-color 0.3s ease;
}
</style>

<style scoped>
.command-center {
  display: flex;
  width: 100vw;
  height: 100vh;
  position: relative;
}

.background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: url('@/assets/123517794_p0.jpg');
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
  z-index: 0;
}

/* 添加一个遮罩层，用于在浅色模式下让背景图变淡，保证文字可读性 */
.background::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--app-bg-overlay);
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  /* 可选：给背景加点模糊 */
  transition: background 0.5s ease;
}

/* ================= 侧边栏 ================= */
.sidebar {
  width: 90px;
  background-color: var(--aside-bg);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 20px;
  z-index: 10;
  transition: background-color 0.3s ease, border-color 0.3s ease;
  /* 增加阴影，在浅色模式下区分侧边栏和内容 */
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.05);
}

.logo-box {
  margin-bottom: 40px;
  text-align: center;
}

.logo-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 5px;
}

.logo-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.logo-text {
  font-size: 10px;
  font-weight: 700;
  color: var(--text-dim);
  letter-spacing: 1px;
}

.nav-links {
  display: flex;
  flex-direction: column;
  gap: 20px;
  width: 100%;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  padding: 10px 0;
  color: var(--text-dim);
  transition: all 0.3s;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  color: var(--text-main);
  background: var(--hover-bg);
}

.nav-item.active {
  color: var(--accent);
  /* 选中状态文字变为强调色 */
  background: linear-gradient(90deg, var(--hover-bg) 0%, transparent 100%);
  border-left-color: var(--accent);
}

.nav-item .icon {
  font-size: 20px;
  margin-bottom: 4px;
  display: flex;
  /* 修复图标垂直居中 */
  align-items: center;
  justify-content: center;
}

.nav-item .icon img {
  width: 20px;
  height: 20px;
  transition: filter 0.3s;
}

.nav-item .label {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
}

.sidebar-footer {
  margin-top: auto;
  padding-bottom: 20px;
  font-size: 10px;
  color: var(--text-dim);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.debug-toggle {
  cursor: pointer;
  padding: 2px 6px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 9px;
  user-select: none;
}

/* ================= 主视口 ================= */
.main-viewport {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 5;
  /* 确保在背景图之上 */
  overflow: hidden;
  /* 防止子组件溢出 */
}
</style>