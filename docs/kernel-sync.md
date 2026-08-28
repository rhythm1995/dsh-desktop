# 内核（deepseek-harness）同步方案

目录：[`README.md`](README.md)。pin 只写在 [`kernel-pin.json`](kernel-pin.json)。架构说明见 [`tauri-v2-sidecar-migration.md`](tauri-v2-sidecar-migration.md)。选 tag 与 pin 不变式的实现：`host/src/runtime/kernel-pin.ts`。

本仓库的产品内核只跟随官方 [DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness)。本地目录是 **`.deepseek-harness/`**（git 忽略，不进本仓库）。内核必须**原封不动**：不改其中任何已跟踪文件，不把 Tauri / Electron 补丁打进内核树。

**唯一内核上游**是 harness 的最新 **`dsh-v*` 标签**（semver，含预发布）。不跟随 `origin/master` HEAD，不跟随 npm `latest` / `next`，不跟随 anywhere-labs `upstream.json`。`.anywhere-labs-dsh-desktop/` 只是可选的 DesktopRuntime 适配器检出与 Electron 形态对照，**不是 pin 权威，也不是「同步上游」的门闩**。

## 当前核对（2026-08-28）

| 项 | 值 |
| --- | --- |
| 远程 | `https://github.com/deepseek-ai/deepseek-harness.git` |
| 跟踪 | `latest-dsh-v-tag` |
| 运行 pin（Host 可加载） | `b150a551b8d465e31e418e1b2eaf5e79bbb7d28e`，标签 `dsh-v0.1.1-rc.2`，`consumption=npm` |
| 最新 `dsh-v*` 标签 | `dsh-v0.1.2-alpha.1` @ `cd5ef8148158c3a752a658978873241fdf8e2bbc` |
| `origin/master` 相对运行 pin | **1079** 提交（观测，不驱动 bump） |
| npm `@deepseek-ai/dsh@0.1.2-alpha.1` | **未发布**（`latest` / `next` 仍是 `0.1.1-rc.2`） |
| 适配器 | `dsh-plugin-desktop` 仍依赖 alpha.1 已删除的 `@deepseek-ai/dsh-host-apiproxy`、`@deepseek-ai/dsh-client-runtime` |

**结论：政策跟随最新 `dsh-v*` 标签；运行 pin 仍是已发布的 `dsh-v0.1.1-rc.2`。** 缺口写在 pin 的 `latestTag` / `latestTagCommit` / `commitsAhead`。在消费路径落地、且适配器不再引用已删包之前，**不要**把 `commit` / `runtimePackageVersion` 改成 `0.1.2-alpha.1`。

