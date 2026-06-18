<template>
  <div class="welcome-wrapper">
    <div class="welcome-bg"></div>
    <div class="welcome-content">
      <div class="welcome-icon">
        <svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="6" y="14" width="68" height="52" rx="12" stroke="currentColor" stroke-width="3" opacity="0.6"/>
          <rect x="22" y="30" width="36" height="24" rx="6" stroke="currentColor" stroke-width="2" opacity="0.6"/>
          <circle cx="54" cy="24" r="10" fill="var(--accent)"/>
          <path d="M50 24h8M54 20v8" stroke="#fff" stroke-width="2.5" stroke-linecap="round"/>
        </svg>
      </div>
      <h1 class="welcome-title">欢迎使用 Mod 管理器</h1>
      <p class="welcome-desc">尚未安装任何 Mod<br/>点击下方按钮选择 .zip 文件导入，或直接打开 Mod 文件夹手动管理。</p>
      <div class="welcome-actions">
        <button class="btn-primary" :disabled="addingMod" @click="handleAddMod">
          <span v-if="addingMod" class="spinner"/>
          <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
          <span>{{ addingMod ? '正在添加...' : '添加 Mod' }}</span>
        </button>
        <button class="btn-secondary" :disabled="openingFolder" @click="handleOpenFolder">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          <span>{{ openingFolder ? '正在打开...' : '打开 Mods 文件夹' }}</span>
        </button>
      </div>
      <button class="help-toggle" @click="showHelp = !showHelp">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
        <span>{{ showHelp ? '收起帮助' : '使用帮助' }}</span>
      </button>
      <Transition name="slide">
        <div v-if="showHelp" class="help-panel">
          <div class="help-step"><span class="step-num">1</span>选择 .zip 格式的 Mod 文件导入</div>
          <div class="help-step"><span class="step-num">2</span>在列表中启用想要使用的 Mod</div>
          <div class="help-step"><span class="step-num">3</span>点击底部「部署」应用到游戏目录</div>
          <div class="help-step"><span class="step-num">4</span>启动游戏体验 Mod 效果</div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { open as shellOpen } from '@tauri-apps/plugin-shell'
import { useNotify } from '@/composables/useNotification'

const emit = defineEmits<{ 'mods-added': [] }>()
const notify = useNotify()
const showHelp = ref(false)
const addingMod = ref(false)
const openingFolder = ref(false)

const handleAddMod = async () => {
  addingMod.value = true
  try {
    const selected = await open({
      title: '请选择 Mod 文件', multiple: true, directory: false,
      filters: [{ name: 'Mod 文件', extensions: ['zip', '7z', 'rar'] }],
    })
    if (!selected) return
    const paths = Array.isArray(selected) ? selected : [selected]
    let ok = 0
    for (const p of paths) {
      try { await invoke('copy_mod_file', { src: p }); ok++ }
      catch (e) { notify.error(`导入失败: ${e}`) }
    }
    if (ok > 0) { notify.success(ok === 1 ? '已添加 1 个 Mod' : `已添加 ${ok} 个 Mod`); emit('mods-added') }
  } finally { addingMod.value = false }
}

const handleOpenFolder = async () => {
  openingFolder.value = true
  try {
    const p: string | null = await invoke('get_mod_repo_path')
    if (p) await shellOpen(p)
    else notify.warning('请先在设置中配置 Mod 存储路径')
  } catch { notify.warning('无法打开文件夹') }
  finally { openingFolder.value = false }
}
</script>

<style scoped>
.welcome-wrapper { flex:1; display:flex; align-items:center; justify-content:center; }
.welcome-bg { display:none; }
.welcome-content { display:flex; flex-direction:column; align-items:center; gap:16px; padding:48px 40px; text-align:center; max-width:420px; border-radius:var(--neu-radius-lg); background:var(--neu-raised); box-shadow:-10px -10px 24px var(--neu-shadow-light),10px 10px 24px var(--neu-shadow-dark); }
.welcome-icon { width:80px; height:80px; color:var(--text-dim); margin-bottom:8px; }
.welcome-title { font-size:24px; font-weight:700; color:var(--text-main); margin:0; }
.welcome-desc { font-size:14px; color:var(--text-dim); line-height:1.8; margin:0; }
.welcome-actions { display:flex; gap:12px; margin-top:12px; }
.btn-primary, .btn-secondary { display:flex; align-items:center; gap:8px; height:44px; padding:0 24px; border:none; border-radius:22px; font-size:14px; font-weight:600; cursor:pointer; white-space:nowrap; transition:all .25s cubic-bezier(.4,0,.2,1); }
.btn-primary { background:linear-gradient(135deg,var(--accent) 0%,#536dfe 100%); color:#fff; box-shadow:0 4px 16px rgba(61,90,254,.25); }
.btn-primary:hover:not(:disabled) { transform:translateY(-2px); box-shadow:0 6px 24px rgba(61,90,254,.35); }
.btn-primary:disabled { opacity:.6; cursor:not-allowed; }
.btn-secondary { background:var(--neu-raised); color:var(--text-main); box-shadow:-4px -4px 10px var(--neu-shadow-light),4px 4px 10px var(--neu-shadow-dark); }
.btn-secondary:hover:not(:disabled) { transform:translateY(-2px); box-shadow:-6px -6px 14px var(--neu-shadow-light),6px 6px 14px var(--neu-shadow-dark); }
.btn-secondary:active:not(:disabled) { transform:translateY(0); box-shadow:inset 3px 3px 6px var(--neu-shadow-dark),inset -3px -3px 6px var(--neu-shadow-light); }
.btn-secondary:disabled { opacity:.5; cursor:not-allowed; }
.spinner { display:inline-block; width:16px; height:16px; border:2px solid rgba(255,255,255,.3); border-top-color:#fff; border-radius:50%; animation:spin .6s linear infinite; }
@keyframes spin { to { transform:rotate(360deg); } }
.help-toggle { display:flex; align-items:center; gap:4px; background:none; border:none; color:var(--text-dim); font-size:13px; cursor:pointer; padding:4px 8px; border-radius:6px; transition:color .2s; }
.help-toggle:hover { color:var(--accent); }
.help-panel { width:100%; display:flex; flex-direction:column; gap:10px; padding:16px 20px; border-radius:var(--neu-radius-sm); background:var(--neu-inset); box-shadow:inset 2px 2px 6px var(--neu-shadow-dark),inset -2px -2px 6px var(--neu-shadow-light); text-align:left; }
.help-step { display:flex; align-items:flex-start; gap:10px; font-size:13px; color:var(--text-dim); line-height:1.5; }
.step-num { flex-shrink:0; width:20px; height:20px; display:flex; align-items:center; justify-content:center; border-radius:50%; background:var(--accent); color:#fff; font-size:11px; font-weight:700; }
.slide-enter-active, .slide-leave-active { transition:all .25s ease; }
.slide-enter-from, .slide-leave-to { opacity:0; transform:translateY(-8px); }
</style>
