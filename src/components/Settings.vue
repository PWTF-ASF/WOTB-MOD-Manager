<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <h2 class="page-title">
        系统设置
        <span class="sub-title">SYSTEM CONFIGURATION</span>
      </h2>

      <!-- 设置组：外观 -->
      <section class="setting-group">
        <h3 class="group-title">外观 / VISUALS</h3>

        <div class="setting-item">
          <div class="text-info">
            <span class="label">启用毛玻璃特效 (Acrylic Blur)</span>
            <span class="desc">开启后背景将呈现模糊透视效果，可能会轻微影响性能。</span>
          </div>
          <n-switch v-model:value="globalBlur" size="small" />
        </div>

        <!-- 新增：自定义背景 -->
        <div class="setting-item vertical">
          <div class="text-info">
            <span class="label">自定义背景图片</span>
            <span class="desc">支持 JPG、PNG、WEBP 等格式，推荐 1920x1080 以上分辨率。</span>
          </div>
          <div class="background-preview" v-if="backgroundPreviewUrl">
            <img :src="backgroundPreviewUrl" alt="背景预览" class="preview-thumb" />
            <n-button size="small" type="error" @click="handleRemoveBackground" :loading="updatingBg">
              移除
            </n-button>
          </div>
          <div class="bg-actions" v-else>
            <n-button size="small" secondary @click="handleSelectBackground" :loading="updatingBg">
              选择图片
            </n-button>
            <span class="desc-sm" style="margin-left: 12px;">未设置，使用默认背景</span>
          </div>
        </div>

        <!-- 新增：背景遮罩 -->
        <div class="setting-item">
          <div class="text-info">
            <span class="label">启用背景遮罩</span>
            <span class="desc">开启后将在背景上添加一层半透明遮罩，增强文字可读性。</span>
          </div>
          <n-switch v-model:value="backgroundMask" size="small" />
        </div>

        <!-- 新增：模糊度调节 -->
        <div class="setting-item vertical">
          <div class="text-info">
            <span class="label">背景模糊度</span>
            <span class="desc">调整背景的模糊程度，值越大模糊效果越明显。</span>
          </div>
          <div class="slider-container">
            <n-slider v-model:value="blurAmount" :min="0" :max="20" :step="1" size="small" :disabled="!globalBlur" />
            <span class="slider-value">{{ blurAmount }}px</span>
          </div>
        </div>

        <!-- 新增：遮罩透明度 -->
        <div class="setting-item vertical">
          <div class="text-info">
            <span class="label">遮罩透明度</span>
            <span class="desc">调整背景遮罩的透明度，值越大遮罩越不透明。</span>
          </div>
          <div class="slider-container">
            <n-slider v-model:value="maskOpacity" :min="0" :max="100" :step="1" size="small" :disabled="!backgroundMask" />
            <span class="slider-value">{{ maskOpacity }}%</span>
          </div>
        </div>

        <!-- 新增：主题模式 -->
        <div class="setting-item vertical">
          <div class="text-info">
            <span class="label">主题模式</span>
            <span class="desc">选择应用的主题模式，可跟随系统主题变化。</span>
          </div>
          <div class="theme-options">
            <n-radio-group v-model:value="themeMode" name="theme-mode">
              <n-radio value="light">浅色模式</n-radio>
              <n-radio value="dark">深色模式</n-radio>
              <n-radio value="system">跟随系统</n-radio>
            </n-radio-group>
          </div>
        </div>
      </section>

      <!-- 设置组：路径 -->
      <section class="setting-group">
        <h3 class="group-title">路径配置 / PATHS</h3>

        <!-- 游戏路径 -->
        <div class="setting-item vertical">
          <div class="label-row">
            <span class="label">游戏安装路径</span>
            <span class="desc-sm">World of Tanks Blitz 的主程序目录</span>
          </div>
          <div class="input-row">
            <n-input v-model:value="config.gamePath" readonly placeholder="例如: C:\Games\World_of_Tanks_Blitz"
              size="small" />
            <n-button size="small" secondary @click="selectPath('game')">
              浏览
            </n-button>
          </div>
        </div>

        <!-- MOD保存路径 -->
        <div class="setting-item vertical">
          <div class="label-row">
            <span class="label">Mod 存储库路径</span>
            <span class="desc-sm">下载的 Mod 文件将保存在此位置</span>
          </div>
          <div class="input-row">
            <n-input v-model:value="config.modRepoPath" readonly placeholder="选择文件夹..." size="small" />
            <n-button size="small" secondary @click="selectPath('mod')">
              浏览
            </n-button>
          </div>
        </div>
      </section>

      <!-- 底部操作区 -->
      <div class="action-footer">
        <n-button size="small" secondary @click="resetToDefaults">
          <template #icon>
            <n-icon :component="RefreshOutline" />
          </template>
          重置为默认设置
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, inject, onMounted, computed, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'
import {
  NButton,
  NSwitch,
  NInput,
  NIcon,
  NSlider,
  NRadio,
  NRadioGroup,
} from 'naive-ui'
import { RefreshOutline } from '@vicons/ionicons5'

