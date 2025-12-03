<template>
    <!-- 顶部信息栏 -->
    <header class="top-deck">
        <div class="search-container">
            <div class="search-module">
                <span class="search-icon"><img src="@/assets/vue.svg" /></span>
                <input type="text" />
            </div>
        </div>

        <!-- 布局切换按钮 -->
        <button class="layout-toggle" @click="toggleLayout" :title="isGridLayout ? '切换为列表布局' : '切换为网格布局'">
            <span class="layout-icon">{{ isGridLayout ? '☰' : '□' }}</span>
            <span class="layout-text">{{ isGridLayout ? '列表' : '网格' }}</span>
        </button>

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

    <!-- 中间列表：卡片式流 -->
    <section class="modules-grid" :class="{ 'grid-layout': isGridLayout, 'list-layout': !isGridLayout }">
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
                    <input type="checkbox" checked />
                    <span class="slider"></span>
                </label>
            </div>
        </div>

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
                    <input type="checkbox" />
                    <span class="slider"></span>
                </label>
            </div>
        </div>

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
                    <input type="checkbox" />
                    <span class="slider"></span>
                </label>
            </div>
        </div>
    </section>

    <!-- 底部控制台 -->
    <footer class="control-deck">
        <div class="deck-left">
            <button class="deck-btn danger">卸载选中</button>
            <button class="deck-btn">加载mod</button>
        </div>

        <div class="deck-right">
            <button class="launch-btn">
                <div class="launch-content">
                    <span class="launch-title">启动游戏</span>
                    <span class="launch-sub">READY TO LAUNCH</span>
                </div>
                <div class="launch-effect"></div>
            </button>
        </div>
    </footer>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// 控制布局切换的状态：true为网格布局（两列），false为列表布局（一列）
const isGridLayout = ref(true);

// 切换布局的方法
const toggleLayout = () => {
    isGridLayout.value = !isGridLayout.value;
};
</script>

<style scoped>
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    border: none;
}

/* --- 顶部栏 --- */
.top-deck {
    height: 80px;
    padding-left: 40px;
    padding-right: 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
}

/* 搜索框和布局切换按钮容器 */
.search-container {
    display: flex;
    align-items: center;
}

.search-module {
    position: relative;
    width: 400px;
    display: block;
}

.search-icon {
    position: absolute;
    left: 15px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-dim);
    font-size: 18px;
    margin-left: 5px;
    z-index: 2;
}

.search-icon img {
    width: 18px;
    height: 18px;
}

.search-module input {
    width: 100%;
    height: 44px;
    background: var(--def-col-fltr);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 25px;
    padding-left: 45px;
    color: var(--text-main);
    font-family: inherit;
    font-size: 16px;
    transition: all 0.3s;
    box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.search-module input:hover {
    background: var(--def-col-fltr-hover);
    backdrop-filter: blur(12px);
}

.search-module input:focus {
    border-radius: 5px;
    background: rgb(242, 243, 244);
    border-color: var(--accent);
    box-shadow: 0 0 12px var(--accent-glow);
}

/* 布局切换按钮样式 */
.layout-toggle {
    height: 44px;
    padding: 0 16px;
    background: var(--def-col-fltr);
    backdrop-filter: blur(12px);
    border-radius: 25px;
    color: var(--text-main);
    font-family: inherit;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.3s;
    box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.layout-toggle:hover {
    background: var(--def-col-fltr-hover);
    backdrop-filter: blur(12px);
}

.layout-toggle:active {
    transform: scale(0.96);
}

.layout-icon {
    font-size: 18px;
}

.layout-text {
    font-weight: 500;
}

/* 模组状态统计信息 */
.stats-module {
    width: 120px;
    height: 44px;
    display: flex;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
    border-radius: 25px;
    justify-content: space-around;
    position: relative;
    overflow: visible;
    box-shadow: 0 0 32px rgba(0, 0, 0, 0.15);
}

.stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
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
    color: var(--accent);
    text-shadow: 0 0 10px rgba(61, 90, 254, 0.5);
}

/* --- Mod 卡片列表 --- */
.modules-grid {
    flex: 1;
    padding: 10px 40px 0;
    overflow-y: auto;
    grid-auto-rows: max-content;
    gap: 15px;
    padding-bottom: 110px;
}

/* 网格布局（两列） */
.modules-grid.grid-layout {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
}

/* 列表布局（一列） */
.modules-grid.list-layout {
    display: grid;
    grid-template-columns: 1fr;
}

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
    border-radius: 6px;
    padding: 15px;
    display: flex;
    align-items: center;
    position: relative;
    transition:
        transform 0.2s,
        box-shadow 0.2s;
    overflow: hidden;
    backdrop-filter: blur(12px);
    box-shadow:
        0 4px 12px rgba(0, 0, 0, 0.25),
        /* 主阴影 */
        0 2px 6px rgba(0, 0, 0, 0.15);
    /* 次阴影 */
}

