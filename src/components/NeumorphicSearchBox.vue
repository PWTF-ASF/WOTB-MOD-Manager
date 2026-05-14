<template>
  <div class="neu-search-box" :class="{ 'is-focused': isFocused }">
    <div class="neu-search-icon">
      <n-icon :component="SearchOutline" size="16" />
    </div>
    <input
      ref="inputRef"
      class="neu-search-input"
      type="text"
      :value="modelValue"
      :placeholder="placeholder"
      @input="onInput"
      @focus="onFocus"
      @blur="onBlur"
    />
    <button
      v-if="modelValue"
      class="neu-search-clear"
      @mousedown.prevent="onClear"
      title="清除"
    >
      <n-icon :component="CloseOutline" size="12" />
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { NIcon } from 'naive-ui'
import { SearchOutline, CloseOutline } from '@vicons/ionicons5'

defineProps({
  modelValue: { type: String, default: '' },
  placeholder: { type: String, default: '搜索...' },
})

const emit = defineEmits(['update:modelValue', 'focus'])

const inputRef = ref(null)
const isFocused = ref(false)

function onInput(e) {
  emit('update:modelValue', e.target.value)
}

function onFocus(e) {
  isFocused.value = true
  emit('focus', e)
}

function onBlur() {
  isFocused.value = false
}

function onClear() {
  emit('update:modelValue', '')
  inputRef.value?.focus()
}
</script>

<style scoped>
.neu-search-box {
  width: 100%;
  height: 44px;
  border-radius: var(--neu-radius-sm);
  background: var(--neu-inset);
  box-shadow:
    inset 3px 3px 8px var(--neu-shadow-dark),
    inset -3px -3px 8px var(--neu-shadow-light);
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  transition: box-shadow 0.3s ease;
}

.neu-search-box.is-focused {
  box-shadow:
    inset 3px 3px 10px var(--neu-shadow-dark),
    inset -3px -3px 10px var(--neu-shadow-light),
    0 0 0 2px var(--neu-shadow-accent);
}

.neu-search-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-dim);
  flex-shrink: 0;
  transition: color 0.3s ease;
}

.neu-search-box.is-focused .neu-search-icon {
  color: var(--accent);
}

.neu-search-input {
  flex: 1;
  min-width: 0;
  height: 100%;
  border: none;
  outline: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-main);
  caret-color: var(--accent);
}

.neu-search-input::placeholder {
  color: var(--text-dim);
}

.neu-search-clear {
  width: 20px;
  height: 20px;
  border: none;
  border-radius: 50%;
  background: var(--neu-shadow-dark);
  color: var(--text-dim);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.2s ease;
  padding: 0;
}

.neu-search-clear:hover {
  background: var(--neu-shadow-accent);
  color: var(--accent);
}
</style>
