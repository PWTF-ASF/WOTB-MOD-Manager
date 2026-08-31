export type DeployItemStatus = 'pending' | 'installing' | 'success' | 'error'

export interface DeployProgressPayload {
  current: number
  total: number
  mod_name: string
  status: 'installing' | 'done' | 'error'
}

export interface DeployItemResult {
  name: string
  status: DeployItemStatus
  error?: string
}
