<template>
  <dialog
    ref="dialogRef"
    class="ui-dialog"
    :aria-labelledby="titleId"
    :aria-describedby="$slots.default ? descriptionId : undefined"
    @cancel="handleCancel"
    @close="handleClose"
    @click="handleBackdropClick"
  >
    <section class="ui-dialog__panel">
      <header class="ui-dialog__header">
        <div>
          <p v-if="eyebrow" class="ui-dialog__eyebrow">{{ eyebrow }}</p>
          <h2 :id="titleId">{{ title }}</h2>
        </div>
        <button class="ui-dialog__close" type="button" aria-label="关闭对话框" @click="requestClose">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="m6 6 12 12M18 6 6 18" />
          </svg>
        </button>
      </header>
      <div :id="descriptionId" class="ui-dialog__content"><slot /></div>
      <footer v-if="$slots.footer" class="ui-dialog__footer"><slot name="footer" /></footer>
    </section>
  </dialog>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'

const props = withDefaults(defineProps<{
  modelValue: boolean
  title: string
  eyebrow?: string
  closeOnBackdrop?: boolean
  closeOnEscape?: boolean
}>(), {
  closeOnBackdrop: true,
  closeOnEscape: true,
})

const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()
const dialogRef = ref<HTMLDialogElement | null>(null)
const titleId = `ui-dialog-title-${Math.random().toString(36).slice(2)}`
const descriptionId = `ui-dialog-description-${Math.random().toString(36).slice(2)}`
let returnFocus: HTMLElement | null = null

const requestClose = () => emit('update:modelValue', false)

const handleCancel = (event: Event) => {
  if (!props.closeOnEscape) event.preventDefault()
  else requestClose()
}

const handleBackdropClick = (event: MouseEvent) => {
  if (props.closeOnBackdrop && event.target === dialogRef.value) requestClose()
}

const handleClose = () => {
  emit('update:modelValue', false)
  returnFocus?.focus()
  returnFocus = null
}

watch(
  () => props.modelValue,
  async visible => {
    const dialog = dialogRef.value
    if (!dialog) return

    if (visible) {
      returnFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null
      if (!dialog.open) dialog.showModal()
      await nextTick()
      dialog.querySelector<HTMLElement>('[autofocus], button, input, select, textarea, [tabindex]:not([tabindex="-1"])')?.focus()
    } else if (dialog.open) {
      dialog.close()
    }
  },
  { immediate: true },
)
</script>

<style scoped>
.ui-dialog {
  width: min(520px, calc(100vw - 32px));
  max-height: min(720px, calc(100vh - 32px));
  margin: auto;
  padding: 0;
  border: 1px solid var(--ui-border-strong);
  border-radius: var(--ui-radius-xl);
  color: var(--ui-text-primary);
  background: transparent;
  box-shadow: var(--ui-shadow-md);
}
.ui-dialog::backdrop { background: var(--ui-bg-overlay); backdrop-filter: blur(4px); }
.ui-dialog__panel { display: flex; max-height: inherit; flex-direction: column; background: var(--ui-bg-elevated); }
.ui-dialog__header { display: flex; align-items: flex-start; justify-content: space-between; gap: var(--ui-space-4); padding: var(--ui-space-5) var(--ui-space-6); border-bottom: 1px solid var(--ui-border-subtle); }
.ui-dialog__header h2 { margin: 0; color: var(--ui-text-primary); font-size: 19px; line-height: 1.25; }
.ui-dialog__eyebrow { margin: 0 0 4px; padding: 0; color: var(--ui-accent); font-family: var(--ui-font-display); font-size: 11px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; }
.ui-dialog__close { display: grid; place-items: center; width: 32px; height: 32px; margin: -4px -8px 0 0; padding: 0; border: 0; border-radius: var(--ui-radius-sm); color: var(--ui-text-muted); background: transparent; cursor: pointer; }
.ui-dialog__close:hover { color: var(--ui-text-primary); background: var(--ui-bg-surface-hover); }
.ui-dialog__close svg { width: 18px; height: 18px; }
.ui-dialog__content { overflow: auto; padding: var(--ui-space-6); color: var(--ui-text-secondary); }
.ui-dialog__footer { display: flex; justify-content: flex-end; gap: var(--ui-space-3); padding: var(--ui-space-4) var(--ui-space-6); border-top: 1px solid var(--ui-border-subtle); }
</style>