// 注入全局状态
const globalBlur = inject('GlobalBlur') as Ref<boolean>
const blurAmount = inject('BlurAmount') as Ref<number>
const backgroundMask = inject('BackgroundMask') as Ref<boolean>
const maskOpacity = inject('MaskOpacity') as Ref<number>
const themeMode = inject('ThemeMode') as Ref<'light' | 'dark' | 'system'>
const backgroundImagePath = inject<Ref<string | null>>('backgroundImagePath')
const setBackgroundImage = inject<(path: string | null) => Promise<void>>('setBackgroundImage')

// 本地状态
const config = reactive({
  gamePath: '',
  modRepoPath: '',
})
const updatingBg = ref(false)

// 预览 URL
const backgroundPreviewUrl = computed(() => {
  if (backgroundImagePath?.value) {
    return convertFileSrc(backgroundImagePath.value)
  }
  return null
})

// 选择背景图片
const handleSelectBackground = async () => {
  const selected = await open({
    title: '选择背景图片',
    multiple: false,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'webp'] }]
  })
  if (!selected || Array.isArray(selected)) return

  updatingBg.value = true
  try {
    if (setBackgroundImage) {
      await setBackgroundImage(selected)
    }
  } catch (err) {
    console.error('设置背景失败:', err)
    alert(`设置背景失败: ${err}`)
  } finally {
    updatingBg.value = false
  }
}

// 移除背景
const handleRemoveBackground = async () => {
  updatingBg.value = true
  try {
    if (setBackgroundImage) {
      await setBackgroundImage(null)
    }
  } catch (err) {
    console.error('移除背景失败:', err)
    alert(`移除背景失败: ${err}`)
  } finally {
    updatingBg.value = false
  }
}

// 加载保存的路径（原有）
onMounted(async () => {
  try {
    const gamePath = await invoke('get_game_path')
    config.gamePath = (gamePath as string) || ''
  } catch (e) { console.error(e) }
  try {
    const modRepoPath = await invoke('get_mod_repo_path')
    config.modRepoPath = (modRepoPath as string) || ''
  } catch (e) { console.error(e) }
})

// 选择文件夹（原有）
const selectPath = async (type: 'game' | 'mod') => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: type === 'game' ? '选择游戏安装目录' : '选择Mod存储库目录',
  })
  if (selected && !Array.isArray(selected)) {
    if (type === 'game') {
      config.gamePath = selected
      await invoke('set_game_path', { path: selected })
    } else {
      config.modRepoPath = selected
      await invoke('set_mod_repo_path', { path: selected })
      if (confirm('是否将当前默认目录中的 Mod 文件移动到新位置？')) {
        await invoke('migrate_mod_repo', { newPath: selected })
      }
    }
  }
}

// 重置设置（原有）
const resetToDefaults = async () => {
  if (confirm('确定要重置所有设置吗？此操作无法撤销。')) {
    config.gamePath = ''
    config.modRepoPath = ''
    await invoke('set_game_path', { path: '' })
    await invoke('set_mod_repo_path', { path: '' })
    // 可选：也可重置背景，但这里不自动重置，以免意外
  }
}
</script>

