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
            <input type="checkbox" v-model="globalBlur">
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
        <button class="btn-reset" @click="resetToDefaults">
          <span class="icon">↺</span> 重置为默认设置
        </button>
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
  modPath: 'D:\\WOTB_Mods\\Library'
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
/* 组件样式 */
.settings-container {
  width: 100%;
  height: 100%;
  padding: 40px;
  box-sizing: border-box;
  overflow-y: auto;

  /* 使用全局变量作为背景色 */
  background: var(--bg-main);

  /* 如果启用了毛玻璃，这里稍微调整透明度逻辑 */
  backdrop-filter: blur(var(--global-blur));
  -webkit-backdrop-filter: blur(var(--global-blur));
  color: var(--text-main);
  transition: background 0.3s ease, color 0.3s ease;
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.content-wrapper {
  max-width: 800px;
  margin: 0 auto;
}

/* 标题样式 */
.page-title {
  font-family: 'Rajdhani', sans-serif;
  font-size: 28px;
  font-weight: 700;
  text-transform: uppercase;
  border-bottom: 2px solid var(--accent);
  padding-bottom: 10px;
  margin-bottom: 30px;
  letter-spacing: 2px;
  color: var(--text-main);
}

.sub-title {
  font-size: 14px;
  color: var(--text-dim);
  margin-left: 10px;
  font-weight: 500;
}

/* 分组样式 */
.setting-group {
  margin-bottom: 40px;
}

.group-title {
  font-size: 14px;
  color: var(--accent);
  margin-bottom: 15px;
  letter-spacing: 1px;
  font-weight: 700;
  opacity: 0.9;
}

/* 单个设置项卡片 */
.setting-item {
  background: var(--bg-card);
  border: 1px solid var(--border);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);

  padding: 20px;
  margin-bottom: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: border-color 0.3s, background 0.3s, box-shadow 0.3s;
}

.setting-item:hover {
  border-color: var(--accent);
  background: var(--bg-card-hover);
  box-shadow: 0 0 15px var(--accent-glow);
}

.setting-item.vertical {
  flex-direction: column;
  align-items: flex-start;
  gap: 15px;
}

/* 文本描述 */
.text-info {
  display: flex;
  flex-direction: column;
}

.label {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 4px;
  color: var(--text-main);
}

.desc {
  font-size: 12px;
  color: var(--text-dim);
}

.desc-sm {
  font-size: 12px;
  color: var(--text-dim);
  margin-left: 10px;
}

.label-row {
  display: flex;
  align-items: baseline;
  width: 100%;
}

/* 输入框和按钮行 */
.input-row {
  display: flex;
  width: 100%;
  gap: 10px;
}

input[type="text"] {
  flex: 1;
  background: var(--bg-input);
  border: 1px solid var(--border);
  color: var(--text-main);

  padding: 10px 15px;
  font-family: 'Rajdhani', sans-serif;
  font-size: 14px;
  letter-spacing: 0.5px;
  outline: none;
  transition: all 0.3s;
}

input[type="text"]:focus {
  border-color: var(--accent);
  box-shadow: 0 0 10px var(--accent-glow);
}

.btn-browse {
  background: transparent;
  border: 1px solid var(--accent);
  color: var(--accent);
  padding: 0 20px;
  font-family: 'Rajdhani', sans-serif;
  font-weight: 700;
  cursor: pointer;
  text-transform: uppercase;
  transition: all 0.3s;
}

.btn-browse:hover {
  background: var(--accent);
  color: #fff;
  box-shadow: 0 0 15px var(--accent-glow);
}

/* Switch 及其旁边的 Label */
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
  transition: .4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: var(--switch-thumb);
  transition: .4s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

/* 选中状态 */
input:checked+.slider {
  background-color: var(--switch-track-checked);
  border-color: var(--switch-border-checked);
}

input:checked+.slider:before {
  transform: translateX(26px);
  background-color: var(--switch-thumb-checked);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

/* 圆角 */
.slider.round {
  border-radius: 24px;
}

.slider.round:before {
  border-radius: 50%;
}

/* 底部按钮 */
.action-footer {
  margin-top: 50px;
  display: flex;
  justify-content: flex-end;
  border-top: 1px solid var(--border);
  padding-top: 20px;
}

.btn-reset {
  background: transparent;
  border: 1px solid #d32f2f;
  color: #d32f2f;
  padding: 10px 20px;
  font-family: 'Rajdhani', sans-serif;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.3s;
  font-size: 13px;
  opacity: 0.7;
}

.btn-reset:hover {
  opacity: 1;
  background: rgba(211, 47, 47, 0.1);
  box-shadow: 0 0 10px rgba(211, 47, 47, 0.2);
}

/* 滚动条美化 */
.settings-container::-webkit-scrollbar {
  width: 6px;
}

.settings-container::-webkit-scrollbar-track {
  background: var(--scroll-track);
}

.settings-container::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 3px;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}
</style>