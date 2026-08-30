# 文档目录

本仓库只维护 **Tauri 壳 + JS Host**（应用内 Bun/Node，pnpm 不变）。文档按用途分三类，**同一事实只写一处**：

| 用途 | 文件 | 写什么 |
| --- | --- | --- |
| 架构说明 | [`tauri-v2-sidecar-migration.md`](tauri-v2-sidecar-migration.md) | 为什么换壳、进程怎么拆、归属权。已落地，不再当待办清单。 |
| 产品缺口 | [`electron-parity-gaps.md`](electron-parity-gaps.md) | 相对 Electron 原版，壳层还缺什么、已经齐了什么。 |
| 上游同步 | [`kernel-sync.md`](kernel-sync.md) | 怎么拉官方 [deepseek-harness](https://github.com/deepseek-ai/deepseek-harness)、跟随最新 `dsh-v*` 标签。pin 只写在 [`kernel-pin.json`](kernel-pin.json)。anywhere-labs 不是 pin 来源。 |
| 弹窗满窗口化 | [`settings-fullscreen.md`](settings-fullscreen.md) | 设置与插件市场弹窗改满窗口 + 返回工作区的注入脚本适配、锚点降级策略、市场已安装 502 根因记录。 |
| 主视图桌面主题 | [`main-view-theme.md`](main-view-theme.md) | 主视图 ZCode 风格暗色的注入式令牌覆盖、强制暗色基底机制、结构性差距清单。 |
| 输入器组件 + 额度插件 | [`composer-surfaces-and-quota.md`](composer-surfaces-and-quota.md) | 权限选择器与上下文容量面板的注入式适配、dsh-quota 同级插件（u 定价、窗口额度、配置后展示）。 |
| 设置冲突重试 + 代理重写 | [`settings-conflict-retry.md`](settings-conflict-retry.md) | 设置写入 settings-conflict 的单次自动重试（replace 除外）、渲染器代理重写按 authority 匹配与相对路径补密钥（修 WS 事件流 404）。 |
| 开发监听 | [`devtools-listen.md`](devtools-listen.md) | 何时是开发模式、日志/网络 NDJSON、AI 怎么听。 |
| CI/CD | [`github-actions-cicd.md`](github-actions-cicd.md) | `ci.yml` / `release.yml` 做什么、怎么发版。开发/生产开关以 devtools 文档为准。 |
| 企业调研 | [`deepseek-harness-enterprise.md`](deepseek-harness-enterprise.md)、[`enterprise-admin-console.md`](enterprise-admin-console.md) | 内核生态与管理后台对标。**不是本仓库实现规格。** |

Agent 入口是仓库根 `AGENTS.md`（原则 + 指向本目录）。对外 README 只链到本页，不再列一份文档表。不要把原则、pin、开发/生产判定、缺口状态再抄到第三份文件。

## 不要写进本仓库文档的

- 排期、里程碑日期（只保留优先级）
- Electron API 翻译、asar / Crashpad / `ELECTRON_RUN_AS_NODE` 当运行时
- 改 `.deepseek-harness` 源码
