# 内核（deepseek-harness）同步方案

目录：[`README.md`](README.md)。pin 只写在 [`kernel-pin.json`](kernel-pin.json)。架构说明见 [`tauri-v2-sidecar-migration.md`](tauri-v2-sidecar-migration.md)。

本仓库的产品内核是官方 [DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness)。本地目录是 **`.deepseek-harness/`**（git 忽略，不进本仓库）。内核必须**原封不动**：不改其中任何文件，不把 Tauri / Electron 补丁打进内核树。

原始 Electron 桌面项目在 **`.anywhere-labs-dsh-desktop/`**。那边对内核的 pin 写在 `upstream.json` 和 git submodule `deepseek-harness`。本仓库把同一 pin 记在 `docs/kernel-pin.json`。

## 当前核对（2026-08-27）

| 项 | 值 |
| --- | --- |
| 远程 | `https://github.com/deepseek-ai/deepseek-harness.git` |
| `origin/master` / `origin/HEAD` | `b150a551b8d465e31e418e1b2eaf5e79bbb7d28e` |
| 标签 | `dsh-v0.1.1-rc.2`（与 HEAD 同一提交） |
| 原始项目 `upstream.json` | 同一 commit，`sourceVersion` / `runtimePackageVersion` = `0.1.1-rc.2` |
| npm `@deepseek-ai/dsh` | `0.1.1-rc.2` |
| 相对 pin 的新提交 | **0**（`git rev-list --count <pin>..origin/master`） |
| 原始桌面 `.anywhere-labs-dsh-desktop` | 已快进到 `681ba66091`（`origin/master`，tag `v2.0.3`；自 `ec0b0e5ebb` 起约 30 个提交，内核 pin 未变） |

**结论：内核没有新提交。** 本次同步只涉及原始桌面项目的产品更新并已改写进本仓库：

- **渲染器准入（上游把 WebServer 门做成强制）**：插件 `index.ts` 现在要求 launcher 提供 `ctx.desktopBrowserAccess`，且桌面 URL 永远带 `dsh-desktop-*` marker——无密钥头的请求默认 403。本仓库三侧跟进：`host/upstream/tauri-host.ts`（`migrateDesktopBrowserAccessSettings` + `hostCtx.provide('desktopBrowserAccess', …)`）；`DesktopShellPayload` 透传 `rendererAccessHeader`（`host/src/runtime/types.ts`、`ipc-desktop-runtime.ts`）；Tauri 壳新增 `src-tauri/src/renderer_proxy.rs`——环回反向代理让 WebView 的请求自动携带密钥头。代理授权模型（实机调试得出）：**首导航以 header 值作 URL 路径密钥；同源子资源解析到根路径不带密钥，改凭 `Referer` 携带密钥授权**（`resolve_authorized_path`），并把代理 origin 的 `Origin` 头重写为官方 origin（启动报告/私有设置路由的同源校验）。HTML `<head>` 注入 origin 重写脚本覆盖硬编码绝对地址；HTML 响应改写后必须去掉 `transfer-encoding` 只留 `content-length`。`prepareDesktopProfile` 的错误多余实参已顺手修正。**遗留**：开发模式包经 Finder/`open` 启动时 Host 进程卡在早期文件 open（栈证据在 `sample`），二进制直启（含 `env -i`、`nohup`）全部正常，待专项排查。
- **桌面对话框改官方 UI + 内容自适应**：上游四个尺寸提交（按内容区定尺寸、autosize、去最小高度、紧凑化）+ `dsh-desktop-dialog://layout?height=N` 上报契约。`open_dialog_window` 切到官方 `native-ui/desktop-dialog.html`（state/platform/frame 查询参数），`dialog.rs` 新增 `parse_dialog_layout`（≤440、仅 height 键），窗口按上报高度钳制 [200, 440]；diagnostic 呈现 680×460。`native-ui/` 资产已从插件 `lib/native-ui` 重建（新 hash），自有 `dialog.html` 移除。
- **随 checkout 免费获得**（插件/市场内部，不改本仓库）：浏览器访问策略与设置迁移、profile 零配置默认物化与内部暂存命名防护、Setup Wizard 欢迎页与免锁写入、Windows ConPTY 中继 / PowerShell 5.1 minimal / subprocess 行替换、Market 包管理器失败输出与图标区分、beta 网络功能标记、隐私政策文档、`dsh-client-ui-settings-general` 补丁 pin。
- **仍按缺口管理**：首次设置向导（P0，见 `docs/electron-parity-gaps.md`；上游本次又加大了它的范围）。

运行时仍解析已发布的 `@deepseek-ai/*@0.1.1-rc.2`（含原始项目 `patches/` 与 `.yarn/patches/`），不把内核源码链进 Yarn/pnpm workspace。

## 原则

