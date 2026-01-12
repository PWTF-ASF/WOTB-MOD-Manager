<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <h2 class="page-title">系统设置 <span class="sub-title">SYSTEM CONFIGURATION</span></h2>

      <!-- 设置组：外观 -->
      <section class="setting-group">
        <h3 class="group-title">外观 / VISUALS</h3>

        <!-- 明暗模式开关已删除 -->

        <div class="setting-item">
          <div class="text-info">
            <span class="label">启用毛玻璃特效 (Acrylic Blur)</span>
            <span class="desc">开启后背景将呈现模糊透视效果，可能会轻微影响性能。</span>
          </div>
          <label class="switch">
            <input type="checkbox" v-model="globalBlur" />
            <span class="slider round"></span>
          </label>
        </div>
      </section>

      <!-- 设置组：路径 -->
      <section class="setting-group">
        <h3 class="group-title">路径配置 / PATHS</h3>

        <!-- 游戏路径 -->
        <div class="setting-item vertical">
          <div class="label-row">
            <span class="label">游戏安装路径</span>
            <span class="desc-sm">World of Tanks Blitz 的主程序目录</span>
          </div>
          <div class="input-row">
            <input type="text" v-model="config.gamePath" placeholder="例如: C:\Games\World_of_Tanks_Blitz" readonly />
            <button class="btn-browse" @click="selectPath('game')">浏览</button>
          </div>
        </div>

        <!-- MOD保存路径 -->
        <div class="setting-item vertical">
          <div class="label-row">
            <span class="label">Mod 存储库路径</span>
            <span class="desc-sm">下载的 Mod 文件将保存在此位置</span>
          </div>
          <div class="input-row">
            <input type="text" v-model="config.modPath" placeholder="选择文件夹..." readonly />
            <button class="btn-browse" @click="selectPath('mod')">浏览</button>
          </div>
        </div>
      </section>

      <!-- 底部操作区 -->
      <div class="action-footer">
        <button class="btn-reset" @click="resetToDefaults"><span class="icon">↺</span> 重置为默认设置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, type Ref, inject } from 'vue'

// 1. 注入全局毛玻璃状态
// 这里的类型断言确保 TypeScript 知道这是一个 ref
const globalBlur = inject('GlobalBlur') as Ref<boolean>

// 模拟配置数据结构
const config = reactive({
  gamePath: 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\World of Tanks Blitz',
  modPath: 'D:\\WOTB_Mods\\Library',
})

// 模拟选择路径功能
const selectPath = (type: 'game' | 'mod') => {
  console.log(`Open file dialog for: ${type}`)
}

// 重置功能
const resetToDefaults = () => {
  if (confirm('确定要重置所有设置吗？此操作无法撤销。')) {
    config.gamePath = ''
    config.modPath = ''
  }
}
</script>

<style scoped>
/* ================= 页面容器 ================= */
.settings-container {
  width: 100%;
  height: 100%;
  padding: 40px;
  box-sizing: border-box;
  overflow-y: auto;
  backdrop-filter: blur(var(--global-blur)) saturate(180%);
  -webkit-backdrop-filter: blur(var(--global-blur)) saturate(180%);
  color: var(--text-main);
  transition: all 0.3s var(--animation-timing);
  animation: page-slide-in 0.4s var(--animation-timing) backwards;
  position: relative;
}

.settings-container::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0.3;
}

.content-wrapper {
  max-width: 800px;
  margin: 0 auto;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.1s backwards;
}

/* ================= 标题样式 ================= */
.page-title {
  font-family: 'Rajdhani', sans-serif;
  font-size: 32px;
  font-weight: 800;
  text-transform: uppercase;
  border-bottom: 3px solid var(--accent);
  padding-bottom: 12px;
  margin-bottom: 40px;
  letter-spacing: 3px;
  color: var(--text-main);
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 12px;
}

.page-title::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border);
  margin-bottom: 5px;
  opacity: 0.3;
}

.sub-title {
  font-size: 14px;
  color: var(--text-dim);
  font-weight: 600;
  letter-spacing: 2px;
  margin-bottom: 8px;
  opacity: 0.8;
}

/* ================= 设置组 ================= */
.setting-group {
  margin-bottom: 50px;
  animation: slide-up-fade 0.5s var(--animation-timing) backwards;
}

