<template>
    <div class="HomePage" :class="{ 'dark-theme': isDark, 'light-theme': !isDark }">
        <div class="background"></div>
        <div class="content">
            <aside>
                <div class="add-mod-btn" @click="selectModFolder">添加MOD</div>
                <div class="setting-btn" @click="goToSettings">设置</div>
            </aside>
            <main>
                <!-- 搜索框 -->
                <div class="search-box">
                    <input type="text" v-model="searchQuery" @input="handleSearch()" />
                </div>
                <!-- mod列表 -->
                <section class="mod-list">
                    <article class="mod-item" v-for="mod in ModData.modList" :key="mod.name">
                        <label class="mod-label">
                            <input type="checkbox" class="mod-checkbox" :value="mod.name" v-model="selectedMods" />
                            <span class="mod">{{ mod.name }}</span>
                        </label>
                    </article>
                </section>
                <footer>
                    <div class="load-mod-btn">加载MOD</div>
                    <div class="delete-mod-btn">删除MOD</div>
                    <div class="start-game-btn" @click="startGame()">启动游戏</div>
                </footer>
            </main>
        </div>
    </div>
</template>

<script setup lang="ts">
import { reactive, computed, ref, onMounted, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'

const activeTap = ref('全部') // 默认为全部
const selectedMods = ref<string[]>([])
const searchQuery = ref('')
const router = useRouter()
const isDark = ref<boolean>(
    (() => {
        const savedMode = localStorage.getItem('darkMode');
        return savedMode ? JSON.parse(savedMode) : window.matchMedia('(prefers-color-scheme: dark)').matches;
    })()
);

function goToSettings() {
    router.push('/settings')
}

// 全选勾选状态
const checkAll = ref(false)
// 是否显示半选状态
const isIndeterminate = computed(() => {
    const len = selectedMods.value.length
    return len > 0 && len < filteredModList.value.length
})

// 当用户点“全选”时，同步更新 selectedMods
function handleCheckAllChange(val: boolean) {
    if (val) {
        selectedMods.value = filteredModList.value.map(m => m.name)
    } else {
        selectedMods.value = []
    }
}

// 当 selectedMods 变化时，更新 checkAll（全选框状态）

watch(selectedMods, newVal => {
    checkAll.value = newVal.length === filteredModList.value.length
})

// 计算属性：根据选中的标签过滤 modList
const filteredModList = computed(() => {
    return activeTap.value === '全部' ? ModData.modList : ModData.modList.filter(mod => mod.type === activeTap.value)
})

//mod数据
let ModData = reactive({
    modList: [] as Array<{ name: string; type: string; author: string; version: string }>,
    tapList: [
        { name: '全部', type: '全部' },
        { name: '语音包', type: '语音包' },
        { name: '坦克模型', type: '坦克模型' },
        { name: '其他', type: '其他' },
    ],
})

// 选择mod压缩包
const selectModFolder = async () => {
    try {
        const selected = (await open({
            title: '请选择 Mod 压缩包文件',
            multiple: false,
            filters: [{ name: 'Mod 包', extensions: ['zip'] }],
        })) as string | null

        if (!selected) {
            return
        }

        await invoke('copy_mod_file', { src: selected })
        await fetchModList()
    } catch (err) {
        console.error(err)
    } finally {
    }
}

// 拉取 mods 目录下所有 ZIP 文件
async function fetchModList() {
    try {
        const mods = (await invoke('get_mod_status')) as Array<{
            name: string
            type: string
            author: string
            version: string
            applied: boolean
        }>
        ModData.modList = mods.map(mod => ({
            name: mod.name,
            type: mod.type,
            author: mod.author,
            version: mod.version,
        }))
        selectedMods.value = mods.filter(mod => mod.applied).map(mod => mod.name)
        console.log('刷新后 Mod 列表：', ModData.modList)
        console.log('已选中：', selectedMods.value)
        console.log(mods)
    } catch (e) {
        console.error('fetchModList error', e)
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
        const selected = (await open({
            title: '请选择 WOTB 游戏目录',
            directory: true,
            multiple: false,
        })) as string | null

        if (!selected) {
            return
        }

        await invoke('set_game_path', { path: selected })
        await invoke('launch_game')
    } catch (err: any) {
        console.error('启动游戏过程中发生错误：', err)
    }
}

// 加载选中的 MOD
const loadSelectedMods = async () => {
    if (selectedMods.value.length === 0) {
        return
    }

    console.log('要发送的 MOD 文件:', selectedMods.value)
    selectedMods.value.forEach(mod => {
        console.log(typeof mod, mod)
    })

    try {
        await invoke('apply_mods', { mods: selectedMods.value })
    } catch (err: any) {
        console.error('加载 MOD 出错：', err)
    } finally {
    }
}

// 删除选中的mod并恢复原文件
const deleteSelectedMods = async () => {
    if (selectedMods.value.length === 0) {
        return
    }

    const confirm = window.confirm(`将恢复原文件并删除以下 MOD：\n${selectedMods.value.join('\n')}`)
    if (!confirm) return

    try {
        await invoke('restore_and_delete_mods', { mods: selectedMods.value })
        await fetchModList()
        selectedMods.value = []
    } catch (err: any) {
        console.error('恢复并删除 MOD 出错：', err)
    } finally {
    }
}

async function handleSearch() {
    try {
        if (!searchQuery.value) {
            // 如果搜索框为空，就重新加载所有mods
            const allMods = await invoke<string[]>('list_mods')
            ModData.modList = allMods.map(name => ({
                name,
                type: '其他', // 这里可以以后扩展成后端解析的type
                author: '',
                version: '',
            }))
            return
        }

        const results = await invoke<string[]>('search_mods', { query: searchQuery.value })
        ModData.modList = results.map(name => ({
            name,
            type: '其他',
            author: '',
            version: '',
        }))
    } catch (err) { }
}

//更改背景色
function applyBackground(settings: any) {
    const background = document.querySelector('.background') as HTMLElement;
    if (!background) return;

    // 先清空所有背景相关样式
    background.style.backgroundImage = '';
    background.style.backgroundColor = '';
    background.style.backdropFilter = '';
    background.style.filter = '';
    background.style.backgroundSize = '';
    background.style.backgroundRepeat = '';
    background.style.backgroundPosition = '';

    switch (settings.type) {
        case 'color':
            // 兜底：如果子组件没传 color，默认用白色
            background.style.backgroundColor = settings.color || '#ffffff';
            break;
        case 'image':
            // 兜底：避免图片路径为空导致报错
            if (settings.imagePath) {
                background.style.backgroundImage = `url("${settings.imagePath}")`;
                background.style.backgroundSize = 'cover';
                background.style.backgroundRepeat = 'no-repeat';
                background.style.backgroundPosition = 'center';
                // 兜底：blur 为负数时设为 0
                background.style.filter = `blur(${Math.max(0, settings.blur || 0)}px)`;
            }
            break;
        case 'gradient':
            if (settings.gradient) {
                background.style.backgroundImage = settings.gradient;
                background.style.backgroundSize = 'cover';
            }
            break;
        // 兜底：默认切回白色背景
        default:
            background.style.backgroundColor = '#ffffff';
    }
}

onMounted(async () => {
    await fetchModList();

    // 新增：初始化样式（让页面加载时就应用正确的主题）
    const root = document.documentElement;
    root.classList.toggle('dark', isDark.value);
    root.classList.toggle('light', !isDark.value);

    const saved = localStorage.getItem('userSettings');
    if (saved) {
        try {
            const settings = JSON.parse(saved);
            applyBackground(settings); // 直接调用处理所有类型的 applyBackground
        } catch (e) {
            console.error('背景设置解析失败', e);
        }
    }
})

watch(isDark, (newMode) => {
    // 1. 同步根元素（html）的类（和Settings组件保持一致）
    const root = document.documentElement;
    root.classList.toggle('dark', newMode);
    root.classList.toggle('light', !newMode);

    // 2. 同步本地存储（确保刷新后状态不丢失）
    localStorage.setItem('darkMode', JSON.stringify(newMode));
});
</script>

<style scoped>
* {
    box-sizing: border-box;
}

/* 暗色模式 */
.dark-theme {
    --bg-color: #1e1e1e;
    --text-color: #f0f0f0;
    --card-bg: #2a2a2a;
    --aside-bg: rgba(30, 30, 30, 0.8);
    --aside-bg-no-filter: #2a2a2a;
    --panel-bg: rgba(30, 30, 30, 0.8);
    --panel-bg-no-filter: #2a2a2a;
    /* 侧边栏样式 */
    --aside-item-active-bg: rgba(30, 41, 59, 0.8);
    --aside-item-active-color: #e2e8f0;
    --aside-item-hover-bg: rgba(45, 55, 72, 0.6);
    --aside-item-hover-color: #f8fafc;
    /* 指示器样式 */
    --indicator-bg: #3b82f6;
    /* 按钮样式 */
    --btn-bg: rgba(64, 158, 255, 0.18);
    /* 加深背景 */
    --btn-text: oklch(65% 0.12 240);
    --btn-border: oklch(65% 0.12 240);
    --btn-hover-bg: oklch(70% 0.13 240);
    --btn-hover-text: #ffffff;
    --btn-hover-shadow: 0 4px 12px hsl(220 40% 0% / 0.5);
    /* 危险操作变量 */
    --danger-border: rgba(245, 108, 108, 0.4);
    --danger-bg: rgba(245, 108, 108, 0.15);
    --danger-text: oklch(80% 0.2 20);
    --danger-hover-bg: oklch(70% 0.2 20);
    --danger-hover-text: #fff;
    /* 主要操作变量 */
    --primary-color: #4096ff;
    --primary-text: #fff;
    --primary-hover: #3684e6;
    /* 图标样式 */
    --icon-filter: brightness(0.9);
    --icon-hover-filter: brightness(1);
    --icon-hover-drop-shadow: drop-shadow(0 0 10px rgba(221, 245, 255, 0.8));
    /* 下拉框样式 */
    --select-border: 1px solid #cbd5e1;
    --select-box-shadow: 0 0 0 1px rgba(203, 213, 225, 0.3);
    --select-focus-boder: #94a3b8;
    --select-focus-box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.2);
    /* 卡片样式 */
    --card-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
    --card-glow: 0 0 5px rgba(255, 255, 255, 0.3);
}

