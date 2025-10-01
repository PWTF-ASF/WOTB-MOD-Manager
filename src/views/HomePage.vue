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
    --aside-bg: rgba(30, 30, 30, 0.7);
    --btn-bg: rgba(64, 158, 255, 0.08);
    /* 透明主色背景，强化按钮边界 */
    --btn-text: oklch(80% 0.12 240);
    /* 高亮度文字，暗背景下清晰 */
    --btn-border: oklch(70% 0.12 240);
    /* 边框稍暗，区分文字与边框 */
    --btn-hover-bg: oklch(70% 0.13 240);
    /*  hover 提亮，不刺眼 */
    --btn-hover-text: #0d1117;
    /* 纯黑文字，保证对比 */
    --btn-hover-shadow: 0 4px 12px hsl(220 40% 0% / 0.6);
    /* 深阴影，强化层次 */
}

/* 浅色模式 */
.light-theme {
    --aside-bg: rgba(255, 255, 255, 0.4);
    --btn-bg: transparent;
    --btn-text: #409eff;
    --btn-border: #409eff;
    --btn-hover-bg: #66b1ff;
    --btn-hover-text: #ffffff;
    --btn-hover-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    /* 浅阴影，不厚重 */
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
    width: 150px;
    box-sizing: border-box;
    backdrop-filter: blur(20px);
    background-color: var(--aside-bg);
    position: relative;
    padding: 20px 10px 10px 10px;
    box-shadow: 10px 0 10px -5px rgba(0, 0, 0, 0.3);
}

aside>div {
    width: 130px;
    height: 32px;
    border: 2px solid var(--btn-border);
    background: linear-gradient(145deg, var(--btn-bg), rgba(64, 158, 255, 0.05));
    color: var(--btn-text);
    text-align: center;
    margin-bottom: 10px;
    line-height: 30px;
    cursor: pointer;
    transition: all 0.3s ease;
    border-radius: 4px;
}

aside>.add-mod-btn:hover,
aside>.setting-btn:hover {
    background: linear-gradient(145deg, var(--btn-hover-bg), oklch(65% 0.13 240));
    color: var(--btn-hover-text);
    transform: scale(1.05);
    box-shadow: var(--btn-hover-shadow);
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
}

main>.search-box {
    width: 100%;
    height: 40px;
    margin: 20px;
}

.search-box>input {
    width: 600px;
    height: 30px;
    border-radius: 5px;
    border: 1px solid #ccc;
    margin-right: 10px;
    padding-left: 10px;
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
}

.mod-label {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    height: 100%;
    cursor: pointer;
    color: black;
    transition: all 0.3s ease;
}

.mod-label:hover {
    background-color: #66b1ff;
    transform: scale(1.05);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    border-radius: 5px;
    padding: 0 10px;
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
    padding: 10px;
}

footer> :last-child {
    margin-right: 10px;
}

footer>div {
    width: 130px;
    height: 32px;
    border: 2px solid var(--btn-border);
    background: linear-gradient(145deg, var(--btn-bg), rgba(64, 158, 255, 0.05));
    color: var(--btn-text);
    text-align: center;
    margin-bottom: 10px;
    line-height: 30px;
    cursor: pointer;
    transition: all 0.3s ease;
    border-radius: 4px;
}

footer>.load-mod-btn:hover,
footer>.delete-mod-btn:hover,
footer>.start-game-btn:hover {
    background: linear-gradient(145deg, var(--btn-hover-bg), oklch(65% 0.13 240));
    color: var(--btn-hover-text);
    transform: scale(1.05);
    box-shadow: var(--btn-hover-shadow);
}
</style>
