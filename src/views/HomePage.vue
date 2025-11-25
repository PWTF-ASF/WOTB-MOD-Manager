<template>
    <div class="command-center">
        <!-- 1. 左侧导航栏 -->
        <aside class="sidebar">
            <div class="logo-box">
                <div class="logo-icon"><img src="@/assets/WOTBIcon.png" /></div>
                <span class="logo-text">MOD<br>Manager</span>
            </div>

            <nav class="nav-links">
                <!-- 模组库导航项：id=1 -->
                <div class="nav-item" :class="{ active: currentActiveId === 1 }" @click="currentActiveId = 1">
                    <span class="icon"><img :src="DarkMode
                        ? (currentActiveId === 1 ? HomeDarkActiveIcon : HomeDarkIcon)
                        : (currentActiveId === 1 ? HomeActiveIcon : HomeIcon)        
                        " alt="设置" /></span>
                    <span class="label">模组库</span>
                </div>
                <!-- 系统设置导航项：id=2 -->
                <div class="nav-item" :class="{ active: currentActiveId === 2 }" @click="currentActiveId = 2">
                    <span class="icon">
                        <img :src="DarkMode
                            ? (currentActiveId === 2 ? SettingDarkActiveIcon : SettingDarkIcon)  // 仅自身选中时切换暗黑选中图标
                            : (currentActiveId === 2 ? SettingActiveIcon : SettingIcon)        // 仅自身选中时切换普通选中图标
                            " alt="设置" />
                    </span>
                    <span class="label">系统设置</span>
                </div>
            </nav>

            <div class="sidebar-footer">
                <div class="version">v1.2.0</div>
            </div>
        </aside>

        <!-- 2. 主操作区（保持不变） -->
        <main class="main-viewport">
            <!-- 顶部信息栏 -->
            <header class="top-deck">
                <div class="search-module">
                    <span class="search-icon"><img src='@/assets/vue.svg' /></span>
                    <input type="text" placeholder="搜索mod..." />
                </div>
                <div class="stats-module">
                    <div class="stat-item">
                        <span class="stat-num">42</span>
                        <span class="stat-label">已安装</span>
                    </div>
                    <div class="stat-item active-stat">
                        <span class="stat-num">12</span>
                        <span class="stat-label">运行中</span>
                    </div>
                </div>
            </header>

            <!-- 中间列表：卡片式流（保持不变） -->
            <section class="modules-grid">
                <!-- 模拟一个已启用的模组卡片 -->
                <div class="mod-card active-card">
                    <div class="status-indicator"></div>
                    <div class="card-content">
                        <div class="mod-header">
                            <span class="mod-title">高清材质包 4K</span>
                            <span class="tag">画质</span>
                        </div>
                        <div class="mod-desc">提升游戏贴图分辨率至 4096x4096</div>
                    </div>
                    <div class="card-action">
                        <label class="switch">
                            <input type="checkbox" checked>
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <!-- 模拟一个未启用的模组卡片 -->
                <div class="mod-card">
                    <div class="status-indicator inactive"></div>
                    <div class="card-content">
                        <div class="mod-header">
                            <span class="mod-title">无限负重作弊</span>
                            <span class="tag">脚本</span>
                        </div>
                        <div class="mod-desc">移除背包重量限制 (可能会影响成就)</div>
                    </div>
                    <div class="card-action">
                        <label class="switch">
                            <input type="checkbox">
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <!-- 循环生成填充 -->
                <div class="mod-card" v-for="n in 6" :key="n">
                    <div class="status-indicator inactive"></div>
                    <div class="card-content">
                        <div class="mod-header">
                            <span class="mod-title">通用前置库 Lib_{{ n }}</span>
                            <span class="tag">核心</span>
                        </div>
                        <div class="mod-desc">必要的前置依赖文件</div>
                    </div>
                    <div class="card-action">
                        <label class="switch">
                            <input type="checkbox">
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>
            </section>

            <!-- 底部控制台（保持不变） -->
            <footer class="control-deck">
                <div class="deck-left">
                    <button class="deck-btn danger">卸载选中</button>
                    <button class="deck-btn">加载mod</button>
                </div>

                <div class="deck-right">
                    <!-- 核心：启动按钮 -->
                    <button class="launch-btn">
                        <div class="launch-content">
                            <span class="launch-title">启动游戏</span>
                            <span class="launch-sub">READY TO LAUNCH</span>
                        </div>
                        <div class="launch-effect"></div>
                    </button>
                </div>
            </footer>
        </main>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
