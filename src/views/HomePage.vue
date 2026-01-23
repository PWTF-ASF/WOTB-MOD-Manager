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
            <img :src="DarkMode ? HomeDarkIcon : NavLinksId === 1 ? HomeActiveIcon : HomeIcon" alt="模组库" />
          </span>
          <span class="label">模组库</span>
        </div>
        <div class="nav-item" :class="{ active: NavLinksId === 2 }" @click="NavLinksId = 2">
          <span class="icon">
            <img :src="DarkMode ? SettingDarkIcon : NavLinksId === 2 ? SettingActiveIcon : SettingIcon" alt="设置" />
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
const DarkMode = ref(false)
const EnableBlur = ref(true)

// ================= 提供数据 (Provide) =================
// 我们把 DarkMode 这个 ref 对象直接提供出去，名字叫 'GlobalTheme'
// 这样子组件拿到后，修改它的 .value，父组件也会同步变
provide('GlobalTheme', DarkMode)
provide('GlobalBlur', EnableBlur)

// ================= 主题切换逻辑 =================
// 监听 DarkMode 变化，动态修改 HTML 根节点的 class
watch(
  DarkMode,
  isDark => {
    const root = document.documentElement
    if (isDark) {
      root.classList.remove('light-mode')
      root.classList.add('dark-mode')
    } else {
      root.classList.remove('dark-mode')
      root.classList.add('light-mode')
    }
    localStorage.setItem('app-theme', isDark ? 'dark' : 'light')
  },
  { immediate: true }
)
</script>

<style>
/* ================= 全局变量定义 ================= */
@import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@400;500;600;700&display=swap');

:root {
  /* --- 动画配置 --- */
  --animation-duration: 0.3s;
  --animation-timing: cubic-bezier(0.4, 0, 0.2, 1);
  --animation-timing-bounce: cubic-bezier(0.68, -0.55, 0.265, 1.55);

  /* --- 尺寸配置 --- */
  --sidebar-width: 90px;
  --sidebar-padding: 20px;
  --nav-item-height: 70px;

  /* --- 颜色配置 (深色模式/默认) --- */
  --aside-bg: #0f1115;
  --app-bg-overlay: rgba(0, 0, 0, 0.2);
  --deck-bg: #0f1115;

  /* 文字颜色 */
  --text-main: #ffffff;
  --text-dim: #9ca3af;
  --text-accent: #3d5afe;

  /* 边框和强调色 */
  --border: #3a3d47;
  --accent: rgb(61, 90, 254);
  --accent-glow: rgba(61, 90, 254, 0.5);

  /* 背景颜色 */
  --hover-bg: rgba(255, 255, 255, 0.05);
  --bg-main: rgba(15, 17, 21, 0.95);
  --bg-card: rgba(255, 255, 255, 0.05);
  --bg-card-hover: rgba(255, 255, 255, 0.08);
  --bg-input: rgba(255, 255, 255, 0.05);
  --bg-input-focus: rgba(255, 255, 255, 0.1);

  /* 毛玻璃效果 */
  --glass-effect: rgba(255, 255, 255, 0.05);
  --glass-effect-hover: rgba(255, 255, 255, 0.08);
  --glass-effect-focus: rgba(255, 255, 255, 0.12);

  /* 滚动条 */
  --scroll-track: rgba(255, 255, 255, 0.05);

  /* --- 开关相关 --- */
  --switch-track: rgba(255, 255, 255, 0.1);
  --switch-track-checked: rgba(61, 90, 254, 0.3);
  --switch-thumb: #ffffff;
  --switch-thumb-checked: #ffffff;
  --switch-border: rgba(255, 255, 255, 0.2);
  --switch-border-checked: rgba(61, 90, 254, 0.8);
}

/* --- 浅色模式覆盖 --- */
:root.light-mode {
  --aside-bg: #ffffff;
  --app-bg-overlay: rgba(255, 255, 255, 0.85);
  --deck-bg: #ffffff;

  /* 文字颜色 */
  --text-main: #1e293b;
  --text-dim: #64748b;
  --text-accent: #3d5afe;

  /* 边框和强调色 */
  --border: #e2e8f0;
  --accent: #3d5afe;
  --accent-glow: rgba(61, 90, 254, 0.2);

  /* 背景颜色 */
  --hover-bg: rgba(61, 90, 254, 0.05);
  --bg-main: rgba(255, 255, 255, 0.95);
  --bg-card: rgba(255, 255, 255, 0.8);
  --bg-card-hover: rgba(255, 255, 255, 0.9);
  --bg-input: rgba(255, 255, 255, 0.9);
  --bg-input-focus: rgba(255, 255, 255, 0.98);

  /* 毛玻璃效果 */
  --glass-effect: rgba(255, 255, 255, 0.8);
  --glass-effect-hover: rgba(255, 255, 255, 0.9);
  --glass-effect-focus: rgba(255, 255, 255, 0.95);

  /* 滚动条 */
  --scroll-track: rgba(0, 0, 0, 0.05);

  /* 开关 */
  --switch-track: rgba(0, 0, 0, 0.1);
  --switch-track-checked: rgba(61, 90, 254, 0.2);
  --switch-thumb: #ffffff;
  --switch-thumb-checked: #ffffff;
  --switch-border: rgba(0, 0, 0, 0.2);
  --switch-border-checked: rgba(61, 90, 254, 0.8);
}
</style>

