<template>
  <div class="settings-panel">
    <h2>背景设置</h2>

    <!-- 背景预览 -->
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

    <!-- 纯色选择 -->
    <div class="option-group">
      <label>选择纯色背景：</label>
      <input type="color" v-model="color" @input="applyColor" />
    </div>

    <div class="option-group">
      <label>渐变背景：</label>
      <button @click="applyGradient">应用渐变</button>
    </div>

    <!-- 图片上传 -->
    <div class="option-group">
      <label>上传背景图片：</label>
      <input type="file" accept="image/*" @change="handleImageUpload" />
    </div>

    <!-- 模糊度滑块 -->
    <div class="option-group" v-if="imagePath">
      <label>模糊度：{{ blur }}px</label>
      <input type="range" min="0" max="20" v-model="blur" @input="applyImage" />
    </div>

    <!-- 明暗切换器 -->
    <div class="option-group">
      <label>暗黑模式：</label>
      <input type="checkbox" v-model="darkMode" @change="toggleDarkMode" />
    </div>

    <!-- 操作按钮 -->
    <div class="button-group">
      <button @click="saveSettings">💾 保存</button>
      <button @click="resetSettings">🔄 重置</button>
      <button @click="goBack">🔙 返回</button>
    </div>

    <!-- 保存提示 -->
    <p v-if="saved" class="save-tip">✅ 设置已保存！</p>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const emit = defineEmits(['update-background'])

const color = ref('#ffffff')
const imagePath = ref('')
const blur = ref(0)
const saved = ref(false)

const darkMode = ref(false)

const previewStyle = computed(() => {
  return imagePath.value
    ? {
        backgroundImage: `url(${imagePath.value})`,
        backdropFilter: `blur(${blur.value}px)`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundColor: '',
      }
    : {
        backgroundColor: color.value,
      }
})

function applyTemplate(name) {
  const templates = {
    sunset: {
      type: 'image',
      imagePath: 'https://img.pconline.com.cn/images/upload/upc/tx/wallpaper/1305/16/c4/20990657_1368686545122.jpg',
      blur: 2,
    },
    forest: {
      type: 'image',
      imagePath: 'https://images.unsplash.com/photo-1506744038136-46273834b3fb',
      blur: 1,
    },
    galaxy: {
      type: 'image',
      imagePath: 'https://images.unsplash.com/photo-1581320540380-7f7c1f3c9a3c',
      blur: 3,
    },
  }

  const selected = templates[name]
  imagePath.value = selected.imagePath
  blur.value = selected.blur
  emit('update-background', selected)
}


function applyColor() {
  emit('update-background', { type: 'color', color: color.value })
}

function applyImage() {
  emit('update-background', {
    type: 'image',
    imagePath: imagePath.value,
    blur: blur.value,
  })
}

function applyGradient() {
  emit('update-background', {
    type: 'gradient',
    gradient: 'linear-gradient(135deg, #ff9a9e 0%, #fad0c4 100%)',
  })
}

function handleImageUpload(event) {
  const file = event.target.files[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = () => {
      imagePath.value = reader.result
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
  const settings = {
    type: imagePath.value ? 'image' : 'color',
    color: color.value,
    imagePath: imagePath.value,
    blur: blur.value,
  }
  localStorage.setItem('userSettings', JSON.stringify(settings))
  emit('update-background', settings)
  saved.value = true
  setTimeout(() => (saved.value = false), 2000)
}

function resetSettings() {
  color.value = '#ffffff'
  imagePath.value = ''
  blur.value = 0
  applyColor()
}
function goBack() {
  router.back()
}

onMounted(() => {
  const savedDark = localStorage.getItem('darkMode')
  if (savedDark) {
    darkMode.value = JSON.parse(savedDark)
    toggleDarkMode()
  }
})
</script>

<style scoped>
.settings-panel {
  padding: 1rem;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 8px;
  max-width: 400px;
  margin: auto;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
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
