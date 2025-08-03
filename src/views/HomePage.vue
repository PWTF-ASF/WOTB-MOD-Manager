<template>
    <div class="HomePage">
        <!-- 顶部卡片 -->
        <div class="top-card">
            <el-button type="success" :icon="Plus" round class="add-mod-btn"> 添加MOD </el-button>
            <el-input placeholder="请输入 MOD 名称" :prefix-icon="Search" clearable style="width: 160px"
                class="search-box" />
            <!-- <div class="search-box">搜索框</div> -->
        </div>
        <!-- 展示mod文件卡片 -->
        <div class="mod-card">
            <!-- 标签栏 -->
            <div class="mod-tap-bar">
                <div class="tap-bar-item">1</div>
                <div class="tap-bar-item">2</div>
                <div class="tap-bar-item">33333</div>
                <div class="tap-bar-item">444</div>
                <div class="tap-bar-item">5</div>
                <div class="tap-bar-item">6</div>
            </div>
            <!-- mod列表 -->
            <div class="mod-list">
                <!-- 列表项 -->
                <div class="mod-list-item">mod1</div>
                <div class="mod-list-item">mod2</div>
                <div class="mod-list-item">mod3</div>
            </div>
        </div>
        <!-- 底部卡片 -->
        <div class="bottom-card">
            <el-button type="primary" class="bottom-card-btn">加载MOD</el-button>
            <el-button type="success" class="bottom-card-btn">启动WOTB</el-button>
        </div>
    </div>
</template>

<script setup lang="ts">
// script setup 中引入图标
import { Plus, Search } from '@element-plus/icons-vue'

</script>

<style scoped>
.HomePage {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px;
    background-color: rgb(29, 32, 40);
    box-sizing: border-box;
}

/* 顶部卡片样式 */
.top-card {
    width: 80%;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: rgb(36, 41, 48);
    border-radius: 8px;
}

/* 添加mod按钮样式 */
.add-mod-btn {
    margin-left: 10px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

/* 搜索框样式 */
.search-box {
    border: 1px solid #ccc;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-left: 20px;
    margin-right: 10px;
}

/* 展示mod文件卡片样式 */
.mod-card {
    width: 80%;
    display: flex;
    flex-flow: column;
    align-items: center;
    margin-top: 20px;
}

/* 标签栏样式 */
.mod-tap-bar {
    max-width: 100%;
    height: 64px;
    display: flex;
    align-items: center;
    border: 1px solid rgb(38, 43, 51);
    border-radius: 10px;
    overflow-x: auto;
    white-space: nowrap;
    gap: 8px;
}

.tap-bar-item {
    flex-shrink: 0;
    /* 不让标签压缩 */
    width: 80px;
    /* 每个标签固定宽度，3个显示共240px */
    padding: 6px 0;
    text-align: center;
    color: white;
    border-radius: 6px;
    border: 1px solid rgb(38, 43, 51);
}

/* mod列表样式 */
.mod-list {
    width: 100%;
    margin-top: 20px;
    display: flex;
    flex-flow: column;
    justify-content: space-around;
}

/* 列表项样式 */
.mod-list-item {
    height: 48px;
    border: 1px solid rgb(38, 43, 51);
    border-radius: 8px;
    margin-bottom: 20px;
}

/* 底部卡片样式 */
.bottom-card {
    position: fixed;
    bottom: 0;
    width: 80%;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: space-around;
    margin-bottom: 20px;
}

.bottom-card-btn {
    width: 160px;
    height: 48px;
    border-radius: 8px;
    background-color: rgb(46, 51, 60);
    color: white;
    text-align: center;
    line-height: 48px;
}

/* "加载mod" 按钮的样式 */
.bottom-card-btn:nth-of-type(1) {
    background-color: #2196F3;
    color: white;
}

/* "启动WOTB" 按钮的样式 */
.bottom-card-btn:nth-of-type(2) {
    background-color: #4CAF50;
    color: white;
}
</style>
