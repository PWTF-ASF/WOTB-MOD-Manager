import { ref } from 'vue'
import { defineStore } from 'pinia'

export interface ConfirmOptions {
  title?: string
  message: string
  confirmLabel?: string
  cancelLabel?: string
  tone?: 'default' | 'danger'
}

interface ConfirmRequest extends Required<ConfirmOptions> {
  resolve: (result: boolean) => void
}

const DEFAULTS = {
  title: '确认操作',
  confirmLabel: '确认',
  cancelLabel: '取消',
  tone: 'default' as const,
}

export const useConfirmStore = defineStore('confirm', () => {
  const current = ref<ConfirmRequest | null>(null)
  const queue: ConfirmRequest[] = []

  const showNext = () => {
    if (!current.value) current.value = queue.shift() ?? null
  }

  function request(options: ConfirmOptions): Promise<boolean> {
    return new Promise(resolve => {
      queue.push({ ...DEFAULTS, ...options, resolve })
      showNext()
    })
  }

  function finish(result: boolean) {
    const active = current.value
    if (!active) return
    current.value = null
    active.resolve(result)
    showNext()
  }

  return {
    current,
    request,
    confirm: () => finish(true),
    cancel: () => finish(false),
  }
})
