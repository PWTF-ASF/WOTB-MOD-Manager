<template>
  <div class="settingsPage" :class="{ 'dark-theme': isDark, 'light-theme': !isDark }">
    <div class="background"></div>
    <div class="content">

      <!-- 侧边栏 -->
      <aside>
        <span>设置</span>
        <div class="aside-card-list">
          <div class="aside-card-list-item" v-for="item in asideList" :key="item.activeItem"
            @click="activeItem = item.activeItem" :class="{ 'active': activeItem === item.activeItem }">
            <span class="indicator"></span>
            <img :src="getItemIcon(item)" />
            <span>{{ item.name }}</span>
          </div>
        </div>

        <!-- 返回按钮 -->
        <div class="back-btn" @click="goBack">返回</div>
      </aside>

      <main>
        <div class="settings-panel" v-if="activeItem === 'personalize'">
          <personalize @update:darkMode="handleDarkModeUpdate" @save-success="applyBackground" />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router'

import personalizeDarkIcon from '../assets/279皮肤、个性化、主题-线性 (1).svg';
import personalizeLightIcon from '../assets/279皮肤、个性化、主题-线性.svg';
import settingLightIcon from '../assets/设置.svg'
import settingDarkIcon from '../assets/设置 (1).svg'

//定义asidelist的接口
interface asideList {
  name: string;
  activeItem: string;
  lightIcon: string;
  darkIcon: string;
}

const activeItem = ref('setting') //默认选择基本设置页
const router = useRouter() //路由
const isDark = ref(false);  // 父组件的主题状态：初始值可设为 false（默认浅色），后续由子组件同步
const handleDarkModeUpdate = (childDarkMode: boolean) => {  // 2. 接收子组件传递的 darkMode：更新父组件的 isDark
  isDark.value = childDarkMode; // 子组件的darkMode同步到父组件
};

//动态切换明暗模式下的图标
const getItemIcon = (item: asideList) => {
  return isDark.value
    ? item.darkIcon  // 暗色主题用深色图标
    : item.lightIcon;  // 亮色主题用浅色图标
};

//侧边栏数组
const asideList = [
  {
    name: '基本设置',
    activeItem: 'setting',
    lightIcon: settingLightIcon,
    darkIcon: settingDarkIcon
  },
  {
    name: '个性化',
    activeItem: 'personalize',
    lightIcon: personalizeLightIcon,
    darkIcon: personalizeDarkIcon
  },
]

//返回到首页
function goBack() {
  router.back()
}

//更改背景色
function applyBackground(settings: any) {
  const background = document.querySelector('.background') as HTMLElement;
  if (!background) return;

  // 先清空所有背景相关样式
  background.style.backgroundImage = '';
  background.style.backgroundColor = '';
  background.style.backdropFilter = '';
  background.style.filter = '';
  background.style.backgroundSize = '';
  background.style.backgroundRepeat = '';
  background.style.backgroundPosition = '';

  switch (settings.type) {
    case 'color':
      // 兜底：如果子组件没传 color，默认用白色
      background.style.backgroundColor = settings.color || '#ffffff';
      break;
    case 'image':
      // 兜底：避免图片路径为空导致报错
      if (settings.imagePath) {
        background.style.backgroundImage = `url("${settings.imagePath}")`;
        background.style.backgroundSize = 'cover';
        background.style.backgroundRepeat = 'no-repeat';
        background.style.backgroundPosition = 'center';
        // 兜底：blur 为负数时设为 0
        background.style.filter = `blur(${Math.max(0, settings.blur || 0)}px)`;
      }
      break;
    case 'gradient':
      if (settings.gradient) {
        background.style.backgroundImage = settings.gradient;
        background.style.backgroundSize = 'cover';
      }
      break;
    // 兜底：默认切回白色背景
    default:
      background.style.backgroundColor = '#ffffff';
  }
}

onMounted(async () => {
  const saved = localStorage.getItem('userSettings');
  if (saved) {
    try {
      const settings = JSON.parse(saved);
      applyBackground(settings); // 直接调用处理所有类型的 applyBackground
    } catch (e) {
      console.error('背景设置解析失败', e);
    }
  }
})

</script>