<style scoped>

.background-preview {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 8px;
}

.preview-thumb {
  width: 120px;
  height: 68px;
  object-fit: cover;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--glass-effect);
}

.bg-actions {
  display: flex;
  align-items: center;
  margin-top: 8px;
}

/* 滑块容器 */
.slider-container {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 16px;
  margin-top: 8px;
}

.slider-container :deep(.n-slider) {
  flex: 1;
}

.slider-value {
  font-size: 13px;
  color: var(--text-dim);
  min-width: 50px;
  text-align: right;
  font-weight: 600;
}

/* 主题选项 */
.theme-options {
  margin-top: 8px;
  width: 100%;
}

.theme-options :deep(.n-radio-group) {
  display: flex;
  gap: 24px;
  flex-wrap: wrap;
}

.theme-options :deep(.n-radio) {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  transition: all 0.3s var(--animation-timing);
}

.theme-options :deep(.n-radio:hover) {
  color: var(--accent);
}

/* ================= 页面容器 ================= */
.settings-container {
  width: 100%;
  height: 100%;
  padding: 40px;
  box-sizing: border-box;
  overflow-y: auto;
  backdrop-filter: blur(var(--global-blur)) saturate(180%);
  -webkit-backdrop-filter: blur(var(--global-blur)) saturate(180%);
  color: var(--text-main);
  transition: all 0.3s var(--animation-timing);
  animation: page-slide-in 0.4s var(--animation-timing) backwards;
  position: relative;
}

.settings-container::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0.3;
}

.content-wrapper {
  max-width: 800px;
  margin: 0 auto;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.1s backwards;
}

/* ================= 标题样式 ================= */
.page-title {
  font-family: 'Rajdhani', sans-serif;
  font-size: 32px;
  font-weight: 800;
  text-transform: uppercase;
  border-bottom: 3px solid var(--accent);
  padding-bottom: 12px;
  margin-bottom: 40px;
  letter-spacing: 3px;
  color: var(--text-main);
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 12px;
}

.page-title::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border);
  margin-bottom: 5px;
  opacity: 0.3;
}

.sub-title {
  font-size: 14px;
  color: var(--text-dim);
  font-weight: 600;
  letter-spacing: 2px;
  margin-bottom: 8px;
  opacity: 0.8;
}

/* ================= 设置组 ================= */
.setting-group {
  margin-bottom: 50px;
  animation: slide-up-fade 0.5s var(--animation-timing) backwards;
}

.setting-group:nth-child(1) {
  animation-delay: 0.15s;
}

.setting-group:nth-child(2) {
  animation-delay: 0.2s;
}

.group-title {
  font-size: 14px;
  color: var(--accent);
  margin-bottom: 20px;
  letter-spacing: 1px;
  font-weight: 700;
  text-transform: uppercase;
  position: relative;
  padding-left: 12px;
  display: inline-block;
}

.group-title::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 16px;
  background: var(--accent);
  border-radius: 2px;
  box-shadow: 0 0 8px var(--accent-glow);
}

/* ================= 设置项 ================= */
.setting-item {
  background: var(--bg-card);
  border: 1px solid var(--border);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  padding: 24px;
  margin-bottom: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.3s var(--animation-timing);
  border-radius: 12px;
  position: relative;
  overflow: hidden;
  animation: fade-scale-in 0.4s var(--animation-timing) backwards;
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
}

.setting-item:nth-child(1) {
  animation-delay: 0.25s;
}

.setting-item:nth-child(2) {
  animation-delay: 0.3s;
}

.setting-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0;
  transition: opacity 0.3s;
}

.setting-item:hover {
  border-color: var(--accent);
  background: var(--bg-card-hover);
  box-shadow: 0 8px 24px var(--accent-glow);
  transform: translateX(4px);
}

.setting-item:hover::before {
  opacity: 1;
}

.setting-item.vertical {
  flex-direction: column;
  align-items: flex-start;
  gap: 16px;
}

