<template>
  <Teleport to="body">
    <div class="toast-host" aria-live="polite" aria-relevant="additions removals">
      <TransitionGroup name="toast">
        <article
          v-for="message in store.messages"
          :key="message.id"
          class="toast-item"
          :class="`toast-item--${message.tone}`"
          @mouseenter="store.pause(message.id)"
          @mouseleave="store.resume(message.id)"
        >
          <span class="toast-item__mark" aria-hidden="true" />
          <div class="toast-item__body">
            <strong>{{ message.title }}</strong>
            <p>{{ message.message }}</p>
          </div>
          <button class="toast-item__close" type="button" aria-label="关闭通知" @click="store.remove(message.id)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <path d="m6 6 12 12M18 6 6 18" />
            </svg>
          </button>
        </article>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useToastStore } from '@/stores/toast'

const store = useToastStore()
</script>

<style scoped>
.toast-host {
  position: fixed;
  top: var(--ui-space-5);
  right: var(--ui-space-5);
  z-index: 10000;
  display: grid;
  width: min(360px, calc(100vw - 32px));
  gap: var(--ui-space-3);
  pointer-events: none;
}

.toast-item {
  display: grid;
  grid-template-columns: 4px 1fr 28px;
  gap: var(--ui-space-3);
  min-height: 72px;
  padding: var(--ui-space-3);
  border: 1px solid var(--ui-border-subtle);
  border-radius: var(--ui-radius-lg);
  color: var(--ui-text-primary);
  background: var(--ui-bg-elevated);
  box-shadow: var(--ui-shadow-md);
  pointer-events: auto;
}

.toast-item__mark { border-radius: 999px; background: var(--ui-accent); }
.toast-item--success .toast-item__mark { background: var(--ui-success); }
.toast-item--warning .toast-item__mark { background: var(--ui-warning); }
.toast-item--error .toast-item__mark { background: var(--ui-danger); }
.toast-item__body { align-self: center; min-width: 0; }
.toast-item__body strong { display: block; margin-bottom: 4px; font-size: 13px; }
.toast-item__body p { margin: 0; padding: 0; color: var(--ui-text-secondary); font-size: 12px; line-height: 1.45; overflow-wrap: anywhere; }
.toast-item__close {
  display: grid;
  place-items: center;
  width: 28px;
  height: 28px;
  margin: 0;
  padding: 0;
  border: 0;
  border-radius: var(--ui-radius-sm);
  color: var(--ui-text-muted);
  background: transparent;
  cursor: pointer;
}
.toast-item__close:hover { color: var(--ui-text-primary); background: var(--ui-bg-surface-hover); }
.toast-item__close svg { width: 15px; height: 15px; }

.toast-enter-active, .toast-leave-active { transition: opacity var(--ui-duration-normal), transform var(--ui-duration-normal); }
.toast-enter-from, .toast-leave-to { opacity: 0; transform: translateX(16px); }

@media (max-width: 600px) {
  .toast-host { top: var(--ui-space-4); right: var(--ui-space-4); left: var(--ui-space-4); width: auto; }
}
</style>
