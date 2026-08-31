<template>
  <RouterView />
  <ConfirmHost />
  <ToastHost />
</template>

<script setup lang="ts">
import ToastHost from '@/ui/feedback/ToastHost.vue'
import ConfirmHost from '@/ui/overlay/ConfirmHost.vue'
</script>
