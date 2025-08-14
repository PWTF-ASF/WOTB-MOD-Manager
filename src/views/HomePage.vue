<template>
    <div class="HomePage">
        <!-- 顶部卡片 -->
        <div class="top-card">
            <el-button type="success" :icon="Plus" class="add-mod-btn" style="height: 48px" @click="selectModFolder">
            </el-button>
            <el-input v-model="searchQuery" :prefix-icon="Search" clearable style="width: 160px" class="search-box"
                @input="handleSearch" />
        </div>
        <!-- 展示mod文件卡片 -->
        <div class="mod-card">
            <!-- 标签栏 -->
            <div class="mod-tap-bar">
                <div class="tap-bar-item" v-for="(tap, index) in ModData.tapList" :key="index"
                    :class="{ active: activeTap === tap.type }" @click="activeTap = tap.type">
                    {{ tap.name }}
                </div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">
                    全选
                </el-checkbox>
            </div>
            <!-- mod列表 -->
            <div class="mod-list">
                <el-checkbox-group v-model="selectedMods">
                    <!-- 列表项 -->
                    <div class="mod-list-item" v-for="(mod, index) in filteredModList" :key="index">
                        <el-checkbox :label="mod.name" style="margin-bottom: 8px" class="mod-checkbox" />
                        <div class="mod-title">
                            <!-- <span>{{ mod.name }}</span> -->
                            <span>{{ mod.type }}</span>
                        </div>
                        <div class="mod-overview">
                            <span>{{ mod.author }}</span>
                            <span>{{ mod.version }}</span>
                        </div>
                    </div>
                </el-checkbox-group>
            </div>
        </div>
        <!-- 底部卡片 -->
        <div class="bottom-card">
            <el-button type="primary" class="bottom-card-btn" @click="loadSelectedMods">加载MOD</el-button>
            <el-button type="danger" class="bottom-card-btn" @click="deleteSelectedMods">删除</el-button>
            <el-button type="success" class="bottom-card-btn" @click="startGame">启动WOTB</el-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { reactive, computed, ref, onMounted, watch } from 'vue';
import { Plus, Search } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core';
import { ElMessage, ElLoading } from 'element-plus'

const activeTap = ref('全部'); // 默认为全部
let loadingInstance: ReturnType<typeof ElLoading.service> | null = null;
const selectedMods = ref<string[]>([]);
const searchQuery = ref('');

// 全选勾选状态
const checkAll = ref(false);
// 是否显示半选状态
const isIndeterminate = computed(() => {
    const len = selectedMods.value.length;
    return len > 0 && len < filteredModList.value.length;
});

// 当用户点“全选”时，同步更新 selectedMods
function handleCheckAllChange(val: boolean) {
    if (val) {
        selectedMods.value = filteredModList.value.map(m => m.name);
    } else {
        selectedMods.value = [];
    }
}

// 当 selectedMods 变化时，更新 checkAll（全选框状态）
watch(selectedMods, (newVal) => {
    checkAll.value = newVal.length === filteredModList.value.length;
});

// 计算属性：根据选中的标签过滤 modList
const filteredModList = computed(() => {
    return activeTap.value === '全部'
        ? ModData.modList
        : ModData.modList.filter((mod) => mod.type === activeTap.value);
});

//mod数据
let ModData = reactive({
    modList: [] as Array<{ name: string; type: string; author: string; version: string }>,
    tapList: [
        { name: '全部', type: '全部' },
        { name: '语音包', type: '语音包' },
        { name: '坦克模型', type: '坦克模型' },
        { name: '其他', type: '其他' },
    ]
})

// 选择mod压缩包
const selectModFolder = async () => {
    // 开始加载动画
    loadingInstance = ElLoading.service({
        lock: true,
        text: '正在添加 MOD...',
        background: 'rgba(0, 0, 0, 0.4)',
    });
    try {
        const selected = await open({
            title: '请选择 Mod 压缩包文件',
            multiple: false,
            filters: [{ name: 'Mod 包', extensions: ['zip'] }],
        }) as string | null;

        if (!selected) {
            ElMessage.info('未选择任何文件');
            return;
        }

        await invoke('copy_mod_file', { src: selected });
        ElMessage.success('已复制 Mod 包到本地 mods 目录');
        await fetchModList();
    } catch (err) {
        console.error(err);
        ElMessage.error(typeof err === 'string' ? err : '复制 Mod 包失败');
    } finally {
        loadingInstance?.close(); // 关闭加载动画
    }
}

// 拉取 mods 目录下所有 ZIP 文件
async function fetchModList() {
    try {
        const mods = await invoke('get_mod_status') as Array<{
            name: string;
            type: string;
            author: string;
            version: string;
            applied: boolean;
        }>;
        ModData.modList = mods.map(mod => ({
            name: mod.name,
            type: mod.type,
            author: mod.author,
            version: mod.version,
        }));
        selectedMods.value = mods.filter(mod => mod.applied).map(mod => mod.name);
        console.log('刷新后 Mod 列表：', ModData.modList);
        console.log('已选中：', selectedMods.value);
    } catch (e) {
        console.error('fetchModList error', e);
    }
}

