<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <h2 class="page-title">
        系统设置
        <span class="sub-title">SYSTEM CONFIGURATION</span>
      </h2>

      <!-- 设置组：背景模糊效果 -->
      <section class="setting-group">
        <h3 class="group-title">背景模糊 / BACKGROUND BLUR</h3>

        <div class="setting-item vertical background-blur-item">
          <div class="setting-header">
            <div class="text-info">
              <span class="label">背景模糊强度</span>
              <span class="desc">单一滑块控制背景整体模糊程度，开启后营造深度层次感。</span>
            </div>
            <n-switch v-model:value="enableBackgroundBlur" size="small" />
          </div>
          <div class="slider-container blur-slider">
            <n-slider
              v-model:value="backgroundBlurAmount"
              :min="0"
              :max="20"
              :step="1"
              size="small"
              :disabled="!enableBackgroundBlur"
            />
            <span class="slider-value">{{ backgroundBlurAmount }}px</span>
          </div>
        </div>
      </section>

      <!-- 设置组：毛玻璃效果 -->
      <section class="setting-group">
        <h3 class="group-title">毛玻璃效果 / GLASS EFFECT</h3>

        <div class="setting-item">
          <div class="text-info">
            <span class="label">启用毛玻璃效果</span>
            <span class="desc">为界面元素添加BewlyCat风格半透明磨砂质感，营造通透轻盈的视觉层次。</span>
          </div>
          <n-switch v-model:value="enableGlassEffect" size="small" />
        </div>

        <div class="setting-item vertical glass-intensity-item">
          <div class="text-info">
            <span class="label">效果强度</span>
            <span class="desc">单一滑块控制毛玻璃整体通透度，值越大磨砂质感越强烈。</span>
          </div>
          <div class="intensity-slider-wrapper">
            <div class="slider-track">
              <n-slider
                v-model:value="glassIntensity"
                :min="0"
                :max="100"
                :step="1"
                size="small"
                :disabled="!enableGlassEffect"
              />
            </div>
            <div class="intensity-labels">
              <span class="label-min">通透</span>
              <span class="label-max">磨砂</span>
            </div>
          </div>
        </div>
      </section>

      <!-- 设置组：其他外观设置 -->
      <section class="setting-group">
        <h3 class="group-title">其他外观 / GENERAL VISUALS</h3>

        <div class="setting-item vertical background-setting-item">
          <div class="text-info">
            <span class="label">背景设置</span>
            <span class="desc">自定义背景图片与遮罩透明度调节，增强界面视觉层次感。</span>
          </div>

          <div class="background-control-row">
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
              <span class="desc-sm">未设置，使用默认背景</span>
            </div>
          </div>

          <div class="mask-toggle-row">
            <span class="toggle-label">启用背景遮罩</span>
            <n-switch v-model:value="backgroundMask" size="small" />
          </div>

          <div class="slider-container mask-slider">
            <span class="slider-label">遮罩透明度</span>
            <n-slider
              v-model:value="maskOpacity"
              :min="0"
              :max="100"
              :step="1"
              size="small"
              :disabled="!backgroundMask"
            />
            <span class="slider-value">{{ maskOpacity }}%</span>
          </div>
        </div>

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
            <n-input
              v-model:value="config.gamePath"
              readonly
              placeholder="例如: C:\Games\World_of_Tanks_Blitz"
              size="small"
            />
            <n-button size="small" secondary @click="selectPath('game')"> 浏览 </n-button>
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
            <n-button size="small" secondary @click="selectPath('mod')"> 浏览 </n-button>
          </div>
        </div>
      </section>

      <!-- 底部操作区 -->
      <div class="action-footer">
        <n-button size="small" secondary @click="handleResetVisuals">
          <template #icon>
            <n-icon :component="RefreshOutline" />
          </template>
          重置视觉效果
        </n-button>
        <n-button size="small" secondary @click="resetToDefaults"> 重置所有设置 </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, inject, onMounted, computed, watch, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { convertFileSrc } from '@tauri-apps/api/core'
import { NButton, NSwitch, NInput, NIcon, NSlider, NRadio, NRadioGroup } from 'naive-ui'
import { RefreshOutline } from '@vicons/ionicons5'

// 注入全局状态 - 背景模糊效果
const enableBackgroundBlur = inject('enableBackgroundBlur') as Ref<boolean>
const backgroundBlurAmount = inject('backgroundBlurAmount') as Ref<number>

// 注入全局状态 - 毛玻璃效果 (BewlyCat单一强度控制)
const enableGlassEffect = inject('enableGlassEffect') as Ref<boolean>
const glassIntensity = inject('glassIntensity') as Ref<number>

