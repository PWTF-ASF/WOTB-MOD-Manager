<template>
  <div class="settings-panel" :class="{ 'glass-effect': glassEffect }">
    <div class="settings-panel-card">
      <div class="card-title">选择纯色背景：</div>
      <div class="card-content">
        <input type="color" v-model="color" @input="applyColor" />
        <!-- 预览当前颜色 -->
        <div class="color-preview" :style="{ backgroundColor: color }"></div>
      </div>
    </div>

    <div class="settings-panel-card">
      <div class="card-title">渐变背景：</div>
      <div class="card-content">
        <!-- 优化：让用户选择渐变方向和颜色 -->
        <div class="gradient-controls">
          <select v-model="gradientDirection" @change="updateGradient">
            <option value="to right">从左到右</option>
            <option value="to bottom">从上到下</option>
            <option value="135deg">对角线（135°）</option>
          </select>
          <input type="color" v-model="gradientColor1" @input="updateGradient" placeholder="颜色1" />
          <input type="color" v-model="gradientColor2" @input="updateGradient" placeholder="颜色2" />
        </div>
        <button @click="applyGradient" class="mt-2">应用渐变</button>
        <!-- 渐变预览 -->
        <div class="gradient-preview" :style="{ backgroundImage: gradient }"></div>
      </div>
    </div>

    <div class="settings-panel-card">
      <div class="card-title">
        <div class="row1">上传背景图片</div>
        <div class="row2" v-if="imagePath">模糊度：{{ blur }}px</div>
      </div>
      <div class="card-content">
        <input type="file" accept="image/*" @change="handleImageUpload" />
        <!-- 优化：用v-show避免DOM频繁销毁/创建，初始隐藏 -->
        <input v-show="imagePath" type="range" min="0" max="20" v-model="blur" @input="applyImage" />
      </div>
    </div>

    <div class="settings-panel-card">
      <div class="card-title">暗黑模式：</div>
      <div class="card-content">
        <div class="theme-switch-container">
          <label class="square-switch">
            <input type="checkbox" v-model="darkMode" class="sr-only" @change="handleThemeChange" />
            <span class="slider"></span>
          </label>
          <span class="theme-label">{{ darkMode ? '暗模式' : '明模式' }}</span>
        </div>
      </div>
    </div>

    <!-- 保存提示：增加过渡动画 -->
    <p v-if="saved" class="save-tip" @animationend="saved = false">✅ 设置已保存！</p>

    <!-- 对话框：优化样式和可访问性 -->
    <dialog ref="dialogRef" :class="{ 'tilt': isTilting }" aria-labelledby="dialogTitle">
      <div class="dialog-header">
        <h3 class="dialog-title" id="dialogTitle">确认重置</h3>
        <button class="close-btn" @click="closeDialog" @mouseenter="isTilting = true" @mouseleave="isTilting = false"
          aria-label="关闭对话框">
          ×
        </button>
      </div>
      <div class="dialog-content">
        <p>确定要重置所有背景和主题设置吗？重置后将恢复为「白色背景+系统默认主题」。</p>
      </div>
      <div class="dialog-footer">
        <button @click="closeDialog" class="secondary-btn">取消</button>
        <button @click="confirmReset" class="primary-btn">确认重置</button>
      </div>
    </dialog>
  </div>

  <!-- 操作按钮：增加样式区分主要/次要按钮 -->
  <div class="button-group">
    <button @click="saveSettings" class="primary-btn">💾 保存</button>
    <button @click="openDialog" class="secondary-btn">🔄 重置</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch ,defineProps} from 'vue'

// 1. 定义接口（严格类型约束）
interface BackgroundSettings {
  type: 'color' | 'image' | 'gradient'
  color?: string
  imagePath?: string
  blur?: number
  gradient?: string
}

// 2. 定义Emits（保持与父组件通信一致）
const emit = defineEmits<{
  (e: 'update-background', settings: BackgroundSettings): void
  (e: 'update:darkMode', darkMode: boolean): void
  (e: 'save-success', settings: BackgroundSettings): void
  (e: 'reset-success', settings: BackgroundSettings): void
}>()

const props = defineProps<{
  glassEffect: boolean; // 新增：父组件传来的毛玻璃状态
}>();

// 3. 响应式状态（初始化更合理的默认值）
const currentType = ref<BackgroundSettings['type']>('color')
const color = ref('#ffffff') // 默认白色
const imagePath = ref('')
const blur = ref(0)
const gradient = ref('linear-gradient(to right, #ff9a9e, #fad0c4)') // 默认渐变
// 新增：渐变自定义状态
const gradientDirection = ref('to right')
const gradientColor1 = ref('#ff9a9e')
const gradientColor2 = ref('#fad0c4')

const darkMode = ref(false)
const saved = ref(false)
const dialogRef = ref<HTMLDialogElement | null>(null)
const isTilting = ref(false)

