<template>
  <n-config-provider :theme-overrides="themeOverrides" :locale="zhCN" :date-locale="dateZhCN">
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
          <div class="nav-item" :class="{ active: NavLinksId === 1 }" @click="NavLinksId = 1">
            <span class="icon">
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
          <!-- 使用 Naive UI 按钮替代原生调试开关 -->
          <n-button
            class="debug-toggle"
            @click="toggleTheme()"
            size="tiny"
            :type="DarkMode ? 'primary' : 'default'"
            title="点击切换主题演示"
          >
            {{ DarkMode ? 'Dark' : 'Light' }}
          </n-button>
          <div class="version">v1.2.0</div>
        </div>
      </aside>

      <!-- 主视口容器 -->
      <main class="main-viewport">
        <ModLibrary v-if="NavLinksId === 1" />
        <Settings v-else-if="NavLinksId === 2" />
      </main>
    </div>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, watch, provide, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { convertFileSrc } from '@tauri-apps/api/core'
import { zhCN, dateZhCN, NConfigProvider, NButton } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import defaultBg from '@/assets/123517794_p0.jpg'

// ================= 图标资源 =================
import SettingIcon from '@/assets/设置.svg'
import SettingActiveIcon from '@/assets/设置_HL.svg'
import SettingDarkIcon from '@/assets/设置Dark.svg'
import HomeIcon from '@/assets/首页.svg'
import HomeActiveIcon from '@/assets/首页_HL.svg'
import HomeDarkIcon from '@/assets/首页Dark.svg'

// ================= 组件 =================
import ModLibrary from '@/components/ModLibrary.vue'
import Settings from '@/components/Settings.vue'

// ================= 平台检测 =================
const isLinux = ref(false)
const isWindows = ref(false)
const isMacOS = ref(false)

// 检测运行平台（Linux适配关键）
function detectPlatform() {
  const userAgent = navigator.userAgent.toLowerCase()
  isLinux.value = userAgent.includes('linux')
  isWindows.value = userAgent.includes('win')
  isMacOS.value = userAgent.includes('mac') || userAgent.includes('apple')
}

// ================= 响应式数据 =================
const NavLinksId = ref(1)
const DarkMode = ref(false)
const BackgroundMask = ref(true) // 是否启用背景遮罩
const MaskOpacity = ref(20) // 遮罩透明度，范围 0-100
const backgroundImagePath = ref<string | null>(null)
const backgroundImageBase64 = ref<string | null>(null)
const ThemeMode = ref<'light' | 'dark' | 'system'>('system') // 主题模式：浅色、深色、跟随系统

// 背景模糊效果相关变量
const enableBackgroundBlur = ref(true)
const backgroundBlurAmount = ref(10) // 模糊度，范围 0-20px

// 毛玻璃效果相关变量 - BewlyCat单一强度控制模式
const enableGlassEffect = ref(true)
const glassIntensity = ref(50) // 单一强度控制 0-100

// 通过单一强度值计算各项玻璃参数 (BewlyCat风格曲线映射)
const computedGlassParams = computed(() => {
  const intensity = glassIntensity.value
  const t = intensity / 100
  
  return {
    opacity: 0.9 - t * 0.3,              // 0.9 → 0.6 (通透 → 磨砂)
    blur: 4 + t * 24,                    // 4px → 28px
    saturate: 140 + t * 80               // 140% → 220%
  }
})

// ================= 默认值配置 =================
const defaultVisualSettings = {
  enableBackgroundBlur: true,
  backgroundBlurAmount: 10,
  enableGlassEffect: true,
  glassIntensity: 50,
  backgroundMask: true,
  maskOpacity: 20
}

// ================= 提供数据 =================
provide('GlobalTheme', DarkMode)
provide('ThemeMode', ThemeMode)
provide('BackgroundMask', BackgroundMask)
provide('MaskOpacity', MaskOpacity)
provide('backgroundImagePath', backgroundImagePath)

// 提供背景模糊相关变量
provide('enableBackgroundBlur', enableBackgroundBlur)
provide('backgroundBlurAmount', backgroundBlurAmount)

// 提供毛玻璃效果相关变量 - BewlyCat单一强度模式
provide('enableGlassEffect', enableGlassEffect)
provide('glassIntensity', glassIntensity)