/* 浅色模式 */
.light-theme {
    --bg-color: #ffffff;
    --text-color: #333333;
    --card-bg: #f5f5f5;
    --aside-bg: rgba(238, 238, 246, 0.8);
    --aside-bg-no-filter: #f0f0f0;
    --panel-bg: rgba(238, 238, 238, 0.8);
    --panel-bg-no-filter: #f0f0f0;
    /* 侧边栏样式 */
    --aside-item-active-bg: rgba(230, 240, 250, 0.85);
    --aside-item-active-color: #333333;
    --aside-item-hover-bg: rgba(240, 242, 245, 0.7);
    --aside-item-hover-color: #333333;
    /* 指示器样式 */
    --indicator-bg: #1677ff;
    /* 按钮样式 */
    --btn-bg: #e6f7ff;
    /* 浅主色背景 */
    --btn-text: #409eff;
    --btn-border: #b3d8ff;
    --btn-hover-bg: #66b1ff;
    --btn-hover-text: #0d1117;
    --btn-hover-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    /* 危险操作变量 */
    --danger-border: rgba(245, 108, 108, 0.3);
    --danger-bg: #fff5f5;
    --danger-text: #f56c6c;
    --danger-hover-bg: #f56c6c;
    --danger-hover-text: #fff;
    /* 主要操作变量 */
    --primary-color: #4096ff;
    --primary-text: #fff;
    --primary-hover: #3684e6;
    /* 图标样式 */
    --icon-filter: brightness(0.7);
    --icon-hover-filter: brightness(0.6);
    --icon-hover-drop-shadow: drop-shadow(0 0 8px rgba(38, 41, 42, 0.8));
    /* 下拉框样式 */
    --select-border: 1px solid #334155;
    --select-box-shadow: 0 0 0 1px rgba(51, 65, 85, 0.3);
    --select-focus-boder: #64748b;
    --select-focus-box-shadow: 0 0 0 2px rgba(100, 116, 139, 0.2);
    /* 卡片样式 */
    --card-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
    --card-glow: 0 0 3px rgba(255, 255, 255, 0.5);
}


