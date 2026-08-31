<template>
  <UiDialog :model-value="modelValue" :title="mode==='rename'?'重命名 Mod':'修改分类'" eyebrow="Edit mod" :close-on-backdrop="!loading" @update:model-value="emit('update:modelValue',$event)">
    <UiInput v-if="mode==='rename'" :model-value="value" label="显示名称" placeholder="输入新的显示名称" :disabled="loading" @update:model-value="emit('update:value',$event)" @keyup.enter="emit('confirm')" />
    <label v-else class="category-field"><span>Mod 分类</span><UiSelect :model-value="value || 'unknown'" :options="categoryOptions" label="Mod 分类" :disabled="loading" @update:model-value="emit('update:value',$event)" /></label>
    <template #footer><UiButton :disabled="loading" @click="emit('update:modelValue',false)">取消</UiButton><UiButton variant="primary" :loading="loading" @click="emit('confirm')">保存</UiButton></template>
  </UiDialog>
</template>
<script setup lang="ts">
import UiDialog from '@/ui/overlay/UiDialog.vue';import UiInput from '@/ui/form/UiInput.vue';import UiButton from '@/ui/button/UiButton.vue';import UiSelect from '@/ui/form/UiSelect.vue'
defineProps<{modelValue:boolean;mode:'rename'|'category';value:string;loading:boolean}>();const emit=defineEmits<{ 'update:modelValue':[value:boolean];'update:value':[value:string];confirm:[] }>()
const categoryOptions=[{value:'model',label:'3D 模型'},{value:'voice',label:'语音包'},{value:'ui',label:'UI'},{value:'lightIcon',label:'点亮图标'},{value:'script',label:'扩展脚本'},{value:'map',label:'地图纹理'},{value:'unknown',label:'未知'}]
</script>
<style scoped>.category-field{display:grid;gap:var(--ui-space-2);color:var(--ui-text-primary);font-size:13px;font-weight:650}</style>
