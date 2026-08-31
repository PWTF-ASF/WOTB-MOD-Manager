import { beforeEach, describe, expect, test } from 'vitest'
import router from '@/router'

describe('application routes', () => {
  beforeEach(async () => {
    await router.push('/library')
  })

  test('redirects the application root to the library', async () => {
    await router.push('/')
    expect(router.currentRoute.value.name).toBe('library')
  })

  test('navigates between the library and settings pages', async () => {
    expect(router.currentRoute.value.path).toBe('/library')

    await router.push('/settings')
    expect(router.currentRoute.value.name).toBe('settings')
  })

  test('redirects unknown locations to the library', async () => {
    await router.push('/missing-page')
    expect(router.currentRoute.value.path).toBe('/library')
  })
})
