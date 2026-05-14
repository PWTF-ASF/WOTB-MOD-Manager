import { useNotification, useDialog } from 'naive-ui'
import type { NotificationApiInjection } from 'naive-ui/es/notification/src/NotificationProvider'
import type { DialogApiInjection } from 'naive-ui/es/dialog/src/DialogProvider'

/**
 * 通知组合式函数
 * 封装 Naive UI 的 useNotification，提供便捷的通知方法
 */
export function useNotify() {
  let notification: NotificationApiInjection | null = null

  try {
    notification = useNotification()
  } catch {
    console.warn('useNotification 不可用，请确保 NNotificationProvider 已包裹应用')
  }

  const createNotification = (
    type: 'success' | 'error' | 'warning' | 'info',
    content: string,
    options?: { duration?: number; title?: string }
  ) => {
    if (!notification) {
      console.warn(`[通知] ${type}: ${content}`)
      return
    }

    const titleMap: Record<string, string> = {
      success: '成功',
      error: '错误',
      warning: '警告',
      info: '提示',
    }

    notification[type]({
      content,
      title: options?.title || titleMap[type],
      duration: options?.duration ?? (type === 'error' ? 4000 : 3000),
      keepAliveOnHover: true,
    })
  }

  return {
    success: (content: string, options?: { duration?: number; title?: string }) =>
      createNotification('success', content, options),
    error: (content: string, options?: { duration?: number; title?: string }) =>
      createNotification('error', content, options),
    warning: (content: string, options?: { duration?: number; title?: string }) =>
      createNotification('warning', content, options),
    info: (content: string, options?: { duration?: number; title?: string }) =>
      createNotification('info', content, options),
  }
}

/**
 * 确认对话框组合式函数
 * 封装 Naive UI 的 useDialog，返回 Promise<boolean>
 */
export function useConfirm() {
  let dialog: DialogApiInjection | null = null

  try {
    dialog = useDialog()
  } catch {
    console.warn('useDialog 不可用，请确保 NDialogProvider 已包裹应用')
  }

  const confirm = (
    content: string,
    options?: { title?: string; okLabel?: string; cancelLabel?: string; kind?: 'warning' | 'error' | 'info' }
  ): Promise<boolean> => {
    return new Promise((resolve) => {
      if (!dialog) {
        resolve(window.confirm(content))
        return
      }

      const kind = options?.kind || 'warning'

      dialog[kind]({
        title: options?.title || '确认',
        content,
        positiveText: options?.okLabel || '确定',
        negativeText: options?.cancelLabel || '取消',
        onPositiveClick: () => resolve(true),
        onNegativeClick: () => resolve(false),
        onClose: () => resolve(false),
      })
    })
  }

  return { confirm }
}