/* 文本信息 */
.text-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.label {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-main);
  display: flex;
  align-items: center;
  gap: 8px;
}

.label::before {
  content: '•';
  color: var(--accent);
  font-size: 24px;
}

.desc {
  font-size: 13px;
  color: var(--text-dim);
  line-height: 1.5;
  max-width: 500px;
}

.label-row {
  display: flex;
  align-items: baseline;
  width: 100%;
  gap: 8px;
}

.desc-sm {
  font-size: 12px;
  color: var(--text-dim);
  opacity: 0.8;
}

/* 输入框和按钮行 */
.input-row {
  display: flex;
  width: 100%;
  gap: 12px;
}

/* 适配 Naive UI 组件在明暗模式下的样式 */
:deep(.n-input) {
  --n-border: var(--border);
  --n-border-hover: var(--accent);
  --n-border-focus: var(--accent);
  --n-color: var(--bg-input) !important;
  --n-color-focus: var(--bg-input-focus) !important;
  --n-text-color: var(--text-main) !important;
  --n-color-disabled: var(--bg-input) !important;
  background: var(--bg-input) !important;
  height: 32px !important;
}

:deep(.n-input:hover) {
  background: var(--bg-input-focus) !important;
}

:deep(.n-input:focus) {
  background: var(--bg-input-focus) !important;
}

:deep(.n-input.n-input--disabled) {
  background: var(--bg-input) !important;
  --n-color: var(--bg-input) !important;
}

:deep(.n-input__input) {
  background: transparent !important;
  color: var(--text-main) !important;
  height: 100% !important;
  display: flex !important;
  align-items: center !important;
}

:deep(.n-button) {
  --n-border: var(--border);
  --n-border-hover: var(--accent);
  --n-color: var(--glass-effect);
  --n-color-hover: var(--glass-effect-hover);
  --n-text-color: var(--text-main);
  --n-text-color-hover: var(--text-accent);
  height: 32px !important;
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

/* ================= 底部操作区 ================= */
.action-footer {
  margin-top: 60px;
  display: flex;
  justify-content: flex-end;
  border-top: 1px solid var(--border);
  padding-top: 24px;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.25s backwards;
}

/* ================= 动画 ================= */
@keyframes page-slide-in {
  from {
    opacity: 0;
    transform: translateX(20px);
  }

  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slide-up-fade {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fade-scale-in {
  from {
    opacity: 0;
    transform: scale(0.98);
  }

  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* ================= 滚动条美化 ================= */
.settings-container::-webkit-scrollbar {
  width: 8px;
}

.settings-container::-webkit-scrollbar-track {
  background: var(--scroll-track, transparent);
  border-radius: 4px;
  margin: 4px 0;
}

.settings-container::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 4px;
  transition: background 0.3s;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}

/* ================= 响应式设计 ================= */
@media (max-width: 900px) {
  .settings-container {
    padding: 30px;
  }

  .content-wrapper {
    max-width: 100%;
  }

  .page-title {
    font-size: 28px;
  }

  .setting-item {
    padding: 20px;
  }
}

@media (max-width: 768px) {
  .settings-container {
    padding: 20px;
  }

  .page-title {
    font-size: 24px;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .page-title::after {
    display: none;
  }

  .sub-title {
    font-size: 12px;
  }

  .label-row {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .desc-sm {
    margin-left: 0;
  }

  .input-row {
    flex-direction: column;
  }

  .action-footer {
    justify-content: center;
  }
}

@media (max-width: 480px) {
  .settings-container {
    padding: 16px;
  }

  .setting-item {
    padding: 16px;
  }

  .text-info {
    gap: 4px;
  }

  .label {
    font-size: 14px;
  }

  .desc {
    font-size: 12px;
  }
}

/* ================= 明暗模式特定的背景 ================= */
:global(.light-mode) .settings-container {
  background: rgba(255, 255, 255, 0.95);
}

:global(.light-mode) .setting-item {
  background: rgba(255, 255, 255, 0.9);
}

:global(.light-mode) .setting-item:hover {
  background: rgba(255, 255, 255, 0.95);
}
</style>