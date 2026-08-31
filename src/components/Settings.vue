<template>
  <div class="settings-page">
    <header class="settings-header">
      <div>
        <p class="eyebrow">SYSTEM CONFIGURATION</p>
        <h1>系统设置</h1>
        <p>管理游戏路径、模组仓库与界面外观。</p>
      </div>
    </header>

    <section class="settings-section" aria-labelledby="paths-heading">
      <div class="section-heading">
        <div>
          <p class="section-index">01 / PATHS</p>
          <h2 id="paths-heading">路径配置</h2>
        </div>
        <UiStatusChip :tone="pathsReady ? 'success' : 'warning'">
          {{ pathsReady ? '已配置' : '待配置' }}
        </UiStatusChip>
      </div>

      <div class="setting-card">
        <div class="setting-copy">
          <strong>游戏安装路径</strong>
          <span>World of Tanks Blitz 主程序所在目录</span>
        </div>
        <div class="path-control">
          <input :value="config.gamePath" readonly placeholder="请选择游戏安装目录" />
          <UiButton size="small" @click="selectPath('game')">浏览</UiButton>
        </div>
      </div>

      <div class="setting-card">
        <div class="setting-copy">
          <strong>Mod 存储库路径</strong>
          <span>下载和管理的 Mod 文件将保存在此位置</span>
        </div>
        <div class="path-control">
          <input :value="config.modRepoPath" readonly placeholder="请选择 Mod 存储目录" />
          <UiButton size="small" @click="selectPath('mod')">浏览</UiButton>
        </div>
      </div>
    </section>

    <section class="settings-section" aria-labelledby="appearance-heading">
      <div class="section-heading">
        <div>
          <p class="section-index">02 / APPEARANCE</p>
          <h2 id="appearance-heading">外观设置</h2>
        </div>
      </div>

      <div class="setting-card setting-card--split">
        <div class="setting-copy">
          <strong>主题模式</strong>
          <span>可以固定主题，也可以跟随系统设置。</span>
        </div>
        <div class="segmented" role="radiogroup" aria-label="主题模式">
          <button
            v-for="option in themeOptions"
            :key="option.value"
            type="button"
            role="radio"
            :aria-checked="themeMode === option.value"
            :class="{ active: themeMode === option.value }"
            @click="themeMode = option.value"
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <div class="setting-card setting-card--background">
        <div class="setting-copy">
          <strong>背景图片</strong>
          <span>支持 PNG、JPG 和 WebP，未设置时使用默认渐变。</span>
        </div>

        <div class="background-panel">
          <div class="background-preview" :class="{ empty: !backgroundPreviewUrl }">
            <img v-if="backgroundPreviewUrl" :src="backgroundPreviewUrl" alt="当前背景预览" />
            <span v-else>DEFAULT GRADIENT</span>
          </div>
          <div class="button-row">
            <UiButton size="small" :loading="updatingBackground" @click="handleSelectBackground">
              {{ backgroundPreviewUrl ? '更换图片' : '选择图片' }}
            </UiButton>
            <UiButton
              v-if="backgroundPreviewUrl"
              size="small"
              variant="ghost"
              :disabled="updatingBackground"
              @click="handleRemoveBackground"
            >
              移除
            </UiButton>
          </div>
        </div>

        <div class="display-options">
          <span class="field-label">显示模式</span>
          <div class="segmented segmented--compact" role="radiogroup" aria-label="背景显示模式">
            <button
              v-for="option in backgroundModes"
              :key="option.value"
              type="button"
              role="radio"
              :aria-checked="backgroundMode === option.value"
              :class="{ active: backgroundMode === option.value }"
              @click="backgroundMode = option.value"
            >
              {{ option.label }}
            </button>
          </div>
        </div>
      </div>

      <div class="setting-card setting-card--controls">
        <div class="control-row">
          <div class="setting-copy">
            <strong>背景遮罩</strong>
            <span>提高文字在背景图片上的可读性。</span>
          </div>
          <UiSwitch v-model="backgroundMask" label="切换背景遮罩" />
        </div>
        <label class="range-row" :class="{ disabled: !backgroundMask }">
          <span>遮罩强度</span>
          <input
            type="range"
            min="0"
            max="100"
            step="1"
            :value="maskOpacity"
            :disabled="!backgroundMask"
            @input="setMaskOpacity"
          />
          <output>{{ maskOpacity }}%</output>
        </label>

        <div class="control-row control-row--bordered">
          <div class="setting-copy">
            <strong>背景模糊</strong>
            <span>柔化背景细节，使内容区域更聚焦。</span>
          </div>
          <UiSwitch v-model="enableBackgroundBlur" label="切换背景模糊" />
        </div>
        <label class="range-row" :class="{ disabled: !enableBackgroundBlur }">
          <span>模糊强度</span>
          <input
            type="range"
            min="0"
            max="20"
            step="1"
            :value="backgroundBlurAmount"
            :disabled="!enableBackgroundBlur"
            @input="setBlurAmount"
          />
          <output>{{ backgroundBlurAmount }}px</output>
        </label>
      </div>
    </section>

    <footer class="settings-footer">
      <UiButton variant="ghost" @click="handleResetBackground">恢复默认背景</UiButton>
      <UiButton variant="secondary" @click="handleResetVisuals">重置视觉效果</UiButton>
      <UiButton variant="danger" @click="resetToDefaults">重置所有设置</UiButton>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, onMounted, reactive, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { open } from '@tauri-apps/plugin-dialog'
