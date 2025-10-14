<template>
    <div class="settings-item-panel">
        <div class="settings-item-panel-card">
            <div class="card-title">禁用毛玻璃效果</div>
            <div class="card-content">
                <div class="theme-switch-container">
                    <label class="square-switch">
                        <input type="checkbox" class="sr-only" />
                        <span class="slider"></span>
                    </label>
                </div>
            </div>
        </div>
    </div>
</template>
<style scoped>
.settings-item-panel {
    padding: 10px 20px 40px 20px;
    background-color: var(--panel-bg);
    margin: 10px 20px 0 20px;
    border-radius: 15px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    backdrop-filter: blur(12px);
}

.settings-item-panel-card {
    height: auto;
}

.card-title {
    font-size: 18px;
    font-weight: bold;
    margin-bottom: 10px;
    display: flex;
    gap: 10px;
}

.card-content {
    align-items: center;
    display: flex;
    max-height: 55px;
    gap: 1rem;
    padding: 10px 20px;
    box-shadow: var(--card-shadow), var(--card-glow);
    border-radius: 5px;
    background-color: var(--card-bg);
}

/* 按钮样式 */
.theme-switch-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    /* 添加容器阴影增强整体层次 */
    padding: 0.25rem;
    border-radius: 8px;
}

.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
}

/* 方形开关样式 */
.square-switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 30px;
    cursor: pointer;
    /* 添加轻微缩放效果增强交互感 */
    transition: transform 0.15s ease;
}

.square-switch:hover {
    transform: scale(1.02);
}

/* 方形滑块 */
.slider {
    position: absolute;
    inset: 0;
    background-color: #ccc;
    transition: .4s;
    border-radius: 6px;
    /* 多层次阴影增强立体感 */
    box-shadow:
        inset 0 1px 2px rgba(0, 0, 0, 0.2),
        0 2px 3px rgba(0, 0, 0, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.15);
}

/* 方形按钮 */
.slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    transition: .4s cubic-bezier(0.34, 1.56, 0.64, 1);
    border-radius: 4px;
    /* 按钮立体效果 */
    box-shadow:
        0 1px 3px rgba(0, 0, 0, 0.25),
        inset 0 1px 1px rgba(255, 255, 255, 0.8);
    /* 添加内发光增强层次 */
    background-image: linear-gradient(135deg, rgba(255, 255, 255, 0.8) 0%, rgba(255, 255, 255, 0.2) 100%);
}

/* 选中状态 */
input:checked+.slider {
    background-color: #409eff;
    /* 激活状态下的阴影变化 */
    box-shadow:
        inset 0 1px 2px rgba(0, 0, 0, 0.15),
        0 2px 5px rgba(64, 158, 255, 0.25);
}

input:checked+.slider:before {
    transform: translateX(30px);
    /* 激活状态下按钮的细微变化 */
    box-shadow:
        0 1px 3px rgba(0, 0, 0, 0.15),
        inset 0 1px 1px rgba(255, 255, 255, 0.8);
}

/* 添加状态指示器增强层次感知 */
.slider:after {
    content: "";
    position: absolute;
    top: 50%;
    left: 20px;
    width: 4px;
    height: 4px;
    background-color: rgba(0, 0, 0, 0.2);
    border-radius: 50%;
    transform: translateY(-50%);
    transition: all 0.3s ease;
    opacity: 0;
}

input:checked+.slider:after {
    left: 46px;
    background-color: rgba(255, 255, 255, 0.6);
    opacity: 1;
}
</style>
<script setup lang="ts">

</script>