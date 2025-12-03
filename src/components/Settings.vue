<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <h2 class="page-title">系统设置 <span class="sub-title">SYSTEM CONFIGURATION</span></h2>

      <!-- 设置组：外观 -->
      <section class="setting-group">
        <h3 class="group-title">外观 / VISUALS</h3>
        <div class="setting-item">
          <div class="text-info">
            <span class="label">启用毛玻璃特效 (Acrylic Blur)</span>
            <span class="desc">开启后背景将呈现模糊透视效果，可能会轻微影响性能。</span>
          </div>
          <label class="switch">
            <input type="checkbox" v-model="config.enableBlur">
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
        <!-- 保存通常是自动的，或者可以加一个保存按钮 -->
        <!-- <button class="btn-save">保存修改</button> -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'

// 模拟配置数据结构
const config = reactive({
  enableBlur: true,
  gamePath: 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\World of Tanks Blitz',
  modPath: 'D:\\WOTB_Mods\\Library'
})

// 模拟选择路径功能 (实际项目中需调用 Electron 的 dialog 或其他 API)
const selectPath = (type: 'game' | 'mod') => {
  console.log(`Open file dialog for: ${type}`)
  // 模拟回调
  if (type === 'game') {
    // config.gamePath = ...
  }
}

// 重置功能
const resetToDefaults = () => {
  if(confirm('确定要重置所有设置吗？此操作无法撤销。')) {
    config.enableBlur = false
    config.gamePath = ''
    config.modPath = ''
  }
}
</script>

<style scoped>
/* 继承全局变量，确保样式统一 */
.settings-container {
  width: 100%;
  height: 100%;
  padding: 40px;
  box-sizing: border-box;
  overflow-y: auto;
  /* 使得内容浮在背景图之上，增加半透明深色底 */
  background: rgba(15, 17, 21, 0.85); 
  backdrop-filter: blur(10px); /* 如果全局背景没有模糊，这里可以加 */
  color: var(--text-main);
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
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
  opacity: 0.8;
}

/* 单个设置项卡片 */
.setting-item {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid var(--border);
  padding: 20px;
  margin-bottom: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: border-color 0.3s, background 0.3s;
}

.setting-item:hover {
  border-color: rgba(61, 90, 254, 0.5);
  background: rgba(255, 255, 255, 0.05);
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
  background: #000;
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

/* 开关 (Switch) 样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
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
  background-color: #2c2c2c;
  transition: .4s;
  border: 1px solid var(--border);
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: var(--text-dim);
  transition: .4s;
}

input:checked + .slider {
  background-color: rgba(61, 90, 254, 0.2);
  border-color: var(--accent);
}

input:checked + .slider:before {
  transform: translateX(26px);
  background-color: var(--accent);
  box-shadow: 0 0 10px var(--accent);
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
  background: rgba(0,0,0,0.3);
}
.settings-container::-webkit-scrollbar-thumb {
  background: var(--border);
}
.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}
</style>