# 项目说明
.anywhere-labs-dsh-desktop 是原始项目

# 原则
1. 关键功能一律用TDD的方式进行开发
2. 实施一律只有优先级，不要带排期

# 文档索引
- 迁移方案：`docs/tauri-v2-sidecar-migration.md`
- 开发者日志 / 网络模块与 AI 监听：`docs/devtools-listen.md`（仅开发模式；生产包无入口。本机默认打包为开发模式，GitHub Actions 或 `npm run tauri:prod-bundle` 为生产模式）