.setting-group:nth-child(1) {
  animation-delay: 0.15s;
}
.setting-group:nth-child(2) {
  animation-delay: 0.2s;
}

.group-title {
  font-size: 14px;
  color: var(--accent);
  margin-bottom: 20px;
  letter-spacing: 1px;
  font-weight: 700;
  text-transform: uppercase;
  position: relative;
  padding-left: 12px;
  display: inline-block;
}

.group-title::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 16px;
  background: var(--accent);
  border-radius: 2px;
  box-shadow: 0 0 8px var(--accent-glow);
}

/* ================= 设置项 ================= */
.setting-item {
  background: var(--bg-card);
  border: 1px solid var(--border);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  padding: 24px;
  margin-bottom: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.3s var(--animation-timing);
  border-radius: 12px;
  position: relative;
  overflow: hidden;
  animation: fade-scale-in 0.4s var(--animation-timing) backwards;
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
}

.setting-item:nth-child(1) {
  animation-delay: 0.25s;
}
.setting-item:nth-child(2) {
  animation-delay: 0.3s;
}

.setting-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--accent), transparent);
  opacity: 0;
  transition: opacity 0.3s;
}

.setting-item:hover {
  border-color: var(--accent);
  background: var(--bg-card-hover);
  box-shadow: 0 8px 24px var(--accent-glow);
  transform: translateX(4px);
}

.setting-item:hover::before {
  opacity: 1;
}

.setting-item.vertical {
  flex-direction: column;
  align-items: flex-start;
  gap: 16px;
}

/* 文本信息 */
.text-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.label {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-main);
  display: flex;
  align-items: center;
  gap: 8px;
}

.label::before {
  content: '•';
  color: var(--accent);
  font-size: 24px;
}

.desc {
  font-size: 13px;
  color: var(--text-dim);
  line-height: 1.5;
  max-width: 500px;
}

.label-row {
  display: flex;
  align-items: baseline;
  width: 100%;
  gap: 8px;
}

.desc-sm {
  font-size: 12px;
  color: var(--text-dim);
  opacity: 0.8;
}

/* 输入框和按钮行 - 优化明暗模式 */
.input-row {
  display: flex;
  width: 100%;
  gap: 12px;
}

input[type='text'] {
  flex: 1;
  background: var(--bg-input);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 12px 16px;
  font-family: 'Rajdhani', sans-serif;
  font-size: 14px;
  letter-spacing: 0.5px;
  outline: none;
  transition: all 0.3s var(--animation-timing);
  border-radius: 8px;
  backdrop-filter: blur(5px);
}

input[type='text']:hover {
  border-color: var(--accent);
  background: var(--bg-input-focus);
}

input[type='text']:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-glow);
  background: var(--bg-input-focus);
}

.btn-browse {
  background: var(--glass-effect);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 0 24px;
  font-family: 'Rajdhani', sans-serif;
  font-weight: 700;
  cursor: pointer;
  text-transform: uppercase;
  transition: all 0.3s var(--animation-timing);
  border-radius: 8px;
  font-size: 12px;
  letter-spacing: 1px;
  position: relative;
  overflow: hidden;
  height: 44px;
}

.btn-browse::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: var(--accent);
  transform: translate(-50%, -50%);
  transition:
    width 0.6s,
    height 0.6s;
  opacity: 0.1;
}

.btn-browse:hover {
  border-color: var(--accent);
  color: var(--text-accent);
  background: var(--glass-effect-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px var(--accent-glow);
}

.btn-browse:hover::before {
  width: 200%;
  height: 200%;
}

.btn-browse:active {
  transform: scale(0.96);
}

/* ================= 开关样式 ================= */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
  flex-shrink: 0;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--switch-track);
  border: 1px solid var(--switch-border);
  transition: 0.4s var(--animation-timing);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

.slider:before {
  position: absolute;
  content: '';
  height: 18px;
  width: 18px;
  left: 2px;
  bottom: 2px;
  background-color: var(--switch-thumb);
  transition: 0.4s var(--animation-timing);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
}

