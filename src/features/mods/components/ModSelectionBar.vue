<template>
  <footer class="selection-bar">
    <div class="selection-summary"><strong>{{ selectedCount }}</strong><span>已选择</span><i v-if="pendingCount">{{ pendingCount }} 项改动待部署</i></div>
    <div class="selection-actions">
      <UiButton size="small" variant="ghost" @click="emit('select-all')">{{ allSelected ? '取消全选' : '全选可见' }}</UiButton>
      <UiButton size="small" :disabled="selectedCount===0" @click="emit('enable')">启用</UiButton>
      <UiButton size="small" :disabled="selectedCount===0" @click="emit('disable')">禁用</UiButton>
      <UiButton size="small" variant="danger" :disabled="selectedCount===0" @click="emit('delete')">删除</UiButton>
    </div>
    <div class="primary-actions">
      <UiButton size="medium" @click="emit('add')">添加 Mod</UiButton>
      <UiButton size="medium" variant="primary" :loading="deploying" @click="emit('deploy')">部署改动</UiButton>
      <UiButton size="medium" variant="secondary" :loading="launching" @click="emit('launch')">启动游戏</UiButton>
    </div>
  </footer>
</template>
<script setup lang="ts">
import UiButton from '@/ui/button/UiButton.vue'
defineProps<{selectedCount:number;pendingCount:number;allSelected:boolean;deploying:boolean;launching:boolean}>()
const emit=defineEmits<{ 'select-all':[];enable:[];disable:[];delete:[];add:[];deploy:[];launch:[] }>()
</script>
<style scoped>
.selection-bar{display:grid;grid-template-columns:minmax(100px,1fr) auto auto;align-items:center;gap:var(--ui-space-4);min-height:72px;box-sizing:border-box;padding:var(--ui-space-3) var(--ui-space-5);border-top:1px solid var(--ui-border-subtle);background:var(--ui-bg-elevated);box-shadow:0 -8px 24px rgba(0,0,0,.12)}.selection-summary{display:flex;align-items:baseline;gap:7px;color:var(--ui-text-secondary);font-size:12px}.selection-summary strong{color:var(--ui-text-primary);font-family:var(--ui-font-display);font-size:18px}.selection-summary i{color:var(--ui-warning);font-style:normal}.selection-actions,.primary-actions{display:flex;align-items:center;gap:var(--ui-space-2)}@media(max-width:900px){.selection-bar{grid-template-columns:1fr auto}.selection-actions{display:none}}@media(max-width:650px){.selection-summary{display:none}.selection-bar{grid-template-columns:1fr}.primary-actions{justify-content:flex-end}}
</style>
