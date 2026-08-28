# Electron 产品形态缺口

目录：[`README.md`](README.md)。架构见 [`tauri-v2-sidecar-migration.md`](tauri-v2-sidecar-migration.md)。

对照对象是 Electron 产品形态（可选检出 `.anywhere-labs-dsh-desktop/dsh-plugin-desktop/`）。本仓库是 Tauri v2 壳 + JS Host（Bun 优先，否则 Node），目标是**产品形态一致**，不是把 Electron API 翻译过来。内核 pin 跟随 harness 的 `dsh-v*` 标签，见 [`kernel-sync.md`](kernel-sync.md)，**不**等这个 Electron 仓库先 pin。

核对日期：2026-08-28。依据是 Electron 原版 `tests/` 与源码，以及本仓库 `host/`、`src-tauri/`、`native-ui/` 的实现。适配器检出若存在，当前是 `07633418c5`。

优先级：下列缺口全部按产品缺口处理，不排期。补一项就把该项从「仍缺」挪到「已对齐」，并补上对应测试。原版测试在 `.anywhere-labs-dsh-desktop/dsh-plugin-desktop/tests/`，本仓库测试在 `host/tests/` 与 `src-tauri/src/*.rs`。

架构分层（决定每个缺口改哪里）：

- **桌面适配器层**（`dsh-plugin-desktop`，可选检出 `.anywhere-labs-dsh-desktop/dsh-plugin-desktop/`）：官方 Host 启动时 `terminal.ts` / `diagnostics.ts` / `updates.ts` / `notifications.ts` / `profiles.ts` 等插件经 profile 组合进入 boot 图，经 `DesktopRuntime` 接口调用适配器。这一层的文案与流程（托盘标签、更新轮询、通知文案）属适配器，不改内核。
- **Host 适配器层**（`host/src/`）：`IpcDesktopRuntime` 实现 `DesktopRuntime`，对应原版 `ElectronDesktopRuntime` 的职责——确认对话框、终端准备、诊断导出编排、重启确认都在这里。
- **原生壳层**（`src-tauri/`）：`shell.*` RPC 的 Rust 侧，对应原版 `electron-shell-generation.ts` / `electron-platform.ts`——托盘渲染、应用菜单、窗口几何、通知呈现、生命周期都在这里。
- 可测纯逻辑优先放 `host/src/runtime/`（vitest）或 `src-tauri/src/*.rs` 单测（cargo test）；只有必须碰 Tauri 实窗的接线留在 `lib.rs`。

## 范围

只记 **Tauri 壳必须自己做的桌面行为**。不记：

- 内核 / Harness / Web Client DOM
- electron-builder、asar、`webPreferences`、Crashpad 文件名
- 用 `ELECTRON_RUN_AS_NODE` 当 Node（Tauri 必须 shim **真实 JS 运行时 + dsh CLI**；运行时是 Bun 或 Node，不是 Electron）

## 已对齐（不要再当缺口）

