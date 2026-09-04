// @vitest-environment jsdom

import { describe, expect, test } from 'vitest'
import { mount } from '@vue/test-utils'
import ModCard from '@/features/mods/components/ModCard.vue'
import ModToolbar from '@/features/mods/components/ModToolbar.vue'
import ModSelectionBar from '@/features/mods/components/ModSelectionBar.vue'
import type { ModLibraryItem } from '@/stores/modLibrary'

const item: ModLibraryItem = {
  id: 'voice.zip',
  filename: 'voice.zip',
  displayName: '中文语音包',
  category: 'voice',
  iconPath: null,
  installDate: null,
  conflicts: [],
  deployed: true,
  desiredEnabled: false,
  selected: false,
}

describe('mod view components', () => {
  test('ModCard exposes selection and desired-state changes without mutating its prop', async () => {
    const wrapper = mount(ModCard, {
      props: { item: { ...item }, iconUrl: null, categoryLabel: '语音包' },
    })

    expect(wrapper.text()).toContain('待部署')
    await wrapper.get('input[type="checkbox"]').setValue(true)
    await wrapper.get('[role="switch"]').trigger('click')

    expect(wrapper.emitted('update:selected')?.[0]).toEqual([true])
    expect(wrapper.emitted('update:enabled')?.[0]).toEqual([true])
    expect(wrapper.props('item').selected).toBe(false)
  })

  test('ModToolbar emits search, category and layout changes', async () => {
    const wrapper = mount(ModToolbar, {
      props: {
        search: '',
        category: 'all',
        categories: [
          { type: 'all', name: '全部' },
          { type: 'voice', name: '语音包' },
        ],
        isGrid: true,
        totalCount: 4,
        visibleCount: 2,
      },
    })

    await wrapper.get('input[type="search"]').setValue('语音')
    await wrapper.get('[aria-label="Mod 分类"]').trigger('click')
    const voiceOption = wrapper.findAll('[role="option"]').find(option => option.text().includes('语音包'))
    await voiceOption?.trigger('click')
    await wrapper.get('[aria-label="切换为列表布局"]').trigger('click')

    expect(wrapper.emitted('update:search')?.[0]).toEqual(['语音'])
    expect(wrapper.emitted('update:category')?.[0]).toEqual(['voice'])
    expect(wrapper.emitted('toggle-layout')).toHaveLength(1)
  })

  test('ModSelectionBar keeps all batch actions available', async () => {
    const wrapper = mount(ModSelectionBar, {
      props: {
        selectedCount: 2,
        pendingCount: 1,
        allSelected: false,
        deploying: false,
        launching: false,
      },
    })

    expect(wrapper.text()).toContain('启用所选')
    expect(wrapper.text()).toContain('禁用所选')
    expect(wrapper.find('.selection-dock').exists()).toBe(true)
    await wrapper.get('.selection-dock__actions button:nth-child(2)').trigger('click')
    await wrapper.get('.selection-dock__actions button:nth-child(3)').trigger('click')
    await wrapper.get('.selection-dock__actions button:nth-child(4)').trigger('click')

    expect(wrapper.emitted('enable')).toHaveLength(1)
    expect(wrapper.emitted('disable')).toHaveLength(1)
    expect(wrapper.emitted('delete')).toHaveLength(1)
  })

  test('ModSelectionBar only shows the floating dock after selecting a Mod', async () => {
    const wrapper = mount(ModSelectionBar, {
      props: {
        selectedCount: 0,
        pendingCount: 0,
        allSelected: false,
        deploying: false,
        launching: false,
      },
    })

    expect(wrapper.find('.selection-dock').exists()).toBe(false)
    await wrapper.setProps({ selectedCount: 1 })
    expect(wrapper.find('.selection-dock').exists()).toBe(true)
  })
})
