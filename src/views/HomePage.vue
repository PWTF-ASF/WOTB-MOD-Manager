<template>
  <div class="command-center">
    <div class="background"></div>
    <!-- 1. 左侧导航栏 -->
    <aside class="sidebar">
      <div class="logo-box">
        <div class="logo-icon"><img src="@/assets/WOTBIcon.png" /></div>
        <span class="logo-text">MOD<br />Manager</span>
      </div>

      <nav class="nav-links">
        <div class="nav-item" :class="{ active: NavLinksId === 1 }" @click="NavLinksId = 1">
          <span class="icon"><img :src="DarkMode
            ? NavLinksId === 1
              ? HomeDarkActiveIcon
              : HomeDarkIcon
            : NavLinksId === 1
              ? HomeActiveIcon
              : HomeIcon
            " alt="模组库" /></span>
          <span class="label">模组库</span>
        </div>
        <div class="nav-item" :class="{ active: NavLinksId === 2 }" @click="NavLinksId = 2">
          <span class="icon">
            <img :src="DarkMode
              ? NavLinksId === 2
                ? SettingDarkActiveIcon
                : SettingDarkIcon
              : NavLinksId === 2
                ? SettingActiveIcon
                : SettingIcon
              " alt="设置" />
          </span>
          <span class="label">系统设置</span>
        </div>
      </nav>

      <div class="sidebar-footer">
        <div class="version">v1.2.0</div>
      </div>
    </aside>

    <!-- 主视口容器 -->
    <main class="main-viewport">
      <ModLibrary v-if="NavLinksId === 1" />
      <!-- 3. 系统设置占位 (Settings) -->
      <template v-if="NavLinksId === 2">
        <div style="display: flex; justify-content: center; align-items: center; height: 100%; color: #666">
          系统设置页面...
        </div>
      </template>
    </main>
  </div>
</template>

<script setup lang="ts">
// ================= Vue3组件 =================
import { ref } from 'vue'

// ================= 图标资源 =================
import SettingIcon from '@/assets/设置.svg'
import SettingActiveIcon from '@/assets/设置_HL.svg'
import SettingDarkIcon from '@/assets/设置Dark.svg'
import SettingDarkActiveIcon from '@/assets/设置Dark_HL.svg'
import HomeIcon from '@/assets/首页.svg'
import HomeActiveIcon from '@/assets/首页_HL.svg'
import HomeDarkIcon from '@/assets/首页Dark.svg'
import HomeDarkActiveIcon from '@/assets/首页Dark_HL.svg'

// ================= 组件 =================
import ModLibrary from '@/components/ModLibrary.vue'

// ================= 响应式数据 =================
const NavLinksId = ref(1)
const DarkMode = ref(false)
</script>

<style>
/* 全局样式保持不变 */
@import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@500;700&display=swap');

:root {
  --aside-bg: #0f1115;
  /* 侧边栏背景色 */
  --def-col: rgb(239, 245, 255);
  /* 默认颜色 */
  --def-col-fltr: rgba(239, 245, 255, 0.03);
  /* 默认模糊颜色 */
  --def-col-hover: rgba(217, 215, 215, 1);
  /* 默认hover状态颜色 */
  --def-col-fltr-hover: rgba(217, 215, 215, 0.5);
  /* 默认hover状态模糊颜色 */
  --accent: #3d5afe;
  /* 主强调色（品牌色） */
  --accent-glow: rgba(61, 90, 254, 0.4);
  /* 强调色光晕 */
  --text-main: #ffffff;
  /* 主要文本色（正常可读性文本） */
  --text-dim: #6b7280;
  /* 次要文本色（辅助说明、禁用状态文本） */
  --success: #00e676;
  /* 成功状态色（如安装完成、操作成功） */
  --border: #3a3d47;
}

html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  background-color: var(--aside-bg);
  color: var(--text-main);
  font-family: 'Rajdhani', 'Segoe UI', sans-serif;
  overflow: hidden;
}
</style>

<style scoped>
.command-center {
  display: flex;
  width: 100vw;
  height: 100vh;
}

.background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: url('@/assets/123517794_p0.jpg');
  /* background-color: #444; */
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
  z-index: 0;
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

.nav-item:hover,
.nav-item.active {
  color: var(--text-main);
  background: linear-gradient(90deg, rgba(61, 90, 254, 0.1) 0%, transparent 100%);
}

.nav-item.active {
  border-left-color: var(--accent);
  text-shadow: 0 0 8px rgba(255, 255, 255, 0.5);
}

.nav-item .icon {
  font-size: 20px;
  margin-bottom: 4px;
}

.nav-item .icon img {
  width: 20px;
  height: 20px;
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
  color: #444;
}

/* ================= 主视口 ================= */
.main-viewport {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
}
</style>
