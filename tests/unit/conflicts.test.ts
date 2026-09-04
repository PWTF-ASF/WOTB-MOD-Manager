import { describe, expect, test } from 'vitest'
import { formatConflictMessage } from '@/features/mods/conflicts'

describe('Mod conflict messages', () => {
  test('shows conflicting mods and limits long path lists', () => {
    const message = formatConflictMessage([
      {
        first_mod: '豹一改模-1.zip',
        second_mod: '豹一改模-2.zip',
        paths: ['Data/a.dvpl', 'Data/b.dvpl', 'Data/c.dvpl', 'Data/d.dvpl'],
      },
    ])

    expect(message).toContain('豹一改模-1.zip ↔ 豹一改模-2.zip')
    expect(message).toContain('Data/a.dvpl、Data/b.dvpl、Data/c.dvpl')
    expect(message).toContain('另有 1 个文件')
  })
})
