<template>
  <UiDialog :model-value="modelValue" :title="title" eyebrow="Mod icon" :close-on-backdrop="!uploading" @update:model-value="emit('update:modelValue',$event)">
    <div class="icon-preview"><UiSpinner v-if="loading" class="preview-spinner" :size="26" /><img v-if="imageUrl" :class="{ 'is-loading':loading }" :src="imageUrl" alt="Mod 图标预览" @load="emit('loaded')" @error="emit('error')" /><svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M6 3h9l3 3v15H6Z"/><path d="M14 3v4h4"/></svg></div>
    <template #footer><UiButton v-if="hasIcon" variant="danger" :disabled="uploading" @click="emit('remove')">移除图标</UiButton><UiButton variant="primary" :loading="uploading" @click="emit('upload')">更换图标</UiButton></template>
  </UiDialog>
</template>
<script setup lang="ts">
import UiDialog from '@/ui/overlay/UiDialog.vue';import UiButton from '@/ui/button/UiButton.vue';import UiSpinner from '@/ui/feedback/UiSpinner.vue'
defineProps<{modelValue:boolean;title:string;imageUrl:string|null;loading:boolean;uploading:boolean;hasIcon:boolean}>();const emit=defineEmits<{ 'update:modelValue':[value:boolean];upload:[];remove:[];loaded:[];error:[] }>()
</script>
<style scoped>.icon-preview{position:relative;display:grid;place-items:center;min-height:220px;overflow:hidden;border:1px solid var(--ui-border-subtle);border-radius:var(--ui-radius-lg);color:var(--ui-text-muted);background:var(--ui-bg-app)}.icon-preview img{max-width:100%;max-height:260px;object-fit:contain;transition:opacity var(--ui-duration-fast)}.icon-preview img.is-loading{opacity:0}.preview-spinner{position:absolute;z-index:1}.icon-preview>svg{width:72px;height:72px}</style>