// 提供平台检测信息 (Linux适配)
provide('isLinux', isLinux)

// 提供重置视觉效果的方法
provide('resetVisualSettings', resetVisualSettings)

// ================= 背景图片Base64转换 (Linux适配核心) =================
const convertBackgroundToBase64 = async (path: string): Promise<string | null> => {
  if (!path) return null
  try {
    const base64 = await invoke('read_image_base64', { path }) as string
    // 根据图片扩展名判断MIME类型
    const ext = path.toLowerCase().split('.').pop()
    let mimeType = 'image/jpeg'
    if (ext === 'png') mimeType = 'image/png'
    if (ext === 'webp') mimeType = 'image/webp'
    return `data:${mimeType};base64,${base64}`
  } catch (err) {
    console.error('转换背景图片Base64失败:', err)
    return null
  }
}

// ================= 提供更新背景图片的方法 =================
const setBackgroundImage = async (imagePath: string | null) => {
  if (imagePath === null) {
    // 移除背景
    await invoke('remove_background_image')
    backgroundImagePath.value = null
    backgroundImageBase64.value = null
  } else {
    // 上传并保存背景
    const savedPath = await invoke('set_background_image', { imagePath }) as string
    backgroundImagePath.value = savedPath
    
    // Linux平台强制使用Base64方式显示图片
    if (isLinux.value) {
      backgroundImageBase64.value = await convertBackgroundToBase64(savedPath)
    }
  }
}
provide('setBackgroundImage', setBackgroundImage)

// 重置视觉效果到默认值
function resetVisualSettings() {
  enableBackgroundBlur.value = defaultVisualSettings.enableBackgroundBlur
  backgroundBlurAmount.value = defaultVisualSettings.backgroundBlurAmount
  enableGlassEffect.value = defaultVisualSettings.enableGlassEffect
  glassIntensity.value = defaultVisualSettings.glassIntensity
  BackgroundMask.value = defaultVisualSettings.backgroundMask
  MaskOpacity.value = defaultVisualSettings.maskOpacity
}

// 将变量应用到全局
watch(
  [enableBackgroundBlur, backgroundBlurAmount, BackgroundMask, MaskOpacity, enableGlassEffect, glassIntensity],
  () => {
    const root = document.documentElement;
    const overlayOpacity = BackgroundMask.value ? MaskOpacity.value / 100 : 0;
    
    // 背景模糊效果 - 仅控制背景层的清晰度
    const bgBlur = enableBackgroundBlur.value ? backgroundBlurAmount.value : 0;
    
    // 毛玻璃效果 - BewlyCat风格，使用单一强度计算所有参数
    const params = computedGlassParams.value;
    const glassAlphaValue = enableGlassEffect.value ? params.opacity : 1;
    const glassBlurValue = enableGlassEffect.value ? params.blur : 0;
    const glassSaturateValue = enableGlassEffect.value ? params.saturate : 100;
    const glassEnabled = enableGlassEffect.value ? 1 : 0;
    
    // 应用CSS变量（数值不带单位，在CSS calc中处理）
    root.style.setProperty('--global-blur', `${bgBlur}`);
    root.style.setProperty('--overlay-opacity', `${overlayOpacity}`);
    root.style.setProperty('--glass-bg-alpha', `${glassAlphaValue}`);
    root.style.setProperty('--glass-blur', `${glassBlurValue}`);
    root.style.setProperty('--glass-saturate', `${glassSaturateValue}`);
    root.style.setProperty('--glass-enabled', glassEnabled.toString());
    root.style.setProperty('--glass-opacity', `${glassAlphaValue}`);
  },
  { immediate: true, deep: true }
);
// 计算背景样式
const backgroundStyle = computed(() => {
  let bgImage = `url("${defaultBg}")`;
  if (backgroundImagePath.value) {
    // Linux平台使用Base64方式，其他平台使用convertFileSrc
    if (isLinux.value && backgroundImageBase64.value) {
      bgImage = `url("${backgroundImageBase64.value}")`;
    } else {
      const url = convertFileSrc(backgroundImagePath.value);
      bgImage = `url("${url}")`;
    }
  }
  // 计算遮罩颜色，根据当前主题模式使用不同的基础颜色
  const maskColor = DarkMode.value ? 
    `rgba(0, 0, 0, ${BackgroundMask.value ? MaskOpacity.value / 100 : 0})` : 
    `rgba(255, 255, 255, ${BackgroundMask.value ? (85 + MaskOpacity.value * 0.15) / 100 : 0})`;
  
  return {
    backgroundImage: bgImage,
    '--global-blur': enableBackgroundBlur.value ? backgroundBlurAmount.value : 0,
    '--app-bg-overlay': maskColor
  };
});

