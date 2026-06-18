import { listen } from "@tauri-apps/api/event"

interface InitProgress {
  step: number
  total: number
  message: string
  status: "running" | "done" | "error"
  error: string | null
}

const versionEl = document.getElementById("version-text")!
const progressBar = document.getElementById("progress-bar")!
const errorMsg = document.getElementById("error-msg")!
const mainSpinner = document.getElementById("main-spinner")!
const tasksEl = document.getElementById("tasks")!

versionEl.textContent = `v${import.meta.env.VITE_APP_VERSION || "--"}`

function updateTask(step: number, status: "running" | "done" | "error") {
  const item = tasksEl.querySelector(`[data-step="${step}"]`)
  if (!item) return
  item.className = `task-item ${status}`
}

let hasError = false

async function main() {
  try {
    const unlisten = await listen<InitProgress>("init-progress", (event) => {
      const { step, total, status, error } = event.payload

      // Update progress bar
      const pct = Math.round((step / total) * 100)
      progressBar.style.width = `${pct}%`

      // Update the task item
      updateTask(step, status)

      // Handle error
      if (status === "error" && error) {
        hasError = true
        errorMsg.textContent = error
        mainSpinner.style.borderTopColor = "#e0556a"
      }

      // All done
      if (status === "done" && step === total) {
        mainSpinner.style.display = "none"
        progressBar.style.background = "#7ecb8a"
      }
    })

    // Keep the listener alive
    ;(window as unknown as Record<string, unknown>).__splash_unlisten = unlisten
  } catch (err) {
    hasError = true
    errorMsg.textContent = `初始化失败: ${String(err)}`
    mainSpinner.style.borderTopColor = "#e0556a"
  }
}

main()