import UiButton from '@/ui/button/UiButton.vue'
import UiSwitch from '@/ui/form/UiSwitch.vue'
import UiStatusChip from '@/ui/feedback/UiStatusChip.vue'
import { useToast } from '@/composables/useToast'
import { useConfirmDialog } from '@/composables/useConfirmDialog'
import { formatTauriError } from '@/services/tauri/errors'
import { settingsService } from '@/services/tauri/settingsService'
import { usePreferencesStore } from '@/stores/preferences'
import type { BackgroundMode, ThemeMode } from '@/types/settings'

const preferences = usePreferencesStore()
const toast = useToast()
const { confirm } = useConfirmDialog()
const invalidateModList = inject<() => void>('invalidateModList', () => {})
const {
  enableBackgroundBlur,
  backgroundBlurAmount,
  backgroundMask,
  maskOpacity,
  themeMode,
  backgroundMode,
  backgroundImageUrl: backgroundPreviewUrl,
} = storeToRefs(preferences)

const themeOptions: Array<{ value: ThemeMode; label: string }> = [
  { value: 'light', label: '浅色' },
  { value: 'dark', label: '深色' },
  { value: 'system', label: '跟随系统' },
]
const backgroundModes: Array<{ value: BackgroundMode; label: string }> = [
  { value: 'cover', label: '铺满' },
  { value: 'contain', label: '适应' },
  { value: 'fill', label: '拉伸' },
  { value: 'tile', label: '平铺' },
]
const config = reactive({ gamePath: '', modRepoPath: '' })
const updatingBackground = ref(false)
const pathsReady = computed(() => Boolean(config.gamePath && config.modRepoPath))

const confirmAction = (message: string, danger = false) => confirm({
  title: danger ? '确认重置' : '确认操作',
  message,
  tone: danger ? 'danger' : 'default',
})

const setMaskOpacity = (event: Event) => {
  maskOpacity.value = Number((event.target as HTMLInputElement).value)
}
const setBlurAmount = (event: Event) => {
  backgroundBlurAmount.value = Number((event.target as HTMLInputElement).value)
}

onMounted(async () => {
  try {
    const paths = await settingsService.getPaths()
    config.gamePath = paths.gamePath || ''
    config.modRepoPath = paths.modRepositoryPath || ''
  } catch (error) {
    toast.error(`读取路径设置失败：${formatTauriError(error)}`)
  }
})

const selectPath = async (type: 'game' | 'mod') => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: type === 'game' ? '选择游戏安装目录' : '选择 Mod 存储库目录',
  })
  if (!selected || Array.isArray(selected)) return

  try {
    if (type === 'game') {
      await settingsService.setGamePath(selected)
      config.gamePath = selected
      toast.success('游戏路径已更新')
      return
    }

    const shouldMigrate = await confirmAction('是否将当前目录中的 Mod 文件移动到新位置？')
    if (shouldMigrate) await settingsService.migrateModRepository(selected)
    await settingsService.setModRepositoryPath(selected)
    config.modRepoPath = selected
    if (shouldMigrate) invalidateModList()
    toast.success(shouldMigrate ? 'Mod 仓库已迁移' : 'Mod 仓库路径已更新')
  } catch (error) {
    toast.error(`保存路径失败：${formatTauriError(error)}`)
  }
}

const handleSelectBackground = async () => {
  const selected = await open({
    title: '选择背景图片',
    multiple: false,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
  })
  if (!selected || Array.isArray(selected)) return

  updatingBackground.value = true
  try {
    await preferences.setBackgroundImage(selected)
    toast.success('背景图片已更新')
  } catch (error) {
    toast.error(`设置背景失败：${formatTauriError(error)}`)
  } finally {
    updatingBackground.value = false
  }
}

const removeBackground = async () => {
  updatingBackground.value = true
  try {
    await preferences.setBackgroundImage(null)
    toast.success('已恢复默认背景')
  } catch (error) {
    toast.error(`移除背景失败：${formatTauriError(error)}`)
  } finally {
    updatingBackground.value = false
  }
}

