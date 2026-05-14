import { useNotification } from 'naive-ui'
import type { NotificationApiInjection } from 'naive-ui/es/notification/src/NotificationProvider'

/**
 * 通知组合式函数
 * 封装 Naive UI 的 useNotification，提供便捷的通知方法
 *
 * 使用方式:
 *   const notify = useNotify()
 *   notify.success('操作成功')
 *   notify.error('操作失败')
 *   notify.warning('请注意')
 *   notify.info('这是一条提示')
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