| 行为 | 原版证据 | 本仓库 |
| --- | --- | --- |
| 标题栏双击最大化 / 再双击还原 | Electron `hiddenInset` + `-webkit-app-region: drag` | `TITLEBAR_DBLCLICK_SCRIPT`、`toggle_maximize`（`src-tauri/src/window_ops.rs`、`lib.rs`） |
| 打开终端：可执行 `welcome.command`、macOS `open -a Terminal`、`dsh`/`pnpm`/`node` shim、真实 Node | `tests/desktop-terminal.spec.ts` | `host/src/runtime/desktop-terminal.ts`、`host/tests/desktop-terminal.spec.ts`；Rust `chmod 0o700` |
| 关窗隐藏、托盘退出才结束 | `electron-runtime.spec.ts` | `CloseRequested` 隐藏；托盘 Quit 与 `shell.prepareToQuit` 置 `quitting` |
| Dock / 应用激活还原隐藏窗口 | `electron-runtime.spec.ts`（`activate` / `did-become-active`） | `RunEvent::Reopen` + `application_needs_reveal`（已在前台不抢焦） |
| 36 / 32 px 标题栏几何常数 | `tests/window-options.spec.ts`、`tests/client-environment.spec.ts` | `host/tests/window-chrome.spec.ts`、`src-tauri/src/window_spec.rs` |
| 材质能力门（Linux 仅 compatibility；Windows 仅 mica，acrylic 已移除并读成 off） | `tests/window-material.spec.ts` | `host/tests/window-material.spec.ts`、`src-tauri/src/materials.rs` |
| 外链 http(s)/mailto vs loopback | `electron-runtime.spec.ts` | `window_ops.rs` + 点击拦截 |
| Recovery / Dialog href 解析 | `tests/desktop-dialog-window.spec.ts`、recovery 测试 | `host/tests/dialog-recovery.spec.ts`、`dialog.rs`、`recovery.rs` |
| 1 终端：Windows 经纪人、Linux 失败、启动失败对话框 | `tests/desktop-terminal.spec.ts` | `host/src/runtime/desktop-terminal.ts`（pwsh→powershell→cmd 探测、`launch.cmd` 经纪人、11 个环境键、脚本纯 ASCII、本地化路径只进 env、Linux 抛错不建 stateDir）；Rust spawn 失败→`terminal_launch_error_dialog`；Host prepare 失败→终端错误对话框。测试：`host/tests/desktop-terminal.spec.ts`（windows broker 组）、`host/tests/ipc-desktop-parity.spec.ts`、cargo `native_effects::tests` |
| 2 macOS 应用菜单 | `tests/native-menu.spec.ts` | `src-tauri/src/app_menu.rs`（`app_menu_plan` 五菜单 + 系统角色 + 注入项四分隔符 + `native_menu_locale`）；`lib.rs` darwin `set_as_app_menu`，win/linux 无应用菜单。测试：cargo `app_menu::tests` |
| 3 托盘：图标、语言、子菜单、单击显示 | `tests/tray-icons.spec.ts`、`electron-runtime.spec.ts` | `src-tauri/src/tray_locale.rs`（`tray_label` 全表中英 + `tray_menu_plan` 组结构：Open/tools/profiles/status/mode/Quit，空组无分隔符）；`parse_tray_item` 解析 submenu（radio/checkbox/checked/enabled）；`show_menu_on_left_click(false)` + `TrayIconEvent::Click` reveal；darwin `trayIcons.templatePath`（文件名 Template 自动模板）/其他 `bluePath`；`shell.setLocale` 触发托盘+应用菜单重建。测试：cargo `tray_locale::tests`、`native_effects::tests::tray_upsert_parses_submenus` |
| 4 通知、Dock badge、Windows 闪框 | `tests/notifications.spec.ts` | 聚焦时 `shell.notifyAttention` 整体跳过（`NativeSession::focused`）；未聚焦累计计数→darwin `NSDockTile.setBadgeLabel` / win32 `FlashWindow`；聚焦/显示/`clear_attention` 清零。测试：cargo `native_effects::tests::notify_attention_skips_while_focused_and_clears_on_demand` |
| 5 诊断导出：确认、揭示、失败提示 | `electron-runtime.spec.ts`、`tests/diagnostic-export.spec.ts` | `host/src/runtime/dialog-copy.ts` 中英隐私确认（warning defaultId/cancelId 1）；确认→`shell.exportDiagnostics`→`shell.revealItem`（opener reveal_item_in_dir）；失败 stderr 日志+错误框；并发合并单任务；ZIP：`system-info.txt`（app/desktop-version/platform/arch/node/exported）、`dsh-YYYY-MM-DD(.error)?(.n)?.log` 过滤、lifecycle-events、crash-dumps、50 MiB 上限、`.tmp`+rename、保留 3 个。测试：`host/tests/ipc-desktop-parity.spec.ts`、`host/tests/dialog-copy.spec.ts`、cargo `native_effects` |
| 6 重启 / 恢复重启确认 | `electron-runtime.spec.ts`（`confirmAndRestart`） | `IpcDesktopRuntime.confirmAndRestart` 两套中英文案（question defaultId/cancelId 1）、取消不重启、并发共享确认、`prepareToQuit` 后 no-op；恢复重启置 `restart_recovery` 使下次以 `--recovery` 拉 Host。测试：`host/tests/ipc-desktop-parity.spec.ts`（restart confirmation 组） |
| 7 窗口位置：工作区裁剪与防抖 | `tests/main-window-state.spec.ts` | `window_ops.rs`：`fit_main_window_bounds`（原版三用例移植）、`load_window_bounds` 返回原版错误串（regular file/4096/version/coordinates/dimensions）、`persist_window_bounds` 4096 上限 + `.{pid}.{stamp}.tmp` rename + 0o600；`lib.rs` Moved/Resized 250ms 防抖（seq 计数合并）、最大化跳过（normal bounds）、CloseRequested 同步 flush、mount 恢复经 fit。测试：cargo `window_ops::tests` |
| 8 生命周期：全屏进出 | `electron-runtime.spec.ts` | `window_ops.rs` `FullscreenHideState` 状态机（`on_close`/`on_leave_fullscreen`/`on_show`：先退全屏后隐藏、显示恢复全屏、过渡期 show 翻转 re-fullscreen）；`lib.rs` CloseRequested→`set_fullscreen(false)`，Resized 事件应用挂起动作；reveal 时按 `on_show` 恢复。测试：cargo `window_ops::tests::fullscreen_*` |
| 9 更新：确认、下载、打开安装包 | `tests/update-*.spec.ts` | `dialog-copy.ts` 全部更新文案；`confirmDownload`/`showManualCheckResult` 原版文案与分支；`update-download.ts` 流式 `.partial`（0o600 wx）+1 GiB 上限+koly/MZ+PE 校验+原子 rename+中止清理；`shell.saveDialog`（保存对话框）→下载→`shell.openUpdate`（darwin openPath + info 框；win32 `--updated --force-run` 拉起安装器后退出；下载前 Restart and Install 确认）。测试：`host/tests/update-download.spec.ts`、`host/tests/ipc-desktop-parity.spec.ts`（update dialogs 组） |
| 10 目录选择与卷类型准入 | `tests/workspace-admission.spec.ts`、`directory-picker-route.spec.ts` | `shell.pickDirectory` 带 locale 标题（选择器标题 en/zh）；`shell.validateDirectory` 非win32 直接 allow，win32 走 `volume_admission.rs`（NTFS/ReFS、固定盘 allow、可移动 confirm、网络/未知/exFAT/FAT32/查询失败 block，FFI `GetDriveTypeW`/`GetVolumeInformationW`）；Host 端 confirm/block 对话框（原版中英文案）。测试：cargo `volume_admission::tests`、`host/tests/ipc-desktop-parity.spec.ts`（workspace admission 组）、`host/tests/dialog-copy.spec.ts` |
| 11 Profile 创建 / Recovery / Dialog 窗口契约 | `tests/profile-create-window.spec.ts`、`tests/desktop-dialog-window.spec.ts` | `src-tauri/src/profile_create.rs`（`dsh-profile-create://submit?name=` ≤1024B 百分号解码 / `cancel`）；Profile 创建窗 480×360、min 420×330、`resizable(false)`、标题 New Profile/新建 Profile、macOS 先 unhide、失败不关窗派发 `dsh-profile-create-error`（中英文案）；Dialog 窗走官方 `native-ui/desktop-dialog.html`（state base64url 无填充 + platform/frame 查询），常规 480×300、min 420×200、max 宽 620，diagnostic 680×460、min 宽 560、max 宽 860，`dsh-desktop-dialog://layout?height=N` 上报渲染高度并钳制 [200,440]、`resizable(false)`、无框+父窗（`parent_raw`）+skip_taskbar；Recovery 800×760、min 680×560、按主窗 −48 夹。测试：cargo `profile_create::tests`、`dialog::tests`、`renderer_proxy` |
| 16 渲染器准入（browser access 门） | `tests/desktop-browser-access.spec.ts`、`tests/webserver.spec.ts` | 上游 WebServer 现按 `x-dsh-desktop-renderer` 密钥头区分渲染器/普通浏览器，marker URL 无头即 403，launcher 必须提供 `ctx.desktopBrowserAccess`。本仓库：`tauri-host.ts` 迁移旧 openBrowser/LAN 设置并 provide；`DesktopShellPayload.rendererAccessHeader` 透传；Tauri 壳 `src-tauri/src/renderer_proxy.rs` 环回反代——首导航路径密钥=header 值、同源子资源凭 `Referer` 授权（`resolve_authorized_path`）、代理 `Origin` 重写为官方 origin、HTTP+WS 隧道、3xx Location 重写、HTML 改写仅留 content-length、`<head>` 注入 origin 重写脚本；`shell.schedule` 启动并改写 webview URL，`shell.release` 关停。实机已验证官方客户端完整启动（`startup.run.completed`）。测试：cargo `renderer_proxy::tests`、`host/tests/ipc-desktop-runtime.spec.ts` |
| 12 Renderer 启动健康的产品 UX | `tests/renderer-health.spec.ts`、`renderer-boot.spec.ts` | `host/tests/renderer-health.spec.ts`（门状态机 9 用例：证据+mount 双条件、顺序无关、begin 前忽略、首失败锁定、超时、commit 失败拒绝、stop 后忽略、去重、非法超时）；`reportRendererBoot` 失败且无活动门时兜底插件恢复对话框（原版文案/按钮/defaultId 0 cancelId 2→openTerminal/restart）；占位 `main.ts` 接真实健康门 + 页面 POST 上报，不再假报 healthy。测试：`host/tests/renderer-health.spec.ts`、`host/tests/ipc-desktop-parity.spec.ts`（renderer boot 组）、`host/tests/loopback-host.spec.ts` |
| 13 主题 / 语言作用到原生壳 | `tests/theme-presenter.spec.ts` | `IpcDesktopRuntime.setLocalePreference(undefined)` 回退系统语言（LC_ALL/LC_MESSAGES/LANG→Intl，`desktopLocaleFromLanguageTag`），相同不动；`shell.setLocale` 记录并重建托盘+应用菜单（后续对话框按 locale 取文案）；`shell.setTheme` Windows 重刷 mica 材质。测试：`host/tests/ipc-desktop-parity.spec.ts`（locale fallback 组）、`host/tests/dialog-copy.spec.ts` |
| 14 键盘缩放与按 mode 红绿灯 | `electron-runtime.spec.ts` | `window_ops.rs` `zoom_shortcut`（keyDown only、ctrl/meta、排除 alt、`+`/`=`、`-`/`_`、`0` 复位）+ `ZOOM_SHORTCUT_SCRIPT` 页面注入 + `zoom_change` 命令（clamp [-4,4]、`documentElement.style.zoom`）；红绿灯按 `plan.chrome.titlebar_height`（36/32）在 mount 时应用。测试：cargo `window_ops::tests::zoom_shortcut_gates_keys_and_modifiers` |
| 15 文件夹拖放到官方页面 | `tests/client-workspace-folder-drop.spec.ts` | `FILE_PATH_BRIDGE_SCRIPT` 在主窗所有页面（loopback + 官方 client）注入：原生 drop 的 CustomEvent 记录 paths，`window.__DSH_DESKTOP_FILE_PATH__.getPathForFile(file)` 在紧随的 HTML5 drop 事件返回单目录路径（空/多目录返回空串，由页面逻辑拒绝）。适配器侧 `12e88bf129` 修了侧栏与聊天拖放遮罩冲突（client + yarn patch），有检出时随 checkout 生效。测试：cargo `window_ops::tests`（脚本契约）、`host/tests/file-path-bridge.spec.ts`（桥语义） |

