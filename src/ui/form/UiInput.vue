<template>
  <label class="ui-field">
    <span v-if="label" class="ui-field__label">{{ label }}</span>
    <input
      class="ui-input"
      :value="modelValue"
      :type="type"
      :placeholder="placeholder"
      :disabled="disabled"
      :readonly="readonly"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <span v-if="hint" class="ui-field__hint">{{ hint }}</span>
  </label>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  modelValue?: string
  label?: string
  hint?: string
  type?: string
  placeholder?: string
  disabled?: boolean
  readonly?: boolean
}>(), {
  modelValue: '',
  type: 'text',
  disabled: false,
  readonly: false,
})

const emit = defineEmits<{ 'update:modelValue': [value: string] }>()
</script>

<style scoped>
.ui-field { display: grid; gap: var(--ui-space-2); color: var(--ui-text-primary); }
.ui-field__label { font-size: 13px; font-weight: 650; }
.ui-field__hint { color: var(--ui-text-muted); font-size: 12px; }
.ui-input {
  width: 100%;
  height: var(--ui-control-md);
  box-sizing: border-box;
  margin: 0;
  padding: 0 var(--ui-space-3);
  border: 1px solid var(--ui-border-subtle);
  border-radius: var(--ui-radius-md);
  color: var(--ui-text-primary);
  background: var(--ui-bg-app);
}
.ui-input:hover:not(:disabled) { border-color: var(--ui-border-strong); }
.ui-input:focus { border-color: var(--ui-accent); }
.ui-input:disabled { opacity: 0.55; cursor: not-allowed; }
</style>