`0.1.2-alpha.1` 发布说明：[tag](https://github.com/deepseek-ai/deepseek-harness/releases/tag/dsh-v0.1.2-alpha.1)。Web 初始化/会话体积、`/` 与 `@` 菜单、轮次折叠、字号、多模态回显、持久终端/Bash、Agent 预设、文件编辑 null 占位、PTC Mode `run_code`、系统提示词分区（Shell 指南前移）、去掉 ApiProxy（改 Remote 网关）。这些是 harness 标签内容，不是「等 Electron 桌面 pin」的理由。

桌面适配器侧（与内核 pin 无关、不挡跟随 harness）：

- Windows Acrylic 已在本仓库去掉：`windowsMaterial` 默认 `off`，持久化 `acrylic` 读成 `off`。
- 侧栏与聊天拖放遮罩冲突由官方 client / 适配器 yarn patch 处理；本仓库不复制 Web Client。
- 仍按缺口管理：首次设置向导（P0，见 `docs/electron-parity-gaps.md`）。

运行时仍解析已发布的 `@deepseek-ai/*@0.1.1-rc.2`（经适配器 Yarn `node_modules`）。不把内核源码链进本仓库 pnpm workspace。

## 原则

1. **内核原封不动。** 禁止编辑 `.deepseek-harness/**` 已跟踪文件。需要改行为时，只改本仓库的 Tauri 壳、`host/`，或适配器注入（`host/upstream/tauri-host.ts`）。
2. **pin 只跟 harness 最新 `dsh-v*` 标签。** 权威是 `docs/kernel-pin.json`。`commit` / `tag` / `sourceVersion` / `runtimePackageVersion` 必须是 Host **真能加载** 的版本。`latestTag` 可以超前。装不上就只更新观测字段，禁止半 bump。
3. **`sourceVersion === runtimePackageVersion`。** 已发布走 npm；未发布走 checkout pack（手续里写「尚未落地」则停在旧运行 pin）。
4. **anywhere-labs 不是上游。** 其 `upstream.json` 忽略。检出可选：给 `DSH_PLUGIN_DESKTOP_ROOT`、Electron 形态对照。窗口/托盘/native-ui 只改写到 `src-tauri/`、`native-ui/`、`host/`，不翻译 Electron API。

## 何时同步

用户说「和远程项目同步」或明确要求同步内核时执行。日常开发不要自动 bump pin。默认只拉 harness。

## 步骤

在仓库根目录执行。

### 1. 拉内核（必需）

```bash
git clone https://github.com/deepseek-ai/deepseek-harness.git .deepseek-harness   # 若无
git -C .deepseek-harness fetch --tags origin
```

### 2. 选最新 `dsh-v*` 标签，只写观测字段

用与 `host/src/runtime/kernel-pin.ts` 相同的规则：剥 `refs/tags/` 与 `^{}`，在 `dsh-v*` 上按 semver（含预发布）取最大。读该 tag 根 `package.json` 的 `version` 作为候选 `sourceVersion`。探测：

```bash
npm view @deepseek-ai/dsh@<version> version
npm view @deepseek-ai/dsh-app-boot@<version> version
```

404 ⇒ 该标签尚未进 npm。`commitsAhead` = `git rev-list --count <pin.commit>..origin/master`（观测）。

把 `latestTag`、`latestTagCommit`、`commitsAhead`、`checkedAt` 写入 `docs/kernel-pin.json`。这一步 **不** 改 `commit` / `tag` / `sourceVersion` / `runtimePackageVersion`。

### 3. 是否 bump 运行 pin

- 最新 tag == 当前 `tag`：停。
- 最新 tag 更新，但 Host 还装不上（npm 没有、pack 脚本未落地、适配器依赖已删包、构建失败）：**保持旧运行 pin**，把缺口留在观测字段与本页核对表。
- 否则检出新 tag，安装对应 `@deepseek-ai/*`，校验 `dsh-plugin-desktop/node_modules/@deepseek-ai/dsh-app-boot/package.json` 的 `version` 等于新 `runtimePackageVersion`，再改运行 pin 字段。`consumption`：registry 有包则为 `npm`，否则为 `packed-from-checkout`（pack 路径尚未落地时不要走这条）。

**不要**再写「原始桌面尚未 pin ⇒ 不要把内核推到新 tag」。

### 4. 检出运行 pin（不改内核已跟踪文件）

```bash
PIN=$(node -p "JSON.parse(require('fs').readFileSync('docs/kernel-pin.json','utf8')).commit")
git -C .deepseek-harness checkout --detach "$PIN"
test "$(git -C .deepseek-harness rev-parse HEAD)" = "$PIN"
git -C .deepseek-harness status --porcelain   # 允许 gitignore 产物；禁止已跟踪文件脏
```

### 5. 可选：适配器 / Electron 对照

本地跑官方 Host 或对照 Electron 形态时：

```bash
git clone https://github.com/anywhere-labs/dsh-desktop.git .anywhere-labs-dsh-desktop   # 若无
git -C .anywhere-labs-dsh-desktop fetch origin
git -C .anywhere-labs-dsh-desktop checkout --detach origin/master
```

其 `upstream.json` **忽略**。Host 启动、profile、Market 仍走 `host/` + `dsh-plugin-desktop`（`DSH_PLUGIN_DESKTOP_ROOT`）。Electron 窗口/托盘/材质改写到 `src-tauri/`；native-ui 落在 `native-ui/`。不要把 Electron `main.ts` / `electron-runtime.ts` 拷进 Tauri。

### 6. 验证

- `git -C .deepseek-harness rev-parse HEAD` 等于 pin 的 `commit`。
- `.deepseek-harness` 已跟踪文件无本地改动。
- Host 加载的 `@deepseek-ai/dsh-app-boot` 版本等于 `runtimePackageVersion`。
- `pnpm test` 覆盖 pin 不变式与选 tag（不访问网络、不克隆 harness）。
- 需要跑产品时：官方 Host 能 `schedule` 出带 `dsh-desktop-mode=` 的 `http://127.0.0.1:` URL。

## pin 字段

权威只在 [`kernel-pin.json`](kernel-pin.json)。`track` 固定 `latest-dsh-v-tag`。`commit`/`tag`/`sourceVersion`/`runtimePackageVersion` 是已检出且允许运行的内核；`latestTag`/`latestTagCommit`/`commitsAhead` 是上次同步看到的缺口。`consumption` 为 `npm` 或 `packed-from-checkout`，描述**当前运行 pin**怎么进 node_modules。`npmPublished` 指**当前运行 pin**的版本已进 npm（`consumption=npm` 时必须是 `true`）；它不是 `latestTag` 的发布状态（`0.1.2-alpha.1` 未发布不影响它）。`checkedAt` 是上次核查日期。

## 本仓库如何用内核

- 运行时：适配器 `dsh-plugin-desktop` 通过 npm `@deepseek-ai/dsh-app-boot` 等加载 **运行 pin** 家族（今天是 `0.1.1-rc.2`）。
- 源码对照：`.deepseek-harness` 检出同一 `commit`。
- 本仓库 git：**不**跟踪 `.deepseek-harness/` 与 `.anywhere-labs-dsh-desktop/`（见根 `.gitignore`）。
- 默认 CI **不**克隆这两个目录。
