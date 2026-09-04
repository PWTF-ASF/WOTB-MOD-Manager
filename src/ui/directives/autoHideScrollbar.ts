import type { Directive } from 'vue'

const ACTIVE_CLASS = 'is-scrolling'
const SCROLLBAR_CLASS = 'ui-auto-hide-scrollbar'
const IDLE_DELAY = 700
const cleanupByElement = new WeakMap<HTMLElement, () => void>()

export const attachAutoHideScrollbar = (element: HTMLElement, idleDelay = IDLE_DELAY) => {
  let idleTimer: number | undefined

  const hide = () => {
    element.classList.remove(ACTIVE_CLASS)
    idleTimer = undefined
  }

  const reveal = () => {
    element.classList.add(ACTIVE_CLASS)
    if (idleTimer !== undefined) window.clearTimeout(idleTimer)
    idleTimer = window.setTimeout(hide, idleDelay)
  }

  element.classList.add(SCROLLBAR_CLASS)
  element.addEventListener('scroll', reveal, { passive: true })

  return () => {
    if (idleTimer !== undefined) window.clearTimeout(idleTimer)
    element.removeEventListener('scroll', reveal)
    element.classList.remove(SCROLLBAR_CLASS, ACTIVE_CLASS)
  }
}

export const vAutoHideScrollbar: Directive<HTMLElement> = {
  mounted(element) {
    cleanupByElement.set(element, attachAutoHideScrollbar(element))
  },
  unmounted(element) {
    cleanupByElement.get(element)?.()
    cleanupByElement.delete(element)
  },
}
