<template><div class="ui-progress" role="progressbar" :aria-valuenow="value" aria-valuemin="0" aria-valuemax="100"><span :style="{ width: `${clamped}%` }" /></div></template>
<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps<{ value: number }>()
const clamped = computed(() => Math.min(100, Math.max(0, props.value)))
</script>
<style scoped>
.ui-progress { height:6px; overflow:hidden; border-radius:999px; background:var(--ui-bg-app); }
.ui-progress span { display:block; height:100%; border-radius:inherit; background:var(--ui-accent); transition:width var(--ui-duration-normal) var(--ui-ease); }
</style>