// 注入平台信息 (Linux适配)
const isLinux = inject('isLinux') as Ref<boolean>

// 注入全局状态 - 其他外观设置
const backgroundMask = inject('BackgroundMask') as Ref<boolean>
const maskOpacity = inject('MaskOpacity') as Ref<number>
const themeMode = inject('ThemeMode') as Ref<'light' | 'dark' | 'system'>
const backgroundImagePath = inject<Ref<string | null>>('backgroundImagePath')
const setBackgroundImage = inject<(path: string | null) => Promise<void>>('setBackgroundImage')
const resetVisualSettings = inject('resetVisualSettings') as () => void

// 本地状态
const config = reactive({
  gamePath: '',
  modRepoPath: '',
})
const updatingBg = ref(false)
const thumbBase64 = ref<string | null>(null)

// 背景图片缩略图Base64转换 (Linux适配)
const convertThumbToBase64 = async (path: string): Promise<string | null> => {
  if (!path) return null
  try {
    const base64 = (await invoke('read_image_base64', { path })) as string
    const ext = path.toLowerCase().split('.').pop()
    let mimeType = 'image/jpeg'
    if (ext === 'png') mimeType = 'image/png'
    if (ext === 'webp') mimeType = 'image/webp'
    return `data:${mimeType};base64,${base64}`
  } catch (err) {
    console.error('转换缩略图Base64失败:', err)
    return null
  }
}

// 预览 URL - 根据平台自动选择显示方式
const backgroundPreviewUrl = computed(() => {
  if (backgroundImagePath?.value) {
    // Linux强制使用Base64 (解决Tauri Linux下asset协议问题)
    if (isLinux.value) {
      return thumbBase64.value
    }
    // Windows/macOS 使用原生方式
    return convertFileSrc(backgroundImagePath.value)
  }
  return null
})

// 监听背景图片变化，自动更新Base64 (Linux环境)
watch(
  () => backgroundImagePath?.value,
  async newPath => {
    if (isLinux.value && newPath) {
      thumbBase64.value = await convertThumbToBase64(newPath)
    }
  },
  { immediate: true }
)