/* 清除input默认样式 */
input {
    all: unset;
}

.HomePage {
    width: 100%;
    height: 100vh;
    overflow: hidden;
    position: relative;
}

.background {
    position: absolute;
    z-index: 0;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    transition:
        background 0.3s ease,
        backdrop-filter 0.3s ease;
}

.content {
    position: relative;
    z-index: 1;
    width: 800px;
    height: 600px;
    display: flex;
    overflow: hidden;
    color: var(--text-color);
}

.HomePage aside {
    height: 100%;
    width: 192px;
    box-sizing: border-box;
    backdrop-filter: blur(20px);
    background-color: var(--aside-bg);
    position: relative;
    padding: 20px 10px 10px 10px;
    box-shadow: 10px 0 10px -5px rgba(0, 0, 0, 0.3);
    flex: 0 0 192px;
    display: flex;
    justify-content: center;
}

aside>div {
    width: 130px;
    height: 36px;
    border: 1px solid var(--btn-border);
    background: var(--btn-bg);
    color: var(--btn-text);
    text-align: center;
    margin-bottom: 12px;
    line-height: 36px;
    cursor: pointer;
    transition: all 0.25s ease;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 500;
    /* 基础阴影：主题自适应 */
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
}

/* 侧边栏按钮hover状态（阴影+发光） */
aside>.add-mod-btn:hover,
aside>.setting-btn:hover {
    background: var(--btn-hover-bg);
    color: var(--btn-hover-text);
    transform: translateY(-2px);
    border-color: transparent;
    /* 阴影+淡蓝色发光 */
    box-shadow:
        var(--btn-hover-shadow),
        0 0 8px rgba(64, 158, 255, 0.4);
}

