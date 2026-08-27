<div align="center">

<img src="src-tauri/icons/icon.png" width="96" alt="DSH Desktop logo" />

# DSH Desktop

[![CI](https://github.com/rhythm1995/dsh-desktop/actions/workflows/ci.yml/badge.svg)](https://github.com/rhythm1995/dsh-desktop/actions/workflows/ci.yml)

**DeepSeek Harness 的 Tauri v2 桌面壳 + Node Host。换壳，不换核。**

</div>

[DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness)（DSH）官方桌面版基于 Electron。本仓库把「窗口引擎」换成 Tauri v2，把产品内核原样保留：Cordis + DSH + 插件 + pnpm 跑在一个长驻 Node Host 进程里，Tauri 只负责窗口、托盘、对话框、安装器这些原生壳层职责。

这不是把 Electron API 翻译成 Tauri——官方 Web Client、Harness 内核、社区插件、Market、profile 语义**一行不改**，本仓库只实现壳层与一个 `IpcDesktopRuntime` 适配层。

> [!IMPORTANT]
> 产品内核（`.deepseek-harness` 检出，pin 记录在 [`docs/kernel-pin.json`](docs/kernel-pin.json)）必须原封不动：不改内核文件，不把桌面补丁打进内核树。版本对齐官方桌面项目的 `upstream.json`，同步流程见 [`docs/kernel-sync.md`](docs/kernel-sync.md)。

## 架构

```text
用户
  └─ Tauri v2 (Rust)          src-tauri/
       ├─ 单实例 / 窗口 / 托盘 / 菜单 / 对话框 / 通知 / 安装器
       ├─ 拖放路径、外链、主题/材质
       └─ spawn Node 进程      host/dist + 官方 dsh-plugin-desktop
            └─ Cordis + DSH + 插件
                 ├─ loopback HTTP/WS → Webview 同一 origin
                 └─ DesktopRuntime → JSON-RPC stdio → Rust
```

- **Tauri 壳**（`src-tauri/`）：窗口、托盘、原生对话框、恢复窗口、渲染器准入反向代理（环回请求自动携带密钥头）。
- **Node Host**（`host/`）：长驻进程，`boot()` 官方 Cordis 树，Web UI 绑在 `127.0.0.1` 临时端口；桌面操作经 JSON-RPC over stdio 回调 Rust。stdout 只走 JSON-RPC，日志走 stderr。
- **native-ui/**：启动屏、对话框、profile 创建、恢复窗口等原生窗口的静态页面。
- Renderer 只加载 loopback 同源内容，不暴露任何原生 API。

## 目录结构

| 目录 | 内容 |
| --- | --- |
| `src-tauri/` | Rust 壳（窗口、托盘、对话框、profile、恢复、渲染器代理） |
| `host/` | Node Host 进程；`host/upstream/tauri-host.ts` 是注入官方 `dsh-plugin-desktop` 的接缝 |
| `native-ui/` | 原生窗口静态页面（从官方插件 `lib/native-ui` 同步重建） |
| `docs/` | 架构、缺口、内核同步、CI/CD、开发监听文档（索引见 [`docs/README.md`](docs/README.md)） |
| `scripts/` | 官方 Host 同步、构建 profile、生产打包脚本 |

`.anywhere-labs-dsh-desktop/`（原始 Electron 项目）与 `.deepseek-harness/`（内核）是 git 忽略的本地检出，只用于对照，不进本仓库。

## 快速开始

环境要求：macOS、Node `^22.19.0 || >=24.0.0`、pnpm 11、Rust toolchain（Tauri v2）。

```bash
pnpm install
pnpm build:host      # 编译 Node Host（同步官方插件 + tsc）
pnpm tauri dev       # 开发运行
```

测试与类型检查：

```bash
pnpm typecheck       # Host TypeScript 类型检查
pnpm test            # Host 测试（vitest）
cargo test           # Rust 壳测试（在 src-tauri/ 下执行）
```

> [!NOTE]
> 本地 `tauri dev` / `tauri build` 默认是**开发模式**：托盘多出 Developer Logs / Developer Network 入口，Host 暴露 `/_dsh/dev/*` 路由，可用于日志与网络监听（AI coding 工具也能接，见 [`docs/devtools-listen.md`](docs/devtools-listen.md)）。生产包没有这些入口。

## 构建与发版

```bash
pnpm tauri:prod-bundle   # 本地生产包
```

CI/CD 用 GitHub Actions（见 [`docs/github-actions-cicd.md`](docs/github-actions-cicd.md)）：

- **ci.yml**：push / PR 时跑 Host 类型检查与测试、macOS 构建验证（生产模式）。
- **release.yml**：push `v*` tag 时构建生产包并发布 GitHub Release，资产为 `.dmg` 与 tar.gz 打包的 `.app`；手动触发只打包不发布。

## 文档

完整目录与「同一事实只写一处」的约定见 [`docs/README.md`](docs/README.md)。架构细节见 [`docs/tauri-v2-sidecar-migration.md`](docs/tauri-v2-sidecar-migration.md)。
