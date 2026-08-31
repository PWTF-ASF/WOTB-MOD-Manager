<template>
  <article class="mod-card" :class="{ 'mod-card--enabled': item.desiredEnabled, 'mod-card--pending': isPending }">
    <header>
      <UiCheckbox :model-value="item.selected" @update:model-value="emit('update:selected', $event)" />
      <div class="card-actions">
        <UiIconButton size="small" label="编辑 Mod" @click="emit('edit')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m4 20 4-.8L19 8.2 15.8 5 4.8 16Z" /></svg></UiIconButton>
        <UiIconButton size="small" label="删除 Mod" @click="emit('delete')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 7h16M9 7V4h6v3m3 0-1 13H7L6 7" /></svg></UiIconButton>
      </div>
    </header>
    <button class="mod-icon" type="button" aria-label="修改 Mod 图标" @click="emit('icon')">
      <img v-if="iconUrl" :src="iconUrl" alt="" @load="emit('icon-load')" @error="emit('icon-error')" />
      <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M6 3h9l3 3v15H6Z" /><path d="M14 3v4h4M9 12h6M9 16h6" /></svg>
    </button>
    <div class="mod-card__body">
      <h3 :title="item.displayName">{{ item.displayName }}</h3>
      <button class="category-button" type="button" @click="emit('category')">{{ categoryLabel }}</button>
    </div>
    <footer>
      <div class="state-copy">
        <span>{{ item.deployed ? '当前已部署' : '当前未部署' }}</span>
        <UiStatusChip v-if="isPending" tone="pending">待部署</UiStatusChip>
      </div>
      <UiSwitch :model-value="item.desiredEnabled" label="下次部署启用" @update:model-value="emit('update:enabled', $event)" />
    </footer>
  </article>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ModLibraryItem } from '@/stores/modLibrary'
import UiCheckbox from '@/ui/form/UiCheckbox.vue'
import UiSwitch from '@/ui/form/UiSwitch.vue'
import UiIconButton from '@/ui/button/UiIconButton.vue'
import UiStatusChip from '@/ui/feedback/UiStatusChip.vue'
const props = defineProps<{ item: ModLibraryItem; iconUrl: string | null; categoryLabel: string }>()
const emit = defineEmits<{ 'update:selected':[value:boolean]; 'update:enabled':[value:boolean]; edit:[]; delete:[]; category:[]; icon:[]; 'icon-load':[]; 'icon-error':[] }>()
const isPending = computed(() => props.item.deployed !== props.item.desiredEnabled)
</script>

<style scoped>
.mod-card { display:grid; grid-template-rows:auto auto 1fr auto; min-width:0; min-height:260px; padding:var(--ui-space-4); border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-lg); color:var(--ui-text-primary); background:var(--ui-bg-surface); box-shadow:var(--ui-shadow-sm); transition:border-color var(--ui-duration-fast),transform var(--ui-duration-fast); }
.mod-card:hover { border-color:var(--ui-border-strong); transform:translateY(-1px); }
.mod-card--enabled { border-top-color:var(--ui-accent); }
.mod-card--pending { box-shadow:inset 0 2px 0 var(--ui-warning),var(--ui-shadow-sm); }
.mod-card header,.mod-card footer { display:flex; align-items:center; justify-content:space-between; gap:var(--ui-space-3); }
.card-actions { display:flex; gap:var(--ui-space-1); }
.mod-icon { display:grid; place-items:center; width:76px; height:76px; margin:var(--ui-space-5) auto var(--ui-space-4); padding:0; overflow:hidden; border:1px solid var(--ui-border-subtle); border-radius:18px; color:var(--ui-text-muted); background:var(--ui-bg-app); cursor:pointer; }
.mod-icon img { width:100%; height:100%; object-fit:cover; }.mod-icon svg{width:36px;height:36px}
.mod-card__body{text-align:center;min-width:0}.mod-card h3{margin:0 0 8px;overflow:hidden;color:var(--ui-text-primary);font-size:14px;text-overflow:ellipsis;white-space:nowrap}.category-button{margin:0;padding:3px 8px;border:0;border-radius:999px;color:var(--ui-accent);background:color-mix(in srgb,var(--ui-accent) 10%,transparent);font-size:11px;cursor:pointer}
.mod-card footer{margin-top:var(--ui-space-4);padding-top:var(--ui-space-3);border-top:1px solid var(--ui-border-subtle)}.state-copy{display:flex;align-items:center;gap:var(--ui-space-2);color:var(--ui-text-muted);font-size:11px}
</style>
