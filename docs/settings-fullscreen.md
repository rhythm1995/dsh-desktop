# 内核弹窗满窗口化（设置 / 插件市场：弹窗 → 满窗口）

内核把设置渲染成主窗口页面里的居中弹窗（800px 圆角面板 + 遮罩，见 `.deepseek-harness/packages/client/ui-settings-general/src/client/SettingsRoot.tsx`），插件市场也是同类居中弹窗（800×700 的 `section`，见市场社区包 `dsh-community-market` 源码 `src/client/MarketOverlay.tsx`——该包不在 harness 内核里，仓库是 anywhere-labs/deepseek-harness-desktop，本机是适配器 node_modules 里的 npm 包）。产品形态要求参考官方新版样式：两者都铺满整窗、设置左上角有「← 返回工作区」。由于内核按 pin 只检出、不改文件，这个形态由 Tauri 层的**注入脚本适配**实现，不改内核源码，也不新建 OS 窗口。

## 实现

- `src-tauri/src/settings_fullscreen.rs`：自包含 ES5 IIFE `SETTINGS_FULLSCREEN_SCRIPT`，在主窗口每次页面加载完成后由 `lib.rs` 的 `on_page_load` 注入（与标题栏拖拽、缩放快捷键脚本同一机制）。
- 脚本行为：
  - MutationObserver 监听子树变化，找到内核设置面板后：给 `<body>` 打 `data-dsh-settings-fullscreen="open"` 开关，给遮罩层/面板盖 `data-dsh-fs="overlay|panel"` 标记，并在导航栏顶部插入「← 返回工作区」按钮。
  - 注入 `#dsh-settings-fullscreen-style` 样式表：面板 `100vw×100vh`、去圆角去阴影、遮罩隐藏、`pointer-events: auto` 防点击穿透；返回工作区按钮为中性配色（`color: inherit`），不绑内核 token 名。
  - 「← 返回工作区」点击即向 `document` 派发 Escape 键事件，复用内核官方关闭逻辑（等价 X 按钮 / Esc）。
  - 面板卸载后清除开关与标记，恢复原状；重复注入由 `window.__DSH_SETTINGS_FULLSCREEN__` 幂等守卫拦住。
- 定位文案语言按 `document.documentElement.lang` / `navigator.language` 是否 zh 前缀选择「返回工作区 / Back to workspace」。

## 锚点与降级策略

只依赖语义属性和结构签名，不碰 CSS-module hash 类名：

- 两类弹窗都从 `[role="dialog"][aria-modal="true"]` 候选里区分：
  - 设置面板唯一签名：`aria-labelledby` 指向的标题节点位于 `panel :scope > nav` 内部（引导向导等其他弹窗不满足该结构）；
  - 插件市场唯一签名：对话框元素自身的第一个子元素是 mask `<button>`（带 aria-label）且含 `:scope > section` 面板；
- 任何一步失配 → 找不到目标 → 什么都不做，静默退化回原弹窗。跟随新的 harness `dsh-v*` 标签若改了外壳，功能不坏，只是形态回退。

观察器只开 `childList`，不开 `attributes`，配合 rAF 合帧，避免脚本自身写属性造成的反馈循环。市场的关闭路径保留其自带 X 按钮与 Escape；市场满窗口后原点遮罩区域已不存在，遮罩按钮隐藏属预期。

## 已安装清单 502 的根因（2026-08-27 修复记录）

「插件市场 → 已安装」报「暂时无法读取当前配置的插件清单」，实为适配器检出里两个包**编译产物契约错位**：`dsh-plugin-desktop/lib` 的 `DesktopPluginsService.list()` 返回含 `uninstallable` 的 5 字段形状，而 `dsh-community-market/lib`（停留在旧构建）的 `reconcileInstallations` 用 `exactKeys` 只认 4 字段 → 全部条目校验失败 → `/api/community-market/installations` 返回 502 `operation-failed: The desktop plugin inventory was invalid.`。处理：在 `dsh-community-market` 目录重跑构建四步（clean / generate-contract-types / tsdown / tsc），使产物契约对齐；该包 vitest 268 通过。这是适配器 `lib/` 错位，**不阻塞**内核 pin。kernel-sync 后如遇运行时行为与源码不符，先比对相关包 `lib/` 的 mtime 与源码差异。

## 验证清单

1. `cargo test`（含 `settings_fullscreen::tests` 七项断言：幂等守卫、语义锚点、双语文案、全屏规则标记、Escape 关闭清理、观察器无反馈循环、市场签名识别）。
2. 真机（先清孤儿 host 再重建重装）：设置满窗口无圆角无暗化遮罩；左上角「← 返回工作区」可关闭设置；Esc 可关；通用 / 模型 / 插件 / 桌面设置 / 使用统计各分区正常渲染切换。插件市场满窗口、「已安装」标签能列出 Profile 直接插件（usage-stats 等）、可安装/发现/来源正常。
3. 聊天工作区不受影响。

## 已知边界

- 与原弹窗一致，弹窗打开期间页面自定义标题栏被覆盖，此时拖拽走系统热区无效，属既有语义非本次回归。
- 截图样式的侧栏分组标题（基础设置 / Agent 能力 / 数据与统计）与底部账号卡片在内核结构中不存在对应物，需向平铺导航插 DOM 且随内核同步易错位，本次未做；如需要作为独立小迭代评估。