1. **内核原封不动。** 禁止编辑 `.deepseek-harness/**`。需要改行为时，只改本仓库的 Tauri 壳、`host/`，或原始项目里 Desktop 自己的 UI（再按下面「桌面 UI」改写过来）。
2. **pin 对齐原始项目。** 内核版本以 `.anywhere-labs-dsh-desktop/upstream.json` 的 `commit` + `runtimePackageVersion` 为准，再写入 `docs/kernel-pin.json`。不要单独追 `deepseek-harness` 的 master，除非原始桌面项目已经 pin 了新版本。
3. **源码树与 npm 包是同一版本的两面。** 检出用于对照、调试、确认官方 Web/Host 契约；Host 进程加载的是已发布包（及原始项目里对该版本的 patch），不是把 `.deepseek-harness` 当 workspace 依赖。
4. **桌面 UI 要改写。** `.anywhere-labs-dsh-desktop` 的 Electron 窗口/托盘/native-ui 不能原样搬进 Tauri；只把产品形态与 UI 语义改写到 `src-tauri/`、`native-ui/`、`host/`。

## 何时同步

用户说「和远程项目同步」或明确要求同步内核 / 原始桌面时执行。日常开发不要自动 bump pin。

## 步骤

在仓库根目录执行。

### 1. 拉原始桌面与内核

```bash
git -C .anywhere-labs-dsh-desktop fetch origin
git -C .anywhere-labs-dsh-desktop checkout --detach origin/master   # 或他们指定的分支
git -C .deepseek-harness fetch --tags origin
```

若本地还没有这两个目录：

```bash
git clone https://github.com/anywhere-labs/dsh-desktop.git .anywhere-labs-dsh-desktop
git clone https://github.com/deepseek-ai/deepseek-harness.git .deepseek-harness
```

### 2. 读 pin，检出内核（不改内核文件）

```bash
python3 - <<'PY'
import json
p=json.load(open(".anywhere-labs-dsh-desktop/upstream.json"))
print(p["commit"], p["sourceVersion"], p["runtimePackageVersion"])
PY

PIN=$(python3 -c 'import json; print(json.load(open(".anywhere-labs-dsh-desktop/upstream.json"))["commit"])')
git -C .deepseek-harness checkout --detach "$PIN"
```

核对：

```bash
test "$(git -C .deepseek-harness rev-parse HEAD)" = "$PIN"
git -C .deepseek-harness status --porcelain   # 必须为空
```

把同一组字段写进 `docs/kernel-pin.json`（`commit`、`tag`、`sourceVersion`、`runtimePackageVersion`、`checkedAt`）。

### 3. 有没有「新的内核更新」

在 `.deepseek-harness`：

```bash
git fetch --tags origin
git rev-list --count "$(jq -r .commit docs/kernel-pin.json)"..origin/master
git log --oneline "$(jq -r .commit docs/kernel-pin.json)"..origin/master
```

- 计数为 0：没有新提交，停。
- 计数 > 0 **且** 原始桌面的 `upstream.json` 仍是旧 pin：只记录「上游 master 已超前，桌面产品尚未 pin」，**不要**自行把内核推到 master。
- 原始桌面已经改了 `upstream.json` / submodule SHA：按第 2 步检出新 pin，再按第 4 步处理桌面侧。

### 4. 同步原始桌面到本仓库（内核除外）

内核树不复制进本仓库。原始项目里 Desktop 自己的变化：

| 来源 | 本仓库怎么接 |
| --- | --- |
| Electron 窗口 / 托盘 / 材质 / 多窗口 | 改写到 `src-tauri/`（Tauri 语义，不是翻译 Electron API） |
| native-ui（Recovery / Dialog / Profile Create） | 产品形态对齐；实现落在 `native-ui/`，不要依赖 Electron `BrowserWindow` |
| Host 启动、profile、Market、官方 Web URL | `host/` + `.anywhere-labs-dsh-desktop/dsh-plugin-desktop`（`DSH_PLUGIN_DESKTOP_ROOT`） |
| `deepseek-harness/` 子模块内容 | 只更新 `.deepseek-harness` 检出与 `docs/kernel-pin.json` |

不要把 Electron `main.ts` / `electron-runtime.ts` 拷进 Tauri。对照契约是 `DesktopRuntime` 与官方 renderer URL，不是 Chromium API。

### 5. 验证

- `git -C .deepseek-harness rev-parse HEAD` 等于 `docs/kernel-pin.json` 的 `commit`。
- `.deepseek-harness` 无本地改动。
- Host 依赖的 `@deepseek-ai/*` 版本与 `runtimePackageVersion` 一致（原始项目 `package.json` / resolutions）。
- 需要跑产品时：官方 Host 能 `schedule` 出带 `dsh-desktop-mode=` 的 `http://127.0.0.1:` URL，而不是占位句 “Host loopback is running”。

## 本仓库如何用内核

- 运行时：`.anywhere-labs-dsh-desktop/dsh-plugin-desktop` 通过 npm 包 `@deepseek-ai/dsh-app-boot` 等加载 **已 pin 的** `0.1.1-rc.2` 家族（含原始项目 `patches/`）。
- 源码对照：`.deepseek-harness` 同一 commit，供阅读官方 Host/Client 契约。
- 本仓库 git：**不**跟踪这两个目录（见根 `.gitignore`）。
