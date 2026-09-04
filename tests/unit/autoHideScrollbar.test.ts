// @vitest-environment jsdom

import { afterEach, describe, expect, it, vi } from 'vitest'

import { attachAutoHideScrollbar } from '@/ui/directives/autoHideScrollbar'

describe('auto-hiding scrollbar', () => {
  afterEach(() => vi.useRealTimers())

  it('reveals while scrolling and hides again after the idle delay', () => {
    vi.useFakeTimers()
    const element = document.createElement('div')
    const cleanup = attachAutoHideScrollbar(element, 700)

    expect(element.classList.contains('ui-auto-hide-scrollbar')).toBe(true)
    expect(element.classList.contains('is-scrolling')).toBe(false)

    element.dispatchEvent(new Event('scroll'))
    expect(element.classList.contains('is-scrolling')).toBe(true)

    vi.advanceTimersByTime(699)
    expect(element.classList.contains('is-scrolling')).toBe(true)

    vi.advanceTimersByTime(1)
    expect(element.classList.contains('is-scrolling')).toBe(false)

    cleanup()
    expect(element.classList.contains('ui-auto-hide-scrollbar')).toBe(false)
  })
})
