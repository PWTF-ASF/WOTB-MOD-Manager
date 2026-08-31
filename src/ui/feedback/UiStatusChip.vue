<template><span class="ui-status" :class="`ui-status--${tone}`"><slot /></span></template>
<script setup lang="ts">
withDefaults(defineProps<{ tone?: 'neutral' | 'info' | 'pending' | 'success' | 'warning' | 'danger' }>(), { tone: 'neutral' })
</script>
<style scoped>
.ui-status { display:inline-flex; align-items:center; min-height:22px; padding:0 8px; border:1px solid var(--ui-border-subtle); border-radius:999px; color:var(--ui-text-secondary); background:var(--ui-bg-app); font-size:11px; font-weight:650; white-space:nowrap; }
.ui-status--info { color:var(--ui-accent); border-color:color-mix(in srgb,var(--ui-accent) 45%,transparent); }
.ui-status--pending,.ui-status--warning { color:var(--ui-warning); border-color:color-mix(in srgb,var(--ui-warning) 45%,transparent); }
.ui-status--success { color:var(--ui-success); border-color:color-mix(in srgb,var(--ui-success) 45%,transparent); }
.ui-status--danger { color:var(--ui-danger); border-color:color-mix(in srgb,var(--ui-danger) 45%,transparent); }
</style>
