<template>
  <UiDialog
    :model-value="visible"
    :title="request?.title ?? '确认操作'"
    eyebrow="Confirm action"
    :close-on-backdrop="false"
    @update:model-value="handleVisibility"
  >
    <div class="confirm-content" :class="{ 'confirm-content--danger': request?.tone === 'danger' }">
      <span class="confirm-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="9"/><path d="M12 8v5M12 16.5h.01"/></svg>
      </span>
      <p>{{ request?.message }}</p>
    </div>
    <template #footer>
      <UiButton @click="store.cancel()">{{ request?.cancelLabel ?? '取消' }}</UiButton>
      <UiButton :variant="request?.tone === 'danger' ? 'danger' : 'primary'" autofocus @click="store.confirm()">
        {{ request?.confirmLabel ?? '确认' }}
      </UiButton>
    </template>
  </UiDialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useConfirmStore } from '@/stores/confirm'
import UiButton from '@/ui/button/UiButton.vue'
import UiDialog from '@/ui/overlay/UiDialog.vue'

const store = useConfirmStore()
const { current: request } = storeToRefs(store)
const visible = computed(() => Boolean(request.value))
const handleVisibility = (value: boolean) => {
  if (!value) store.cancel()
}
</script>

<style scoped>
.confirm-content { display:grid; grid-template-columns:40px 1fr; align-items:start; gap:var(--ui-space-4); }
.confirm-content p { margin:0; padding:3px 0 0; color:var(--ui-text-primary); font-size:14px; line-height:1.65; }
.confirm-icon { width:40px; height:40px; display:grid; place-items:center; border-radius:50%; color:var(--ui-accent); background:color-mix(in srgb,var(--ui-accent) 12%,transparent); }
.confirm-icon svg { width:22px; height:22px; }
.confirm-content--danger .confirm-icon { color:var(--ui-danger); background:color-mix(in srgb,var(--ui-danger) 12%,transparent); }
</style>