// 检测系统主题
const checkSystemTheme = () => {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
}

// 根据主题模式更新 DarkMode 状态
const updateThemeMode = () => {
  if (ThemeMode.value === 'system') {
    DarkMode.value = checkSystemTheme()
  } else {
    DarkMode.value = ThemeMode.value === 'dark'
  }
}

// 监听系统主题变化
const setupSystemThemeListener = () => {
  if (window.matchMedia) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', () => {
      if (ThemeMode.value === 'system') {
        updateThemeMode()
      }
    })
  }
}

// 加载保存的背景图片和主题设置
onMounted(async () => {
  // 第一步：检测运行平台
  detectPlatform()
  console.log(`运行平台检测: Linux=${isLinux.value}, Windows=${isWindows.value}, macOS=${isMacOS.value}`)
  
  try {
    const savedPath = await invoke('get_background_image')
    if (savedPath && typeof savedPath === 'string') {
      backgroundImagePath.value = savedPath
      
      // Linux平台强制转换为Base64显示
      if (isLinux.value) {
        backgroundImageBase64.value = await convertBackgroundToBase64(savedPath)
        console.log('Linux环境: 背景图片已转换为Base64格式')
      }
    }
  } catch (err) {
    console.error('加载背景图片失败:', err)
  }
  
  // 加载保存的主题设置
  const savedThemeMode = localStorage.getItem('app-theme-mode') as 'light' | 'dark' | 'system'
  if (savedThemeMode) {
    ThemeMode.value = savedThemeMode
  }
  
  // 更新主题
  updateThemeMode()
  
  // 设置系统主题监听器
  setupSystemThemeListener()
})

// ================= 主题切换逻辑 =================
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

// 切换主题的函数
const toggleTheme = () => {
  // 切换 DarkMode 状态
  DarkMode.value = !DarkMode.value
  // 更新 ThemeMode 为对应的模式，确保设置页同步
  ThemeMode.value = DarkMode.value ? 'dark' : 'light'
  // 保存主题设置到本地存储
  localStorage.setItem('app-theme', DarkMode.value ? 'dark' : 'light')
  localStorage.setItem('app-theme-mode', ThemeMode.value)
  // 更新根元素的类
  const root = document.documentElement
  if (DarkMode.value) {
    root.classList.remove('light-mode')
    root.classList.add('dark-mode')
  } else {
    root.classList.remove('dark-mode')
    root.classList.add('light-mode')
  }
}

// 监听主题模式变化
watch(
  ThemeMode,
  () => {
    updateThemeMode()
    localStorage.setItem('app-theme-mode', ThemeMode.value)
  }
)