/* 选中状态 */
input:checked + .slider {
  background-color: var(--switch-track-checked);
  border-color: var(--switch-border-checked);
  box-shadow:
    inset 0 2px 4px rgba(0, 0, 0, 0.1),
    0 0 8px var(--accent-glow);
}

input:checked + .slider:before {
  transform: translateX(26px);
  background-color: var(--switch-thumb-checked);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

/* 圆角 */
.slider.round {
  border-radius: 24px;
}

.slider.round:before {
  border-radius: 50%;
}

.switch:hover .slider {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent-glow);
}

/* ================= 底部操作区 ================= */
.action-footer {
  margin-top: 60px;
  display: flex;
  justify-content: flex-end;
  border-top: 1px solid var(--border);
  padding-top: 24px;
  animation: slide-up-fade 0.5s var(--animation-timing) 0.25s backwards;
}

.btn-reset {
  background: var(--glass-effect);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 12px 28px;
  font-family: 'Rajdhani', sans-serif;
  font-weight: 700;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.3s var(--animation-timing);
  font-size: 14px;
  border-radius: 8px;
  text-transform: uppercase;
  letter-spacing: 1px;
  position: relative;
  overflow: hidden;
}

.btn-reset::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: #ef4444;
  transform: translate(-50%, -50%);
  transition:
    width 0.6s,
    height 0.6s;
  opacity: 0.1;
}

.btn-reset:hover {
  border-color: #ef4444;
  color: #ef4444;
  background: var(--glass-effect-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.2);
}

.btn-reset:hover::before {
  width: 200%;
  height: 200%;
}

.btn-reset:active {
  transform: scale(0.96);
}

.btn-reset .icon {
  font-size: 18px;
  transition: transform 0.6s var(--animation-timing);
}

.btn-reset:hover .icon {
  transform: rotate(180deg);
}

/* ================= 滚动条美化 ================= */
.settings-container::-webkit-scrollbar {
  width: 8px;
}

.settings-container::-webkit-scrollbar-track {
  background: var(--scroll-track, transparent);
  border-radius: 4px;
  margin: 4px 0;
}

.settings-container::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 4px;
  transition: background 0.3s;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}

/* ================= 响应式设计 ================= */
@media (max-width: 900px) {
  .settings-container {
    padding: 30px;
  }

  .content-wrapper {
    max-width: 100%;
  }

  .page-title {
    font-size: 28px;
  }

  .setting-item {
    padding: 20px;
  }
}

@media (max-width: 768px) {
  .settings-container {
    padding: 20px;
  }

  .page-title {
    font-size: 24px;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .page-title::after {
    display: none;
  }

  .sub-title {
    font-size: 12px;
  }

  .label-row {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .desc-sm {
    margin-left: 0;
  }

  .input-row {
    flex-direction: column;
  }

  .btn-browse {
    width: 100%;
    justify-content: center;
  }

  .action-footer {
    justify-content: center;
  }
}

@media (max-width: 480px) {
  .settings-container {
    padding: 16px;
  }

  .setting-item {
    padding: 16px;
  }

  .text-info {
    gap: 4px;
  }

  .label {
    font-size: 14px;
  }

  .desc {
    font-size: 12px;
  }

  .btn-reset {
    width: 100%;
    justify-content: center;
  }
}

/* ================= 明暗模式特定的调整 ================= */
:global(.light-mode) .settings-container {
  background: rgba(255, 255, 255, 0.95);
}

:global(.light-mode) .setting-item {
  background: rgba(255, 255, 255, 0.9);
}

:global(.light-mode) input[type='text'] {
  background: rgba(255, 255, 255, 0.95);
}

:global(.light-mode) input[type='text']:focus {
  background: rgba(255, 255, 255, 0.98);
}

:global(.light-mode) .btn-browse {
  background: rgba(255, 255, 255, 0.8);
}

:global(.light-mode) .btn-browse:hover {
  background: rgba(255, 255, 255, 0.9);
}

:global(.light-mode) .btn-reset {
  background: rgba(255, 255, 255, 0.8);
}

:global(.light-mode) .btn-reset:hover {
  background: rgba(255, 255, 255, 0.9);
}

/* ================= 性能优化 ================= */
.setting-item,
.switch,
.btn-browse,
.btn-reset,
input[type='text'] {
  will-change: transform, box-shadow, border-color;
}
</style>
