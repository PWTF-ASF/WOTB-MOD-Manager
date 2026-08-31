<template>
  <section class="welcome" aria-labelledby="welcome-title">
    <div class="welcome__visual" aria-hidden="true">
      <span class="welcome__grid" />
      <svg viewBox="0 0 80 80" fill="none">
        <rect x="7" y="15" width="66" height="50" rx="8" stroke="currentColor" stroke-width="2.5" />
        <path d="M18 28h44M18 39h25M18 50h34" stroke="currentColor" stroke-width="2" stroke-linecap="round" opacity=".52" />
        <circle cx="61" cy="20" r="10" fill="var(--ui-accent)" />
        <path d="M57 20h8M61 16v8" stroke="white" stroke-width="2.4" stroke-linecap="round" />
      </svg>
    </div>

    <div class="welcome__copy">
      <p class="welcome__eyebrow">MOD REPOSITORY / EMPTY</p>
      <h1 id="welcome-title">建立你的 Mod 库</h1>
      <p class="welcome__description">
        添加 ZIP 格式的 Mod 文件，然后选择需要启用的内容并统一部署到游戏目录。
      </p>
    </div>

    <div class="welcome__actions">
      <UiButton variant="primary" size="large" :loading="addingMod" @click="handleAddMod">
        <template #icon>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M12 5v14M5 12h14" />
          </svg>
        </template>
        {{ addingMod ? '正在导入' : '添加 Mod' }}
      </UiButton>

      <UiButton size="large" :loading="openingFolder" @click="handleOpenFolder">
        <template #icon>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M3 19V6a2 2 0 0 1 2-2h5l2 3h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z" />
          </svg>
        </template>
        {{ openingFolder ? '正在打开' : '打开仓库' }}
      </UiButton>
    </div>

    <div class="welcome__drop-hint">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <path d="M12 3v12m0 0 4-4m-4 4-4-4M5 19h14" />
      </svg>
      也可以将 ZIP 文件直接拖入窗口
    </div>

    <button class="welcome__help" type="button" @click="showHelp = true">
      第一次使用？查看部署流程
    </button>
  </section>

  <UiDialog v-model="showHelp" title="从导入到部署" eyebrow="Quick start">
    <ol class="help-steps">
      <li><span>01</span><div><strong>导入文件</strong><p>选择一个或多个 ZIP 格式的 Mod 文件。</p></div></li>
      <li><span>02</span><div><strong>选择状态</strong><p>在 Mod 库中决定下次部署需要启用的内容。</p></div></li>
      <li><span>03</span><div><strong>部署改动</strong><p>检查待处理项目，然后统一应用到游戏目录。</p></div></li>
      <li><span>04</span><div><strong>启动游戏</strong><p>部署成功后启动游戏并检查 Mod 效果。</p></div></li>
    </ol>
    <template #footer>
      <UiButton variant="primary" @click="showHelp = false">知道了</UiButton>
    </template>
  </UiDialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useToast } from '@/composables/useToast'
import { formatTauriError } from '@/services/tauri/errors'
import { modService } from '@/services/tauri/modService'
import { settingsService } from '@/services/tauri/settingsService'
import UiButton from '@/ui/button/UiButton.vue'
import UiDialog from '@/ui/overlay/UiDialog.vue'

const emit = defineEmits<{ 'mods-added': [] }>()
const toast = useToast()
const showHelp = ref(false)
const addingMod = ref(false)
const openingFolder = ref(false)

const handleAddMod = async () => {
  addingMod.value = true
  try {
    const selected = await open({
      title: '选择 Mod 文件',
      multiple: true,
      directory: false,
      filters: [{ name: 'Mod 压缩包', extensions: ['zip'] }],
    })
    if (!selected) return

    const paths = Array.isArray(selected) ? selected : [selected]
    const summary = await modService.importFiles(paths)

    if (summary.failed === 0) {
      toast.success(summary.succeeded === 1 ? '已添加 1 个 Mod' : `已添加 ${summary.succeeded} 个 Mod`)
    } else if (summary.succeeded === 0) {
      const firstError = summary.results.find(result => !result.success)?.error
      toast.error(firstError ? `导入失败：${firstError}` : '所选 Mod 均未能导入')
    } else {
      toast.warning(`已添加 ${summary.succeeded} 个 Mod，${summary.failed} 个导入失败`)
    }

    if (summary.succeeded > 0) emit('mods-added')
  } catch (error) {
    toast.error(`无法选择或导入文件：${formatTauriError(error)}`)
  } finally {
    addingMod.value = false
  }
}

