<template>
  <div ref="root" class="ui-select" :class="{ 'ui-select--open': open, 'ui-select--disabled': disabled }">
    <button
      class="ui-select__trigger"
      type="button"
      :disabled="disabled"
      :aria-label="label"
      aria-haspopup="listbox"
      :aria-expanded="open"
      @click="open = !open"
      @keydown.down.prevent="move(1)"
      @keydown.up.prevent="move(-1)"
      @keydown.esc="open = false"
    >
      <span>{{ selectedLabel }}</span>
      <svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
        <path d="m6 8 4 4 4-4" />
      </svg>
    </button>

    <Transition name="select-menu">
      <div v-if="open" class="ui-select__menu" role="listbox" :aria-label="label">
        <button
          v-for="option in options"
          :key="option.value"
          type="button"
          role="option"
          :aria-selected="option.value === modelValue"
          :class="{ selected: option.value === modelValue }"
          @click="select(option.value)"
        >
          <span>{{ option.label }}</span>
          <svg v-if="option.value === modelValue" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="m5 10 3 3 7-7" /></svg>
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

const props = withDefaults(defineProps<{
  modelValue: string
  options: Array<{ value: string; label: string }>
  label: string
  disabled?: boolean
}>(), { disabled: false })
const emit = defineEmits<{ 'update:modelValue': [value: string] }>()
const root = ref<HTMLElement | null>(null)
const open = ref(false)
const selectedLabel = computed(() => props.options.find(option => option.value === props.modelValue)?.label ?? '请选择')

const select = (value: string) => {
  emit('update:modelValue', value)
  open.value = false
}
const move = (direction: number) => {
  if (!open.value) {
    open.value = true
    return
  }
  const current = props.options.findIndex(option => option.value === props.modelValue)
  const next = (current + direction + props.options.length) % props.options.length
  if (props.options[next]) emit('update:modelValue', props.options[next].value)
}
const closeOnOutsideClick = (event: PointerEvent) => {
  if (!root.value?.contains(event.target as Node)) open.value = false
}

onMounted(() => document.addEventListener('pointerdown', closeOnOutsideClick))
onBeforeUnmount(() => document.removeEventListener('pointerdown', closeOnOutsideClick))
</script>

<style scoped>
.ui-select { position:relative; min-width:0; }
.ui-select__trigger { width:100%; height:var(--ui-control-md); display:flex; align-items:center; justify-content:space-between; gap:var(--ui-space-3); padding:0 11px 0 13px; border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-md); color:var(--ui-text-primary); background:var(--ui-bg-elevated); font:500 13px var(--ui-font-body); cursor:pointer; transition:border-color var(--ui-duration-fast),background var(--ui-duration-fast),box-shadow var(--ui-duration-fast); }
.ui-select__trigger:hover { border-color:var(--ui-border-strong); background:var(--ui-bg-surface-hover); }
.ui-select__trigger:focus-visible,.ui-select--open .ui-select__trigger { border-color:var(--ui-accent); box-shadow:var(--ui-focus-ring); outline:0; }
.ui-select__trigger svg { width:16px; height:16px; flex:none; color:var(--ui-text-muted); transition:transform var(--ui-duration-normal) var(--ui-ease); }
.ui-select--open .ui-select__trigger svg { transform:rotate(180deg); }
.ui-select__menu { position:absolute; z-index:80; top:calc(100% + 6px); left:0; width:100%; max-height:260px; overflow:auto; box-sizing:border-box; padding:5px; border:1px solid var(--ui-border-subtle); border-radius:var(--ui-radius-lg); background:var(--ui-bg-elevated); box-shadow:var(--ui-shadow-md); }
.ui-select__menu button { width:100%; min-height:34px; display:flex; align-items:center; justify-content:space-between; gap:8px; padding:0 9px; border:0; border-radius:var(--ui-radius-md); color:var(--ui-text-secondary); background:transparent; font:500 13px var(--ui-font-body); text-align:left; cursor:pointer; }
.ui-select__menu button:hover { color:var(--ui-text-primary); background:var(--ui-bg-surface-hover); }
.ui-select__menu button.selected { color:var(--ui-accent); background:color-mix(in srgb,var(--ui-accent) 12%,transparent); font-weight:650; }
.ui-select__menu button svg { width:15px; height:15px; flex:none; }
.ui-select--disabled { opacity:.5; }
.select-menu-enter-active,.select-menu-leave-active { transition:opacity var(--ui-duration-fast),transform var(--ui-duration-fast); transform-origin:top; }
.select-menu-enter-from,.select-menu-leave-to { opacity:0; transform:translateY(-4px) scale(.98); }
</style>