// 选择背景图片
const handleSelectBackground = async () => {
  const selected = await open({
    title: '选择背景图片',
    multiple: false,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
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
  } catch (e) {
    console.error(e)
  }
  try {
    const modRepoPath = await invoke('get_mod_repo_path')
    config.modRepoPath = (modRepoPath as string) || ''
  } catch (e) {
    console.error(e)
  }
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

// 重置视觉效果
const handleResetVisuals = () => {
  if (confirm('确定要重置所有视觉效果为默认值吗？')) {
    if (resetVisualSettings) {
      resetVisualSettings()
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
    // 同时重置视觉效果
    if (resetVisualSettings) {
      resetVisualSettings()
    }
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

/* BewlyCat风格 - 毛玻璃强度滑块样式 */
.intensity-slider-wrapper {
  width: 100%;
  margin-top: 12px;
}

.slider-track {
  width: 100%;
  padding: 8px 0;
}

.slider-track :deep(.n-slider) {
  --n-rail-color: linear-gradient(90deg, rgba(61, 90, 254, 0.2) 0%, rgba(61, 90, 254, 0.8) 100%);
  --n-fill-color: rgba(61, 90, 254, 0.9);
  --n-fill-color-hover: rgba(61, 90, 254, 1);
}

.slider-track :deep(.n-slider-handle) {
  background: white;
  border: 2px solid rgba(61, 90, 254, 0.8);
  box-shadow:
    0 2px 8px rgba(61, 90, 254, 0.3),
    0 0 12px rgba(61, 90, 254, 0.2);
  transition:
    transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    background 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.slider-track :deep(.n-slider-handle:hover) {
  transform: scale(1.2);
  box-shadow:
    0 4px 16px rgba(61, 90, 254, 0.4),
    0 0 20px rgba(61, 90, 254, 0.3);
}

.intensity-labels {
  display: flex;
  justify-content: space-between;
  margin-top: 4px;
  padding: 0 4px;
}

.intensity-labels span {
  font-size: 11px;
  color: var(--text-dim);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}

.intensity-labels .label-min {
  color: var(--accent);
  opacity: 0.7;
}

.intensity-labels .label-max {
  color: var(--accent);
  opacity: 0.9;
}

.glass-intensity-item {
  transition:
    transform 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    background 0.4s ease,
    border-color 0.4s ease;
}

.glass-intensity-item:hover {
  transform: translateX(6px);
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
  transition:
    transform 0.3s var(--animation-timing),
    opacity 0.3s var(--animation-timing),
    color 0.3s var(--animation-timing);
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
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  color: var(--text-main);
  transition:
    background 0.3s var(--animation-timing),
    color 0.3s var(--animation-timing);
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
  background: rgba(15, 17, 21, 0.75);
  border: 1px solid rgba(255, 255, 255, 0.12);
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.15),
    0 0 0 1px rgba(255, 255, 255, 0.05) inset;
  padding: 24px;
  margin-bottom: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition:
    transform 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    background 0.4s ease,
    border-color 0.4s ease;
  border-radius: 16px;
  position: relative;
  overflow: hidden;
  animation: fade-scale-in 0.4s var(--animation-timing) backwards;
}

.setting-item > * {
  position: relative;
  z-index: 10;
}

.setting-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(ellipse at 15% -10%, rgba(255, 255, 255, 0.18) 0%, transparent 55%);
  pointer-events: none;
  z-index: 1;
  transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.setting-item::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.35) 15%,
    rgba(255, 255, 255, 0.35) 85%,
    transparent 100%
  );
  pointer-events: none;
  z-index: 2;
  border-radius: 16px 16px 0 0;
}

.setting-item:nth-child(1) {
  animation-delay: 0.25s;
}

.setting-item:nth-child(2) {
  animation-delay: 0.3s;
}

.setting-item:nth-child(3) {
  animation-delay: 0.35s;
}

.setting-item:nth-child(4) {
  animation-delay: 0.4s;
}

.setting-item:nth-child(5) {
  animation-delay: 0.45s;
}

.setting-item:hover {
  border-color: rgba(61, 90, 254, 0.7);
  box-shadow:
    0 16px 48px rgba(61, 90, 254, 0.2),
    0 0 0 1px rgba(61, 90, 254, 0.4) inset;
  transform: translateX(6px);
}

:global(html.dark-mode) .setting-item:hover {
  background: rgba(18, 22, 29, 0.88) !important;
}

:global(html.light-mode) .setting-item:hover {
  background: rgba(255, 255, 255, 0.82) !important;
  box-shadow:
    0 16px 48px rgba(61, 90, 254, 0.12),
    0 0 0 1px rgba(61, 90, 254, 0.6) inset;
}

:global(html.dark-mode) .setting-item {
  background: rgba(15, 17, 21, 0.78) !important;
  border: 1px solid rgba(255, 255, 255, 0.14) !important;
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.18),
    0 0 0 1px rgba(255, 255, 255, 0.06) inset !important;
}

:global(html.light-mode) .setting-item {
  background: rgba(255, 255, 255, 0.75) !important;
  border: 1px solid rgba(255, 255, 255, 0.9) !important;
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.06),
    0 0 0 1px rgba(255, 255, 255, 0.95) inset,
    0 2px 8px rgba(0, 0, 0, 0.04) !important;
}

:global(html.light-mode) .setting-item::before {
  background: radial-gradient(ellipse at 15% -10%, rgba(255, 255, 255, 0.7) 0%, transparent 55%);
}

:global(html.light-mode) .setting-item::after {
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.85) 15%,
    rgba(255, 255, 255, 0.85) 85%,
    transparent 100%
  );
}

.setting-item.vertical {
  flex-direction: column;
  align-items: flex-start;
  gap: 16px;
}

.setting-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.background-blur-item {
  transition:
    transform 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    background 0.4s ease,
    border-color 0.4s ease;
}

.background-blur-item:hover {
  transform: translateX(6px);
}

.background-setting-item {
  transition:
    transform 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    box-shadow 0.4s cubic-bezier(0.4, 0, 0.2, 1),
    background 0.4s ease,
    border-color 0.4s ease;
}

.background-setting-item:hover {
  transform: translateX(6px);
}

.background-control-row {
  display: flex;
  align-items: center;
  gap: 16px;
  width: 100%;
}

.mask-toggle-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 8px 0;
}

.mask-slider {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.slider-label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
  min-width: 80px;
}

.toggle-label {
  font-size: 13px;
  color: var(--text-primary);
  font-weight: 500;
}

.desc-sm {
  font-size: 12px;
  color: var(--text-dim);
  margin-left: 12px;
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
  --n-color: var(--bg-input);
  --n-color-focus: var(--bg-input-focus);
  --n-text-color: var(--text-main);
}

:deep(.n-button) {
  --n-border: var(--border);
  --n-border-hover: var(--accent);
  --n-color: var(--glass-effect);
  --n-color-hover: var(--glass-effect-hover);
  --n-text-color: var(--text-main);
  --n-text-color-hover: var(--text-accent);
}

/* ================= 底部操作区 ================= */
.action-footer {
  margin-top: 60px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
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
