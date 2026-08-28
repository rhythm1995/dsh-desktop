# 输入器组件适配 + 额度插件

目录：[`README.md`](README.md)。主视图主题见 [`main-view-theme.md`](main-view-theme.md)。

产品形态要求：输入器的**权限选择器**与**上下文容量面板**对齐参考版本布局；额度（剩余额度）不做在壳里，独立成插件 [`dsh-quota`](../../../dsh-quota)（同级仓，与 `dsh-usage-stats` 同构）。内核 pin 只检出，两组件的改动仍走 Tauri 注入脚本。

## 权限选择器（composer_surfaces.rs 第一部分）

内核 `ui-conversation` 的 `PermissionSelect`：触发按钮 `aria-label="访问模式，当前：{name}"`，弹层是共享 `Menu`（`role="menu"`，选项文案硬编码英文 `Full access` / `Workspace Write`）。注入脚本做的事：

- 菜单行改两行布局（CSS grid）：图标 | 中文标题+描述 | 选中勾。文案映射：`Full access → 完全访问（减少确认次数。）`、`Workspace Write → 自动编辑（工作区内自动编辑文件。）`、`Read Only → 只读模式`；未知预设静默退回原样。
- 触发按钮当前项为完全访问时染橙色（`data-dsh-cs-accent`）。
- 锚点全部语义化（aria 前缀 + label 文本），React 重渲染由 rAF 合帧的 sweep 重放，所有写入先比对后写。

## 上下文容量面板（composer_surfaces.rs 第二部分）

内核 `ContextMeter` 的 `div[role="dialog"][aria-label="上下文已用"]` 重排为参考布局：

- 头部改写为「上下文容量 | 31.8万/100万（31.8%）」——K/M 单位换算为万（`formatWan`），原 header 行 CSS 隐藏、新节点为外来节点（React 不管理，unmount 随父节点一起销毁）。
- 明细行由 `~tokens` 改为占比百分比（`dd` 文本解析求和）。
- 末尾追加「剩余额度」块：XHR 同源拉 `/api/dsh-quota/state`（5 秒缓存、2.5 秒超时、失败静默收起为未配置提示）。插件不在或未配置时该块不出现。

## 额度插件 dsh-quota（同级独立仓）

骨架复制 `dsh-usage-stats`（构建/安装/加载链路完全一致，详见其仓库 README）：

- **host**：`GET /api/dsh-quota/state` + `POST /api/dsh-quota/config`（同源校验、写校验失败 400）。配置持久化在 `$DSH_HOME/quota/config.json`（原子写）。数据源是 `~/.dsh/sessions/**/session*.jsonl(.zstd)` 的用量记录（records/session-store/zstd-frames 三模块与 usage-stats 同源）。
- **配置模型**：`pricing = {输入价格, 输出价格, 缓存读取, 缓存写入}`，单位 **u（美元）/ 1M token**；`quotas[] = {名称, 窗口(5小时滚动|每周), 上限, 单位(u 金额|次 调用)}`。花费 = 各桶 token × 单价 / 1M。
- **client**：设置新增「额度」分区（order 111）：剩余额度条（三色进度条 + 重置时间）+ 平均缓存命中率 + 定价/窗口表单，「后台配置后展示」。
- **重置时间**：滚动窗 = 窗口内最近一次用量 + 5h（`HH:MM`）；每周 = 下周一 00:00（`M月D日`）。

## 验证清单

1. `cargo test`（`composer_surfaces::tests` 八项：锚点语义化、中文文案、上下文重排、额度块降级、比对后写、rAF 合帧、ES5、幂等守卫）。dsh-quota 仓：vitest 26 项 + typecheck。
2. 真机已验证：设置 → 额度分区出现，写入 `~/.dsh/quota/config.json` 后展示平均缓存命中率与三条剩余额度条（含真实重置时间 23:13 / 8月31日），配色/布局与参考版本一致；主视图暗色无回归。
3. 待实机复核：权限菜单两行布局与上下文面板重排需要打开一个真实会话才能看到（注入锚点已由测试覆盖）；今日新 profile 上控制台的工作区/会话列表一直「正在加载/暂无」（先于本次改动复现，与额度/主题无关），会话列表恢复后一眼可验。
