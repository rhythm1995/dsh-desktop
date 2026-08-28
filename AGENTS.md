# 项目说明
- `.deepseek-harness` 是产品内核（官方 [DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness)，原封不动）
- `.anywhere-labs-dsh-desktop` 是可选的 Electron 形态对照 / `dsh-plugin-desktop` 适配器检出，**不是**内核上游，也不是 pin 权威
- 除非我要求，否则你不能自行更新 AGENTS.md，更新该文件必须和我请求同意

# 原则
- 关键功能一律用TDD的方式进行开发
- 文档一律只有优先级，不要带排期
- 当我说和远程项目同步：按 `docs/kernel-sync.md` **先拉 `.deepseek-harness`**，跟随最新 `dsh-v*` 标签写入 `docs/kernel-pin.json`。不要从 `.anywhere-labs-dsh-desktop/upstream.json` 抄 pin，不要等官方 Electron 先 pin。`.deepseek-harness` 只检出运行 pin、不改已跟踪文件。anywhere-labs 检出可选：只把桌面 UI / 产品形态改写到 Tauri（`src-tauri/`、`native-ui/`、`host/`），不要翻译 Electron API

# 文档索引
完整目录：`docs/README.md`（同一事实只写一处）。Agent 日常：
- 同步上游：`docs/kernel-sync.md`（pin：`docs/kernel-pin.json`）
- 产品缺口：`docs/electron-parity-gaps.md`
- 开发模式监听：`docs/devtools-listen.md`（本机默认打包为开发模式；GitHub Actions 或 `npm run tauri:prod-bundle` 为生产模式）
