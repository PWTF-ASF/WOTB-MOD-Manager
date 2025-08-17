<template>
  <div class="settings-page">
    <el-page-header content="背景设置" @back="handleBack" />

    <el-form label-position="top" class="settings-form">
      <!-- 明暗模式 -->
      <el-switch v-model="settings.mode" active-text="暗黑模式" inactive-text="明亮模式" />

      <!-- 背景类型选择 -->
      <el-radio-group v-model="settings.type">
        <el-radio label="color">纯色背景</el-radio>
        <el-radio label="image">图片背景</el-radio>
      </el-radio-group>

      <!-- 纯色背景 -->
      <el-color-picker v-if="settings.type === 'color'" v-model="settings.color" />

      <!-- 图片背景 -->
      <div v-if="settings.type === 'image'" class="image-settings">
        <el-upload action="" :auto-upload="false" :show-file-list="false" :on-change="handleImageUpload">
          <el-button>上传背景图片</el-button>
        </el-upload>

        <!-- 模糊度调节 -->
        <el-slider v-model="settings.blur" :min="0" :max="20" show-input />

      </div>

      <el-button type="primary" @click="saveSettings">保存设置</el-button>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from 'vue';
import { ElMessage } from 'element-plus';
import { useRouter } from 'vue-router';

const router = useRouter();

function handleBack() {
  router.back(); // 或 router.push('/') 返回首页
}

const settings = reactive({
  mode: 'light',
  type: 'color',
  color: '#ffffff',
  imagePath: '',
  blur: 5,
  crop: null,
});

function handleImageUpload(uploadFile: any) {
  const file = uploadFile.raw;
  const reader = new FileReader();
  reader.onload = () => {
    const base64 = reader.result as string;
    console.log('读取到的 base64:', base64);
    settings.imagePath = base64;
    console.log('设置后的 imagePath:', settings.imagePath);
  };
  reader.readAsDataURL(file);
  
  if (!file.type.startsWith('image/')) {
    ElMessage.error('请上传图片文件');
    return;
  }
}

function saveSettings() {

  // 保存到本地
  localStorage.setItem('userSettings', JSON.stringify(settings));

  // 应用背景
  applyBackground();

  localStorage.setItem('userSettings', JSON.stringify(settings));
  ElMessage.success('设置已保存');
}

function applyBackground() {
  const body = document.body;
  console.log('应用背景图：', settings.imagePath);
  if (settings.type === 'color') {
    body.style.backgroundImage = '';
    body.style.backgroundColor = settings.color;
    body.style.backdropFilter = '';
  } else if (settings.type === 'image') {
    if (!settings.imagePath) return;

    document.body.style.backgroundImage = `url("${settings.imagePath}")`;
    body.style.backgroundSize = 'cover';
    body.style.backgroundRepeat = 'no-repeat';
    body.style.backgroundPosition = 'center';
    body.style.backgroundColor = '';
    body.style.backdropFilter = `blur(${settings.blur}px)`;
  }
}

//页面加载时读取设置
const saved = localStorage.getItem('userSettings');
if (saved) {
  Object.assign(settings, JSON.parse(saved));
}

onMounted(() => {
  const saved = localStorage.getItem('userSettings');
  if (saved) {
    Object.assign(settings, JSON.parse(saved));
    applyBackground(); // ✅ 自动应用背景
  }
});
</script>

<style scoped>
.settings-page {
  padding: 2rem;
}

.image-settings {
  margin-top: 1rem;
}
</style>
