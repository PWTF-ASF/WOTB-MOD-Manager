# WOTB MOD Manager

> 坦克世界闪击战（World of Tanks Blitz）模组管理器 — 基于 Tauri v2 的跨平台桌面应用，支持模组的浏览、管理、部署与一键启动游戏。

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

## 功能特性

- **模组浏览** — 网格 / 列表双视图，支持按分类筛选与关键词搜索
- **批量操作** — 全选、批量启用 / 禁用、批量删除
- **一键部署** — 将启用的模组部署到游戏目录
- **启动游戏** — 内置游戏路径检测，一键拉起《坦克世界闪击战》
- **模组管理** — 重命名、修改分类、上传自定义图标
- **双主题** — 新拟态（Neumorphism）暗色 / 亮色主题，跟随系统自动切换
- **背景自定义** — 支持自定义背景图片、模糊与遮罩效果
- **跨平台** — Windows / Linux / macOS 全平台支持

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 22
- [pnpm](https://pnpm.io/) >= 9
- [Rust](https://www.rust-lang.org/) >= 1.77
- [Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)

### 安装

```sh
# 克隆仓库
git clone <repo-url> && cd WOTB-MOD-Manager

# 安装前端依赖
pnpm install
```

### 运行

```sh
# 启动开发模式（同时编译 Rust 后端与 Vue 前端）
pnpm tauri dev

# 仅启动前端开发服务器
pnpm dev
```

### 构建

```sh
# 生产构建
pnpm tauri build
```

### 测试

```sh
pnpm test
```

## 使用说明

### 添加模组

点击底部控制台「添加」按钮，选择 `.wotbmod` 或模组文件即可导入。

### 管理模组

- 点击卡片上的编辑图标可**重命名**模组
- 点击分类标签可**修改分类**
- 点击图标区域可**更换模组图标**

### 启用与部署

1. 在模组列表中切换「已启用」开关，或使用底部批量操作按钮
2. 点击「部署」将启用的模组写入游戏目录
3. 点击「启动游戏」可直接拉起游戏客户端

### 自定义背景

在系统设置页面，可以上传自定义背景图片，并调整模糊强度与遮罩透明度。

## 项目结构

```
├── src/                      # 前端源码
│   ├── assets/               # 静态资源（图标、背景图）
│   ├── components/           # Vue 组件
│   │   ├── ModLibrary.vue          # 模组库主界面
│   │   ├── NeumorphicSearchBox.vue # 新拟态搜索框
│   │   └── Settings.vue            # 系统设置页
│   ├── views/                # 页面视图
│   │   └── HomePage.vue            # 主页（路由 + 主题注入）
│   ├── composables/          # 组合式函数
│   ├── router/               # 路由配置
│   ├── store.ts              # Pinia 全局状态
│   └── main.ts               # 应用入口
├── src-tauri/                # Rust 后端
│   ├── src/                  # Rust 源码
│   ├── Cargo.toml            # Rust 依赖
│   └── tauri.conf.json       # Tauri 配置
├── package.json              # 前端依赖与脚本
├── tailwind.config.mjs       # Tailwind CSS 配置
└── vite.config.ts            # Vite 构建配置
```

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri v2 |
| 前端框架 | Vue 3 + TypeScript |
| 构建工具 | Vite 7 |
| UI 库 | Naive UI + 自定义新拟态主题 |
| 样式 | Tailwind CSS 4 |
| 状态管理 | Pinia |
| 后端语言 | Rust |
| 包管理器 | pnpm |

## 贡献指南

欢迎提交 Issue 和 Pull Request。请遵循 [行为准则](./CODE_OF_CONDUCT.md)。

## 许可证

本项目基于 [MIT License](./LICENSE) 开源。
