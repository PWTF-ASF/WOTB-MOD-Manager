<template>
  <div class="settingsPage" :class="{ 'dark-theme': darkMode }">
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
          <div class="settings-panel-card">
            <div class="card-title">
              <span>背景模板：</span>
            </div>
            <div class="card-content">
              <button @click="applyTemplate('sunset')">日落</button>
              <button @click="applyTemplate('forest')">森林</button>
              <button @click="applyTemplate('galaxy')">星空</button>
            </div>
          </div>

          <div class="settings-panel-card">
            <div class="card-title">
              选择纯色背景：
            </div>
            <div class="card-content">
              <input type="color" v-model="color" @input="applyColor" />
            </div>
          </div>
          <div class="settings-panel-card">
            <div class="card-title">
              渐变背景：
            </div>
            <div class="card-content">
              <button @click="applyGradient">应用渐变</button>
            </div>
          </div>
          <div class="settings-panel-card">
            <div class="card-title">
              <div class="row1">
                上传背景图片
              </div>
              <div class="row2">
                模糊度：{{ blur }}px
              </div>
            </div>
            <div class="card-content">
              <input type="file" accept="image/*" @change="handleImageUpload" />
              <input v-if="imagePath" type="range" min="0" max="20" v-model="blur" @input="applyImage" />
            </div>
          </div>

          <div class="settings-panel-card">
            <div class="card-title">
              暗黑模式：
            </div>
            <div class="card-content">
              <div class="theme-switch-container">
                <label class="square-switch">
                  <input type="checkbox" v-model="darkMode" class="sr-only" @change="toggleTheme">
                  <span class="slider"></span>
                </label>
                <span class="theme-label">{{ darkMode ? '暗模式' : '明模式' }}</span>
              </div>
            </div>
          </div>
          操作按钮
          <div class="button-group">
            <button @click="saveSettings">💾 保存</button>
            <button @click="openDialog">🔄 重置</button>
          </div>

          保存提示
          <p v-if="saved" class="save-tip">✅ 设置已保存！</p>
        </div>
      </main>
    </div>
  </div>

  <!-- 对话框：添加头部容器放置标题和X按钮 -->
  <dialog ref="dialogRef" :class="{ 'tilt': isTilting }">
    <!-- 对话框头部：控制标题和X按钮的布局 -->
    <div class="dialog-header">
      <h3 class="dialog-title">确认重置</h3>
      <!-- 右上角X关闭按钮 -->
      <button class="close-btn" @click="closeDialog" @mouseenter="isTilting = true" @mouseleave="isTilting = false"
        aria-label="关闭对话框">
        ×
      </button>
    </div>

    <!-- 对话框内容 -->
    <div class="dialog-content">
      <p>确定要重置所有背景设置吗？重置后将恢复为默认白色背景。</p>
    </div>

    <!-- 对话框底部：确认按钮 -->
    <div class="dialog-footer">
      <button @click="confirmReset" class="confirm-btn">确认重置</button>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'


// 定义背景设置的接口
interface BackgroundSettings {
  type: 'color' | 'image' | 'gradient'
  color?: string
  imagePath?: string
  blur?: number
  gradient?: string
}

// 定义模板类型接口
interface Template {
  imagePath: string;
  blur: number;
}

// 定义 emits 类型
const emit = defineEmits<{
  (e: 'update-background', settings: BackgroundSettings): void
}>()

const router = useRouter()
const currentType = ref<'color' | 'image' | 'gradient'>('color') // 限定类型为三种之一
const color = ref('black')
const imagePath = ref('')
const blur = ref(0)
const gradient = ref('')
const saved = ref(false)
const darkMode = ref(false)
const activeItem = ref('setting')
const dialogRef = ref<HTMLDialogElement | null>(null);  // 对话框 DOM 引用
const isTilting = ref(false); // 倾斜状态控制

// 打开对话框
const openDialog = () => {
  dialogRef.value?.showModal();
};

// 关闭对话框（重置倾斜状态）
const closeDialog = () => {
  dialogRef.value?.close();
  isTilting.value = false;
};

// 确认重置逻辑
const confirmReset = () => {
  resetSettings();  // 调用重置函数
  closeDialog();  // 可添加重置成功提示（如 Toast）
};

// 计算属性添加返回类型
const previewStyle = computed<Record<string, string | undefined>>(() => {
  if (imagePath.value) {
    return {
      backgroundImage: `url(${imagePath.value})`,
      backdropFilter: `blur(${blur.value}px)`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      // 移除空字符串赋值，改为不设置该属性或明确为undefined
    }
  } else if (gradient.value) {
    return {
      backgroundImage: gradient.value,
      // 同样移除backgroundColor的空字符串赋值
    }
  } else {
    return {
      backgroundColor: color.value,
    }
  }
})

