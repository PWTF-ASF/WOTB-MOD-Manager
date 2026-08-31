<template>
  <label class="ui-checkbox" :class="{ 'ui-checkbox--disabled': disabled }">
    <input type="checkbox" :checked="modelValue" :disabled="disabled" @change="emit('update:modelValue', ($event.target as HTMLInputElement).checked)" />
    <span class="ui-checkbox__box" aria-hidden="true">
      <svg viewBox="0 0 16 16" fill="none"><path d="m3 8 3 3 7-7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" /></svg>
    </span>
    <span v-if="$slots.default" class="ui-checkbox__label"><slot /></span>
  </label>
</template>

<script setup lang="ts">
withDefaults(defineProps<{ modelValue: boolean; disabled?: boolean }>(), { disabled: false })
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>

<style scoped>
.ui-checkbox { display:inline-flex; align-items:center; gap:var(--ui-space-2); color:var(--ui-text-secondary); cursor:pointer; }
.ui-checkbox input { position:absolute; width:1px; height:1px; opacity:0; pointer-events:none; }
.ui-checkbox__box { display:grid; place-items:center; width:17px; height:17px; box-sizing:border-box; border:1px solid var(--ui-border-strong); border-radius:5px; color:white; background:var(--ui-bg-app); transition:all var(--ui-duration-fast); }
.ui-checkbox__box svg { width:13px; height:13px; opacity:0; transform:scale(.7); transition:all var(--ui-duration-fast); }
.ui-checkbox input:checked + .ui-checkbox__box { border-color:var(--ui-accent); background:var(--ui-accent); }
.ui-checkbox input:checked + .ui-checkbox__box svg { opacity:1; transform:scale(1); }
.ui-checkbox input:focus-visible + .ui-checkbox__box { box-shadow:var(--ui-focus-ring); }
.ui-checkbox--disabled { opacity:.5; cursor:not-allowed; }
.ui-checkbox__label { font-size:13px; }
</style>
