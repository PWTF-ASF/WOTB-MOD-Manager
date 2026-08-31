import { defineStore } from 'pinia'

export type ToastTone = 'success' | 'info' | 'warning' | 'error'

export interface ToastMessage {
  id: number
  tone: ToastTone
  title: string
  message: string
  duration: number
  createdAt: number
}

let nextToastId = 1
const timers = new Map<number, ReturnType<typeof setTimeout>>()

const DEFAULT_TITLES: Record<ToastTone, string> = {
  success: '操作成功',
  info: '提示',
  warning: '请注意',
  error: '操作失败',
}

export const useToastStore = defineStore('toast', {
  state: () => ({
    messages: [] as ToastMessage[],
  }),

  actions: {
    push(tone: ToastTone, message: string, options?: { title?: string; duration?: number }): number {
      const now = Date.now()
      const duplicate = this.messages.find(
        item => item.tone === tone && item.message === message && now - item.createdAt < 800,
      )
      if (duplicate) return duplicate.id

      const id = nextToastId++
      const duration = options?.duration ?? (tone === 'error' ? 5000 : 3200)
      this.messages.push({
        id,
        tone,
        title: options?.title ?? DEFAULT_TITLES[tone],
        message,
        duration,
        createdAt: now,
      })
      this.startTimer(id, duration)
      return id
    },

    remove(id: number) {
      const timer = timers.get(id)
      if (timer) clearTimeout(timer)
      timers.delete(id)
      this.messages = this.messages.filter(message => message.id !== id)
    },

    pause(id: number) {
      const timer = timers.get(id)
      if (timer) clearTimeout(timer)
      timers.delete(id)
    },

    resume(id: number) {
      const message = this.messages.find(item => item.id === id)
      if (message) this.startTimer(id, Math.min(message.duration, 1800))
    },

    startTimer(id: number, duration: number) {
      const previous = timers.get(id)
      if (previous) clearTimeout(previous)
      timers.set(id, setTimeout(() => this.remove(id), duration))
    },
  },
})
