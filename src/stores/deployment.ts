import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { listen } from '@tauri-apps/api/event'
import { modService } from '@/services/tauri/modService'
import { formatTauriError } from '@/services/tauri/errors'
import type { DeployItemResult, DeployProgressPayload } from '@/types/deployment'

export interface DeploymentSummary {
  succeeded: number
  failed: number
  commandError: string | null
}

export const useDeploymentStore = defineStore('deployment', () => {
  const isDeploying = ref(false)
  const showOverlay = ref(false)
  const current = ref(0)
  const total = ref(0)
  const currentName = ref('')
  const results = ref<DeployItemResult[]>([])
  const errors = ref<string[]>([])
  let stopListening: (() => void) | null = null

  const progressPercent = computed(() =>
    total.value > 0 ? Math.round((current.value / total.value) * 100) : 0,
  )

  function handleProgress(payload: DeployProgressPayload) {
    current.value = payload.current
    total.value = payload.total
    currentName.value = payload.mod_name
    if (!payload.mod_name) return

    const item = results.value.find(result => result.name === payload.mod_name)
    if (!item) return
    item.status = payload.status === 'done' ? 'success' : payload.status
  }

  async function deploy(filenames: string[]): Promise<DeploymentSummary> {
    if (isDeploying.value) {
      return { succeeded: 0, failed: 0, commandError: '部署任务已在进行中' }
    }

    isDeploying.value = true
    showOverlay.value = true
    current.value = 0
    total.value = filenames.length
    currentName.value = ''
    results.value = filenames.map(name => ({ name, status: 'pending' }))
    errors.value = []
    let commandError: string | null = null

    try {
      stopListening = await listen<DeployProgressPayload>('deploy-progress', event => {
        handleProgress(event.payload)
      })
      await modService.deploy(filenames)
    } catch (error) {
      commandError = formatTauriError(error)
    } finally {
      stopListening?.()
      stopListening = null
      isDeploying.value = false
    }

    const itemErrors = results.value
      .filter(result => result.status === 'error')
      .map(result => `${result.name}: 安装失败`)
    errors.value = commandError
      ? [`部署失败：${commandError}`, ...itemErrors]
      : itemErrors

    return {
      succeeded: results.value.filter(result => result.status === 'success').length,
      failed: results.value.filter(result => result.status === 'error').length,
      commandError,
    }
  }

  function closeOverlay() {
    if (!isDeploying.value) showOverlay.value = false
  }

  function dispose() {
    stopListening?.()
    stopListening = null
  }

  return {
    isDeploying,
    showOverlay,
    current,
    total,
    currentName,
    results,
    errors,
    progressPercent,
    deploy,
    closeOverlay,
    dispose,
  }
})
