<template>
  <div class="settingsPage" :class="{ 'dark-theme': isDark, 'light-theme': !isDark }">
    <div class="background"></div>
    <div class="content">

      <!-- 侧边栏 -->
      <aside>
        <span>设置</span>
        <div class="aside-card-list">
          <div class="aside-card-list-item" @click="activeItem = 'setting'"
            :class="{ 'active': activeItem === 'setting' }">
            <span class="indicator"></span>
            <img src="../assets/设置.svg" />
            <span>基本设置</span>
          </div>

          <div class="aside-card-list-item" @click="activeItem = 'personalize'"
            :class="{ 'active': activeItem === 'personalize' }">
            <span class="indicator"></span>
            <img src="../assets/279皮肤、个性化、主题-线性.svg" alt="个性化图标">
            <span>个性化</span>
          </div>
        </div>

        <!-- 返回按钮 -->
        <div class="back-btn" @click="goBack">返回</div>
      </aside>

      <main>
        <div class="settings-panel" v-if="activeItem === 'personalize'">
          <personalize 
          @update:darkMode="handleDarkModeUpdate" 
          @save-success="applyBackground" />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router'

const activeItem = ref('setting') //默认选择基本设置页
const router = useRouter() //路由
const isDark = ref(false);  // 父组件的主题状态：初始值可设为 false（默认浅色），后续由子组件同步
const handleDarkModeUpdate = (childDarkMode: boolean) => {  // 2. 接收子组件传递的 darkMode：更新父组件的 isDark
  isDark.value = childDarkMode; // 子组件的darkMode同步到父组件
};

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
  --aside-item-acitve-bg: #1e293b;
  --aside-item-acitve-color: #e2e8f0;
  --aside-item-hover-bg: #2d3748;
  --aside-item-hover-color: #f8fafc;
  --indicator-bg: #3b82f6;
}

/* 浅色模式 */
.light-theme {
  --bg-color: #ffffff;
  /* 白色背景 */
  --text-color: #333333;
  /* 深灰色文字 */
  --card-bg: #f5f5f5;
  /* 卡片浅灰背景 */
  --aside-item-acitve-bg: #e6f0fa;
  --aside-item-active-color: #333333;
  /* 侧边项激活状态文字 */
  --aside-item-hover-bg: #f0f2f5;
  /* 侧边项悬停背景 */
  --aside-item-hover-color: #333333;
  /* 侧边项悬停文字 */
  --indicator-bg: #1677ff;
  /* 指示器蓝色 */
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
  color: black;
  box-shadow: 10px 0 10px -5px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(20px);
  background-color: rgba(255, 255, 255, 0.123);
}

.aside-card-list {
  margin-top: 20px;
}

.aside-card-list-item {
  display: flex;
  width: 100%;
  height: 32px;
  align-items: center;
  gap: 5px;
  transition: all 0.2s ease;
  cursor: pointer;
  position: relative;
}

.aside-card-list-item:hover {
  border-radius: 5px;
  padding-left: 10px;
  background-color: var(--aside-item-hover-bg);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  z-index: 10;
  color: var(--aside-item-hover-color);
}

.aside-card-list-item>img {
  width: 24px;
  height: 24px;
  padding-left: 5px;
  transition: transform 0.2s ease;
}

/* 激活状态指示器 */
.indicator {
  width: 3px;
  height: 20px;
  background-color: var(--indicator-bg);
  border-radius: 2px;
  opacity: 0;
  /* 默认隐藏 */
  transition: opacity 0.2s ease;
}

/* 激活状态样式 */
.aside-card-list-item.active .indicator {
  opacity: 1;
}

.aside-card-list-item.active {
  border-radius: 5px;
  padding-left: 10px;
  background-color: var(--aside-item-acitve-bg);
  color: var(--aside-item-acitve-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  z-index: 10;
  font-weight: 500;
}

.aside-card-list-item:hover>img {
  transform: scale(1.05);
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