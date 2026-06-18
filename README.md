# WOTB MOD Manager

> 坦克世界闪击战（World of Tanks Blitz）模组管理器 — Tauri v2 跨平台桌面应用，AI 驱动开发（vibe coding）。

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey)
![Version](https://img.shields.io/badge/version-1.0.0-blue)

---

## 截图

启动加载页 → 欢迎菜单 → 模组列表（网格/列表双视图） → 部署进度面板

---

## 功能

| 模块 | 说明 |
|---|---|
| **加载页** | 启动时展示进度动画，依次扫描 Mod 目录、读取列表、检测冲突、加载设置 |
| **欢迎菜单** | 零 Mod 时显示引导页，可直接添加 Mod 或打开文件夹 |
| **模组库** | 网格/列表双视图，分类筛选（3D模型/语音/UI/地图等），关键词搜索 |
| **模组管理** | 重命名、修改分类、上传自定义图标，Naive UI 模态框交互 |
| **批量操作** | 全选、批量启用/禁用、批量删除 |
| **一键部署** | 实时进度面板，逐项展示安装状态，单 Mod 失败不影响其余 |
| **冲突检测** | 部署时自动检测文件覆盖冲突，卸载冲突旧 Mod 后安装 |
| **启动游戏** | 自动检测/配置游戏路径，一键拉起 wotblitz.exe |
| **系统设置** | 游戏路径、Mod 仓库路径、仓库迁移 |
| **外观设置** | 自定义背景图片、显示模式（铺满/适应/拉伸/平铺）、遮罩透明度、背景模糊强度 |
| **双主题** | 新拟态暗色/亮色，跟随系统自动切换 |
| **页面缓存** | `<KeepAlive>` 缓存组件状态，页面切换不重复请求、不丢失滚动和筛选 |
| **拖拽导入** | 支持拖拽 ZIP 文件到窗口直接导入 |

---

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri v2 |
| 前端框架 | Vue 3（Composition API + `<script setup>`） |
| 类型系统 | TypeScript |
| 构建工具 | Vite 7 |
| UI 组件库 | Naive UI |
| 设计风格 | 新拟态（Neumorphism）自定义 CSS 变量体系 |
| 样式 | Tailwind CSS 4 |
| 状态管理 | Pinia |
| 后端语言 | Rust |
| 测试 | Vitest（前端）+ Rust `#[cfg(test)]` |
| 包管理器 | pnpm |
| 代码质量 | Biome + ESLint + vue-tsc |

---

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) ≥ 22
- [pnpm](https://pnpm.io/) ≥ 9
- [Rust](https://www.rust-lang.org/) ≥ 1.77
- [Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)

```bash
# 克隆
git clone https://github.com/PWTF-ASF/WOTB-MOD-Manager.git
cd WOTB-MOD-Manager

# 安装
pnpm install

# 开发
pnpm tauri dev

# 构建
pnpm tauri build

# 测试
pnpm test
```

---

## 项目结构

```
├── index.html                  # 主应用 HTML 入口
├── splash.html                 # 启动加载页 HTML 入口
├── src/
│   ├── main.ts                 # Vue 应用入口
│   ├── store.ts                # Pinia 全局状态
│   ├── splash.ts               # 加载页逻辑（监听 init-progress 事件）
│   ├── assets/                 # 静态资源
│   ├── components/
│   │   ├── ModLibrary.vue             # 模组库主界面（核心组件）
│   │   ├── NeumorphicSearchBox.vue    # 新拟态搜索框
│   │   ├── Settings.vue               # 系统设置页
│   │   └── WelcomeMenu.vue           # 欢迎引导页（空态）
│   ├── views/
│   │   └── HomePage.vue               # 主页布局（侧边栏 + 主题注入 + KeepAlive）
│   ├── composables/
│   │   └── useNotification.ts         # 通知/确认对话框封装
│   └── router/
│       └── index.ts                   # 路由配置
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Rust 入口
│   │   └── lib.rs               # 全部 Tauri 命令（~40 个）
│   ├── Cargo.toml
│   ├── tauri.conf.json          # 窗口、打包、权限配置
│   └── capabilities/
│       └── default.json         # 安全能力声明
├── tests/
│   ├── unit/
│   │   └── store.test.ts        # Pinia 单元测试
│   └── setup/
│       ├── mocks.ts             # Tauri API Mock
│       ├── testglobals.ts       # 测试全局配置
│       └── install-pinia.ts     # 测试用 Pinia 安装
├── tailwind.config.mjs
├── vite.config.ts
├── vitest.config.ts
├── tsconfig*.json
├── biome.json
├── eslint.config.cjs
└── .github/workflows/release.yaml   # CI 自动构建发布
```

---

## 架构

```
┌── Vue 3 前端 ──────────────────────────┐
│  HomePage                               │
│  ├── 侧边栏（导航）                      │
│  └── <KeepAlive>                        │
│       ├── ModLibrary ←→ WelcomeMenu     │
│       └── Settings                      │
│            │                             │
│  invoke() ─┤ Tauri IPC ────────┐        │
│  listen() ─┤ 事件订阅          │        │
└────────────┼───────────────────┼────────┘
             ▼                   ▼
┌── Rust 后端 ───────────────────────────┐
│  Tauri Commands                         │
│  ├── get_mods_with_status / list_mods   │
│  ├── deploy_mods → deploy-progress 事件 │
│  ├── apply_mod_exclusive / restore_mod  │
│  ├── copy_mod_file / delete_mod_file    │
│  ├── rename_mod / update_mod_category   │
│  ├── set_mod_icon / clear_mod_icon      │
│  ├── get_game_path / set_game_path      │
│  ├── set_background_image / ...         │
│  └── greet（调试用）                     │
│                                          │
│  文件系统：mods_meta.json、game_path.json│
│  备份机制：mod_backups/（纯净镜像）       │
└──────────────────────────────────────────┘
```

---

## 开发笔记

本项目采用 **vibe coding** 方式开发——与 AI 结对编程，快速迭代。

- 所有组件使用 `<script setup lang="ts">`
- 新拟态样式通过 CSS 变量（`--neu-raised`、`--neu-inset`、`--neu-shadow-*`）统一管理
- 暗色/亮色主题通过 `:root.light-mode` / `:root.dark-mode` 切换 CSS 变量
- Rust 命令同步执行文件 I/O，耗时的部署操作通过 `#[command] async fn` + `app.emit()` 推送进度
- `<KeepAlive>` 作用于 `HomePage.vue` 内部的 `v-if` 切换，而非路由层
- 加载页（splash）通过 Vite 多页面构建独立打包

---

## 许可证

MIT License — 详见 [LICENSE](./LICENSE)。