const handleOpenFolder = async () => {
  openingFolder.value = true
  try {
    await settingsService.openModRepository()
  } catch (error) {
    toast.error(`无法打开 Mod 仓库：${formatTauriError(error)}`)
  } finally {
    openingFolder.value = false
  }
}
</script>

<style scoped>
.welcome {
  position: relative;
  display: flex;
  flex: 1;
  min-height: 100%;
  box-sizing: border-box;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: clamp(32px, 7vh, 72px) 32px;
  color: var(--ui-text-primary);
  text-align: center;
}
.welcome__visual {
  position: relative;
  display: grid;
  place-items: center;
  width: 112px;
  height: 112px;
  margin-bottom: var(--ui-space-6);
  overflow: hidden;
  border: 1px solid var(--ui-border-subtle);
  border-radius: 26px;
  color: var(--ui-text-secondary);
  background: linear-gradient(145deg, var(--ui-bg-elevated), var(--ui-bg-surface));
  box-shadow: var(--ui-shadow-md);
}
.welcome__visual svg { position: relative; width: 72px; height: 72px; }
.welcome__grid {
  position: absolute;
  inset: 0;
  opacity: 0.24;
  background-image: linear-gradient(var(--ui-border-subtle) 1px, transparent 1px), linear-gradient(90deg, var(--ui-border-subtle) 1px, transparent 1px);
  background-size: 16px 16px;
  mask-image: radial-gradient(circle, black, transparent 72%);
}
.welcome__copy { max-width: 540px; }
.welcome__eyebrow { margin: 0 0 var(--ui-space-2); padding: 0; color: var(--ui-accent); font-family: var(--ui-font-display); font-size: 12px; font-weight: 700; letter-spacing: 0.14em; }
.welcome h1 { margin: 0; color: var(--ui-text-primary); font-size: clamp(26px, 4vw, 34px); font-weight: 720; letter-spacing: -0.025em; }
.welcome__description { max-width: 480px; margin: var(--ui-space-3) auto 0; padding: 0; color: var(--ui-text-secondary); font-size: 14px; line-height: 1.75; }
.welcome__actions { display: flex; flex-wrap: wrap; justify-content: center; gap: var(--ui-space-3); margin-top: var(--ui-space-8); }
.welcome__drop-hint { display: flex; align-items: center; gap: var(--ui-space-2); margin-top: var(--ui-space-5); color: var(--ui-text-muted); font-size: 12px; }
.welcome__drop-hint svg { width: 15px; height: 15px; }
.welcome__help { margin-top: var(--ui-space-6); padding: 4px 8px; border: 0; border-radius: var(--ui-radius-sm); color: var(--ui-text-secondary); background: transparent; font-size: 12px; text-decoration: underline; text-decoration-color: var(--ui-border-strong); text-underline-offset: 4px; cursor: pointer; }
.welcome__help:hover { color: var(--ui-accent); text-decoration-color: currentColor; }
.help-steps { display: grid; gap: var(--ui-space-4); margin: 0; padding: 0; list-style: none; }
.help-steps li { display: grid; grid-template-columns: 36px 1fr; align-items: start; gap: var(--ui-space-3); }
.help-steps li > span { display: grid; place-items: center; width: 32px; height: 32px; border: 1px solid var(--ui-border-strong); border-radius: var(--ui-radius-md); color: var(--ui-accent); font-family: var(--ui-font-display); font-size: 12px; font-weight: 700; background: var(--ui-bg-surface); }
.help-steps strong { display: block; margin-top: 1px; color: var(--ui-text-primary); font-size: 13px; }
.help-steps p { margin: 3px 0 0; padding: 0; color: var(--ui-text-secondary); font-size: 12px; line-height: 1.5; }
@media (max-height: 650px) {
  .welcome { justify-content: flex-start; padding-block: 28px; }
  .welcome__visual { width: 88px; height: 88px; margin-bottom: var(--ui-space-4); border-radius: 20px; }
  .welcome__visual svg { width: 58px; height: 58px; }
  .welcome__actions { margin-top: var(--ui-space-5); }
  .welcome__help { margin-top: var(--ui-space-4); }
}
</style>
