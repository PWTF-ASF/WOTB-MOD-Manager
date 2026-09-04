import { describe, expect, it } from 'vitest'

import splashCapability from '../../src-tauri/capabilities/splash.json'
import tauriConfig from '../../src-tauri/tauri.conf.json'

describe('splash capability', () => {
  it('allows the splash window to subscribe to initialization events', () => {
    expect(splashCapability.identifier).toBe('splash')
    expect(splashCapability.windows).toEqual(['splash'])
    expect(splashCapability.permissions).toEqual([
      'core:event:allow-listen',
      'core:event:allow-unlisten',
    ])
    expect(tauriConfig.app.security.capabilities).toContain('splash')
  })
})
