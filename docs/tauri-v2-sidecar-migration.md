# 架构：Tauri 壳 + JS Host

目录：[`docs/README.md`](README.md)。产品缺口见 [`electron-parity-gaps.md`](electron-parity-gaps.md)。上游同步见 [`kernel-sync.md`](kernel-sync.md)。

**已落地。** 不是把 Electron 应用翻译成 Tauri，而是换壳保核：Tauri v2 管窗口、托盘、安装器；JS Host（Cordis + DSH + 插件 + pnpm）是长驻进程。运行时优先 Bun，没有可用的 Bun 时回退 Node。包管理仍是 pnpm，不要换成 `bun install`。代码目录叫 `host/`，不要用 sidecar 当目录名。

分析基于 git 忽略的检出：`.deepseek-harness/`（内核，pin 见 `kernel-pin.json`，跟随最新 `dsh-v*` 标签）和可选的 `.anywhere-labs-dsh-desktop/`（`dsh-plugin-desktop` 适配器 + Electron 形态对照，**不是** pin 来源）。

## 为什么这样拆

Electron 同时是窗口引擎和 Node 运行时。产品本身已经把「页面」和「原生壳」拆开：

- Host 在同一 Node 进程里 `boot()` 官方 Cordis 树，Web UI 绑在 `127.0.0.1` 临时端口。
- Renderer 只加载 loopback HTTP/WebSocket，不走 Electron IPC；preload 原先只做拖放路径。
- 桌面操作走 Host 的 loopback HTTP，再回调 `ctx.desktopRuntime`（`dsh-plugin-desktop/src/runtime.ts` 纯接口）。

因此 Web Client、Harness、社区插件、Market、profile **不为 Tauri 重写**。本仓库只实现 `IpcDesktopRuntime` + Rust 壳。

不能做的：把 Host 打成单文件（native addon）、在 Tauri 里嵌 libnode、重写 Web UI。

## 进程与启动

```text
用户
  └─ Tauri v2 (Rust)          src-tauri/
       ├─ 单实例 / 窗口 / 托盘 / 菜单 / 对话框 / 通知 / 安装器
       ├─ 拖放路径、外链、主题/材质
       └─ spawn 应用内 JS 运行时   vendor/runtimes（Bun，否则 Node）→ host/dist
            └─ Cordis + DSH + 插件
                 ├─ loopback HTTP/WS → Webview 同一 origin
                 └─ DesktopRuntime → JSON-RPC stdio → Rust
```

1. Tauri 拿单实例锁，读 `native-bootstrap.json` 里的 profile 名和 mode。
2. 拉起 **打进安装包的** JS 运行时，不依赖用户自己装的 Node/Bun。顺序：`DSH_NODE_BINARY` 覆盖 → 应用内 `runtimes/bun`（探测通过才用）→ 应用内 `runtimes/node` → 开发机上的系统 Bun/Node。二进制由 `pnpm fetch:runtimes` 按 `scripts/runtimes.lock.json` 拉到 `vendor/runtimes/`，Tauri 打进 `Contents/Resources/runtimes/`。Bun 额外带 `--bun --no-env-file --no-install`。官方 Host 仍需要 Node 的 `findPackageJSON` / `registerHooks`；当前 Bun 没有这两项时用应用内 Node，避免进 Recovery。垫片仍保留。`DSH_NODE_BINARY` 可强制指定。
3. Host 绑定 loopback，RPC `schedule` / `mount`。
4. 壳按 `DesktopShellSpec` 加载 `http://127.0.0.1:<port>`。
5. 关窗隐藏；托盘退出或 `prepareToQuit` 才结束进程。
6. profile/mode 切换：释放 generation 后重启，不跨代缓存窗口。

stdout 只走 JSON-RPC；Host 日志走 stderr。

### 归属权

1. **profile/mode 首读**在 Rust（两个字段）。其余决策在 Host。启动成功后 Host 回写 `native-bootstrap.json`。
2. **恢复窗口**归 Tauri。Host 起不来时 `event.sidecarFailed` 打开 Recovery，不依赖 Host 已起来。
3. Webview 不暴露原生 API。拖放由 Rust 拿路径再注入页面桥（见缺口文档已对齐第 15 项）。

## 本仓库改什么 / 不改什么

改：`src-tauri/`、`host/`、`native-ui/`。官方 Host 启动器仍从可选检出 `dsh-plugin-desktop` 引入（`DSH_PLUGIN_DESKTOP_ROOT`），注入文件是 `host/upstream/tauri-host.ts`。内核版本以 `docs/kernel-pin.json` 为准，跟随 harness `dsh-v*` 标签。

不改：内核树、社区插件契约、官方 Web client DOM、profile/checkpoint/recovery **语义**、npm `dsh-plugin-desktop` CLI 启动器。

## 源码位置

- 内核检出：`.deepseek-harness/`（gitignore；同步见 [`kernel-sync.md`](kernel-sync.md)）
- 适配器 / Electron 对照（可选）：`.anywhere-labs-dsh-desktop/`（gitignore，不是 pin）
- 适配器 seam：`dsh-plugin-desktop/src/runtime.ts`、`electron-runtime.ts`、`electron-shell-generation.ts`、`main.ts`

| 论据 | 上游位置（相对 `dsh-plugin-desktop/`） |
| --- | --- |
| `DesktopRuntime` 纯接口 | `src/runtime.ts` |
| 全 `src/` 零 `ipcMain`/`ipcRenderer` | `src/preload.ts` |
| Host `boot()` + loopback | `src/main.ts` |
| 桌面设置走 loopback HTTP | `src/desktop-settings-route.ts` |
| 自定义标题栏 `-webkit-app-region` | `src/client/styles.ts`、`src/native-ui/shared/theme.css` |