// 4. 对话框相关逻辑
const openDialog = () => dialogRef.value?.showModal()
const closeDialog = () => {
  dialogRef.value?.close()
  isTilting.value = false // 关闭时重置倾斜状态
}

// 5. 纯色背景逻辑（增加即时预览）
const applyColor = () => {
  imagePath.value = ''
  gradient.value = ''
  currentType.value = 'color'
  emit('update-background', {
    type: 'color',
    color: color.value
  })
}

// 6. 渐变背景逻辑（支持用户自定义）
const updateGradient = () => {
  gradient.value = `linear-gradient(${gradientDirection.value}, ${gradientColor1.value}, ${gradientColor2.value})`
}
const applyGradient = () => {
  imagePath.value = ''
  currentType.value = 'gradient'
  emit('update-background', {
    type: 'gradient',
    gradient: gradient.value
  })
}

// 7. 图片上传逻辑（优化异步处理）
const handleImageUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = (e) => {
    const result = e.target?.result as string
    imagePath.value = result
    gradient.value = ''
    applyImage()
  }
  reader.onerror = () => {
    console.error('图片读取失败')
    alert('图片上传失败，请重试')
  }
  reader.readAsDataURL(file)
  // 清空input值，避免重复上传同一张图不触发change事件
  target.value = ''
}
const applyImage = () => {
  currentType.value = 'image'
  emit('update-background', {
    type: 'image',
    imagePath: imagePath.value,
    blur: blur.value
  })
}

// 8. 暗黑模式逻辑（优化持久化时机：仅在保存时存储）
const applyTheme = (mode: boolean) => {
  const root = document.documentElement
  root.classList.toggle('dark', mode)
  root.classList.toggle('light', !mode)
  // 仅通知父组件，不直接存储localStorage
  emit('update:darkMode', mode)
}
// 仅触发主题变更，不自动保存
const handleThemeChange = () => {
  applyTheme(darkMode.value)
}

// 9. 保存逻辑（统一存储背景+主题）
const saveSettings = () => {
  const settings: BackgroundSettings = {
    type: currentType.value,
    color: currentType.value === 'color' ? color.value : undefined,
    imagePath: currentType.value === 'image' ? imagePath.value : undefined,
    blur: currentType.value === 'image' ? blur.value : undefined,
    gradient: currentType.value === 'gradient' ? gradient.value : undefined
  }
  // 同时保存背景和主题到localStorage
  localStorage.setItem('userSettings', JSON.stringify(settings))
  localStorage.setItem('darkMode', JSON.stringify(darkMode.value))
  // 通知父组件更新背景
  emit('update-background', settings)
  emit('save-success', settings)
  // 保存提示
  saved.value = true
}

// 10. 重置逻辑（同步重置背景+主题）
const resetSettings = () => {
  // 重置背景状态
  color.value = '#ffffff'
  imagePath.value = ''
  blur.value = 0
  gradient.value = 'linear-gradient(to right, #ff9a9e, #fad0c4)'
  gradientDirection.value = 'to right'
  gradientColor1.value = '#ff9a9e'
  gradientColor2.value = '#fad0c4'
  currentType.value = 'color'

  // 重置主题状态（恢复为系统默认）
  const systemMode = window.matchMedia('(prefers-color-scheme: dark)').matches
  darkMode.value = systemMode
  applyTheme(systemMode)

  // 清空并重新存储默认设置
  const defaultSettings: BackgroundSettings = { type: 'color', color: '#ffffff' }
  localStorage.setItem('userSettings', JSON.stringify(defaultSettings))
  localStorage.setItem('darkMode', JSON.stringify(systemMode))

  // 通知父组件应用默认背景
  emit('update-background', defaultSettings)
  emit('reset-success', defaultSettings)

}
const confirmReset = () => {
  resetSettings()
  closeDialog()
}

// 11. 初始化逻辑（优化容错）
onMounted(() => {
  // 初始化主题
  const initTheme = () => {
    const savedMode = localStorage.getItem('darkMode')
    const mode = savedMode ? JSON.parse(savedMode) : window.matchMedia('(prefers-color-scheme: dark)').matches
    darkMode.value = mode
    applyTheme(mode)
  }

  // 初始化背景
  const initBackground = () => {
    const savedSettings = localStorage.getItem('userSettings')
    if (!savedSettings) return

    try {
      const settings = JSON.parse(savedSettings) as BackgroundSettings
      currentType.value = settings.type || 'color'
      if (currentType.value === 'color') color.value = settings.color || '#ffffff'
      if (currentType.value === 'image') {
        imagePath.value = settings.imagePath || ''
        blur.value = settings.blur || 0
      }
      if (currentType.value === 'gradient') gradient.value = settings.gradient || 'linear-gradient(to right, #ff9a9e, #fad0c4)'
      // 初始化时通知父组件应用保存的背景
      emit('update-background', settings)
    } catch (e) {
      console.error('加载背景设置失败', e)
      localStorage.removeItem('userSettings') // 清除损坏的设置
      alert('设置加载失败，已恢复默认值')
    }
  }

  initTheme()
  initBackground()
})

