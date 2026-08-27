# 开发者日志 / 网络模块与 AI 监听方案

目录：[`README.md`](README.md)。CI/CD 见 [`github-actions-cicd.md`](github-actions-cicd.md)。开发/生产判定以本文表格为准，不要在别处再抄一份。

这两个模块只在 **开发模式** 出现。生产包没有托盘入口、没有 `/_dsh/dev/*` 路由，也不会写 `devtools/` 目录。

## 何时是开发模式

判定写在 `host/src/runtime/build-profile.ts` 与 `src-tauri/build.rs`，规则相同：

| 场景 | 开发工具 |
| --- | --- |
| 本地 `npx tauri build` / `npm run tauri build`（默认） | 开 |
| GitHub Actions（`GITHUB_ACTIONS=true`） | 关 |
| 本地显式生产包 `npm run tauri:prod-bundle`，或 `DSH_DEV_TOOLS=0` / `DSH_PROFILE=production` | 关 |
| 运行时再设 `DSH_DEV_TOOLS=0` 或 `1` | 覆盖编译结果 |

Host 进程由 Tauri 拉起时会带上与壳一致的 `DSH_DEV_TOOLS`。

## 开发者入口

1. 系统托盘 **Developer Logs** / **Developer Network**
2. 浏览器或 Webview：`http://127.0.0.1:<port>/_dsh/dev/ui?tab=logs|network`
3. JSON 表：
   - `GET /_dsh/dev/logs?q=&level=info&limit=200`
   - `GET /_dsh/dev/network?q=&method=GET&limit=200`
   - `GET /_dsh/dev/meta`

`<port>` 每次启动会变，以 listen 清单为准。

## AI coding 工具怎么听

不要去刮 UI。读磁盘上的 NDJSON 和 `listen.json`。

macOS 默认目录：

```text
~/Library/Application Support/DSH Desktop/devtools/
  listen.json
  logs.ndjson
  network.ndjson
```

Windows：`%APPDATA%\DSH Desktop\devtools\`

Linux：`~/.config/DSH Desktop/devtools/`

`listen.json` 在 Host 绑好 loopback 后写入，包含当前 `enabled`、两个文件路径和 HTTP 入口。

### 推荐监听

```bash
# 当前端口与路径
cat "$HOME/Library/Application Support/DSH Desktop/devtools/listen.json"

# 持续读日志（每行一条 JSON）
tail -f "$HOME/Library/Application Support/DSH Desktop/devtools/logs.ndjson"

# 网络表
tail -f "$HOME/Library/Application Support/DSH Desktop/devtools/network.ndjson"
```

过滤示例：

```bash
python3 -c "
import json,sys
for line in sys.stdin:
    row=json.loads(line)
    if row.get('level') in ('warn','error') or 'error' in json.dumps(row).lower():
        print(line, end='')
"
```

HTTP 拉全表（适合一次性上下文，不适合当流）：

```bash
ORIGIN=$(python3 -c "import json; print(json.load(open('$HOME/Library/Application Support/DSH Desktop/devtools/listen.json'))['http']['origin'])")
curl -s "$ORIGIN/_dsh/dev/logs?level=warn"
curl -s "$ORIGIN/_dsh/dev/network?q=dshdesktop"
```

### 给 Agent 的约定

1. 先读 `listen.json`。`enabled: false` 表示这是生产包，不要假设有 inspector。
2. 只把 `devtools/*.ndjson` 当遥测；不要把诊断 zip 或用户 session 当默认输入。
3. 日志行字段：`ts`、`level`、`source`、`message`、可选 `data`。
4. 网络行字段：`ts`、`id`、`method`、`url`、可选 `status`、`durationMs`、`error`。
5. `/_dsh/dev/*` 自己的轮询不会写入网络表，避免把 inspector 刷屏记进去。

## 本地打包 vs CI

- 测开发入口：本机直接 `npx tauri build --bundles app,dmg`（开发模式）。
- 测生产包：`npm run tauri:prod-bundle`。
- GitHub `release` workflow 已设 `DSH_DEV_TOOLS=0` 与 `DSH_PROFILE=production`。