## 仍缺

### 首次设置向导（P0）

原版：`src/setup-wizard-window.ts`、`native-ui/setup-wizard/`、`tests/setup-wizard-window.spec.ts`。首次启动若无向导状态，打开独立窗口，经 `dsh-setup-wizard://complete|skip` 写回 mode / 材质 / 浏览器访问 / 网络暴露 / Market / 通知。2026-08-27 上游又扩大了它的范围：欢迎页、浏览器访问与 compatibility 模式联动（`dsh-desktop-` marker 门）、Windows 启动即显示（`show: platform === 'win32'`）、免锁原子写（`withFileLock` 全部退场）、`migrateDesktopBrowserAccessSettings` 首启迁移。

现状：Electron 适配器检出里已有向导源码，`lib/native-ui/setup-wizard.html` 已构建未接入。Tauri 壳没有对应窗口；`host/upstream/tauri-host.ts` 只接了迁移调用，不拦首启状态。这是壳层缺口，不挡跟随 harness 标签。

还差：辅助窗加载 `setup-wizard.html`、解析 `dsh-setup-wizard:` href、把结果交回 Host。测试写在 `host/tests` 解析函数 + cargo 开窗契约。

win32 专属 FFI（卷查询、`FlashWindow`、安装器拉起）在 macOS 交付线上不编译；决策与文案有平台无关单测。

