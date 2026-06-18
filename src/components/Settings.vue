<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <h2 class="page-title">
        系统设置
        <span class="sub-title">SYSTEM CONFIGURATION</span>
      </h2>

      <!-- 设置组：外观设置 -->
      <section class="setting-group">
        <h3 class="group-title">外观设置 / APPEARANCE</h3>

        <!-- 背景图片 -->
        <div class="setting-item vertical">
          <div class="setting-header">
            <div class="text-info">
              <span class="label">背景图片</span>
              <span class="desc">自定义背景图片与显示模式，增强界面视觉层次感。</span>
            </div>
          </div>

          <div class="background-control-row">
            <div class="background-preview" v-if="backgroundPreviewUrl">
              <img :src="backgroundPreviewUrl" alt="背景预览" class="preview-thumb" />
              <div class="bg-btn-group">
                <n-button size="small" secondary @click="handleSelectBackground" :loading="updatingBg">
                  更换
                </n-button>
                <n-button size="small" type="error" @click="handleRemoveBackground" :loading="updatingBg">
                  移除
                </n-button>
              </div>
            </div>
            <div class="bg-actions" v-else>
              <n-button size="small" secondary @click="handleSelectBackground" :loading="updatingBg">
                选择图片
              </n-button>
              <span class="desc-sm">未设置，使用主题默认渐变背景</span>
            </div>
          </div>

          <!-- 背景模式 -->
          <div class="mode-selector-row">
            <span class="toggle-label">显示模式</span>
            <div class="mode-buttons">
              <button
                v-for="mode in backgroundModes" :key="mode.value"
                class="neu-mode-btn"
                :class="{ active: backgroundMode === mode.value }"
                @click="backgroundMode = mode.value"
              >
                {{ mode.label }}
              </button>
            </div>
          </div>

          <!-- 背景遮罩 -->
          <div class="mask-toggle-row">
            <span class="toggle-label">背景遮罩</span>
            <n-switch v-model:value="backgroundMask" size="small" />
          </div>

          <div class="slider-container">
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

          <!-- 背景模糊 -->
          <div class="mask-toggle-row">
            <span class="toggle-label">背景模糊</span>
            <n-switch v-model:value="enableBackgroundBlur" size="small" />
          </div>

          <div class="slider-container">
            <span class="slider-label">模糊强度</span>
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

          <div class="reset-bg-row">
            <n-button size="tiny" quaternary @click="handleResetBackground">
              恢复默认渐变背景
            </n-button>
          </div>
        </div>

        <!-- 主题模式 -->
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
import { useNotify, useConfirm } from '@/composables/useNotification'

// 注入全局状态 - 背景模糊效果
const enableBackgroundBlur = inject('enableBackgroundBlur') as Ref<boolean>
const backgroundBlurAmount = inject('backgroundBlurAmount') as Ref<number>

// 注入平台信息 (Linux适配)
const isLinux = inject('isLinux') as Ref<boolean>

// 注入全局状态 - 其他外观设置
const backgroundMask = inject('BackgroundMask') as Ref<boolean>
const maskOpacity = inject('MaskOpacity') as Ref<number>
const themeMode = inject('ThemeMode') as Ref<'light' | 'dark' | 'system'>
const backgroundImagePath = inject<Ref<string | null>>('backgroundImagePath')
const setBackgroundImage = inject<(path: string | null) => Promise<void>>('setBackgroundImage')
const resetVisualSettings = inject('resetVisualSettings') as () => void
const invalidateModList = inject<() => void>('invalidateModList', () => {})

// 通知 + 确认对话框
const notify = useNotify()
const { confirm } = useConfirm()

// 背景模式
const backgroundModes = [
  { value: 'cover', label: '铺满' },
  { value: 'contain', label: '适应' },
  { value: 'fill', label: '拉伸' },
  { value: 'tile', label: '平铺' },
]
const backgroundMode = inject('backgroundMode') as Ref<string>