//启动游戏
async function startGame() {
    try {
        // 先读配置
        const path: string | null = await invoke('get_game_path')
        console.log('本地存储的游戏路径：', path)

        // 如果配置里有路径，直接尝试启动
        if (path) {
            await invoke('launch_game')
            return
        }

        // 否则让用户选目录、设路径、再启动
        const selected = await open({
            title: '请选择 WOTB 游戏目录',
            directory: true,
            multiple: false
        }) as string | null

        if (!selected) {
            ElMessage.warning('未选择游戏目录，已取消启动')
            return
        }

        await invoke('set_game_path', { path: selected })
        await invoke('launch_game')
    }
    catch (err: any) {
        console.error('启动游戏过程中发生错误：', err)
        ElMessage.error(err.message || '启动游戏失败，请检查路径或日志')
    }
}

// 加载选中的 MOD
const loadSelectedMods = async () => {
    if (selectedMods.value.length === 0) {
        ElMessage.warning('请先选择要加载的 MOD');
        return;
    }

    console.log('要发送的 MOD 文件:', selectedMods.value);
    selectedMods.value.forEach((mod) => {
        console.log(typeof mod, mod);
    });

    loadingInstance = ElLoading.service({
        lock: true,
        text: '正在应用 MOD...',
        background: 'rgba(0, 0, 0, 0.4)',
    });

    try {
        await invoke('apply_mods', { mods: selectedMods.value });
        ElMessage.success('MOD 已成功应用到游戏目录');
    } catch (err: any) {
        console.error('加载 MOD 出错：', err);
        ElMessage.error(err.message || 'MOD 加载失败，请检查路径或日志');
    } finally {
        loadingInstance?.close();
    }
};

// 删除选中的mod并恢复原文件
const deleteSelectedMods = async () => {
    if (selectedMods.value.length === 0) {
        ElMessage.warning('请先选择要删除的 MOD');
        return;
    }

    const confirm = window.confirm(`将恢复原文件并删除以下 MOD：\n${selectedMods.value.join('\n')}`);
    if (!confirm) return;

    loadingInstance = ElLoading.service({
        lock: true,
        text: '正在恢复并删除 MOD...',
        background: 'rgba(0, 0, 0, 0.4)',
    });

    try {
        await invoke('restore_and_delete_mods', { mods: selectedMods.value });
        ElMessage.success('MOD 已恢复并删除');
        await fetchModList();
        selectedMods.value = [];
    } catch (err: any) {
        console.error('恢复并删除 MOD 出错：', err);
        ElMessage.error(err.message || '操作失败，请检查路径或日志');
    } finally {
        loadingInstance?.close();
    }
};

async function handleSearch() {
    try {
        if (!searchQuery.value) {
            // 如果搜索框为空，就重新加载所有mods
            const allMods = await invoke<string[]>('list_mods');
            ModData.modList = allMods.map(name => ({
                name,
                type: '其他',    // 这里可以以后扩展成后端解析的type
                author: '',
                version: ''
            }));
            return;
        }

        const results = await invoke<string[]>('search_mods', { query: searchQuery.value });
        ModData.modList = results.map(name => ({
            name,
            type: '其他',
            author: '',
            version: ''
        }));
    } catch (err) {
        ElMessage.error('搜索失败: ' + err);
    }
}

onMounted(async () => {
    const allMods = await invoke<string[]>('list_mods');
    ModData.modList = allMods.map(name => ({
        name,
        type: '其他',
        author: '',
        version: ''
    }));
});
</script>

<style scoped>
/* 让 checkbox 整行铺满，label 部分可收缩截断 */
.mod-checkbox {
    display: flex;
    align-items: center;
    width: 100%;
}

/* 调整 checkbox 输入框与文字的间距 */
.mod-checkbox .el-checkbox__input {
    margin-right: 8px;
}

/* 使 label 部分 flex 收缩，超出截断 */
:deep(.mod-checkbox .el-checkbox__label) {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
}

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

/* mod列表项样式 */
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

.mod-list-item span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
    display: inline-block;
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
    width: 100%;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: space-around;
    margin-bottom: 20px;
}

.bottom-card-btn {
    width: 120px;
    height: 48px;
    border-radius: 8px;
    background-color: rgb(46, 51, 60);
    color: white;
    text-align: center;
    line-height: 48px;
}

/* "加载mod" 按钮的样式 */
/* .bottom-card-btn:nth-of-type(1) {
    background-color: #2196f3;
    color: white;
} */

/* "启动WOTB" 按钮的样式 */
/* .bottom-card-btn:nth-of-type(2) {
    background-color: #4caf50;
    color: white;
} */
</style>
