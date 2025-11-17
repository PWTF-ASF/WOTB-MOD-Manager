<template>
    <div class="HomePage">
        <div class="background"></div>
        <div class="content">
            <aside>
                <div class="add-mod-btn">
                    <img :src="AddModBtnIcon()">
                    添加MOD
                </div>
                <div class="setting-btn">
                    <img :src="SettingBtnIcon()">
                    设置
                </div>
            </aside>
            <main>
                <!-- 搜索框 -->
                <div class="search-box">
                    <input type="text" />
                </div>
                <!-- mod列表 -->
                <section class="mod-list">

                </section>
                <footer>
                    <div class="load-mod-btn">加载MOD</div>
                    <div class="delete-mod-btn">删除MOD</div>
                    <div class="start-game-btn">启动游戏</div>
                </footer>
            </main>
        </div>
    </div>
</template>

<script setup lang="ts">
import { reactive, computed, ref, onMounted, watch } from 'vue'

import settingLightIcon from '../assets/设置.svg'
import settingDarkIcon from '../assets/设置 (1).svg'
import addModLightIcon from '../assets/添加.svg'
import addModDarkIcon from '../assets/添加 (1).svg'

const darkmode = ref(false); // 默认浅色模式

const SettingBtnIcon = () => {
    return darkmode.value ? settingDarkIcon : settingLightIcon;
}
const AddModBtnIcon = () => {
    return darkmode.value ? addModDarkIcon : addModLightIcon;
}
</script>

<style scoped>
:root {}

* {
    border: 0;
}

.HomePage {
    height: 100vh;
}

.content {
    display: flex;
    height: 100%;
}

aside {
    width: 200px;
    height: 100%;
    padding: 20px 20px 0 20px;
    border: 1px solid white;
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.add-mod-btn,
.setting-btn {
    height: 48px;
    padding: 10px;
    border-radius: 5px;
    background-color: white;
    display: flex;
    align-items: center;
    gap: 15px;
}

.add-mod-btn img,
.setting-btn img {
    width: 32px;
    height: 32px;
}

main {
    flex: 1 0 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    position: relative;
}
.search-box{
    width: 100%;
    border: 1px solid white;
    padding-left: 20px;
}
.search-box input{
    width: 300px;
    height: 32px;
    padding: 0 10px;
    border-radius: 15px;
    border: 1px solid white;
    background-color: transparent;
    color: white;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275)
}
.search-box input:focus{
    border-radius: 5px;
}
footer{
    width: 100%;
    position: absolute;
    height: 64px;
    border: 1px solid white;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: end;
}
footer div{
    min-width: 96px;
    height: 48px;
    margin-right: 20px;
    border-radius: 5px;
    background-color: white;
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>