// 恢复默认渐变背景
const handleResetBackground = async () => {
  if (await confirm('确定恢复为默认渐变背景吗？')) {
    if (setBackgroundImage) {
      await setBackgroundImage(null)
    }
  }
}

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
    notify.error(`设置背景失败: ${err}`)
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
    notify.error(`移除背景失败: ${err}`)
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
      if (await confirm('是否将当前默认目录中的 Mod 文件移动到新位置？')) {
        await invoke('migrate_mod_repo', { newPath: selected })
        invalidateModList()
      }
    }
  }
}

// 重置视觉效果
const handleResetVisuals = async () => {
  if (await confirm('确定要重置所有视觉效果为默认值吗？')) {
    if (resetVisualSettings) {
      resetVisualSettings()
    }
  }
}

// 重置设置（原有）
const resetToDefaults = async () => {
  if (await confirm('确定要重置所有设置吗？此操作无法撤销。')) {
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
/* ===========================
   新拟态 (Neumorphism) 暗黑模式 3.0
   =========================== */

/* ---- 页面容器 ---- */
.settings-container {
  width: 100%;
  height: 100%;
  padding: 40px;
  box-sizing: border-box;
  overflow-y: auto;
  color: var(--text-main);
  position: relative;
}

.content-wrapper {
  max-width: 800px;
  margin: 0 auto;
}

/* ---- 标题 ---- */
.page-title {
  font-family: 'Rajdhani', 'Segoe UI', 'Arial Black', sans-serif;
  font-size: 32px;
  font-weight: 800;
  text-transform: uppercase;
  padding-bottom: 12px;
  margin-bottom: 40px;
  letter-spacing: 3px;
  color: var(--text-main);
  display: flex;
  align-items: flex-end;
  gap: 12px;
  border-bottom: 2px solid var(--accent);
}

.sub-title {
  font-size: 14px;
  color: var(--text-dim);
  font-weight: 600;
  letter-spacing: 2px;
  margin-bottom: 8px;
  opacity: 0.8;
}

/* ---- 设置组 ---- */
.setting-group {
  margin-bottom: 50px;
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

/* ---- 设置项：新拟态凸起卡片 ---- */
.setting-item {
  background: var(--neu-raised);
  box-shadow:
    -8px -8px 16px var(--neu-shadow-light),
    8px 8px 16px var(--neu-shadow-dark);
  padding: 24px;
  margin-bottom: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: var(--neu-radius);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.setting-item:hover {
  box-shadow:
    -10px -10px 20px var(--neu-shadow-light),
    10px 10px 20px var(--neu-shadow-dark),
    0 0 0 1px var(--neu-shadow-accent);
  transform: translateX(4px);
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

/* ---- 滑块容器 ---- */
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

/* ---- 背景预览 ---- */
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
  border-radius: var(--neu-radius-sm);
  background: var(--neu-inset);
  box-shadow:
    inset 2px 2px 6px var(--neu-shadow-dark),
    inset -2px -2px 6px var(--neu-shadow-light);
}

.bg-actions {
  display: flex;
  align-items: center;
  margin-top: 8px;
}

.background-control-row {
  display: flex;
  align-items: center;
  gap: 16px;
  width: 100%;
}

.bg-btn-group {
  display: flex;
  gap: 8px;
}

/* ---- 背景模式选择 ---- */
.mode-selector-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 4px 0;
}

.mode-buttons {
  display: flex;
  gap: 6px;
}

.neu-mode-btn {
  height: 30px;
  padding: 0 14px;
  border: none;
  border-radius: 15px;
  background: var(--neu-raised);
  color: var(--text-dim);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
  box-shadow:
    -2px -2px 4px var(--neu-shadow-light),
    2px 2px 4px var(--neu-shadow-dark);
  transition: all 0.2s ease;
}

.neu-mode-btn:hover {
  color: var(--text-main);
  box-shadow:
    -3px -3px 6px var(--neu-shadow-light),
    3px 3px 6px var(--neu-shadow-dark);
}

.neu-mode-btn.active {
  color: var(--accent);
  background: var(--neu-inset);
  box-shadow:
    inset 2px 2px 4px var(--neu-shadow-dark),
    inset -2px -2px 4px var(--neu-shadow-light);
}

/* ---- 恢复默认渐变背景 ---- */
.reset-bg-row {
  display: flex;
  justify-content: flex-end;
  width: 100%;
  margin-top: 4px;
}

/* ---- 背景遮罩 ---- */
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
  color: var(--text-dim);
  font-weight: 500;
  min-width: 80px;
}

.toggle-label {
  font-size: 13px;
  color: var(--text-main);
  font-weight: 500;
}

/* ---- 主题选项 ---- */
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

/* ---- 文本 ---- */
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

.desc-sm {
  font-size: 12px;
  color: var(--text-dim);
  opacity: 0.8;
}

.label-row {
  display: flex;
  align-items: baseline;
  width: 100%;
  gap: 8px;
}

/* ---- 输入行 ---- */
.input-row {
  display: flex;
  width: 100%;
  gap: 12px;
}

/* ---- Naive UI 组件新拟态适配 ---- */
:deep(.n-input) {
  --n-border: transparent;
  --n-border-hover: var(--accent);
  --n-border-focus: var(--accent);
  --n-color: var(--neu-inset);
  --n-color-focus: var(--neu-inset);
  --n-text-color: var(--text-main);
  --n-box-shadow-focus: 0 0 0 2px var(--neu-shadow-accent);
  --n-border-radius: var(--neu-radius-sm);
}

:deep(.n-input .n-input__wrapper) {
  box-shadow:
    inset 2px 2px 5px var(--neu-shadow-dark),
    inset -2px -2px 5px var(--neu-shadow-light) !important;
  transition: box-shadow 0.3s ease;
}

:deep(.n-input .n-input__wrapper:hover) {
  box-shadow:
    inset 2px 2px 6px var(--neu-shadow-dark),
    inset -2px -2px 6px var(--neu-shadow-light),
    0 0 0 1px var(--neu-shadow-accent) !important;
}

:deep(.n-button) {
  --n-border: transparent;
  --n-border-hover: transparent;
  --n-color: var(--neu-raised);
  --n-color-hover: var(--neu-raised);
  --n-text-color: var(--text-dim);
  --n-text-color-hover: var(--text-main);
  --n-border-radius: var(--neu-radius-sm);
  box-shadow:
    -3px -3px 6px var(--neu-shadow-light),
    3px 3px 6px var(--neu-shadow-dark) !important;
  transition: all 0.2s ease !important;
}

:deep(.n-button:hover) {
  box-shadow:
    -4px -4px 8px var(--neu-shadow-light),
    4px 4px 8px var(--neu-shadow-dark) !important;
  transform: translateY(-1px);
}

:deep(.n-button:active) {
  box-shadow:
    inset 2px 2px 5px var(--neu-shadow-dark),
    inset -2px -2px 5px var(--neu-shadow-light) !important;
  transform: translateY(0);
}

/* ---- 底部操作区 ---- */
.action-footer {
  margin-top: 60px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 24px;
  border-top: 1px solid var(--border);
}

/* ---- 滚动条 ---- */
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

/* ---- 响应式 ---- */
@media (max-width: 900px) {
  .settings-container { padding: 30px; }
  .content-wrapper { max-width: 100%; }
  .page-title { font-size: 28px; }
  .setting-item { padding: 20px; }
}

@media (max-width: 768px) {
  .settings-container { padding: 20px; }
  .page-title { font-size: 24px; flex-direction: column; align-items: flex-start; gap: 8px; }
  .sub-title { font-size: 12px; }
  .label-row { flex-direction: column; align-items: flex-start; gap: 4px; }
  .desc-sm { margin-left: 0; }
  .input-row { flex-direction: column; }
  .action-footer { justify-content: center; }
}

@media (max-width: 480px) {
  .settings-container { padding: 16px; }
  .setting-item { padding: 16px; }
  .text-info { gap: 4px; }
  .label { font-size: 14px; }
  .desc { font-size: 12px; }
}
</style>