/* 侧边栏按钮active状态（阴影收缩+发光减弱） */
aside>.add-mod-btn:active,
aside>.setting-btn:active {
    transform: translateY(0);
    box-shadow:
        0 2px 6px var(--dark-theme, rgba(0, 0, 0, 0.2)) var(--light-theme, rgba(0, 0, 0, 0.1)),
        0 0 4px var(--dark-theme, rgba(100, 180, 255, 0.2)) var(--light-theme, rgba(64, 158, 255, 0.15));
}

.setting-btn {
    position: absolute;
    bottom: 0px;
}

.HomePage main {
    width: 100%;
    height: 100%;
    position: relative;
    flex: 1;
    padding: 20px;
    box-sizing: border-box;
}

main>.search-box {
    width: 100%;
    height: 40px;
    box-sizing: border-box;
    margin-bottom: 20px;
}

.search-box>input {
    width: 100%;
    height: 30px;
    border-radius: 5px;
    border: 1px solid #ccc;
    padding-left: 10px;
    box-sizing: border-box;
}

.search-box>input:focus {
    border-color: #409eff;
    box-shadow: 0 0 5px rgba(64, 158, 255, 0.5);
}

.mod-list {
    width: 100%;
    height: 400px;
    padding: 0px 20px 10px 20px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow-y: auto;
}

.mod-list::-webkit-scrollbar-button {
    display: none;
}

.mod-list::-webkit-scrollbar {
    width: 8px;
}

.mod-list::-webkit-scrollbar-thumb {
    background-color: #999;
    border-radius: 4px;
}

.mod-item {
    width: 100%;
    height: 40px;
    backdrop-filter: blur(10px);
    background-color: rgba(255, 255, 255, 0.123);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    border-radius: 5px;
    display: flex;
    align-items: center;
    padding: 0 10px;
    box-sizing: border-box;
    transition: all 0.3s ease;
}