<style scoped>
/* ================= 布局容器 ================= */
.command-center {
  display: flex;
  width: 100vw;
  height: 100vh;
  position: relative;
  animation: fade-scale-in 0.5s var(--animation-timing);
}

/* ================= 背景层 ================= */
.background {
  position: absolute;
  inset: 0;
  background-image: url('@/assets/123517794_p0.jpg');
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
  background-attachment: fixed;
  z-index: 0;
  transition: filter 0.5s ease;
}

.background::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--app-bg-overlay);
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  transition:
    background var(--animation-duration) ease,
    backdrop-filter var(--animation-duration) ease;
}

/* ================= 侧边栏 ================= */
.sidebar {
  width: var(--sidebar-width);
  background-color: var(--aside-bg);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: var(--sidebar-padding);
  z-index: 10;
  transition:
    background-color var(--animation-duration) ease,
    border-color var(--animation-duration) ease;
  box-shadow: 2px 0 12px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;
}

.sidebar::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0.5;
}

/* 侧边栏 Logo */
.logo-box {
  margin-bottom: 40px;
  text-align: center;
  animation: slide-up-fade 0.6s var(--animation-timing) 0.1s backwards;
}

.logo-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 5px;
  transition: transform 0.3s ease;
}

.logo-icon:hover {
  transform: scale(1.1);
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
  text-transform: uppercase;
  transition: color var(--animation-duration);
}

/* 导航链接 */
.nav-links {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  padding: 12px 0;
  color: var(--text-dim);
  transition: all var(--animation-duration) var(--animation-timing);
  border-left: 3px solid transparent;
  position: relative;
  min-height: var(--nav-item-height);
  animation: slide-up-fade 0.6s var(--animation-timing) backwards;
}

.nav-item:nth-child(1) {
  animation-delay: 0.15s;
}
.nav-item:nth-child(2) {
  animation-delay: 0.2s;
}

.nav-item:hover {
  color: var(--text-main);
  background: var(--hover-bg);
}

.nav-item:hover .icon img {
  filter: brightness(1.2);
}

.nav-item.active {
  color: var(--accent);
  background: linear-gradient(90deg, rgba(var(--accent-rgb, 61, 90, 254), 0.1) 0%, transparent 100%);
  border-left-color: var(--accent);
}

.nav-item.active .icon img {
  filter: drop-shadow(0 0 8px var(--accent-glow));
}

.nav-item .icon {
  font-size: 20px;
  margin-bottom: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--animation-duration);
}

.nav-item .icon img {
  width: 22px;
  height: 22px;
  transition:
    filter var(--animation-duration),
    transform var(--animation-duration);
}

.nav-item.active .icon img {
  transform: scale(1.1);
}

.nav-item .label {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  transition: all var(--animation-duration);
}

/* 侧边栏底部 */
.sidebar-footer {
  margin-top: auto;
  padding-bottom: var(--sidebar-padding);
  font-size: 10px;
  color: var(--text-dim);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  animation: slide-up-fade 0.6s var(--animation-timing) 0.25s backwards;
}

.debug-toggle {
  cursor: pointer;
  padding: 4px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 9px;
  font-weight: 600;
  user-select: none;
  transition: all var(--animation-duration);
  background: var(--glass-effect);
}

.debug-toggle:hover {
  background: var(--glass-effect-hover);
  border-color: var(--accent);
  color: var(--text-main);
}

.version {
  font-size: 9px;
  letter-spacing: 0.5px;
  opacity: 0.7;
  transition: opacity var(--animation-duration);
}

.version:hover {
  opacity: 1;
}

/* ================= 主视口 ================= */
.main-viewport {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 5;
  overflow: hidden;
  animation: fade-scale-in 0.5s var(--animation-timing) 0.1s backwards;
}

/* ================= 响应式设计 ================= */
@media (max-width: 768px) {
  :root {
    --sidebar-width: 70px;
    --sidebar-padding: 15px;
  }

  .sidebar {
    width: var(--sidebar-width);
  }

  .nav-item .label {
    font-size: 8px;
  }

  .logo-icon {
    width: 32px;
    height: 32px;
  }

  .nav-item .icon img {
    width: 18px;
    height: 18px;
  }
}

@media (max-height: 600px) {
  .logo-box {
    margin-bottom: 20px;
  }

  .nav-links {
    gap: 4px;
  }

  .nav-item {
    padding: 8px 0;
    min-height: 60px;
  }
}

/* ================= 主题切换动画 ================= */
.command-center,
.sidebar,
.nav-item,
.logo-text,
.debug-toggle,
.version {
  transition-property: background-color, color, border-color, box-shadow, transform;
  transition-duration: var(--animation-duration);
  transition-timing-function: var(--animation-timing);
}

/* ================= 性能优化 ================= */
.sidebar,
.nav-item,
.logo-icon,
.icon img {
  will-change: transform, opacity;
  transform: translateZ(0);
}

/* ================= 加载状态 ================= */
.command-center.loading {
  opacity: 0.5;
  pointer-events: none;
}

.command-center.loading::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--aside-bg);
  opacity: 0.7;
  z-index: 100;
  animation: pulse-subtle 2s ease-in-out infinite;
}
</style>
