<template>
  <Transition name="selection-dock">
    <div
      v-if="selectedCount > 0"
      class="selection-dock"
      role="toolbar"
      aria-label="批量管理 Mod"
      aria-live="polite"
    >
      <div class="selection-dock__count">
        <strong>{{ selectedCount }}</strong>
        <span>项已选择</span>
      </div>
      <span class="selection-dock__divider" aria-hidden="true"></span>
      <div class="selection-dock__actions">
        <UiButton size="small" variant="ghost" @click="emit('select-all')">
          {{ allSelected ? '取消全选' : '全选可见' }}
        </UiButton>
        <UiButton size="small" @click="emit('enable')">启用所选</UiButton>
        <UiButton size="small" @click="emit('disable')">禁用所选</UiButton>
        <UiButton size="small" variant="danger" @click="emit('delete')">删除</UiButton>
      </div>
    </div>
  </Transition>

  <footer class="selection-bar">
    <div class="change-summary" :class="{ 'change-summary--pending': pendingCount > 0 }">
      <strong>{{ pendingCount }}</strong>
      <span>{{ pendingCount > 0 ? '项改动待部署' : '当前没有待部署改动' }}</span>
    </div>
    <div class="primary-actions">
      <UiButton size="medium" @click="emit('add')">添加 Mod</UiButton>
      <UiButton size="medium" variant="primary" :loading="deploying" @click="emit('deploy')">部署改动</UiButton>
      <UiButton size="medium" variant="secondary" :loading="launching" @click="emit('launch')">启动游戏</UiButton>
    </div>
  </footer>
</template>
<script setup lang="ts">
import UiButton from '@/ui/button/UiButton.vue'
defineProps<{selectedCount:number;pendingCount:number;allSelected:boolean;deploying:boolean;launching:boolean}>()
const emit=defineEmits<{ 'select-all':[];enable:[];disable:[];delete:[];add:[];deploy:[];launch:[] }>()
</script>
<style scoped>
.selection-bar {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: var(--ui-space-4);
  min-height: 72px;
  box-sizing: border-box;
  padding: var(--ui-space-3) var(--ui-space-5);
  border-top: 1px solid var(--ui-border-subtle);
  background: var(--ui-bg-elevated);
  box-shadow: 0 -8px 24px rgba(0, 0, 0, .12);
}

.change-summary {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: 7px;
  color: var(--ui-text-muted);
  font-size: 12px;
}

.change-summary strong {
  color: inherit;
  font-family: var(--ui-font-display);
  font-size: 18px;
}

.change-summary--pending {
  color: var(--ui-warning);
}

.primary-actions {
  display: flex;
  align-items: center;
  gap: var(--ui-space-2);
}

.selection-dock {
  position: absolute;
  z-index: 30;
  bottom: 88px;
  left: 50%;
  display: flex;
  max-width: calc(100% - 32px);
  align-items: center;
  gap: var(--ui-space-3);
  box-sizing: border-box;
  padding: 9px 10px 9px 14px;
  border: 1px solid color-mix(in srgb, var(--ui-accent) 24%, var(--ui-border-strong));
  border-radius: var(--ui-radius-xl);
  background: color-mix(in srgb, var(--ui-bg-elevated) 90%, transparent);
  box-shadow: 0 16px 44px rgba(0, 0, 0, .28);
  backdrop-filter: blur(18px) saturate(120%);
  transform: translateX(-50%);
}

.selection-dock__count {
  display: flex;
  flex: none;
  align-items: baseline;
  gap: 5px;
  color: var(--ui-text-secondary);
  font-size: 12px;
  white-space: nowrap;
}

.selection-dock__count strong {
  color: var(--ui-accent);
  font-family: var(--ui-font-display);
  font-size: 20px;
  line-height: 1;
}

.selection-dock__divider {
  width: 1px;
  height: 22px;
  flex: none;
  background: var(--ui-border-subtle);
}

.selection-dock__actions {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: var(--ui-space-2);
}

.selection-dock-enter-active,
.selection-dock-leave-active {
  transition:
    opacity var(--ui-duration-normal) var(--ui-ease),
    transform var(--ui-duration-normal) var(--ui-ease);
}

.selection-dock-enter-from,
.selection-dock-leave-to {
  opacity: 0;
  transform: translate(-50%, 12px) scale(.97);
}

@media (max-width: 700px) {
  .selection-dock {
    width: calc(100% - 24px);
    justify-content: space-between;
  }

  .selection-dock__actions {
    overflow-x: auto;
  }

  .selection-bar {
    grid-template-columns: 1fr;
  }

  .change-summary { display: none; }
  .primary-actions { justify-content: flex-end; }
}
</style>
