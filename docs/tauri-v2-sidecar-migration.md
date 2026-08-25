# DSH Desktop → Tauri v2 迁移方案

**结论：能改，但不是把 Electron 应用翻译成 Tauri。正确做法是「换壳保核」——用 Tauri v2 替换窗口、托盘、安装器，把现有 Node Host（Cordis + DSH + 插件 + pnpm）改成 sidecar 长驻进程。**

分析基于本地检出 `anywhere-labs-dsh-desktop/`（已在仓库根 `.gitignore` 中忽略）：`dsh-plugin-desktop@2.0.3`，Electron `43.4.0`，上游 DSH `0.1.1-rc.2`。全部现状论据已逐条对照源码核查（2026-08），定位见文末「已核证据」。对照文档：[Tauri v2](https://v2.tauri.app/)、[Node.js sidecar](https://v2.tauri.app/learn/sidecar-nodejs/)。

本仓库实现的是 Tauri 壳 + Node Host 进程。上游 DSH Host 仍从 `anywhere-labs-dsh-desktop/` 引入，不重写 Web Client、Harness、插件契约。代码目录是 `host/`，不要用 Tauri 的 sidecar 包装术语当目录名。

## 为什么可行

当前产品已经把「页面」和「原生壳」拆开了：

- Electron main 承载单实例、profile、market/settings/插件服务、诊断与生命周期；除窗口/托盘外，这些全部随 Host 整体搬进 sidecar，无需逐个拆分。
- Host 在 **同一 Node 进程** 里启动官方 Cordis 树，把 Web UI 绑在 `127.0.0.1` 临时端口。
- Renderer 是沙箱页面，只加载 loopback HTTP/WebSocket，**不走 Electron IPC 插件系统**（全 `src/` 零 `ipcMain`/`ipcRenderer`），也不把 Electron API 暴露给页面——preload 仅 10 行，只做拖放路径解析。
- 桌面操作（终端、重启、DevTools、设置）走 Host 自己的 loopback HTTP（`desktop-settings-route`），再回调 `ctx.desktopRuntime`。
- 原生能力已经收口在 `DesktopRuntime`（`dsh-plugin-desktop/src/runtime.ts`）：窗口 generation、托盘、通知、目录选择、主题、重启/恢复。这是可以换成 `TauriDesktopRuntime` 的 seam。

所以 Web Client、官方 Harness、社区插件、Market、profile 体系 **都不需要为 Tauri 重写**。

## 为什么不能「直接换成 Tauri」

最大的耦合不是 UI，而是 **Electron 同时是窗口引擎和 Node 运行时**。

| 现状 | Tauri v2 含义 |
| --- | --- |
| Host 跑在 Electron main 里，`ctx.desktopRuntime` 是同进程对象 | Tauri 核心是 Rust，不能内嵌这套 Cordis/DSH。Host 必须变成 sidecar，`desktopRuntime` 变成 RPC |
| `electronFuses.runAsNode: true` + `ELECTRON_RUN_AS_NODE`：用 Electron 当 `node` | 必须随包装真正的 Node 二进制；pnpm / DSH CLI / Windows ACL runner 的 shim 全部要改 |
| `npm_config_runtime=electron`，原生模块按 Electron ABI 编译 | sidecar 改为 Node ABI。对 `node-pty`、`koffi`、`sharp`、`@vscode/ripgrep` 反而是简化 |
| `webUtils.getPathForFile` preload 解析拖入文件夹 | 改成 Tauri drag-drop / 自建桥；页面仍走官方 `workspaces.create` |
| `-webkit-app-region: drag/no-drag` 自定义标题栏 | Electron/Chromium 语义。Tauri 在 macOS 是 WKWebView，必须改成 `data-tauri-drag-region` 或 Rust 拖拽 |
| macOS `hiddenInset` + `trafficLightPosition` + `vibrancy: sidebar`；Windows Acrylic（koffi）/ Mica | 有对应能力（`titleBarStyle`、`window-vibrancy`），像素级复现要单独调 |
| 多窗口：主窗、`DesktopDialogWindow`、Recovery、Profile Create | Tauri 支持多 Webview；这些页面已经是独立 HTML（`native-ui/`），可复用 |
| electron-builder + `app.asar` / `app.asar.unpacked`（几乎整棵 `node_modules` 物理展开） | 改成 Tauri bundle + resources + Node sidecar，不是改个 `tauri.conf.json` |
| `npx dsh-plugin-desktop` / `electron` peer 启动 | 打包应用走 Tauri；npm CLI 启动器不在本仓库范围 |
| Crashpad / `crashReporter` | Tauri 无等价物，诊断导出换成本地 ZIP + 运行标记 |
| 更新：自研下载 DMG/NSIS（`dshdesktop.cn`） | 继续自研（Rust 或 sidecar），不必绑 Tauri Updater 格式 |

不建议的路线：

- 把 DSH Host 用 pkg / SEA 打成单文件：`node-pty`、`koffi` 等 native addon 过不了。
- 在 Tauri 里嵌 libnode 同进程跑 Cordis：无官方支持，ABI/生命周期风险高于 sidecar。
- 重写 Web UI 或插件系统：与 Desktop 的设计相反（「不另造 renderer IPC」）。

## 目标架构

```text
用户
  └─ Tauri v2 (Rust)
       ├─ 单实例 / 窗口 / 托盘 / 菜单 / 对话框 / 通知 / 安装器
       ├─ 拖放路径、系统打开、主题/材质
       └─ spawn Node sidecar
            └─ 现有 dsh-plugin-desktop Host
                 ├─ Cordis + 官方 DSH + 第三方插件
                 ├─ loopback HTTP/WS  →  Webview 加载同一 origin
                 ├─ desktopProfiles / desktopPnpm（保留）
                 └─ DesktopRuntime 代理  →  JSON-RPC（stdio 或 UDS）→ Rust
```

启动顺序保持现产品语义：

1. Tauri 拿单实例锁，读 Desktop 私有 profile/mode。
2. 拉起 Node sidecar，注入 `DSH_HOME`、profile、bundled pnpm/node 路径。
3. Sidecar 启动 Host，绑定 loopback。
4. Sidecar RPC `schedule/mount`，Tauri 按 `DesktopShellSpec` 建窗并加载 `http://127.0.0.1:<port>`。
5. Web surface 成功后再提交 last-known-good；关窗隐藏、托盘退出才杀 sidecar。
6. profile/mode 切换：dispose generation → 重启 sidecar（或整应用 relaunch），不跨 generation 缓存窗口/进程。

### 归属权（已冻结）

1. **profile/mode 的首读。** Rust 只读两个最小字段：默认/上次 profile 名、上次 mode。其余决策留在 sidecar。成功启动后 sidecar 回写 `native-bootstrap.json`。
2. **启动恢复窗口。** 恢复决策可以在 sidecar，窗口归 Tauri。sidecar 启动失败时走独立通道 `event.sidecarFailed` → Tauri 打开 Recovery UI，不依赖 Host 已经起来。

Webview 仍然不暴露原生 API。拖放路径是唯一必须补的页面桥：WKWebView 里页面拿到的 `File` 对象不含磁盘路径，而 Tauri 的原生 drag-drop 拦截（`dragDropEnabled`）与页面接收 HTML5 drop 事件互斥——由 Rust 侧拿到路径后合成 `dsh-desktop-folder-drop` 事件转发进页面并做位置关联。

`-webkit-app-region` 除 `native-ui/` 外还注入在 Desktop 自有的 client overlay（`src/client/styles.ts`），不碰上游 Web client 的 DOM。本仓库标题栏使用 `data-tauri-drag-region`。

stdout 专供 JSON-RPC；sidecar 日志只走 stderr。

## 实施范围

全部按同一优先级交付：

- `src-tauri/`：窗口、托盘、单实例、外链、缩放、窗口状态、Host 进程、JSON-RPC、材质、拖放、更新下载、诊断导出、多窗口。
- `host/`：无 `import 'electron'` 的 Host 入口；`DesktopRuntime` 经 `IpcDesktopRuntime` 调用原生壳。
- 官方 Node 二进制（开发用当前 `process.execPath`）+ `host/` 编译产物作为 resources；原生模块按 Node ABI，不要 pkg。
- Recovery / Profile Create / DesktopDialog 多窗口（`native-ui/`）。
- 目录选择、通知、badge/任务栏闪烁。
- 拖放文件夹路径桥。
- 标题栏拖拽适配。
- `ELECTRON_RUN_AS_NODE` shim → 真实 `node`。
- compatibility / extended / advanced 几何；macOS vibrancy、Windows acrylic/mica；Linux 仅 compatibility。
- Tauri bundler（macOS / Windows）；更新流保留 `dshdesktop.cn` version API。
- 本仓库门禁用 Host/RPC/材质/shim 测试覆盖。

## 明确不改的范围

- `deepseek-harness/` 子模块与上游 DSH 包
- `dsh-community-fabric/`、`dsh-community-market/` 的插件契约
- 官方 Web client / 三种模式的 DOM 结构（只改拖拽 region 适配）
- profile、checkpoint、recovery 工作流语义
- 不把 Desktop 插件改成 Tauri command 系统
- npm `dsh-plugin-desktop` CLI 启动器

## 主要风险

1. **macOS WKWebView vs Chromium**：上游 UI 在 Electron/Chromium 上长出来。Windows WebView2 风险低；macOS 必须验证 WebSocket、布局、`backdrop-filter`、拖拽（含 HTML5 drop 与 Tauri 拦截互斥）。
2. **进程拆分**：今天 Host 与窗口同生共死。拆 sidecar 后要处理崩溃、卡死、端口占用、孤儿进程。
3. **安装包体积**：去掉 Chromium 会瘦一截，但 Node + 整棵 DSH `node_modules` 仍在。别预期变成「几 MB 的 Tauri 应用」。
4. **代码签名**：sidecar 里的 `node`、native addon、解包 JS 树，macOS notarize / Windows Authenticode 比 asar 更碎。
5. **自定义标题栏**：不改 drag region，三种模式都会废。
6. **双壳维护**：若同时留 Electron npm 启动器和 Tauri 安装包，`DesktopRuntime` 要两套实现。本仓库只维护 Tauri 壳。

## 建议决策

1. 现有 Electron 壳已经把 Host/Client 边界做对了，Tauri 只替换 native adapter。
2. 打包应用走 Tauri；开发期 `tauri dev` + sidecar Host。
3. 放弃「Host 继续跑在 GUI 进程里」；以 `DesktopRuntime` IPC 为迁移主轴。
4. npm `dsh-plugin-desktop` CLI 不阻塞安装包迁移。

## 本地源码位置与已核证据

- 检出：`anywhere-labs-dsh-desktop/`（工作区相对路径）
- 忽略规则：仓库根 `.gitignore` 中的 `anywhere-labs-dsh-desktop/`
- 本仓库实现：`host/`、`src-tauri/`、`native-ui/`
- 上游关键 seam：`dsh-plugin-desktop/src/runtime.ts`、`electron-runtime.ts`、`electron-shell-generation.ts`、`main.ts`、`desktop-runtime-environment.ts`

以下论据已逐条对照源码核实（2026-08，路径相对 `dsh-plugin-desktop/`）：

| 论据 | 位置 |
| --- | --- |
| `DesktopRuntime` 为纯接口、无 Electron 运行时依赖 | `src/runtime.ts:157` |
| 全 `src/` 零 `ipcMain`/`ipcRenderer`；preload 仅暴露 `webUtils.getPathForFile` | `src/preload.ts:9` |
| 渲染层 `sandbox:true / nodeIntegration:false / contextIsolation:true`，`loadURL(spec.url)` | `src/window-options.ts:28-30`、`src/electron-shell-generation.ts:319` |
| Host 同进程 `boot()`，绑定 `127.0.0.1` 随机端口 | `src/main.ts:621`、`src/main.ts:734` |
| 桌面设置走严格 loopback HTTP | `src/desktop-settings-route.ts` |
| `ELECTRON_RUN_AS_NODE` shim 波及 7 个文件 | `pnpm.ts`、`desktop-terminal.ts`、`windows-pwsh-sandbox.ts`、`windows-acl-runner.ts`、`profile-materializer.ts`、`desktop-cli.ts`、`desktop-runtime-environment.ts` |
| `npm_config_runtime=electron` + electron headers | `src/pnpm.ts:219-221`、`src/desktop-runtime-environment.ts:233,267` |
| macOS `hiddenInset`/`trafficLightPosition`/`vibrancy:'sidebar'` | `src/window-options.ts:117-125` |
| Windows Acrylic/卷诊断走 koffi | `src/windows-acrylic.ts`、`src/windows-volume-diagnostics.ts` |
| native-ui 三套独立 HTML 窗口 | `src/native-ui/{desktop-dialog,profile-create,recovery}.html` |
| 自研更新 endpoint（非 autoUpdater） | `src/update-checker.ts:4` |
| `-webkit-app-region` 注入在 Desktop 自有 client overlay，不碰上游 DOM | `src/client/styles.ts:27-44`、`src/native-ui/shared/theme.css:88` |
| 单实例锁 | `src/main.ts:183` |

上游 `@deepseek-ai/dsh-app-boot` 的 Electron 独立性在 Host 侧通过「无 electron import 的 `host/src/main.ts` + 可选 `DSH_PLUGIN_DESKTOP_ROOT`」接入；未安装上游依赖时 Host 启动自有 loopback carrier，原生壳仍可完整行使。
