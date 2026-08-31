export class TauriServiceError extends Error {
  constructor(
    public readonly operation: string,
    public readonly cause: unknown,
  ) {
    super(formatTauriError(cause))
    this.name = 'TauriServiceError'
  }
}

export function formatTauriError(error: unknown): string {
  if (error instanceof Error) return error.message
  if (typeof error === 'string') return error

  try {
    return JSON.stringify(error)
  } catch {
    return '发生未知错误'
  }
}

export async function runTauriOperation<T>(operation: string, task: () => Promise<T>): Promise<T> {
  try {
    return await task()
  } catch (error) {
    throw new TauriServiceError(operation, error)
  }
}
