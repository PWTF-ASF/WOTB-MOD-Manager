<template>
  <div class="command-center">
      <!-- 背景图：使用 CSS 变量控制，浅色模式下可降低透明度或更换 -->
      <div class="background" :style="backgroundStyle"></div>

      <!-- 1. 左侧导航栏 -->
      <aside class="sidebar">
        <div class="logo-box">
          <div class="logo-icon"><img src="@/assets/WOTBIcon.png" /></div>
          <span class="logo-text">MOD<br />Manager</span>
        </div>

        <nav class="nav-links">
          <RouterLink to="/library" class="nav-item" active-class="active">
            <span class="icon">
              <img :src="libraryIcon" alt="" />
            </span>
            <span class="label">模组库</span>
          </RouterLink>
          <RouterLink to="/settings" class="nav-item" active-class="active">
            <span class="icon">
              <img :src="settingsIcon" alt="" />
            </span>
            <span class="label">系统设置</span>
          </RouterLink>
        </nav>

        <div class="sidebar-footer">
          <div class="version">v{{ version }}</div>
        </div>
      </aside>

      <!-- 主视口容器 -->
      <main class="main-viewport">
        <RouterView v-slot="{ Component }">
          <KeepAlive>
            <component :is="Component" />
          </KeepAlive>
        </RouterView>
      </main>
  </div>
</template>

<script setup lang="ts">
import { ref, provide, computed, onMounted, onUnmounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useRoute } from 'vue-router'
import { usePreferencesStore } from '@/stores/preferences'

// ================= 图标资源 =================
import SettingIcon from '@/assets/设置.svg'
import SettingActiveIcon from '@/assets/设置_HL.svg'
import SettingDarkIcon from '@/assets/设置Dark.svg'
import SettingDarkActiveIcon from '@/assets/设置Dark_HL.svg'
import HomeIcon from '@/assets/首页.svg'
import HomeActiveIcon from '@/assets/首页_HL.svg'
import HomeDarkIcon from '@/assets/首页Dark.svg'
import HomeDarkActiveIcon from '@/assets/首页Dark_HL.svg'

// ================= 平台检测 =================
const store = useStore()
const version = computed(() => store.version)
const route = useRoute()
const preferences = usePreferencesStore()
const { isDark: DarkMode, backgroundStyle } = storeToRefs(preferences)
const libraryIcon = computed(() => {
  if (DarkMode.value) return route.name === 'library' ? HomeDarkActiveIcon : HomeDarkIcon
  return route.name === 'library' ? HomeActiveIcon : HomeIcon
})
const settingsIcon = computed(() => {
  if (DarkMode.value) return route.name === 'settings' ? SettingDarkActiveIcon : SettingDarkIcon
  return route.name === 'settings' ? SettingActiveIcon : SettingIcon
})

// Mod 列表缓存失效信号（供跨组件通知刷新）
const modListVersion = ref(0)
provide('invalidateModList', () => { modListVersion.value++ })
provide('modListVersion', modListVersion)

onMounted(() => preferences.initialize())
onUnmounted(() => preferences.dispose())

</script>

<style>
/* ================= 全局变量定义 ================= */
/* 显示字体：Rajdhani 打包在本地，加载失败时回退到系统字体 */
@font-face {
  font-family: 'Rajdhani';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('../assets/fonts/rajdhani-latin-400.woff2') format('woff2');
}
@font-face {
  font-family: 'Rajdhani';
  font-style: normal;
  font-weight: 500;
  font-display: swap;
  src: url('../assets/fonts/rajdhani-latin-500.woff2') format('woff2');
}
@font-face {
  font-family: 'Rajdhani';
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url('../assets/fonts/rajdhani-latin-600.woff2') format('woff2');
}
@font-face {
  font-family: 'Rajdhani';
  font-style: normal;
  font-weight: 700;
  font-display: swap;
  src: url('../assets/fonts/rajdhani-latin-700.woff2') format('woff2');
}

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

  /* --- 默认背景渐变 (深色) --- */
  --bg-gradient-start: #1a1d23;
  --bg-gradient-mid: #0f1115;
  --bg-gradient-end: #1a1d23;

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
  --card-bg: #1a1a24;

  /* --- 新拟态 (Neumorphism) --- */
  --neu-base: #1a1d23;
  --neu-raised: #1e2128;
  --neu-inset: #161920;
  --neu-shadow-light: rgba(255, 255, 255, 0.04);
  --neu-shadow-mid: rgba(255, 255, 255, 0.02);
  --neu-shadow-dark: rgba(0, 0, 0, 0.5);
  --neu-shadow-accent: rgba(61, 90, 254, 0.12);
  --neu-radius: 16px;
  --neu-radius-sm: 12px;
  --neu-radius-lg: 24px;

  /* 滚动条 */
  --scroll-track: rgba(255, 255, 255, 0.05);

}