// 应用模板函数添加参数类型
function applyTemplate(name: 'sunset' | 'forest' | 'galaxy') {
  // 明确指定模板对象的类型，与Template接口严格匹配
  const templates: Record<'sunset' | 'forest' | 'galaxy', Template> = {
    sunset: {
      imagePath: 'https://img.pconline.com.cn/images/upload/upc/tx/wallpaper/1305/16/c4/20990657_1368686545122.jpg',
      blur: 0,
    },
    forest: {
      imagePath: 'https://images.unsplash.com/photo-1506744038136-46273834b3fb',
      blur: 0,
    },
    galaxy: {
      imagePath: 'https://images.unsplash.com/photo-1581320540380-7f7c1f3c9a3c',
      blur: 0,
    },
  }

  const selected = templates[name];
  imagePath.value = selected.imagePath;
  blur.value = selected.blur;
  currentType.value = 'image';
  gradient.value = '';
  color.value = '';

  // 确保emit的参数完全符合BackgroundSettings接口
  emit('update-background', {
    type: 'image',
    imagePath: selected.imagePath,
    blur: selected.blur
  } as BackgroundSettings);
}


function applyColor() {
  imagePath.value = ''
  gradient.value = ''
  currentType.value = 'color'
  emit('update-background', {
    type: 'color',
    color: color.value
  })
}

function applyImage() {
  currentType.value = 'image'
  emit('update-background', {
    type: 'image',
    imagePath: imagePath.value,
    blur: blur.value,
  })
}

function applyGradient() {
  const gradientValue = 'linear-gradient(135deg, #ff9a9e 0%, #fad0c4 100%)'
  gradient.value = gradientValue

  currentType.value = 'gradient'
  imagePath.value = ''
  emit('update-background', {
    type: 'gradient',
    gradient: gradientValue,
  })
  console.log('子组件发送渐变设置：', gradientValue)
}

// 添加事件类型定义
function handleImageUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]

  if (file) {
    const reader = new FileReader()
    reader.onload = () => {
      imagePath.value = reader.result as string
      gradient.value = ''
      applyImage()
    }
    reader.readAsDataURL(file)
  }
}

function toggleDarkMode() {
  const root = document.documentElement
  if (darkMode.value) {
    root.classList.add('dark')
  } else {
    root.classList.remove('dark')
  }
  localStorage.setItem('darkMode', JSON.stringify(darkMode.value))
}

// 统一的主题应用函数
const applyTheme = () => {
  const root = document.documentElement;
  // 确保只存在当前主题的类名
  if (darkMode.value) {
    root.classList.add('dark');
    root.classList.remove('light');
  } else {
    root.classList.add('light');
    root.classList.remove('dark');
  }
};

// 统一的主题切换函数
const toggleTheme = () => {
  applyTheme();
  // 只使用一个键存储主题设置，避免冲突
  console.log(darkMode.value)
  localStorage.setItem('darkMode', JSON.stringify(darkMode.value));
};

function saveSettings() {
  const settings: BackgroundSettings = {
    type: currentType.value,
    color: color.value,
    imagePath: imagePath.value,
    blur: blur.value,
    gradient: gradient.value,
  }

  localStorage.setItem('userSettings', JSON.stringify(settings))
  emit('update-background', settings)
  saved.value = true
  setTimeout(() => (saved.value = false), 2000)
}

function resetSettings() {
  //重置本地状态
  color.value = '#ffffff';
  imagePath.value = '';
  blur.value = 0;
  gradient.value = '';
  currentType.value = 'color';
  localStorage.removeItem('userSettings');  //清除之前保存在本地的内容

  //强制触发父组件更新
  const resetSettings: BackgroundSettings = {
    type: 'color',
    color: color.value,
    imagePath: '',
    blur: 0,
    gradient: ''
  };

  localStorage.setItem('userSettings', JSON.stringify(resetSettings)) //重新赋值
  emit('update-background', resetSettings);
  console.log('重置成功：已恢复默认背景并立即应用');
}

function goBack() {
  router.back()
}

