import type { ModConflict } from '@/types/mod'

export function formatConflictMessage(conflicts: ModConflict[]): string {
  const details = conflicts.slice(0, 4).map(conflict => {
    const paths = conflict.paths.slice(0, 3).join('、')
    const remainingPaths = conflict.paths.length - 3
    const suffix = remainingPaths > 0 ? `，另有 ${remainingPaths} 个文件` : ''
    return `• ${conflict.first_mod} ↔ ${conflict.second_mod}\n  ${paths}${suffix}`
  })
  const remainingConflicts = conflicts.length - details.length
  if (remainingConflicts > 0) details.push(`• 另有 ${remainingConflicts} 组冲突`)
  return `以下 Mod 会写入相同路径但文件内容不同，无法同时部署：\n\n${details.join('\n')}`
}