const handleRemoveBackground = async () => {
  if (await confirmAction('确定移除当前背景图片吗？')) await removeBackground()
}
const handleResetBackground = async () => {
  if (!backgroundPreviewUrl.value) return
  if (await confirmAction('确定恢复为默认渐变背景吗？')) await removeBackground()
}
const handleResetVisuals = async () => {
  if (await confirmAction('确定将所有视觉效果恢复为默认值吗？')) {
    preferences.resetVisualSettings()
    toast.success('视觉效果已重置')
  }
}
const resetToDefaults = async () => {
  if (!(await confirmAction('确定重置所有设置吗？路径和视觉选项都将清空。', true))) return
  try {
    await Promise.all([
      settingsService.setGamePath(''),
      settingsService.setModRepositoryPath(''),
      preferences.setBackgroundImage(null),
    ])
    config.gamePath = ''
    config.modRepoPath = ''
    preferences.resetVisualSettings()
    invalidateModList()
    toast.success('所有设置已重置')
  } catch (error) {
    toast.error(`重置设置失败：${formatTauriError(error)}`)
  }
}
</script>

<style scoped>
.settings-page { width:100%; height:100%; overflow-y:auto; padding:clamp(24px,5vw,48px); color:var(--ui-text-primary); }
.settings-header,.settings-section,.settings-footer { width:min(860px,100%); margin-inline:auto; }
.settings-header { margin-bottom:40px; }
.eyebrow,.section-index { margin:0 0 6px; color:var(--ui-accent); font:700 11px/1 var(--ui-font-display); letter-spacing:.18em; }
h1,h2 { margin:0; font-family:var(--ui-font-display); }
h1 { font-size:clamp(30px,4vw,42px); letter-spacing:.04em; }
h2 { font-size:20px; }
.settings-header > div > p:last-child { margin:10px 0 0; color:var(--ui-text-secondary); }
.settings-section { margin-bottom:36px; }
.section-heading { display:flex; align-items:flex-end; justify-content:space-between; margin-bottom:14px; padding:0 2px; }
.setting-card { padding:20px; margin-bottom:12px; border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-lg); background:color-mix(in srgb,var(--ui-bg-surface) 88%,transparent); box-shadow:var(--ui-shadow-sm); backdrop-filter:blur(14px); }
.setting-card--split,.control-row { display:flex; align-items:center; justify-content:space-between; gap:24px; }
.setting-copy { display:flex; flex-direction:column; gap:5px; min-width:0; }
.setting-copy strong { font-size:14px; }
.setting-copy span,.field-label { color:var(--ui-text-secondary); font-size:12px; line-height:1.5; }
.path-control { display:grid; grid-template-columns:minmax(0,1fr) auto; align-items:center; gap:10px; margin-top:16px; }
.path-control input { width:100%; height:var(--ui-control-sm); box-sizing:border-box; margin:0; padding:0 12px; border:1px solid var(--ui-border-strong); border-radius:var(--ui-radius-md); color:var(--ui-text-primary); background:var(--ui-bg-app); outline:none; }
.path-control input:focus { border-color:var(--ui-accent); }
.segmented { display:inline-flex; padding:3px; border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-md); background:var(--ui-bg-app); }
.segmented button { min-height:32px; padding:0 14px; border:0; border-radius:calc(var(--ui-radius-md) - 3px); color:var(--ui-text-secondary); background:transparent; font:650 12px var(--ui-font-body); cursor:pointer; }
.segmented button:hover { color:var(--ui-text-primary); }
.segmented button.active { color:var(--ui-text-on-accent); background:var(--ui-accent); }
.segmented--compact button { min-height:28px; padding-inline:11px; }
.setting-card--background { display:grid; grid-template-columns:1fr auto; gap:18px 28px; }
.background-panel { grid-column:2; grid-row:1 / span 2; display:flex; align-items:center; gap:12px; }
.background-preview { width:126px; height:76px; overflow:hidden; display:grid; place-items:center; border:1px solid var(--ui-border-strong); border-radius:var(--ui-radius-md); background:var(--ui-bg-app); }
.background-preview img { width:100%; height:100%; object-fit:cover; }
.background-preview.empty span { color:var(--ui-text-muted); font:700 9px var(--ui-font-display); letter-spacing:.12em; }
.button-row { display:flex; flex-direction:column; align-items:stretch; gap:5px; }
.display-options { display:flex; align-items:center; gap:14px; }
.setting-card--controls { display:grid; gap:15px; }
.control-row--bordered { padding-top:18px; border-top:1px solid var(--ui-border-subtle); }
.range-row { display:grid; grid-template-columns:92px 1fr 50px; align-items:center; gap:14px; color:var(--ui-text-secondary); font-size:12px; }
.range-row.disabled { opacity:.48; }
.range-row input { width:100%; accent-color:var(--ui-accent); cursor:pointer; }
.range-row output { text-align:right; color:var(--ui-text-primary); font-variant-numeric:tabular-nums; }
.settings-footer { display:flex; justify-content:flex-end; flex-wrap:wrap; gap:8px; padding:4px 0 32px; }
@media (max-width:720px) {
  .setting-card--split,.control-row { align-items:flex-start; }
  .setting-card--split { flex-direction:column; }
  .setting-card--background { grid-template-columns:1fr; }
  .background-panel { grid-column:1; grid-row:auto; flex-wrap:wrap; }
  .display-options { flex-wrap:wrap; }
}
</style>
