<template>
  <header class="mod-toolbar">
    <label class="mod-search">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="11" cy="11" r="7" /><path d="m20 20-4-4" /></svg>
      <input :value="search" type="search" placeholder="搜索 Mod" aria-label="搜索 Mod" @input="emit('update:search', ($event.target as HTMLInputElement).value)" />
    </label>
    <UiSelect :model-value="category" :options="selectOptions" label="Mod 分类" @update:model-value="updateCategory" />
    <span class="mod-count">{{ visibleCount }} / {{ totalCount }}</span>
    <UiIconButton :label="isGrid ? '切换为列表布局' : '切换为网格布局'" @click="emit('toggle-layout')">
      <svg v-if="isGrid" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 7h14M5 12h14M5 17h14" /></svg>
      <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7" /><rect x="14" y="3" width="7" height="7" /><rect x="3" y="14" width="7" height="7" /><rect x="14" y="14" width="7" height="7" /></svg>
    </UiIconButton>
  </header>
</template>

<script setup lang="ts">
import UiIconButton from '@/ui/button/UiIconButton.vue'
import UiSelect from '@/ui/form/UiSelect.vue'
import { computed } from 'vue'
import type { ModLibraryCategory } from '@/stores/modLibrary'

const props = defineProps<{ search: string; category: ModLibraryCategory; categories: Array<{ type: ModLibraryCategory; name: string }>; isGrid: boolean; totalCount: number; visibleCount: number }>()
const emit = defineEmits<{ 'update:search': [value: string]; 'update:category': [value: ModLibraryCategory]; 'toggle-layout': [] }>()
const selectOptions = computed(() => props.categories.map(item => ({ value: item.type, label: item.name })))
const updateCategory = (value: string) => emit('update:category', value as ModLibraryCategory)
</script>

<style scoped>
.mod-toolbar { display:grid; grid-template-columns:minmax(180px,280px) minmax(120px,170px) 1fr 38px; align-items:center; gap:var(--ui-space-3); min-height:68px; box-sizing:border-box; padding:var(--ui-space-3) var(--ui-space-5); border-bottom:1px solid var(--ui-border-subtle); background:color-mix(in srgb,var(--ui-bg-surface) 92%,transparent); }
.mod-search { display:flex; align-items:center; gap:var(--ui-space-2); height:var(--ui-control-md); padding:0 var(--ui-space-3); border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-md); color:var(--ui-text-muted); background:var(--ui-bg-app); }
.mod-search:focus-within { border-color:var(--ui-accent); box-shadow:var(--ui-focus-ring); }
.mod-search svg { width:17px; height:17px; flex:none; }
.mod-search input { width:100%; height:100%; margin:0; padding:0; border:0; outline:0; color:var(--ui-text-primary); background:transparent; box-shadow:none; }
.mod-search input::-webkit-search-cancel-button { display:none; }
.mod-count { justify-self:end; color:var(--ui-text-muted); font-family:var(--ui-font-display); font-size:12px; font-weight:600; letter-spacing:.08em; }
@media(max-width:700px){.mod-toolbar{grid-template-columns:1fr 120px 38px}.mod-count{display:none}}
</style>
