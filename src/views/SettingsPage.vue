<template>
  <div class="settingsPage">
    <div class="background"></div>
    <div class="content">
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

        <div class="back-btn" @click="goBack">返回</div>
      </aside>
      <main class="card">
        <div class="card-title"></div>
        <div class="card-content"></div>
      </main>
    </div>
  </div>
  <!-- <div class="settings-panel">
    <h2>背景设置</h2>

    背景预览
    <div class="preview" :style="previewStyle">
      <p>背景预览区域</p>
    </div>

    <div class="option-group">
      <label>背景模板：</label>
      <div class="template-buttons">
        <button @click="applyTemplate('sunset')">日落</button>
        <button @click="applyTemplate('forest')">森林</button>
        <button @click="applyTemplate('galaxy')">星空</button>
      </div>
    </div>

    纯色选择
    <div class="option-group">
      <label>选择纯色背景：</label>
      <input type="color" v-model="color" @input="applyColor" />
    </div>

    <div class="option-group">
      <label>渐变背景：</label>
      <button @click="applyGradient">应用渐变</button>
    </div>

    图片上传
    <div class="option-group">
      <label>上传背景图片：</label>
      <input type="file" accept="image/*" @change="handleImageUpload" />
    </div>

    模糊度滑块
    <div class="option-group" v-if="imagePath">
      <label>模糊度：{{ blur }}px</label>
      <input type="range" min="0" max="20" v-model="blur" @input="applyImage" />
    </div>

    明暗切换器
    <div class="option-group">
      <label>暗黑模式：</label>
      <input type="checkbox" v-model="darkMode" @change="toggleDarkMode" />
    </div>

    操作按钮
    <div class="button-group">
      <button @click="saveSettings">💾 保存</button>
      <button @click="resetSettings">🔄 重置</button>
      <button @click="goBack">🔙 返回</button>
    </div>

    保存提示
    <p v-if="saved" class="save-tip">✅ 设置已保存！</p>
  </div> -->
</template>

<script setup>
import { image } from '@tauri-apps/api'
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const emit = defineEmits(['update-background'])
const currentType = ref('color') // 默认是纯色
const color = ref('#ffffff')
const imagePath = ref('')
const blur = ref(0)
const gradient = ref('')
const saved = ref(false)
const darkMode = ref(false)
const activeItem = ref(''); //激活状态

const previewStyle = computed(() => {
  if (imagePath.value) {
    return {
      backgroundImage: `url(${imagePath.value})`,
      backdropFilter: `blur(${blur.value}px)`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundColor: '',
    }
  } else if (gradient.value) {
    return {
      backgroundImage: gradient.value,
      backgroundColor: '',
    }
  } else {
    return {
      backgroundColor: color.value,
    }
  }
})

function applyTemplate(name) {
  const templates = {
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

  const selected = templates[name]
  imagePath.value = selected.imagePath
  blur.value = selected.blur
  currentType.value = 'image' // 模板属于图片类型
  gradient.value = '' // 切换模板时，清空渐变值
  color.value = '' // 切换模板时，清空渐变值
  emit('update-background', selected)
}

function applyColor() {
  // 1. 重置其他背景类型的状态（关键：清空图片和渐变的残留值）
  imagePath.value = '' // 清空图片路径
  gradient.value = '' // 清空渐变值
  // 2. 更新当前背景类型为「纯色」
  currentType.value = 'color'
  // 3. 向父组件发送纯色设置事件
  emit('update-background', { type: 'color', color: color.value })
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

function handleImageUpload(event) {
  const file = event.target.files[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = () => {
      imagePath.value = reader.result
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

function saveSettings() {
  // 根据当前类型构建对应的设置
  const settings = {
    type: currentType.value, // 使用 currentType 明确类型
    color: color.value,
    imagePath: imagePath.value,
    blur: blur.value,
    gradient: gradient.value, // 新增：保存渐变值（如果之前定义了 gradient 变量）
  }

  localStorage.setItem('userSettings', JSON.stringify(settings))
  emit('update-background', settings) // 发送完整的设置
  saved.value = true
  setTimeout(() => (saved.value = false), 2000)
}

function resetSettings() {
  color.value = '#ffffff'
  imagePath.value = ''
  blur.value = 0
  gradient.value = ''
  applyColor()
}

function goBack() {
  router.back()
}

onMounted(() => {
  // 加载暗黑模式设置
  const savedDark = localStorage.getItem('darkMode')
  if (savedDark) {
    darkMode.value = JSON.parse(savedDark)
    toggleDarkMode()
  }

  // 新增：加载保存的背景设置
  const savedSettings = localStorage.getItem('userSettings')
  if (savedSettings) {
    try {
      const settings = JSON.parse(savedSettings)
      // 恢复所有状态到最后保存时的值
      currentType.value = settings.type || 'color'
      color.value = settings.color || '#ffffff'
      imagePath.value = settings.imagePath || ''
      blur.value = settings.blur || 0
      gradient.value = settings.gradient || ''
    } catch (e) {
      console.error('加载背景设置失败', e)
    }
  }
})
</script>

<style scoped>
.settingsPage {
  width: 100%;
  height: 100vh;
  overflow: hidden;
  position: relative;
}

.background {
  position: absolute;
  z-index: 0;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: RGB(240, 252, 255);
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

.settings-panel {
  padding: 1rem;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 8px;
  max-width: 400px;
  margin: auto;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
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
  /* 添加鼠标指针效果 */
  position: relative;
  /* 为指示器提供定位上下文 */
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
  bottom: 20px;
  cursor: pointer;
}

aside>.back-btn:hover {
  background-color: #409eff;
  color: white;
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
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
</style>
