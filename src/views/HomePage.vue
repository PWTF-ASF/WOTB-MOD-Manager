<template>
    <div class="HomePage">
        <!-- 顶部卡片 -->
        <div class="top-card">
            <el-button type="success" :icon="Plus" class="add-mod-btn" style="height: 48px"> </el-button>
            <el-input :prefix-icon="Search" clearable style="width: 160px" class="search-box" />
        </div>
        <!-- 展示mod文件卡片 -->
        <div class="mod-card">
            <!-- 标签栏 -->
            <div class="mod-tap-bar">
                <div class="tap-bar-item" v-for="(tap, index) in ModData.tapList" :key="index"
                    :class="{ active: activeTap === tap.type }" @click="activeTap = tap.type">
                    {{ tap.name }}
                </div>
            </div>
            <!-- mod列表 -->
            <div class="mod-list">
                <!-- 列表项 -->
                <div class="mod-list-item" v-for="(mod, index) in filteredModList" :key="index">
                    <div class="mod-title">
                        <span>{{ mod.name }}</span>
                        <span>{{ mod.type }}</span>
                    </div>
                    <div class="mod-overview">
                        <span>{{ mod.author }}</span>
                        <span>{{ mod.version }}</span>
                    </div>
                </div>
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
import { reactive, computed } from 'vue';
import { Plus, Search } from '@element-plus/icons-vue'

const activeTap = ref('全部'); // 默认为全部

// 计算属性：根据选中的标签过滤 modList
const filteredModList = computed(() => {
    if (activeTap.value === '全部') {
        return ModData.modList
    }
    return ModData.modList.filter(mod => mod.type === activeTap.value)
})

//mod数据
let ModData = reactive({
    modList: [
        { name: 'mod1', type: '语音包', author: '张三', version: '1.0.0' },
        { name: 'mod2', type: '坦克模型', author: '李四', version: '2.0.0' },
        { name: 'mod3', type: '语音包', author: '王五', version: '3.0.0' },
        { name: 'mod3', type: '语音包', author: '王五', version: '3.0.0' },
        { name: 'mod3', type: '语音包', author: '王五', version: '3.0.0' },
        { name: 'mod3', type: '语音包', author: '王五', version: '3.0.0' },
        { name: 'mod3', type: '其他', author: '王五', version: '3.0.0' },
    ],
    tapList: [
        { name: '全部', type: '全部' },
        { name: '语音包', type: '语音包' },
        { name: '坦克模型', type: '坦克模型' },
        { name: '其他', type: '其他' },
    ]
})
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
    border-radius: 8px;
    margin-left: 10px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

/* 搜索框样式 */
.search-box {
    height: 48px;
    border: 1px solid #ccc;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-right: 20px;
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
    min-height: 32px;
    display: flex;
    align-items: center;
    border-radius: 10px;
    overflow-x: auto;
    white-space: nowrap;

}
.tap-bar-item.active {
    background-color: rgb(74, 81, 91);
    color: #fff;
}
/* WebKit (Chrome, Safari) */
.mod-tap-bar::-webkit-scrollbar {
    width: 6px;
    height: 6px;
    background: transparent;
}

.mod-tap-bar::-webkit-scrollbar-thumb {
    background-color: rgb(54, 61, 71);
    /* 滚动条滑块的颜色 */
    border-radius: 3px;
    /* 滚动条滑块的圆角 */
}

.mod-tap-bar::-webkit-scrollbar-track {
    background-color: transparent;
    /* 滚动条轨道的颜色 */
}

.mod-tap-bar::-webkit-scrollbar-thumb:hover {
    background-color: rgb(74, 81, 91);
    /* 鼠标悬停在滑块上时的颜色 */
}

.tap-bar-item {
    flex-shrink: 0;
    /* 不让标签压缩 */
    width: 80px;
    /* 每个标签固定宽度，3个显示共240px */
    min-height: 32px;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
    margin-bottom: 10px;
}

.tap-bar-item:hover {
    background-color: rgb(46, 51, 60);
    cursor: pointer;
}

/* mod列表样式 */
.mod-list {
    width: 100%;
    margin-top: 20px;
    height: 480px;
    overflow-y: auto;
    display: flex;
    flex-flow: column;
    padding-right: 12px;
    box-sizing: border-box;
}

/* 优化滚动条样式 */
.mod-list::-webkit-scrollbar {
    width: 8px;
    background: transparent;
}

.mod-list::-webkit-scrollbar-thumb {
    background-color: rgb(54, 61, 71);
    border-radius: 4px;
}

.mod-list::-webkit-scrollbar-track {
    background: transparent;
}

.mod-list::-webkit-scrollbar-thumb:hover {
    background-color: rgb(74, 81, 91);
    /* 鼠标悬停在滑块上时的颜色 */
}

/* 列表项样式 */
.mod-list-item {
    min-height: 96px;
    display: flex;
    flex-flow: column;
    justify-content: space-around;
    border: 1px solid rgb(38, 43, 51);
    border-radius: 8px;
    margin-bottom: 20px;
    padding: 0 20px;
}

.mod-title {
    width: 100%;
    display: flex;
    justify-content: space-between;
}

.mod-overview {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: rgb(150, 150, 150);
    font-size: 14px;
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
    background-color: #2196f3;
    color: white;
}

/* "启动WOTB" 按钮的样式 */
.bottom-card-btn:nth-of-type(2) {
    background-color: #4caf50;
    color: white;
}
</style>
