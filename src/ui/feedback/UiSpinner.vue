<template>
  <span class="ui-spinner" role="status" :aria-label="label" />
</template>

<script setup lang="ts">
withDefaults(defineProps<{ label?: string; size?: number }>(), {
  label: '正在加载',
  size: 16,
})
</script>

<style scoped>
.ui-spinner {
  display: inline-block;
  width: v-bind('`${size}px`');
  height: v-bind('`${size}px`');
  flex: 0 0 auto;
  border: 2px solid currentColor;
  border-right-color: transparent;
  border-radius: 50%;
  opacity: 0.82;
  animation: ui-spin 0.7s linear infinite;
}

@keyframes ui-spin {
  to { transform: rotate(360deg); }
}
</style>
