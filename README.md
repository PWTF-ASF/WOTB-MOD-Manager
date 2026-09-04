# WOTB MOD Manager

> 坦克世界闪击战（World of Tanks Blitz）Windows 模组管理器，基于 Tauri v2 与 Vue 3。

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey)
![Version](https://img.shields.io/badge/version-1.0.2-blue)

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
| **模组管理** | 重命名、修改分类、上传自定义图标，自研弹窗交互 |
| **批量操作** | 全选、批量启用/禁用、批量删除 |
| **一键部署** | 事务化部署并实时展示进度；任一写入失败时自动回滚整个部署 |
| **冲突检测** | 部署前检测目标路径和文件内容，阻止内容不同的 Mod 同时覆盖同一文件 |
| **启动游戏** | 自动检测/配置游戏路径，一键拉起 wotblitz.exe |
| **系统设置** | 游戏路径、Mod 仓库路径、仓库迁移 |
| **外观设置** | 自定义背景图片、显示模式（铺满/适应/拉伸/平铺）、遮罩透明度、背景模糊强度 |
| **双主题** | 扁平化暗色/亮色主题，跟随系统自动切换 |
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
| UI 组件 | 项目内自研基础组件，无第三方 UI 组件库 |
| 设计风格 | 扁平化战术控制台，自定义设计令牌与主题体系 |
| 样式 | Tailwind CSS 4 |
| 状态管理 | Pinia |
| 后端语言 | Rust |
| 测试 | Vitest（前端）+ Rust `#[cfg(test)]` |
| 包管理器 | pnpm |
| 代码质量 | ESLint + vue-tsc |

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

# 完整检查
pnpm verify

# 构建 Windows NSIS 安装包
pnpm tauri build --bundles nsis
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
│   │   ├── ModLibrary.vue             # 模组库页面编排
│   │   ├── Settings.vue               # 系统设置页
│   │   └── WelcomeMenu.vue           # 欢迎引导页（空态）
│   ├── features/mods/components/      # Mod 卡片、工具栏及业务弹窗
│   ├── ui/                            # 按钮、表单、反馈和弹窗基础组件
│   ├── stores/                        # 偏好、Mod、部署、通知与确认状态
│   ├── services/tauri/                # Tauri IPC 服务适配层
│   ├── styles/                        # 设计令牌、主题、基础与动效样式
│   ├── types/                         # 领域类型
│   ├── views/
│   │   └── HomePage.vue               # 主页布局（侧边栏 + KeepAlive）
│   ├── composables/
│   │   ├── useToast.ts                # 应用内通知
│   │   └── useConfirmDialog.ts        # 自定义确认弹窗
│   └── router/
│       └── index.ts                   # 路由配置
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Rust 入口
│   │   ├── lib.rs               # Tauri 命令与应用启动流程
│   │   └── deployment.rs        # 冲突分析、事务部署、备份与回滚
│   ├── Cargo.toml
│   ├── tauri.conf.json          # 窗口、打包、权限配置
│   └── capabilities/
│       ├── default.json         # 主窗口安全能力声明
│       └── splash.json          # 启动页最小事件权限
├── tests/
│   ├── unit/
│   │   ├── store.test.ts        # 全局状态测试
│   │   └── *.test.ts            # 服务、路由与组件测试
│   └── setup/
│       ├── mocks.ts             # Tauri API Mock
│       ├── testglobals.ts       # 测试全局配置
│       └── install-pinia.ts     # 测试用 Pinia 安装
├── tailwind.config.mjs
├── vite.config.ts
├── vitest.config.ts
├── tsconfig*.json
├── eslint.config.cjs
└── .github/workflows/release.yaml   # 可选的手动 GitHub Actions 构建流程
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
│  ├── get_mods_with_status                │
│  ├── analyze_mod_conflicts               │
│  ├── deploy_mods → deploy-progress 事件 │
│  ├── copy_mod_file / delete_mod_file    │
│  ├── rename_mod / update_mod_category   │
│  ├── set_mod_icon / clear_mod_icon      │
│  ├── get_game_path / set_game_path      │
│  ├── set_background_image / ...         │
│                                          │
│  文件系统：mods_meta.json、game_path.json│
│  部署状态：deployment_state.json          │
│  备份机制：mod_backups/（按游戏目录隔离） │
│  事务目录：deployment_transactions/       │
└──────────────────────────────────────────┘
```

---

## 开发笔记

本项目采用 **vibe coding** 方式开发——与 AI 结对编程，快速迭代。

- 所有组件使用 `<script setup lang="ts">`
- 基础组件位于 `src/ui`，通过 `--ui-*` 设计令牌保持控件和主题一致
- 暗色/亮色主题通过 `:root.light-mode` / `:root.dark-mode` 切换 CSS 变量
- 耗时文件操作通过 `spawn_blocking` 离开异步运行时，部署进度通过 `app.emit()` 推送
- 路由页面通过 `<KeepAlive>` 缓存，跨页面切换时保留筛选、草稿和滚动状态
- 加载页（splash）通过 Vite 多页面构建独立打包

---

## 许可证

MIT License — 详见 [LICENSE](./LICENSE)。