// ================= Naive UI 主题覆盖 =================
// 根据 DarkMode 动态生成覆盖变量，与全局 CSS 变量保持一致
const themeOverrides = computed<GlobalThemeOverrides>(() => {
  if (DarkMode.value) {
    return {
      common: {
        primaryColor: '#3d5afe',
        primaryColorHover: '#536dfe',
        primaryColorPressed: '#2a3eb1',
        primaryColorSuppl: '#3d5afe',
        // 其他颜色可以从 CSS 变量中读取，但这里直接映射简化示例
        bodyColor: 'transparent', // 背景透明，让背景图显示
        textColorBase: '#ffffff',
        textColor1: '#ffffff',
        textColor2: '#9ca3af',
        textColor3: '#6b7280',
        borderColor: '#3a3d47',
        borderRadius: '4px',
        boxShadow1: '0 2px 8px 0 rgba(0, 0, 0, 0.2)',
        // 输入框基础颜色
        inputColor: '#1e1f24',
        inputColorHover: '#25262b',
        inputColorFocus: '#1a1b1f',
        inputColorDisabled: '#2d2e33',
        // 可添加更多变量以覆盖其他组件
      },
      Button: {
        textColor: '#ffffff',
        textColorHover: '#ffffff',
        color: 'rgba(255, 255, 255, 0.05)',
        colorHover: 'rgba(255, 255, 255, 0.08)',
        border: '1px solid #3a3d47',
        borderHover: '1px solid #3d5afe',
      },
      Input: {
        // 输入框深色背景 - 暗色模式规范
        color: '#1e1f24',
        colorHover: '#25262b',
        colorFocus: '#1a1b1f',
        colorDisabled: '#2d2e33',
        // 边框颜色
        border: '1px solid #3a3d47',
        borderHover: '1px solid #4a4d57',
        borderFocus: '1px solid #3d5afe',
        // 文本颜色 - 高对比度
        textColor: '#ffffff',
        textColorPlaceholder: '#6b7280',
        // 图标颜色
        iconColor: '#9ca3af',
        iconColorHover: '#ffffff',
        // 清除按钮颜色
        clearColor: '#6b7280',
        clearColorHover: '#ffffff',
      },
    }
  } else {
    // 亮色模式
    return {
      common: {
        primaryColor: '#3d5afe',
        primaryColorHover: '#536dfe',
        primaryColorPressed: '#2a3eb1',
        primaryColorSuppl: '#3d5afe',
        bodyColor: 'transparent',
        textColorBase: '#1e293b',
        textColor1: '#1e293b',
        textColor2: '#64748b',
        textColor3: '#94a3b8',
        borderColor: '#e2e8f0',
        borderRadius: '4px',
        boxShadow1: '0 2px 8px 0 rgba(0, 0, 0, 0.05)',
      },
      Button: {
        textColor: '#1e293b',
        textColorHover: '#1e293b',
        color: 'rgba(255, 255, 255, 0.8)',
        colorHover: 'rgba(255, 255, 255, 0.9)',
        border: '1px solid #e2e8f0',
        borderHover: '1px solid #3d5afe',
      },
      Input: {
        // 亮色模式输入框 - 浅色背景
        color: 'rgba(255, 255, 255, 0.9)',
        colorHover: 'rgba(255, 255, 255, 0.95)',
        colorFocus: '#ffffff',
        colorDisabled: '#f1f5f9',
        // 边框颜色
        border: '1px solid #e2e8f0',
        borderHover: '1px solid #cbd5e1',
        borderFocus: '1px solid #3d5afe',
        // 文本颜色 - 高对比度
        textColor: '#1e293b',
        textColorPlaceholder: '#94a3b8',
        // 图标颜色
        iconColor: '#64748b',
        iconColorHover: '#1e293b',
        // 清除按钮颜色
        clearColor: '#94a3b8',
        clearColorHover: '#1e293b',
      },
    }
  }
})
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

/* 深色模式下顶部栏和底部栏样式 */
html.dark-mode .top-deck {
  background: rgba(15, 17, 21, var(--glass-bg-alpha, 1)) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

html.dark-mode .control-deck {
  background: rgba(15, 17, 21, var(--glass-bg-alpha, 1)) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

/* 浅色模式顶部栏和底部栏样式 */
html.light-mode .top-deck {
  background: rgba(255, 255, 255, var(--glass-bg-alpha, 1)) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

html.light-mode .control-deck {
  background: rgba(255, 255, 255, var(--glass-bg-alpha, 1)) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

/* 设置页设置项深色模式样式 */
html.dark-mode .setting-item {
  background: rgba(15, 17, 21, var(--glass-bg-alpha, 1)) !important;
  border: 1px solid rgba(255, 255, 255, 0.08) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

html.light-mode .setting-item {
  background: rgba(255, 255, 255, var(--glass-bg-alpha, 1)) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
  -webkit-backdrop-filter: blur(calc(var(--glass-enabled, 1) * var(--glass-blur, 0) * 1px)) saturate(calc(var(--glass-enabled, 1) * var(--glass-saturate, 180) * 1%));
}

html.dark-mode .setting-item:hover {
  background: rgba(15, 17, 21, var(--glass-bg-alpha, 1)) !important;
}

html.light-mode .setting-item:hover {
  background: rgba(255, 255, 255, var(--glass-bg-alpha, 1)) !important;
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
  background-size: cover;
  background-position: center;
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
  backdrop-filter: blur(calc(var(--global-blur) * 1px));
  -webkit-backdrop-filter: blur(calc(var(--global-blur) * 1px));
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