/* 列表布局下的卡片样式优化 */
.list-layout .mod-card {
    padding: 20px;
}

.mod-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.25),
        /* 主阴影 */
        0 2px 15px rgba(0, 0, 0, 0.15);
}

.active-card {
    border-left: 2px solid var(--accent);
    box-shadow:
        -3px 0 8px -1px rgba(61, 90, 254, 0.5),
        0 0 0 0 transparent;
    transition:
        box-shadow 0.3s ease,
        transform 0.2s ease;
}

.active-card:hover {
    border-left: 2px solid var(--accent);
    box-shadow:
        -4px 0 12px -1px rgba(61, 90, 254, 0.5),
        0 0 0 0 transparent;
    transform: translateY(-2px);
}

.status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
    box-shadow: 0 0 8px var(--accent);
    margin-right: 15px;
}

.status-indicator.inactive {
    background: #333;
    box-shadow: none;
}

.card-content {
    flex: 1;
    min-width: 0;
}

/* 列表布局下的内容样式优化 */
.list-layout .card-content {
    display: flex;
    align-items: center;
    gap: 20px;
}

.list-layout .mod-header {
    margin-bottom: 0;
}

.list-layout .mod-desc {
    white-space: normal;
    line-height: 1.4;
    max-width: 600px;
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

.switch {
    position: relative;
    display: inline-block;
    width: 40px;
    /* 稍微宽一点 */
    height: 18px;
    /* 稍微扁一点 */
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

/* 滑块轨道背景 */
.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: transparent;
    /* 背景透明 */
    border: 1px solid #555;
    /* 只有边框 */
    transition: 0.3s;
    /* 关键：改为直角或极小圆角 */
    border-radius: 2px;
}

/* 滑块方块 */
.slider:before {
    position: absolute;
    content: "";
    height: 10px;
    width: 10px;
    left: 3px;
    bottom: 3px;
    background-color: var(--text-dim);
    /* 默认灰色 */
    transition: 0.3s cubic-bezier(0.4, 0.0, 0.2, 1);
    /* 机械感的运动曲线 */
    border-radius: 1px;
    /* 方形滑块 */
}

/* 选中状态：轨道 */
input:checked+.slider {
    background-color: rgba(61, 90, 254, 0.1);
    /* 激活时微弱背景色 */
    border-color: var(--accent);
    /* 边框变亮 */
    box-shadow: 0 0 8px var(--accent-glow);
    /* 荧光效果 */
}

/* 选中状态：滑块 */
input:checked+.slider:before {
    transform: translateX(22px);
    background-color: var(--accent);
    /* 滑块变亮色 */
    box-shadow: 0 0 5px var(--accent);
    /* 滑块荧光 */
}

/* 悬停效果 (增加交互感) */
.switch:hover .slider {
    border-color: #888;
}

input:checked:hover+.slider {
    border-color: #536dfe;
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

.launch-btn {
    height: 60px;
    padding: 0 50px;
    background: linear-gradient(90deg, #3d5afe, #536dfe);
    border: none;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    clip-path: polygon(10px 0, 100% 0, 100% 100%, 0 100%, 0 10px);
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

/* 响应式调整 */
@media (max-width: 768px) {
    .search-container {
        flex-direction: column;
        align-items: flex-start;
        gap: 8px;
    }

    .search-module {
        width: 100%;
    }

    .layout-toggle {
        width: 100%;
        justify-content: center;
    }

    .modules-grid.grid-layout {
        grid-template-columns: 1fr;
    }
}
</style>