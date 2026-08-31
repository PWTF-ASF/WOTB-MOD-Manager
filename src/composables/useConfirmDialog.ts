import { useConfirmStore, type ConfirmOptions } from '@/stores/confirm'

export function useConfirmDialog() {
  const store = useConfirmStore()
  return {
    confirm: (options: ConfirmOptions) => store.request(options),
  }
}