.mod-label {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    height: 100%;
    cursor: pointer;
    color: black;
}

.mod-item:hover {
    background-color: #66b1ff;
    transform: scale(1.05);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    border-radius: 5px;
    color: white;
}

.mod-checkbox {
    width: 18px;
    height: 18px;
    opacity: 1;
    display: inline-block;
    appearance: checkbox;
    -webkit-appearance: checkbox;
    -moz-appearance: checkbox;
    accent-color: #007bff;
}

.mod {
    flex: 1;
    font-size: 16px;
    line-height: 40px;
}

.HomePage footer {
    height: 50px;
    width: 100%;
    position: absolute;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    padding: 20px;
}

footer> :last-child {
    margin-right: 10px;
}

/* 页脚按钮基础样式 */
footer>div {
    min-width: 100px;
    padding: 0 16px;
    height: 36px;
    line-height: 36px;
    text-align: center;
    cursor: pointer;
    transition: all 0.25s ease;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    /* 基础阴影：主题自适应 */
    box-shadow: 0 2px 4px var(--dark-theme, rgba(0, 0, 0, 0.2)) var(--light-theme, rgba(0, 0, 0, 0.08));
}

/* 1. 加载MOD（次要操作） */
footer>.load-mod-btn {
    border: 1px solid var(--btn-border);
    background: var(--btn-bg);
    color: var(--btn-text);
}

footer>.load-mod-btn:hover {
    background: var(--btn-hover-bg);
    color: var(--btn-hover-text);
    transform: translateY(-2px);
    border-color: transparent;
    /* 阴影+淡蓝色发光 */
    box-shadow:
        var(--btn-hover-shadow),
        0 0 8px var(--dark-theme, rgba(100, 180, 255, 0.4)) var(--light-theme, rgba(64, 158, 255, 0.3));
}

footer>.load-mod-btn:active {
    transform: translateY(0);
    box-shadow:
        0 2px 6px var(--dark-theme, rgba(0, 0, 0, 0.2)) var(--light-theme, rgba(0, 0, 0, 0.1)),
        0 0 4px var(--dark-theme, rgba(100, 180, 255, 0.2)) var(--light-theme, rgba(64, 158, 255, 0.15));
}

/* 2. 删除MOD（危险操作） */
footer>.delete-mod-btn {
    border: 1px solid var(--danger-border);
    background: var(--danger-bg);
    color: var(--danger-text);
}

footer>.delete-mod-btn:hover {
    background: var(--danger-hover-bg);
    color: var(--danger-hover-text);
    transform: translateY(-2px);
    border-color: transparent;
    /* 阴影+红色发光 */
    box-shadow:
        0 4px 12px rgba(245, 108, 108, 0.2),
        0 0 8px var(--dark-theme, rgba(255, 120, 120, 0.4)) var(--light-theme, rgba(245, 108, 108, 0.35));
}

footer>.delete-mod-btn:active {
    transform: translateY(0);
    box-shadow:
        0 2px 6px rgba(0, 0, 0, 0.1),
        0 0 4px var(--dark-theme, rgba(255, 120, 120, 0.2)) var(--light-theme, rgba(245, 108, 108, 0.2));
}

/* 3. 启动游戏（主要操作） */
footer>.start-game-btn {
    border: 1px solid transparent;
    background: var(--primary-color);
    color: var(--primary-text);
}

footer>.start-game-btn:hover {
    background: var(--primary-hover);
    transform: translateY(-2px);
    /* 阴影+强蓝色发光 */
    box-shadow:
        0 4px 12px rgba(64, 158, 255, 0.25),
        0 0 10px var(--dark-theme, rgba(80, 170, 255, 0.5)) var(--light-theme, rgba(64, 158, 255, 0.4));
}

footer>.start-game-btn:active {
    transform: translateY(0);
    box-shadow:
        0 2px 6px rgba(0, 0, 0, 0.1),
        0 0 5px var(--dark-theme, rgba(80, 170, 255, 0.3)) var(--light-theme, rgba(64, 158, 255, 0.25));
}
</style>
