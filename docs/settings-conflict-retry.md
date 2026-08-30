# 设置写入冲突自动重试 与 渲染器代理重写修复

目录：[`README.md`](README.md)。本文只写这两项适配的事实与边界，不重复内核机制。

## 1. 设置写入冲突自动重试（`settings_conflict_retry.rs`）

内核 Models 页的编辑卡在挂载时冻结命名空间 revision，保存时作为 `expectedRevision` 发回。命名空间 revision 是内核**内存态**计数（每次启动从 0 起，按提交写入 +1），首次提交实际成功而卡片未记录成功状态时，重试必撞 `settings-conflict`（即用户看到的 "expected revision 0, now 1" 原文报错）。

壳层在主窗口每次页面加载时注入 fetch 包装（与 [`composer-surfaces-and-quota.md`](composer-surfaces-and-quota.md) 同机制）：

- 仅对 **目标性写入** `settings.mutate` / `settings.update` 重试 **一次**，用 wire 错误 `details.actual` 作为新 `expectedRevision`，保留原 rpcId（客户端校验回声）。
- **`settings.replace` 永不重试**：全量替换重放到已变化的命名空间会删掉并发写入者的改动（含脱敏视图未携带的密钥）。
- 二次冲突原样透出；重试传输失败回退返回原冲突响应。

优先级说明：这是壳层注入适配，内核保持原封；上游若实现同类重试，可移除本模块。

## 2. 渲染器代理重写修复（`renderer_proxy.rs`）

`origin_rewrite_script` 把页面同源请求改写到代理密钥路径。旧实现只重写指向 carrier 的**绝对** URL，而内核客户端实际用**相对** `/api/...` 路径（fetch/XHR/WebSocket），按当前 origin 解析后不带密钥——WebSocket 握手无 `Referer` 可兜底，事件流（`/api/events.mux`、`/api/events.host`）因此 404 循环，工作区/会话数据面挂起。

修复后的匹配规则：

- **按 authority（`u.host`）比较**，不用 `origin`：WebKit 对 `ws:` URL 的 origin 计算为 `ws://host:port`（Chromium/Node 为 `http://host:port`），origin 相等判定在 WKWebView 里对 WS 永不命中。
- 相对路径与代理源无密钥路径 → 前置密钥；已带密钥与外部 origin 原样放行。
- WebSocket 包装器把 `http(s)` 归一为 `ws(s)`（对 `new WebSocket` 传 http URL 会 SyntaxError）。

修复同时解决了既有问题「会话级验证被工作区列表挂起阻塞」。

## 测试与验证

- `host/tests/settings-conflict-retry.spec.ts`：从 Rust 源提取脚本做行为级测试（重试/单次/replace 不重试/无 expectedRevision 不重试/非冲突透传）。
- `host/tests/renderer-proxy-rewrite.spec.ts`：同样行为级（相对 fetch、相对与 ws: 绝对 WebSocket、已带密钥、外部 origin 原样）。
- 实机（打包开发模式）：事件流 101/隧道建立、零拒绝；打开 ali-token-plan 编辑卡后从外部顶高 revision 再保存，卡片段落显示「已保存」且无冲突报错，`settings.yaml` 数据无损。