已完成项的实现细节以代码和上表「本仓库」列为准，不再另写一份实现方案（避免和代码双份漂移）。


## Electron 专有、不要搬

| 项 | 原因 |
| --- | --- |
| `webPreferences` sandbox / preload | Tauri Webview 安全模型不同 |
| electron-builder / asar | 改用 Tauri bundle |
| `ELECTRON_RUN_AS_NODE` 当运行时 | 终端/pnpm shim 必须指向真实 Node |
| Crashpad 路径名 | 诊断 ZIP 用本仓库格式即可 |
| `BrowserWindow` 构造选项袋 | 用 Tauri window builder 表达同一产品几何 |

## 补缺口时怎么用原版测试

不要把 `dsh-plugin-desktop/tests/*.spec.ts` 原样拷进本仓库：它们依赖 Electron 注入。做法：

1. 读原版测试断言的**产品契约**（文件名、权限位、文案、对话框按钮、尺寸）。
2. 在 `host/tests/` 或 `src-tauri` 单测里用同一契约写失败用例。
3. 实现只放 `host/`、`src-tauri/`、`native-ui/`。不要改 `.deepseek-harness`。适配器检出里除注入 `tauri-host.ts` 外不要当内核源码改。

已经这样同步过的例子：`host/tests/desktop-terminal.spec.ts` ← 原版 `desktop-terminal.spec.ts` 的 macOS 部分（Node shim，不是 `ELECTRON_RUN_AS_NODE`）。