// 确保以下图标路径和实际文件一致（如果报错，检查文件名/路径）
import SettingIcon from '@/assets/设置.svg';
import SettingActiveIcon from '@/assets/设置_HL.svg';
import SettingDarkIcon from '@/assets/设置Dark.svg';
import SettingDarkActiveIcon from '@/assets/设置Dark_HL.svg';
import HomeIcon from '@/assets/首页.svg';
import HomeActiveIcon from '@/assets/首页_HL.svg';
import HomeDarkIcon from '@/assets/首页Dark.svg';
import HomeDarkActiveIcon from '@/assets/首页Dark_HL.svg';

// 关键：用选中ID控制两个导航项，1=模组库，2=系统设置
const currentActiveId = ref(1); // 默认选中「模组库」（id=1）
const DarkMode = ref(false); // 暗黑模式状态（保持不变）
</script>

<style>
/* 引入科幻感字体 (可选，没有会自动回退) */
@import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@500;700&display=swap');

:root {
    /* 调色板：深空灰 + 赛博蓝 */
    --bg-deep: #0f1115;
    --bg-panel: #161920;
    --bg-card: #1c2029;

    --accent: #3d5afe;
    /* 核心蓝 */
    --accent-glow: rgba(61, 90, 254, 0.4);

    --text-main: #ffffff;
    --text-dim: #6b7280;

    --success: #00e676;
    --border: #2a2f3a;
}

html,
body {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    background-color: var(--bg-deep);
    color: var(--text-main);
    font-family: 'Rajdhani', 'Segoe UI', sans-serif;
    /* 使用硬朗的字体 */
    overflow: hidden;
}
</style>

<style scoped>
/* 原有样式保持不变，无需修改 */
.command-center {
    display: flex;
    width: 100vw;
    height: 100vh;
}

/* ================= 侧边栏 ================= */
.sidebar {
    width: 90px;
    /* 窄侧边栏，强调图标 */
    background-color: var(--bg-deep);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 20px;
    z-index: 10;
}

.logo-box {
    margin-bottom: 40px;
    text-align: center;
}

.logo-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto 5px;
}

.logo-icon img{
    width: 100%;
    height: 100%;
    object-fit: contain;
}

.logo-text {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-dim);
    letter-spacing: 1px;
}

.nav-links {
    display: flex;
    flex-direction: column;
    gap: 20px;
    width: 100%;
}

.nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
    padding: 10px 0;
    color: var(--text-dim);
    transition: all 0.3s;
    border-left: 3px solid transparent;
    /* 选中指示条 */
}

.nav-item:hover,
.nav-item.active {
    color: var(--text-main);
    background: linear-gradient(90deg, rgba(61, 90, 254, 0.1) 0%, transparent 100%);
}

.nav-item.active {
    border-left-color: var(--accent);
    text-shadow: 0 0 8px rgba(255, 255, 255, 0.5);
}

.nav-item .icon {
    font-size: 20px;
    margin-bottom: 4px;
}

.nav-item .icon img {
    width: 20px;
    height: 20px;
}

.nav-item .label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
}

.sidebar-footer {
    margin-top: auto;
    padding-bottom: 20px;
    font-size: 10px;
    color: #444;
}

/* ================= 主视口 ================= */
.main-viewport {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-panel);
    position: relative;
}

/* --- 顶部栏 --- */
.top-deck {
    height: 80px;
    padding: 0 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.search-module {
    position: relative;
    width: 400px;
}

.search-icon {
    position: absolute;
    left: 15px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-dim);
    font-size: 18px;
    margin-left: 5px;
}

.search-icon img {
    width: 18px;
    height: 18px;
}

.search-module input {
    width: 100%;
    height: 44px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 25px;
    /* 小圆角 */
    padding-left: 45px;
    color: var(--text-main);
    font-family: inherit;
    font-size: 16px;
    transition: all 0.3s;
}

.search-module input:focus {
    border-radius: 5px;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px rgba(61, 90, 254, 0.2);
}

.stats-module {
    display: flex;
    gap: 30px;
}

.stat-item {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
}

.stat-num {
    font-size: 24px;
    font-weight: 700;
    line-height: 1;
}

.stat-label {
    font-size: 12px;
    color: var(--text-dim);
    text-transform: uppercase;
}