// 监听darkMode变化，自动应用主题（无需额外操作，handleThemeChange已处理）
watch(darkMode, applyTheme)
</script>

<style scoped>
.settings-panel {
  padding: 10px 20px 20px 20px;
  background-color: var(--panel-bg-no-filter);
  margin: 10px 20px 0px 20px;
  border-radius: 15px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(12px);
}

.settings-panel.glass-effect{
  background-color: var(--panel-bg);
  backdrop-filter: blur(12px);
}

.settings-panel-card {
  height: auto;
  margin-bottom: 20px;
}

.card-title {
  font-size: 18px;
  font-weight: bold;
  margin-bottom: 10px;
  display: flex;
  gap: 10px;
}

.card-content {
  align-items: center;
  display: flex;
  max-height: 55px;
  gap: 1rem;
  padding: 10px 20px;
  box-shadow: var(--card-shadow), var(--card-glow);
  border-radius: 5px;
  background-color: var(--card-bg);
}

.gradient-controls select {
  background-color: var(--card-bg);
  color: var(--text-color);
  border: var(--select-boder);
  /* 主边框色 */
  box-shadow: var(--select-box-shadow);
}

.gradient-controls select:focus {
  border-color: var(--select-focus-border);
  /* 聚焦时的边框色（略深） */
  box-shadow: var(--select-focus-box-shadow);
}

.button-group {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
  margin-left: 20px;
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

/* 保存提示动画 */
.save-tip {
  color: #52c41a;
  animation: fade 2s ease;
}

@keyframes fade {
  0% {
    opacity: 0;
  }

  20% {
    opacity: 1;
  }

  80% {
    opacity: 1;
  }

  100% {
    opacity: 0;
  }
}

/* 按钮样式 */
.theme-switch-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  /* 添加容器阴影增强整体层次 */
  padding: 0.25rem;
  border-radius: 8px;
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
  /* 添加轻微缩放效果增强交互感 */
  transition: transform 0.15s ease;
}

.square-switch:hover {
  transform: scale(1.02);
}

/* 方形滑块 */
.slider {
  position: absolute;
  inset: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 6px;
  /* 多层次阴影增强立体感 */
  box-shadow:
    inset 0 1px 2px rgba(0, 0, 0, 0.2),
    0 2px 3px rgba(0, 0, 0, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.15);
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
  transition: .4s cubic-bezier(0.34, 1.56, 0.64, 1);
  border-radius: 4px;
  /* 按钮立体效果 */
  box-shadow:
    0 1px 3px rgba(0, 0, 0, 0.25),
    inset 0 1px 1px rgba(255, 255, 255, 0.8);
  /* 添加内发光增强层次 */
  background-image: linear-gradient(135deg, rgba(255, 255, 255, 0.8) 0%, rgba(255, 255, 255, 0.2) 100%);
}

/* 选中状态 */
input:checked+.slider {
  background-color: #2c3e50;
  /* 激活状态下的阴影变化 */
  box-shadow:
    inset 0 1px 2px rgba(0, 0, 0, 0.15),
    0 2px 5px rgba(64, 158, 255, 0.25);
}

input:checked+.slider:before {
  transform: translateX(30px);
  /* 激活状态下按钮的细微变化 */
  box-shadow:
    0 1px 3px rgba(0, 0, 0, 0.15),
    inset 0 1px 1px rgba(255, 255, 255, 0.8);
}

/* 添加状态指示器增强层次感知 */
.slider:after {
  content: "";
  position: absolute;
  top: 50%;
  left: 20px;
  width: 4px;
  height: 4px;
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 50%;
  transform: translateY(-50%);
  transition: all 0.3s ease;
  opacity: 0;
}

input:checked+.slider:after {
  left: 46px;
  background-color: rgba(255, 255, 255, 0.6);
  opacity: 1;
}


/* 主题标签 */
.theme-label {
  font-size: 1rem;
  font-weight: 500;
  transition: color 0.3s ease;
  color: var(--text-color);
}

.primary-btn {
  padding: 8px 16px;
  background: #1677ff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.secondary-btn {
  padding: 8px 16px;
  background: #f5f5f5;
  color: #333;
  border-radius: 4px;
  cursor: pointer;
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
  gap: 10px;
  /* 右对齐按钮 */
}

/* 对话框背景遮罩 */
dialog::backdrop {
  background-color: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(2px);
  /* 遮罩模糊（可选，增强层次感） */
}
</style>