onMounted(() => {
  // 初始化主题设置
  const initTheme = () => {
    const savedMode = localStorage.getItem('darkMode');
    // 优先使用保存的设置，没有则根据系统偏好自动判断
    if (savedMode !== null) {
      darkMode.value = JSON.parse(savedMode);
    } else {
      // 自动检测系统主题偏好
      darkMode.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    applyTheme();
  };

  // 初始化背景设置
  const initBackground = () => {
    const savedSettings = localStorage.getItem('userSettings');
    if (savedSettings) {
      try {
        const settings = JSON.parse(savedSettings) as BackgroundSettings;
        currentType.value = settings.type || 'color';
        color.value = settings.color || '#ffffff';
        imagePath.value = settings.imagePath || '';
        blur.value = settings.blur || 0;
        gradient.value = settings.gradient || '';
      } catch (e) {
        console.error('加载背景设置失败', e);
        // 可以在这里添加默认背景设置的恢复逻辑
      }
    }
  };

  // 执行初始化
  initTheme();
  initBackground();
});

// 监听主题变化并自动应用
watch(darkMode, applyTheme);

</script>

<style scoped>

.dark-theme {
  --bg-color: #1e1e1e;
  --text-color: #f0f0f0;
  --card-bg: #2a2a2a;
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
  background-color: #f0f2f5;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  z-index: 10;
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
  background-color: #1677ff;
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
  background-color: #e6f0fa;
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

.settings-panel-card {
  height: auto;
  margin: 10px 20px;
}

.card-title {
  font-size: 18px;
  font-weight: bold;
  margin-bottom: 10px;
  display: flex;
  gap: 10px;
  /* color: var(--text-color); */
}

.card-content {
  align-items: center;
  display: flex;
  max-height: 55px;
  gap: 1rem;
  padding: 10px 20px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
  border-radius: 5px;
  background-color: var(--card-bg);
}

.preview {
  height: 100px;
  margin-bottom: 1rem;
  border: 1px solid #ccc;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #333;
  transition: all 0.5s ease;
}

.option-group {
  margin-bottom: 1rem;
  color: black;
}

.button-group {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

button {
  padding: 0.5rem 1rem;
  border: none;
  background-color: #4caf50;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

button:hover {
  background-color: #45a049;
}

.save-tip {
  margin-top: 1rem;
  color: green;
}

.theme-switch-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

/* 方形开关样式 */
.square-switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 30px;
  cursor: pointer;
}

/* 方形滑块 */
.slider {
  position: absolute;
  inset: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 6px;
  /* 小圆角实现方形效果 */
  box-shadow: inset 0 0 2px rgba(0, 0, 0, 0.2);
}

/* 方形按钮 */
.slider:before {
  position: absolute;
  content: "";
  height: 22px;
  width: 22px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 4px;
  /* 按钮也是方形 */
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

/* 选中状态（暗模式） */
input:checked+.slider {
  background-color: #2c3e50;
}

input:checked+.slider:before {
  transform: translateX(30px);
  /* 方形开关的平移距离 */
}

/* 主题标签 */
.theme-label {
  font-size: 1rem;
  font-weight: 500;
  transition: color 0.3s ease;
  /* 可添加默认颜色（可选，避免初始无样式） */
  color: #1e293b;
}

/* 1. 明确根元素 + 全局类，提高优先级 */
:global(html.dark) .theme-label {
  color: #f8fafc !important;
  /* !important 临时用于测试（确认后可移除） */
}

:global(html.light) .theme-label {
  color: #1e293b !important;
}

/* 对话框整体样式 */
dialog {
  width: 360px;
  padding: 0;
  /* 取消默认内边距，由内部容器控制 */
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  /* 水平垂直居中 */
  /* 倾斜过渡动画（核心） */
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  transform-origin: center center;
  /* 以中心为旋转原点 */
}

/*  向右倾斜（右下抬起，左上下沉） */
.tilt {
  transform: translate(-50%, -50%) perspective(1500px) rotateY(10deg);
}

/* 对话框头部：控制 X 按钮位置 */
.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e5e7eb;
}

/* 对话框标题 */
.dialog-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
}

/* 右上角 X 关闭按钮（核心样式） */
.close-btn {
  width: 32px;
  height: 32px;
  padding: 0;
  /* 取消内边距，确保圆形点击区 */
  border: none;
  border-radius: 10px;
  /* 圆形按钮 */
  background-color: transparent;
  color: #6b7280;
  /* X 符号大小 */
  font-size: 20px;
  /* 垂直居中 */
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

/* X 按钮 hover 效果（增强交互） */
.close-btn:hover {
  background-color: #f3f4f6;
  color: #ef4444;
  /*  hover 时变红 */
}

/* 对话框内容区 */
.dialog-content {
  padding: 16px;
  color: #4b5563;
  line-height: 1.5;
}

/* 对话框底部（确认按钮容器） */
.dialog-footer {
  padding: 12px 16px;
  border-top: 1px solid #e5e7eb;
  display: flex;
  justify-content: flex-end;
  /* 右对齐按钮 */
}

/* 确认按钮样式 */
.confirm-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background-color: #ef4444;
  color: white;
  cursor: pointer;
  transition: background-color 0.2s;
}

.confirm-btn:hover {
  background-color: #dc2626;
}

/* 对话框背景遮罩 */
dialog::backdrop {
  background-color: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(2px);
  /* 遮罩模糊（可选，增强层次感） */
}
</style>