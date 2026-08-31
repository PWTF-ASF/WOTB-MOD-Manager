<template>
  <button class="ui-switch" :class="{ 'ui-switch--checked': modelValue }" type="button" role="switch" :aria-checked="modelValue" :aria-label="label" :disabled="disabled" @click="emit('update:modelValue', !modelValue)">
    <span />
  </button>
</template>

<script setup lang="ts">
withDefaults(defineProps<{ modelValue: boolean; label: string; disabled?: boolean }>(), { disabled: false })
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>

<style scoped>
.ui-switch { position:relative; width:38px; height:22px; margin:0; padding:0; border:1px solid var(--ui-border-strong); border-radius:999px; background:var(--ui-bg-app); cursor:pointer; transition:all var(--ui-duration-fast); }
.ui-switch span { position:absolute; top:3px; left:3px; width:14px; height:14px; border-radius:50%; background:var(--ui-text-muted); transition:transform var(--ui-duration-normal) var(--ui-ease), background var(--ui-duration-fast); }
.ui-switch--checked { border-color:var(--ui-accent); background:color-mix(in srgb, var(--ui-accent) 28%, var(--ui-bg-app)); }
.ui-switch--checked span { transform:translateX(16px); background:var(--ui-accent); }
.ui-switch:disabled { opacity:.45; cursor:not-allowed; }
</style>
