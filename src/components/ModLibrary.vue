<template>
    <!-- 顶部信息栏 -->
    <header class="top-deck">
        <div class="search-module">
            <span class="search-icon"><img src="@/assets/vue.svg" /></span>
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

    <!-- 中间列表：卡片式流 -->
    <section class="modules-grid">
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
            <div class="status-indicator"></div>
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

<script setup lang="ts"></script>

<style scoped>
/* --- 顶部栏 --- */
.top-deck {
    height: 80px;
    padding: 0 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    /* 防止被压缩 */
    flex-shrink: 0;
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
    grid-auto-rows: max-content;
    gap: 15px;
    /* 底部留出 Control Deck 的高度 + 20px 边距 */
    padding-bottom: 110px;
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
    /* background: var(--bg-card); */
    /* border: 1px solid var(--border); */
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
}

.mod-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 20px -5px rgba(0, 0, 0, 0.5);
    border-color: #444;
}

.active-card {
    border-left: 2px solid var(--success);
    /* 仅左边框发光：水平偏移向左，垂直压缩光晕范围，限制模糊半径 */
    box-shadow:
        -3px 0 8px -1px rgba(0, 230, 118, 0.5),
        /* 左侧核心发光（仅左右方向扩散） */
        0 0 0 0 transparent;
    /* 其他方向无发光 */
    transition:
        box-shadow 0.3s ease,
        transform 0.2s ease;
}

.active-card:hover {
    border-left: 2px solid var(--success);
    box-shadow:
        -4px 0 12px -1px rgba(0, 230, 118, 0.7),
        /* 左边框发光增强 */
        0 0 0 0 transparent;
    /* 保留卡片悬浮阴影，不影响左右发光 */
    transform: translateY(-2px);
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
    transition: 0.4s;
    border-radius: 20px;
}

.slider:before {
    position: absolute;
    content: '';
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.4s;
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
</style>