<style scoped>
/* 暗色模式 */
.dark-theme {
  --bg-color: #1e1e1e;
  --text-color: #f0f0f0;
  --card-bg: #2a2a2a;
  --aside-bg: rgba(30, 30, 30, 0.7);
  /* 半透明背景增强毛玻璃效果 */
  --aside-item-active-bg: rgba(30, 41, 59, 0.8);
  --aside-item-active-color: #e2e8f0;
  --aside-item-hover-bg: rgba(45, 55, 72, 0.6);
  --aside-item-hover-color: #f8fafc;
  --indicator-bg: #3b82f6;
  --icon-filter: brightness(0.9);
  --icon-hover-filter: brightness(1);
  --icon-hover-drop-shadow: drop-shadow(0 0 10px rgba(221, 245, 255, 0.8))
}

/* 浅色模式 */
.light-theme {
  --bg-color: #ffffff;
  --text-color: #333333;
  --card-bg: #f5f5f5;
  --aside-bg: rgba(255, 255, 255, 0.4);
  /* 更透明的背景增强毛玻璃效果 */
  --aside-item-active-bg: rgba(230, 240, 250, 0.85);
  --aside-item-active-color: #333333;
  --aside-item-hover-bg: rgba(240, 242, 245, 0.7);
  --aside-item-hover-color: #333333;
  --indicator-bg: #1677ff;
  --icon-filter: brightness(0.7);
  --icon-hover-filter: brightness(0.6);
  --icon-hover-drop-shadow: drop-shadow(0 0 8px rgba(38, 41, 42, 0.8))
}

.settingsPage {
  width: 100%;
  height: 100vh;
  overflow: hidden;
  position: relative;
  background-color: white;
  color: var(--text-color);
}

.background {
  position: absolute;
  z-index: 0;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: var(--bg-color);
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
  transition:
    background 0.3s ease,
    backdrop-filter 0.3s ease;
}

.content {
  position: relative;
  z-index: 1;
  width: 800px;
  height: 600px;
  display: flex;
  overflow: hidden;
}

aside {
  width: 30%;
  height: 100%;
  padding: 10px 20px;
  color: var(--text-color);
  box-shadow: 10px 0 20px -10px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  /* 兼容Safari */
  background-color: var(--aside-bg);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
  /* 增加细微边框增强层次感 */
  transition: all 0.3s ease;
  /* 平滑过渡效果 */
}

.aside-card-list {
  margin-top: 20px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  /* 增加项目间距，提升可读性 */
}

.aside-card-list-item {
  display: flex;
  width: 100%;
  height: 36px;
  /* 略微增加高度提升点击体验 */
  align-items: center;
  gap: 10px;
  /* 增加图标与文字间距 */
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  /* 更自然的过渡曲线 */
  cursor: pointer;
  position: relative;
  padding: 0 8px;
  /* 增加内边距 */
  border-radius: 6px;
  /* 圆角 */
}

.aside-card-list-item:hover {
  padding-left: 10px;
  background-color: var(--aside-item-hover-bg);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 10;
  color: var(--aside-item-hover-color);
  transform: translateX(2px);
  /* 微小位移增强交互感 */
}

.aside-card-list-item>img {
  width: 24px;
  height: 24px;
  transition: transform 0.2s ease, filter 0.2s ease;
  filter: var(--icon-filter);
}

/* 激活状态指示器 */
.indicator {
  width: 3px;
  height: 20px;
  background-color: var(--indicator-bg);
  border-radius: 2px;
  opacity: 0;
  transition: opacity 0.2s ease, transform 0.2s ease;
  transform: scaleY(0.8);
  /* 初始略小 */
}

/* 激活状态样式 */
.aside-card-list-item.active .indicator {
  opacity: 1;
  transform: scaleY(1);
  /* 激活时恢复正常大小 */
}

.aside-card-list-item.active {
  padding-left: 10px;
  background-color: var(--aside-item-active-bg);
  color: var(--aside-item-active-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 10;
  font-weight: 500;
}

.aside-card-list-item:hover>img {
  transform: scale(1.1);
  filter: var(--icon-hover-filter);
  /* 悬停时图标效果变化 */
  filter: var(--icon-hover-drop-shadow);
}

aside>.back-btn {
  width: 130px;
  height: 32px;
  border: 2px solid #409eff;
  text-align: center;
  line-height: 30px;
  transition: all 0.3s ease;
  position: absolute;
  bottom: 10px;
  cursor: pointer;
}

aside>.back-btn:hover {
  background-color: #66b1ff;
  color: white;
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

main {
  width: 100%;
  overflow-y: auto;
}
</style>