.active-stat .stat-num {
    color: var(--success);
    text-shadow: 0 0 10px rgba(0, 230, 118, 0.4);
}

/* --- Mod 卡片列表 --- */
.modules-grid {
    flex: 1;
    padding: 10px 40px 0;
    overflow-y: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    /* 响应式网格 */
    grid-auto-rows: max-content;
    gap: 15px;
    padding-bottom: 100px;
    /* 给底部栏留空间 */
}

/* 自定义滚动条 */
.modules-grid::-webkit-scrollbar {
    width: 6px;
}

.modules-grid::-webkit-scrollbar-track {
    background: transparent;
}

.modules-grid::-webkit-scrollbar-thumb {
    background: #333;
    border-radius: 3px;
}

.mod-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 15px;
    display: flex;
    align-items: center;
    position: relative;
    transition: transform 0.2s, box-shadow 0.2s;
    overflow: hidden;
}

.mod-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 20px -5px rgba(0, 0, 0, 0.5);
    border-color: #444;
}

/* 选中高亮状态 */
.active-card {
    border-left: 2px solid var(--success);
    background: linear-gradient(135deg, var(--bg-card) 0%, rgba(0, 230, 118, 0.05) 100%);
}

.active-card:hover {
    border-left: 2px solid var(--success);
}

.status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--success);
    box-shadow: 0 0 8px var(--success);
    margin-right: 15px;
}

.status-indicator.inactive {
    background: #333;
    box-shadow: none;
}

.card-content {
    flex: 1;
    min-width: 0;
    /* 防止文字溢出 */
}

.mod-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
}

.mod-title {
    font-weight: 700;
    font-size: 16px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.tag {
    font-size: 10px;
    padding: 2px 6px;
    background: #333;
    border-radius: 4px;
    color: #aaa;
}

.mod-desc {
    font-size: 12px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

/* iOS 风格开关 - 缩小版 */
.switch {
    position: relative;
    display: inline-block;
    width: 36px;
    height: 20px;
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
    background-color: #333;
    transition: .4s;
    border-radius: 20px;
}

.slider:before {
    position: absolute;
    content: "";
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .4s;
    border-radius: 50%;
}

input:checked+.slider {
    background-color: var(--success);
}

input:checked+.slider:before {
    transform: translateX(16px);
}

/* ================= 底部控制台 (Deck) ================= */
.control-deck {
    height: 90px;
    background: rgba(22, 25, 32, 0.95);
    backdrop-filter: blur(10px);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 40px;
    position: absolute;
    bottom: 0;
    width: 100%;
    z-index: 50;
}

.deck-btn {
    height: 40px;
    padding: 0 24px;
    background: transparent;
    border: 1px solid #444;
    color: #ccc;
    font-family: inherit;
    font-weight: 600;
    cursor: pointer;
    margin-right: 15px;
    transition: all 0.2s;
    text-transform: uppercase;
    text-align: center;
    line-height: 40px;
    font-size: 12px;
    letter-spacing: 1px;
}

.deck-btn:hover {
    border-color: #fff;
    color: #fff;
    background: rgba(255, 255, 255, 0.05);
}

.deck-btn.danger {
    border-color: #662222;
    color: #ff5555;
}

.deck-btn.danger:hover {
    background: rgba(255, 85, 85, 0.1);
}

/* 启动按钮 - 核心视觉 */
.launch-btn {
    height: 60px;
    padding: 0 50px;
    background: linear-gradient(90deg, #3d5afe, #536dfe);
    border: none;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    clip-path: polygon(10px 0, 100% 0, 100% 100%, 0 100%, 0 10px);
    /* 科技感切角 */
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 20px var(--accent-glow);
    transition: transform 0.1s;
}

.launch-btn:hover {
    transform: scale(1.02);
    filter: brightness(1.1);
}

.launch-btn:active {
    transform: scale(0.98);
}

.launch-content {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    z-index: 2;
    color: white;
}

.launch-title {
    font-size: 20px;
    font-weight: 800;
    letter-spacing: 2px;
    text-transform: uppercase;
}

.launch-sub {
    font-size: 10px;
    opacity: 0.7;
    letter-spacing: 1px;
}

/* 扫光动画 */
.launch-effect {
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    animation: shimmer 3s infinite;
}

@keyframes shimmer {
    0% {
        left: -100%;
    }

    20% {
        left: 100%;
    }

    100% {
        left: 100%;
    }
}
</style>