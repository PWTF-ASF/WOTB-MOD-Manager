<template>
  <UiDialog :model-value="modelValue" title="部署进度" eyebrow="Deployment" :close-on-backdrop="!deploying" :close-on-escape="!deploying" @update:model-value="emit('update:modelValue',$event)">
    <div class="deploy-summary"><span>{{ current }} / {{ total }}</span><strong>{{ deploying ? currentName ? `正在部署：${currentName}` : '准备部署' : errors.length ? '部署完成，存在失败项' : '部署完成' }}</strong></div>
    <UiProgress :value="progress" />
    <div class="deploy-list">
      <div v-for="item in results" :key="item.name" class="deploy-row" :class="`deploy-row--${item.status}`">
        <span class="deploy-mark" aria-hidden="true"><UiSpinner v-if="item.status==='installing'" :size="13" /><template v-else-if="item.status==='success'">✓</template><template v-else-if="item.status==='error'">×</template><template v-else>·</template></span>
        <span>{{ item.name }}</span><em>{{ statusLabel(item.status) }}</em>
      </div>
    </div>
    <div v-if="errors.length" class="deploy-errors"><p v-for="error in errors" :key="error">{{ error }}</p></div>
    <template #footer><UiButton :disabled="deploying" variant="primary" @click="emit('update:modelValue',false)">{{ errors.length ? '知道了' : '完成' }}</UiButton></template>
  </UiDialog>
</template>
<script setup lang="ts">
import type { DeployItemResult, DeployItemStatus } from '@/types/deployment'
import UiDialog from '@/ui/overlay/UiDialog.vue'
import UiButton from '@/ui/button/UiButton.vue'
import UiProgress from '@/ui/feedback/UiProgress.vue'
import UiSpinner from '@/ui/feedback/UiSpinner.vue'
defineProps<{modelValue:boolean;deploying:boolean;current:number;total:number;currentName:string;progress:number;results:DeployItemResult[];errors:string[]}>()
const emit=defineEmits<{ 'update:modelValue':[value:boolean] }>()
const statusLabel=(status:DeployItemStatus)=>({pending:'等待中',installing:'部署中',success:'成功',error:'失败'}[status])
</script>
<style scoped>
.deploy-summary{display:flex;justify-content:space-between;gap:var(--ui-space-3);margin-bottom:var(--ui-space-3);color:var(--ui-text-muted);font-family:var(--ui-font-display);font-size:12px}.deploy-summary strong{overflow:hidden;color:var(--ui-text-secondary);text-overflow:ellipsis;white-space:nowrap}.deploy-list{display:grid;max-height:300px;margin-top:var(--ui-space-5);overflow:auto;border:1px solid var(--ui-border-subtle);border-radius:var(--ui-radius-md)}.deploy-row{display:grid;grid-template-columns:22px 1fr auto;align-items:center;gap:var(--ui-space-2);min-height:38px;padding:0 var(--ui-space-3);border-bottom:1px solid var(--ui-border-subtle);font-size:12px}.deploy-row:last-child{border-bottom:0}.deploy-row>span:nth-child(2){overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.deploy-row em{color:var(--ui-text-muted);font-style:normal}.deploy-row--success .deploy-mark,.deploy-row--success em{color:var(--ui-success)}.deploy-row--error .deploy-mark,.deploy-row--error em{color:var(--ui-danger)}.deploy-mark{display:grid;place-items:center}.deploy-errors{margin-top:var(--ui-space-4);padding:var(--ui-space-3);border:1px solid color-mix(in srgb,var(--ui-danger) 35%,transparent);border-radius:var(--ui-radius-md);color:var(--ui-danger);background:color-mix(in srgb,var(--ui-danger) 8%,transparent)}.deploy-errors p{margin:0;padding:2px 0;font-size:11px}
</style>
