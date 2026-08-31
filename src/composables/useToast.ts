import { useToastStore, type ToastTone } from '@/stores/toast'

export function useToast() {
  const store = useToastStore()
  const show = (
    tone: ToastTone,
    message: string,
    options?: { title?: string; duration?: number },
  ) => store.push(tone, message, options)

  return {
    show,
    success: (message: string, options?: { title?: string; duration?: number }) =>
      show('success', message, options),
    info: (message: string, options?: { title?: string; duration?: number }) =>
      show('info', message, options),
    warning: (message: string, options?: { title?: string; duration?: number }) =>
      show('warning', message, options),
    error: (message: string, options?: { title?: string; duration?: number }) =>
      show('error', message, options),
    dismiss: store.remove,
  }
}