/* --- 浅色模式覆盖 --- */
:root.light-mode {
  --aside-bg: #ffffff;
  --app-bg-overlay: rgba(255, 255, 255, 0.85);
  --deck-bg: #ffffff;

  /* --- 默认背景渐变 (浅色) --- */
  --bg-gradient-start: #e8ecf1;
  --bg-gradient-mid: #f1f5f9;
  --bg-gradient-end: #e8ecf1;

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
  --card-bg: #ffffff;

  /* --- 新拟态 (Neumorphism) --- */
  --neu-base: #eef0f3;
  --neu-raised: #f0f2f5;
  --neu-inset: #e8eaf0;
  --neu-shadow-light: rgba(255, 255, 255, 0.8);
  --neu-shadow-mid: rgba(255, 255, 255, 0.5);
  --neu-shadow-dark: rgba(174, 181, 196, 0.6);
  --neu-shadow-accent: rgba(61, 90, 254, 0.1);
  --neu-radius: 16px;
  --neu-radius-sm: 12px;
  --neu-radius-lg: 24px;

  /* 滚动条 */
  --scroll-track: rgba(0, 0, 0, 0.05);

}

</style>

<style scoped>
/* ===========================
   新拟态 (Neumorphism) 暗黑模式 3.0
   =========================== */

/* ---- 布局容器 ---- */
.command-center {
  display: flex;
  width: 100vw;
  height: 100vh;
  position: relative;
  animation: fade-scale-in 0.5s var(--animation-timing);
}

/* ---- 背景层（保持不变） ---- */
.background {
  position: absolute;
  inset: 0;
  background-image: linear-gradient(135deg, var(--bg-gradient-start) 0%, var(--bg-gradient-mid) 50%, var(--bg-gradient-end) 100%);
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  background-attachment: fixed;
  z-index: 0;
  transition: filter 0.5s ease, background-image 0.5s ease;
}

.background::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--app-bg-overlay);
  transition: background var(--animation-duration) ease;
}

/* ---- 侧边栏：扁平导航面板 ---- */
.sidebar {
  width: var(--sidebar-width);
  background: color-mix(in srgb, var(--ui-bg-sidebar) 96%, transparent);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: var(--sidebar-padding);
  z-index: 10;
  position: relative;
  border-right: 1px solid var(--ui-border-subtle);
  box-shadow: none;
  transition: background var(--animation-duration) ease, border-color var(--animation-duration) ease;
}

/* ---- Logo ---- */
.logo-box {
  margin-bottom: 40px;
  text-align: center;
  animation: slide-up-fade 0.6s var(--animation-timing) 0.1s backwards;
}

.logo-icon {
  width: 44px;
  height: 44px;
  border: 1px solid var(--ui-border-subtle);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg-surface);
  box-shadow: none;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 5px;
  transition: all 0.3s ease;
  padding: 8px;
}

.logo-icon:hover { border-color: var(--ui-border-strong); }

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

/* ---- 导航链接 ---- */
.nav-links {
  display: flex;
  flex-direction: column;
  gap: 6px;
  width: 100%;
  padding: 0 10px;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  padding: 10px 0;
  color: var(--text-dim);
  min-height: var(--nav-item-height);
  border: 1px solid transparent;
  border-radius: var(--ui-radius-md);
  background: transparent;
  box-shadow: none;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  animation: slide-up-fade 0.6s var(--animation-timing) backwards;
  position: relative;
}

.nav-item:nth-child(1) { animation-delay: 0.15s; }
.nav-item:nth-child(2) { animation-delay: 0.2s; }

.nav-item:hover {
  color: var(--text-main);
  border-color: var(--ui-border-subtle);
  background: var(--ui-bg-surface-hover);
  box-shadow: none;
  transform: none;
}

.nav-item:hover .icon img {
  filter: brightness(1.2);
}

.nav-item.active {
  color: var(--accent);
  border-color: color-mix(in srgb, var(--ui-accent) 24%, var(--ui-border-subtle));
  background: color-mix(in srgb, var(--ui-accent) 10%, transparent);
  box-shadow: none;
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: -10px;
  width: 3px;
  height: 28px;
  border-radius: 0 3px 3px 0;
  background: var(--ui-accent);
}

.nav-item.active .icon img {
  filter: none;
}

.nav-item .icon {
  font-size: 20px;
  margin-bottom: 4px;
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
  transform: none;
}

.nav-item .label {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  transition: all var(--animation-duration);
}

/* ---- 侧边栏底部 ---- */
.sidebar-footer {
  margin-top: auto;
  padding-bottom: var(--sidebar-padding);
  font-size: 10px;
  color: var(--text-dim);
  text-decoration: none;
  display: flex;
  flex-direction: column;
  align-items: center;
  animation: slide-up-fade 0.6s var(--animation-timing) 0.25s backwards;
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

/* ---- 主视口 ---- */
.main-viewport {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 5;
  overflow: hidden;
  animation: fade-scale-in 0.5s var(--animation-timing) 0.1s backwards;
}

/* ---- 响应式 ---- */
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
    width: 36px;
    height: 36px;
    padding: 6px;
  }

  .nav-item .icon img {
    width: 18px;
    height: 18px;
  }

  .nav-item {
    padding: 8px 0;
    min-height: 60px;
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

/* ---- 全局过渡 ---- */
.command-center,
.sidebar,
.nav-item,
.logo-text,
.version {
  transition-property: background-color, color, box-shadow, transform;
  transition-duration: var(--animation-duration);
  transition-timing-function: var(--animation-timing);
}

/* ---- 性能优化 ---- */
.sidebar,
.nav-item,
.logo-icon,
.icon img {
  transform: translateZ(0);
}

/* ---- 加载状态 ---- */
.command-center.loading {
  opacity: 0.5;
  pointer-events: none;
}

.command-center.loading::after {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--neu-base);
  opacity: 0.7;
  z-index: 100;
  animation: pulse-subtle 2s ease-in-out infinite;
}
</style>
