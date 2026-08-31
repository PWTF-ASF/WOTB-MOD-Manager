<template>
  <button class="ui-icon-button" :class="`ui-icon-button--${size}`" type="button" :disabled="disabled" :aria-label="label">
    <slot />
  </button>
</template>

<script setup lang="ts">
withDefaults(defineProps<{ label: string; size?: 'small' | 'medium'; disabled?: boolean }>(), {
  size: 'medium',
  disabled: false,
})
</script>

<style scoped>
.ui-icon-button { display:grid; place-items:center; margin:0; padding:0; border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-md); color:var(--ui-text-secondary); background:var(--ui-bg-elevated); cursor:pointer; transition:all var(--ui-duration-fast) var(--ui-ease); }
.ui-icon-button--small { width:30px; height:30px; }
.ui-icon-button--medium { width:var(--ui-control-md); height:var(--ui-control-md); }
.ui-icon-button:hover:not(:disabled) { color:var(--ui-text-primary); border-color:var(--ui-accent); background:var(--ui-bg-surface-hover); }
.ui-icon-button:disabled { opacity:.45; cursor:not-allowed; }
.ui-icon-button :deep(svg) { display:block; width:18px; height:18px; overflow:visible; stroke-linecap:round; stroke-linejoin:round; }
</style>
