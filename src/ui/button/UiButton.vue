<template>
  <button
    class="ui-button"
    :class="[`ui-button--${variant}`, `ui-button--${size}`]"
    :type="type"
    :disabled="disabled || loading"
    :aria-busy="loading || undefined"
  >
    <UiSpinner v-if="loading" label="正在处理" />
    <span v-else-if="$slots.icon" class="ui-button__icon" aria-hidden="true"><slot name="icon" /></span>
    <span class="ui-button__label"><slot /></span>
  </button>
</template>

<script setup lang="ts">
import UiSpinner from '@/ui/feedback/UiSpinner.vue'

withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  size?: 'small' | 'medium' | 'large'
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
}>(), {
  variant: 'secondary',
  size: 'medium',
  type: 'button',
  disabled: false,
  loading: false,
})
</script>

<style scoped>
.ui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-2);
  min-width: 0;
  margin: 0;
  border: 1px solid transparent;
  border-radius: var(--ui-radius-md);
  color: var(--ui-text-primary);
  font-family: var(--ui-font-body);
  font-weight: 650;
  line-height: 1;
  white-space: nowrap;
  cursor: pointer;
  transition:
    color var(--ui-duration-fast) var(--ui-ease),
    background var(--ui-duration-fast) var(--ui-ease),
    border-color var(--ui-duration-fast) var(--ui-ease),
    transform var(--ui-duration-fast) var(--ui-ease);
}

.ui-button--small { height: var(--ui-control-sm); padding: 0 var(--ui-space-3); font-size: 12px; }
.ui-button--medium { height: var(--ui-control-md); padding: 0 var(--ui-space-4); font-size: 13px; }
.ui-button--large { height: var(--ui-control-lg); padding: 0 var(--ui-space-5); font-size: 14px; }

.ui-button--primary {
  color: var(--ui-text-on-accent);
  background: var(--ui-accent);
  border-color: var(--ui-accent);
}
.ui-button--primary:hover:not(:disabled) { background: var(--ui-accent-hover); border-color: var(--ui-accent-hover); }
.ui-button--primary:active:not(:disabled) { background: var(--ui-accent-active); transform: translateY(1px); }

.ui-button--secondary {
  background: var(--ui-bg-elevated);
  border-color: var(--ui-border-strong);
}
.ui-button--secondary:hover:not(:disabled) { background: var(--ui-bg-surface-hover); border-color: var(--ui-accent); }

.ui-button--ghost { color: var(--ui-text-secondary); background: transparent; }
.ui-button--ghost:hover:not(:disabled) { color: var(--ui-text-primary); background: var(--ui-bg-surface-hover); }

.ui-button--danger { color: var(--ui-text-on-accent); background: var(--ui-danger); border-color: var(--ui-danger); }
.ui-button--danger:hover:not(:disabled) { filter: brightness(1.08); }

.ui-button:disabled { opacity: 0.5; cursor: not-allowed; }
.ui-button__icon { display: inline-flex; width: 18px; height: 18px; }
.ui-button__icon :deep(svg) { width: 100%; height: 100%; }
.ui-button__label { overflow: hidden; text-overflow: ellipsis; }
</style>
