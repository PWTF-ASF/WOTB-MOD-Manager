<template>
  <article class="mod-row" :class="{ 'mod-row--pending': isPending }">
    <UiCheckbox :model-value="item.selected" @update:model-value="emit('update:selected',$event)" />
    <button class="row-icon" type="button" aria-label="修改 Mod 图标" @click="emit('icon')">
      <img v-if="iconUrl" :src="iconUrl" alt="" @load="emit('icon-load')" @error="emit('icon-error')" />
      <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M6 3h9l3 3v15H6Z" /><path d="M14 3v4h4" /></svg>
    </button>
    <div class="row-info"><strong>{{ item.displayName }}</strong><span>{{ categoryLabel }} · {{ dateLabel }}</span></div>
    <UiStatusChip :tone="isPending ? 'pending' : item.deployed ? 'success' : 'neutral'">{{ isPending ? '待部署' : item.deployed ? '已部署' : '未部署' }}</UiStatusChip>
    <UiSwitch :model-value="item.desiredEnabled" label="下次部署启用" @update:model-value="emit('update:enabled',$event)" />
    <UiIconButton size="small" label="编辑 Mod" @click="emit('edit')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m4 20 4-.8L19 8.2 15.8 5 4.8 16Z" /></svg></UiIconButton>
    <UiIconButton size="small" label="删除 Mod" @click="emit('delete')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 7h16M9 7V4h6v3m3 0-1 13H7L6 7" /></svg></UiIconButton>
  </article>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ModLibraryItem } from '@/stores/modLibrary'
import UiCheckbox from '@/ui/form/UiCheckbox.vue'
import UiSwitch from '@/ui/form/UiSwitch.vue'
import UiIconButton from '@/ui/button/UiIconButton.vue'
import UiStatusChip from '@/ui/feedback/UiStatusChip.vue'
const props=defineProps<{item:ModLibraryItem;iconUrl:string|null;categoryLabel:string;dateLabel:string}>()
const emit=defineEmits<{ 'update:selected':[value:boolean];'update:enabled':[value:boolean];edit:[];delete:[];icon:[];'icon-load':[];'icon-error':[] }>()
const isPending=computed(()=>props.item.deployed!==props.item.desiredEnabled)
</script>

<style scoped>
.mod-row{display:grid;grid-template-columns:20px 42px minmax(150px,1fr) auto 40px 30px 30px;align-items:center;gap:var(--ui-space-3);min-height:62px;padding:8px 12px;border:1px solid var(--ui-border-subtle);border-radius:var(--ui-radius-md);color:var(--ui-text-primary);background:var(--ui-bg-surface)}
.mod-row--pending{border-left:3px solid var(--ui-warning)}.row-icon{display:grid;place-items:center;width:40px;height:40px;margin:0;padding:0;overflow:hidden;border:1px solid var(--ui-border-subtle);border-radius:10px;color:var(--ui-text-muted);background:var(--ui-bg-app);cursor:pointer}.row-icon img{width:100%;height:100%;object-fit:cover}.row-icon svg{width:20px;height:20px}.row-info{display:grid;min-width:0;gap:3px}.row-info strong,.row-info span{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.row-info strong{font-size:13px}.row-info span{color:var(--ui-text-muted);font-size:11px}@media(max-width:760px){.mod-row{grid-template-columns:20px 38px 1fr 40px 30px}.mod-row>:nth-child(4),.mod-row>:last-child{display:none}}
</style>
