# 主视图桌面主题（ZCode 风格暗色）

目录：[`README.md`](README.md)。产品缺口见 [`electron-parity-gaps.md`](electron-parity-gaps.md)。

产品形态要求：主视图（内核控制台 + 桌面框架）呈现参考版本的暗色风格——近黑主底 `#161616`、中灰侧栏 `#3a3b3b`、卡片/输入面 `#2b2b2b`、亮色主按钮、白色文字层级。内核按 pin 只检出、不改文件，因此和 [`settings-fullscreen.md`](settings-fullscreen.md) 同一套路：**Tauri 层注入脚本适配**，不新建 OS 窗口、不动内核源码、不改适配器检出。

## 实现

- `src-tauri/src/main_view_theme.rs`：自包含 ES5 IIFE `MAIN_VIEW_THEME_SCRIPT`，在主窗口每次页面加载完成后由 `lib.rs` 的 `on_page_load` 注入（与设置满窗口脚本同一机制）。
- 脚本行为：
  - 注入 `#dsh-desktop-theme-style` 样式表：`html { color-scheme: dark !important }`（压过 ThemePresenter 写的内联 `color-scheme`），以及 `body[data-ds-dark-theme]` 作用域的 `--dsw-alias-*` / `--dsw-specific-*` 令牌覆盖（全部 `!important`，压过任何 presenter 内联主题令牌）。
  - 桌面框架标题栏 `[data-dsh-desktop-frame="titlebar"]` / `.dshDesktopFrameTitlebar`（两者都是 `window_ops.rs` 已在用的稳定锚点）覆盖为不透明 `#161616`，压过适配器默认的半透明材质混色。
  - 强制暗色基底：给 `<body>` 补 `data-ds-dark-theme` 属性（内核 design-platform/scrollbar/shiki 等样式表按该属性整体切到暗色静态令牌）。ThemePresenter 按快照移除属性时，MutationObserver 在 rAF 内补回。
  - 幂等守卫 `window.__DSH_DESKTOP_THEME__`；所有写操作先比对后写（属性已暗、colorScheme 已暗则不写），观察器自身触发的回调不会产生写操作，无反馈循环。
  - 任何一步失配（无 body 等）→ 静默退化，不影响页面功能。
- `lib.rs` `create_main_window`：主窗口 builder 链上加 `theme(Dark)` + `background_color(#161616)`。标题栏条带的底色来自原生窗口材质（CSS 够不到），暗色外观 + 窗口底色让条带与 `#161616` 内容无缝衔接。

## 为什么走令牌覆盖而不是改类名样式

内核 `ui-theme` 的 `--dsw-*` 令牌是官方设计的换肤扩展点（`ThemeTokenInspection`），适配器桌面框架（`dsh-plugin-desktop` client styles）与内核全部组件样式都消费令牌。覆盖令牌一层即整体翻转，且不依赖 CSS-module hash 类名，跟随内核 `dsh-v*` 标签同步不会错位。结构性的 DOM 改造（侧栏项目分组、命令面板等）不在此层，见下方边界。

## 令牌映射（采样自参考版本截图）

| 角色 | 值 |
| --- | --- |
| 主底（内容区/顶栏） | `--dsw-alias-bg-base: #161616` |
| 侧栏底 | `--dsw-specific-sidebar-fill: #3a3b3b` |
| 侧栏悬停 / 选中项 | `#444545` / `#4e4f4e` |
| 卡片 / 弹层 / 菜单 / 输入面 | `layer-1 #1e1e1e`、`layer-2 #222222`、`layer-3 #2b2b2b`、`overlay #2e2e2e` |
| 输入主面 / 气泡 | `--dsw-specific-input-major: #2b2b2b`、`--dsw-specific-bubble: #222222` |
| 代码块 / 横幅 / 行内码 | `#2b2b2b` / `#2b2b2b` / `#2e2e2e` |
| 文本层级 | primary `#e6e6e6`、secondary `#9b9b9b`、tertiary `#8a8a8a`、caption `#707070`、dimmed `#5e5e5e` |
| 边框 | l1–l4 `rgba(255,255,255,.07/.10/.14/.18)` |
| 主按钮（白底深字） | `--dsw-alias-brand-primary: #ffffff`、`label-primary-foreground: #161616` |
| 成功态 | `--dsw-alias-state-success-primary: #46bf72` |

## 与参考版本的结构性边界（不在本层做，只记差距）

CSS 令牌层做不出的产品形态差距（需要 DOM 改造或内核能力，随内核同步易错位），按优先级记：

- P1：侧栏任务按项目分组 + 置顶分区 + 分组/项目切换（内核工作区列表是平铺 + 视图选项）
- P1：⌘K 命令面板（搜索操作、任务或文件，含最近任务/建议/面板分区）
- P2：顶栏任务标题 + 项目目录 + 分支选择器（内核顶栏是桌面框架版本/模式徽标）
- P2：输入器权限预设（完全访问）+ 模型 + 推理力度（最高）三段选择器的排布样式
- P3：会话多选删除模式（侧栏条目右键出现删除/归档操作列）

## 验证清单

1. `cargo test`（含 `main_view_theme::tests` 十项：幂等守卫、暗色属性强制、colorScheme 强制、令牌作用域、参考配色锚点、无 hash 类名、无写循环、ES5、单一样式表、标题栏语义锚点）。2026-08-28：113 项 cargo 全绿，`pnpm typecheck` 干净。
2. 真机已验证（重建重装后）：欢迎页/侧栏/输入器/顶栏/设置满窗口/插件市场满窗口均为暗色，实测渲染值与目标一致（侧栏 `#3a3b3b`、主底 `#161616`、输入面 `#2b2b2b`、主文本 `#e6e6e6`）；设置里选「浅色」被主题层拉回暗色属预期（桌面产品形态固定暗色），偏好已还原「跟随系统」；设置满窗口「← 返回工作区」与插件市场安装清单不受影响。
3. 未逐项实机验证：会话进行中的对话流（markdown 正文、代码块、轨迹卡片）——其配色走同一令牌机制（`markdown-*` 令牌已覆盖），但演示环境原生目录选择器无法自动化，未跑真实会话；首次真机使用时留意一眼即可。

## 已知边界

- 重装/重签应用会重置 TCC「桌面文件夹」授权，未授权时应用卡在 Starting Host（宿主等一个永远弹不出来的授权框）。重建重装后首次启动要去前台点一次 Allow——这也是「重建重装必须实机观察」的原因之一。
