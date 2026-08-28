# DeepSeek Harness 企业级管理后台功能调研（对标 Codex / Cursor / Claude Code 企业版）

> 调研文档，**不是本仓库实现规格**。本仓库只做 Tauri 桌面壳。入口：[`README.md`](README.md)。生态与治理插件普查见 [`deepseek-harness-enterprise.md`](deepseek-harness-enterprise.md)。**pin 策略已改为跟随 harness 最新 `dsh-v*` 标签**（见 [`kernel-sync.md`](kernel-sync.md)），anywhere-labs 不是 pin 权威。

调研日期：2026-08-26。问题：如果给 DeepSeek Harness（本地检出 `.deepseek-harness/`，deepseek-ai 官方内核）做一个企业级管理后台，需要哪些功能？对标对象是 OpenAI（ChatGPT Enterprise / Codex）、Cursor（Teams / Enterprise）、Anthropic（Claude Enterprise / Claude Code）三家官方管理后台。

本文只写事实与可核验来源，每一条对标结论都注明出处；功能建议只分 P0 / P1 / P2，不排期。前置阅读：`docs/deepseek-harness-enterprise.md`（内核无企业能力的证据、AgentKit 十模块、治理插件普查）。

## 0. 结论（TLDR）

- **三家后台的能力高度收敛为九个功能域**：①身份与席位（SSO/SCIM/角色）②组织与分组 ③用量与成本（看板/限额/告警）④Agent 行为管控（集中下发策略：沙箱/审批/网络出口/工具白名单）⑤模型授权 ⑥数据与隐私治理 ⑦审计与合规 ⑧Agent 资产盘点与分析 ⑨开放集成（Admin API/网关/SIEM）。差异只在深度与形态，没有一家缺整块。
- **AI 编码代理后台与传统 SaaS 后台的分水岭在④**：三家都把「管理员如何约束 agent 的执行行为」做成了一等公民——Codex 的 managed configuration（requirements.toml）+ 网络代理域名白名单、Claude Code 的 managed settings 四级来源 + OS 级网络沙箱、Cursor 的 privacy mode 强制 + repo blocklist。**这是给 dsh 做后台时最不能抄传统后台模板的部分。**
- **第二个分水岭是非人类身份**：OpenAI 给 Codex 自动化做了 service account（User/Configure/Manager 三级授权），Cursor 的用量事件里 serviceAccountId / automationId / hostingType 是一等过滤维度。dsh 生态里 agent 跑批/定时任务已是常态，后台若只设计「人」的账号体系，第一天就会错。
- **市场空位确认**：三家全部是「云托管控制台」形态；dsh 是本地优先 + BYOK 的开源内核，没有任何一家覆盖「自托管团队网关 + 本地桌面端纳管」的组合（火山 AgentKit 只覆盖云端承载）。这与 `deepseek-harness-enterprise.md` §6 的 P0 结论一致：企业管控面在 dsh 生态里仍是空白。
- **给本仓库的建议**：功能基线见 §4（P0 六项）；产品形态建议「自托管团队网关 + Web 控制台 + 桌面端受管」，理由见 §5。

## 1. 对标对象与信息源

| 产品 | 后台形态 | 本文引用的核心页面 |
| --- | --- | --- |
| OpenAI ChatGPT Enterprise / Codex | Global Admin Console（Tenant → 多 Workspace） | help.openai.com Global Admin Console；learn.chatgpt.com Agent approvals & security；ChatGPT Enterprise/Edu Release Notes；developers.openai.com Codex 云安全 |
| Anthropic Claude Enterprise / Claude Code | claude.ai Admin Console + Claude Console + 托管策略文件下发 | code.claude.com admin-setup（部署决策图）；claude.com 博客《Giving admins more visibility and control over Claude spend》（2026-07-02）；anthropic.com/news/claude-code-on-team-and-enterprise |
| Cursor Teams / Enterprise | cursor.com/dashboard 团队仪表盘 + Admin/Organization/Analytics 三套 API | cursor.com/docs Members/Roles/Seats；Admin API；Privacy and Data；Enterprise |

检索与抓取日期均为 2026-08-26；OpenAI 帮助中心页面标注「Updated: 2 hours ago」，属活跃维护中的最新口径。

## 2. 九大功能域逐项对照

### 2.1 身份与席位（SSO / SCIM / 角色 / 席位）

| 能力点 | OpenAI | Anthropic | Cursor |
| --- | --- | --- | --- |
| SSO | Tenant 级单条 SSO 连接，按域映射应用到各 Workspace；Required / Optional / Off 三态，可按 workspace 单独覆写（workspace 设置优先于全局） | Enterprise Administrator Guide 提供 SSO 配置；managed settings 可经 admin console 服务端下发 | SAML 2.0，Teams 档即提供；SSO enforcement 选项；Okta 等 IdP 集成 |
| 域验证 | 域唯一性校验 + Domain Eligibility/Mapping（映射决定 SSO、自动建号、外部邀请、账号合并四个功能的生效范围），新建域默认 NotMapped 防锁死 | —（未在本次抓取页展开） | 域验证 + Domain matching（同域自动加入免邀请）+ Restrict invites to verified domains |
| SCIM | Manage Invites 区预告中（规划支持直连与 SCIM） | SCIM 组直接作为后台分析的过滤维度（按 IT 已管的 SCIM 组拆解用量） | Billing groups 与 SCIM 目录组同步（SCIM 同步的组不可经 API 改成员） |
| 角色模型 | Global Admin（独立于 workspace 成员）/ Workspace Owner / Admin / **Analytics Viewer（仅 Enterprise/Edu）** / Member（只能看自己的活动） | Owners / Admins / 普通成员；org 级 effort limit 可按角色设上限 | Member / Admin / **Unpaid Admin**（不占席位的 IT/财务管理员，管理权完整但不给产品访问） |
| 席位 | Plan 页显示 license 与 Seat usage | 席位管理（Administrator Guide）；「seat 不含 Claude Code 访问」时登录报错引导管理员改权限 | Standard $40 / Premium $120 双席位（Premium=5×用量），成员菜单即时升降级按比例计费 |

要点：角色设计的共性是「**把看板的人和管理策略的人分开**」（OpenAI 的 Analytics Viewer、Cursor 的 Unpaid Admin 都是为 IT/财务/安全这类不写代码的管理者设计的）。dsh 后台照抄 developer-only 的单一 admin 角色会直接丢掉采购方里的安全/财务干系人。

### 2.2 组织与分组

- OpenAI：Tenant（多 ChatGPT workspace + 多 API org + 多域 + 全局管理员），资源选择器切换上下文，URL 带 workspace 粒度便于收藏分享；Groups 用于用量限额的三级覆盖（§2.3）。
- Anthropic：按 SCIM 组过滤分析（跟随企业现有组织架构，不需要在 Claude 里重建一份组织树）。
- Cursor：Billing Groups（每成员同时只属一个，未分配进保留的 Unassigned 组；带当周期 spend、日粒度 spend 历史、前成员留存），明确用途是「reporting、内部成本分摊（chargeback）、预算」；目录组可由 IdP 同步。

### 2.3 用量与成本（看板 / 限额 / 告警）

三家共同骨架是「**多维看板 + 三级限额 + 阈值告警 + 导出/API**」，但指标设计差异很大：

| 能力点 | OpenAI（Global Admin Console） | Anthropic（Claude Enterprise） | Cursor |
| --- | --- | --- | --- |
| 看板维度 | Overview（活跃用户/credits/tokens/messages）+ Leaderboard（Users/Groups/Agents 三 tab）+ Credits（按产品区、按计量项 input/output/**cached input** tokens、按模型拆解）+ Codex 专属视图 | 按组、按人的用量与成本并排展示；产出物指标（创建的 artifacts、编辑的文件、用过的 skills/connectors）直接标在成本旁边；Analytics chat 自然语言问答出图并可导出 | Daily usage：行数增删、**接受/拒绝数**、Tab 补全展示/采纳、composer/chat/agent 请求数、最常用模型、客户端版本；Usage events 到单次请求级（token 明细 + chargedCents 可与账单对账） |
| Codex/Claude Code 专属指标 | active users、credits、tokens、message runs、**lines of code generated、plugin calls、skills used、code review activity** | Usage tab：活跃开发者、session 数、top commands（日更）；Value tab：估算生产力提升、**cost per commit、年化价值**，公式公开且参数可调 | AI Code Tracking API：commit 指标与 code changes 的 JSON/CSV 导出 |
| 限额 | Usage limits 独立页：Workspace 默认额度 → Group 覆盖 → User 覆盖（可 Unlimited）；**Pending requests 页审批成员的提额申请**；usage period 可选自然月或对齐账期 | Spend caps「每一层都有」（原文）；org 级 75%/90% 告警，用户端 75%/95% 应用内通知 + 应用内一键申请提额；Admin API 把限额工作流脚本化 | on-demand 用量总开关、月度 spending limit、per-user 硬限（API 仅 Enterprise），hardLimitOverride 与 monthlyLimit 分开存储、effectivePerUserLimit 单字段给出最终生效值 |
| 成员侧可见性 | 「Allow users to see usage in credits and dollars」独立开关（只影响可见性不动限额） | 管理员可选择把成本/产品/模型拆解和限额进度开放给个人，用户也能看自己的历史趋势 | Member 可看自己的用量与剩余额度 |
| 导出与对账 | CSV 导出；credits 旁显示按 overage 价折算的预估金额（标注 planning aid 非账单）；credit 数据窗口 120 天，ChatGPT 用量 12 个月 | Analytics API 按日期/团队/产品/模型过滤，官方点名 Datadog Cloud Cost Management、CloudZero 集成 | filtered-usage-events 的 chargedCents 字段官方声明用于与 /teams/spend 及发票对账 |

### 2.4 Agent 行为管控（集中策略下发）——编码代理后台的分水岭

| 能力点 | OpenAI Codex | Claude Code | Cursor |
| --- | --- | --- | --- |
| 策略载体与优先级 | Managed configuration（管理员侧 requirements.toml）+ ChatGPT workspace 设置；managed 要求优先于本地 config | Managed settings 四级来源按优先级取第一个生效：admin console 服务端下发（认证时拉取、会话中每小时刷新）→ MDM plist / HKLM 注册表（需管理员权限写入，抗篡改）→ 磁盘文件；数组类 permissions.allow/deny 各来源合并（开发者只能追加不能删），fallbackModel/availableModels 则整体替换 | Dashboard 下发：Privacy Mode 强制、on-demand 开关、spend limit、「enforce allowed team IDs and extensions on user devices」（设备侧锁定） |
| 沙箱与审批档位 | sandbox_mode（read-only/workspace-write/danger-full-access）× approval_policy（untrusted/on-request/never/granular 细分类别）矩阵；writable roots 内 .git/.agents/.codex 递归只读保护 | 权限预设（需审批的 workspace-write 与 danger-full-access 两档）+ OS 级沙箱网络域名白名单（明确说明 deny WebFetch 不够，Bash curl 仍可达，必须 OS 层收口）+ 工具/命令/hooks 锁定 | （客户端内为 IDE 权限模型，后台侧主要是 privacy/用量/上下文黑名单，行为管控弱于前两家） |
| 网络出口管控 | 默认断网；network_proxy 特性：allowlist-first 域名规则（精确/`*.`/`**.`/全局`*`，deny 恒胜 allow）、默认禁本地/内网地址（防 SSRF）、DNS rebinding 缓解（解析到非公网 IP 即拒）、unix socket 白名单；Codex 云两阶段运行时（setup 阶段可装依赖、agent 阶段默认离线，secrets 只在 setup 阶段存在随后移除）；公网关闭时经「managed destination allowlist」放行必需目标 | 沙箱网络域名白名单（OS 层强制）；Claude Code on the web 的 Cloud environments 页由 Owners 建 org 共享环境（网络访问级别、环境变量、setup script） | Repo blocklists：按仓库配 glob 黑名单（`*.env`、`config/*`、`**/*.secret`）阻止文件被索引为上下文——数据出口管控而非网络出口 |
| MCP/工具/扩展白名单 | approved `mcp_servers`、`allowed_web_search_modes`（cached/live/disabled/indexed 四态）、apps/plugins/browsers/Computer Use 的 feature requirements | managed settings 可限制 MCP server 与插件来源、控制哪些 hooks 可跑；org 级模型限制/默认模型/effort 上限服务端强制 | 审计事件类型含 `mcp_server_config`（MCP 配置变更入审计流） |
| 二审机制（auto-review） | approvals_reviewer = auto_review：需要审批的动作先过评审 agent（查数据外泄/凭据探测/安全削弱/破坏性操作），低中风险放行、critical 直接拒、解析失败 fail-closed；企业可用 guardian_policy_config 替换租户策略段；管理员可用 allowed_approvals_reviewers 收权 | （生态插件 dsh-auto-review 即同类，官方未内置——见 §3 映射） | — |
| 生效验证 | `/status` 类验证：Claude Code 的 Status tab 显示 `Setting sources: Enterprise managed settings (HKLM)` 等，官方把它写进入职流程 | 同左（Claude Code） | — |

### 2.5 模型授权与路由

- OpenAI：RBAC 控制 Codex 入口（Release Notes：Workspace settings → Permissions & roles）；credits 按 metered item/model 拆解。
- Anthropic（最细）：org 模型限制（服务端强制禁用具体模型）、org 默认模型（新会话起点，可选强制）、**按角色的 effort 上限**；模型默认值与 entitlements 覆盖 chat/Cowork/Claude Code 三条产品线；BYO 云（Bedrock / Google Cloud Agent Platform / Microsoft Foundry）改变计费与合规姿态，部分功能（web 版等）仅限 claude.ai 席位——「provider 选择」本身是一等管理决策。
- Cursor：Model access API（preview）：团队基线 = custom policy 开关 + 新 provider/模型默认值 + per-provider/per-model 开关 + per-model 参数（Fast、reasoning effort）；Organization Groups 可对部分成员放宽；个人 BYOK key 的控制在 dashboard。

### 2.6 数据与隐私治理

- Cursor：Privacy Mode 团队默认开启、管理员可强制全员不可关闭；零数据保留承诺。
- Anthropic：Team/Enterprise/API 一律不用客户代码与 prompt 训练；保留策略由所选 API provider 决定；成员移除时的数据处置政策明确。
- OpenAI：External Access 控制成员能否对外用 Sign in with ChatGPT + Approved applications 清单（外部应用准入）；compliance API 管理 Codex 使用日志。

### 2.7 审计与合规

- Cursor：Audit Logs API 事件类型清单本身就是需求清单：login/logout、add_user/remove_user/update_user_role、team_settings、**mcp_server_config**、team_api_key/user_api_key、**privacy_mode**、user_spend_limit、team_rule/team_repo/team_hook/team_command、directory group 五种事件、bugbot 六种设置事件；带 IP、UA、分页、30 天窗口限制。
- OpenAI：Global Admin Console 内置 Audit logs（可搜索/过滤/详情视图，随 workspace 与权限变化）；Admin key 权限里显式包含 compliance log access。
- Anthropic：per-request 审计走网关——自托管 Claude apps gateway 记录带 IdP 身份的请求级审计日志；OTel 遥测（Codex 亦然）：tool_decision（approved/denied + 来源是配置还是用户）、api_request、user_prompt（默认脱敏）等事件类别 + SIEM 侧脱敏建议。
- 三家共同点：审计不只记「管理员改了什么」，还记「agent 的每次危险动作是谁批准/为什么放行」。这一点对 dsh 尤其重要（dsh 会话日志已有完整 approval 链，见 §3）。

### 2.8 Agent 资产盘点与分析

- OpenAI Agents 区：组织内所有 workspace agent 清单——Agent ID、跳转 Builder 编辑、最近活动、连接的 app、memory 文件、schedules、独立分析（unique users / runs over time）。
- Cursor Analytics API 的「采用度」指标族：MCP adoption、Commands adoption、Plans adoption、Skills adoption、Ask Mode adoption、Client Versions、Top File Extensions、Conversation Insights、Leaderboard、Bugbot analytics。
- Anthropic：skills 自报用量与成本、plugin adoption 与 artifact creation 有专门 endpoint；Claude Code Usage tab 的 top commands。

含义：「企业买了多少席位」之后的问题必然是「大家拿它干了什么、哪些技能/插件在真被用」——资产盘点与采用度分析是续费叙事的一部分，不是锦上添花。

### 2.9 开放集成（Admin API / 网关 / 非人类身份 / 设备管理）

- Admin API：Cursor 三套 API（Admin / Analytics+AI Code Tracking / Organization）+ Webhooks；Anthropic Analytics API + Admin API；OpenAI Credentials 页发 Admin keys（可配过期与权限，显式不含推理权限），支撑合规日志、分析与成本报表、限额自动化（Spend Controls API）、workspace 组管理五类场景。
- Service account（非人类身份）：OpenAI 为 Codex 自动化提供 workspace 级 service account，可被赋角色入组，授权分 User（发 token）/ Configure（配账号与插件）/ Manager（两者）；service-account token 让 Codex 以该身份运行。Cursor 用量事件原生携带 serviceAccountId 且可按其过滤计费。
- LLM 网关：Anthropic 官方支持在开发者与 provider 之间架网关（自托管 apps gateway 或第三方 LLM gateway）以获得统一入口 + 请求级审计；Bedrock/Foundry/GCP Agent Platform 三条 BYO 云路径。
- 设备侧：Claude Code 经 MDM/plist/HKLM 下发策略；Cursor Enterprise「在用户设备上强制允许的 team ID 与扩展」。

## 3. 与 dsh 现状的差距映射（内核 + 生态 + 本仓库）

内核事实（沿用 `deepseek-harness-enterprise.md` §2）：官方代码 grep `enterprise/billing/admin/多租户/计费` 零命中，一切皆插件。把九大功能域对照 dsh 已有的接缝与社区件：

| 功能域 | dsh 已有的接缝 / 社区雏形 | 差距判断 |
| --- | --- | --- |
| 身份席位 SSO/SCIM | 无官方件；`dsh-full-remote` 有令牌+一次性邀请+按设备会话（单机远程访问级，非企业 IdP 级）；早期件 dsh-multi-tenant / dsh-accounts(PG+admin) | **空白**。SSO/SCIM 必须自建（网关层做，见 §5） |
| 组织分组 | dsh-multi-tenant（租户身份/会话隔离/审计，★7 未发 npm）、dsh-gov（按 agent token 配额） | 仅有概念验证级，不可复用为企业组件 |
| 用量成本 | 数据丰富：会话日志含 token 与定价要素；dsh-whale-report（确定性报告引擎）、dsh-cost-meter、dsh-budget-guard（硬拦截）、dsh-context | 单机视角齐全，**聚合视角为零**——缺的是把 N 台机器的数据收上来，不是算不出 |
| Agent 行为管控 | 权限瀑布 tools/pre-execute 官方接缝；dsh-permission-rules（声明式 YAML 规则+网络策略+审计事件）、HOL Guard（pre-execute 异步审查 fail-closed）、sandbox-micro/k8e/mirage（执行隔离） | **底座最好的一块**：规则引擎与审查链已有社区实现，缺「控制台统一下发 + 防篡改 + 生效验证」（对应 managed settings 语义） |
| 模型授权 | credentials 接缝可换 provider/key（control-center 已证）；无 org 级模型白名单 | 需要在策略层新增 |
| 数据隐私 | 凭据明文 `.credentials.yaml`（dsh-credentials-keyring 可换 OS 钥匙串） | 至少要把密钥治理纳入 P0（钥匙串化），训练承诺取决于自选模型 provider |
| 审计合规 | 会话日志天然完整（dsh-auto-review 的 verdict 链「approval/asked→verdict→decided」可重放）；permission-rules 决策审计事件 | 单机日志齐全，**集中收集/检索/导出缺失** |
| 资产盘点 | 插件/skill/MCP 清单本机可得；upstream-radar（兼容性实证）、DShScan/HOL Guard plugin-scanner（供应链扫描） | 盘点数据源现成，聚合与采用度分析缺失 |
| 开放集成 | 官方 RPC/stdio 协议（本仓库 host 即用户）、OTel 有阿里/腾讯两家 trace 插件先例 | Admin API 需自建；trace 导出有参照实现 |

## 4. 功能基线清单（给 dsh 企业后台的需求定义，按优先级）

**P0（没有就不成其为企业管理后台，采购安全审查第一轮就会被问到的）**

1. **SSO（SAML/OIDC）+ RBAC + 席位管理**：至少三种角色——普通成员 / 组织管理员 / 只读分析员（对标 OpenAI Analytics Viewer 与 Cursor Unpaid Admin 的「管理者不占开发席位」设计）。
2. **集中托管策略（managed settings）**：一套从控制台下发、优先级高于本地配置、可在客户端 `/status` 类界面验证生效的策略包；首批键：审批档位（对应 dsh 权限预设）、deny/allow 工具规则、网络开关。数组类规则合并语义（只能加不能删）直接照抄 Claude Code。
3. **网络出口白名单**：deny-wins 的域名规则 + 默认禁内网/loopback 目标 + 解析到私网 IP 即拒（DNS rebinding 缓解）。Wavect 评测指出的「dsh 沙箱不覆盖网络」正是这块。
4. **用量与成本看板（按人/按组/按模型）+ 三级预算限额 + 阈值告警**：数据源用现有会话日志即可起步；限额要有硬拦截选项（dsh-budget-guard 已验证需求，npm 周下载半年翻倍）。
5. **审计日志**：管理员操作 + agent 危险动作审批链两类事件，可检索、可导出（API 或 SIEM 投递）。
6. **隐私与数据承诺的落地开关**：组织级「不上传/不留存」策略强制 + 密钥一律 OS 钥匙串（淘汰明文 credentials 文件）。

**P1（形成差异化与运维闭环）**

7. 模型授权：org 级模型白名单 / 默认模型 / 按角色 effort 上限；BYOK key 的组织策略（允许哪些 provider）。
8. MCP / 插件 / Skill 治理：白名单 + 采用度统计（对标 Cursor 的 mcp_server_config 审计事件与 adoption 指标族）+ 供应链扫描门禁（集成 DShScan / hol-guard plugin-scanner）。
9. Admin API + Webhook（成员、限额、审计、用量四组端点起步）。
10. Service account / 非人类身份：给定时任务、CI 里的 agent 独立身份与独立限额，用量事件按主体归因。
11. 部门/成本中心分摊：组级 spend 报表与 chargeback 导出（对标 Cursor Billing Groups）。
12. 提额审批流：成员应用内申请 → 管理员审批 → 临时生效（对标 OpenAI Pending requests）。
13. LLM 网关兼容：允许流量经企业自有网关（统一审计/路由），provider 可替换（DeepSeek 官方 / 自建 vLLM / 其他）。
14. 二审机制：高危动作第二模型复核、fail-closed（dsh-auto-review 已给出完整审计链设计可直接吸收）。

**P2（规模化之后的深化）**

15. 价值度量：cost-per-commit / 生产力估算面板（公式透明可调，学 Anthropic Value tab）；AI 生成代码占比追踪。
16. 自然语言分析问答（Analytics chat）。
17. 多 workspace / 多租户层级（Tenant 概念，集团型客户才需要）。
18. 设备侧强制（MDM 下发、注册表级抗篡改、客户端版本锁定）。
19. 合规认证与区域部署（SOC 2 / 数据驻留）——商业化的前置资质而非功能。

## 5. 形态建议：自托管团队网关 + Web 控制台 + 桌面端受管

三家全是云托管控制台，因为它们同时卖模型；dsh 是 BYOK 本地内核，「控制台在哪」是个真实的产品决策：

- **不建议先做云托管租户制**：那要求经营计费、代管密钥、合规实体，且与「你的 key 你的基础设施」的开源心智冲突；该路径上火山 AgentKit（十模块管控面）已是成熟答案，自建无差异化。
- **建议形态**：一个自托管的「团队网关 + Web 控制台」服务（企业内网一台即可），桌面端（本仓库 Tauri shell / dsh CLI）启动时向网关注册并拉取托管策略、上报用量与审计事件。这正好落在 Anthropic「apps gateway + managed settings」的同构位置，但覆盖了 Anthropic 不做的「纳管任意本地桌面实例」。
- **与本仓库资产的衔接**：host 进程已有 IPC 运行时与会话事件流（上报通道现成）、`docs/kernel-sync.md` 保持内核原封不动（策略引擎应挂在官方 tools/pre-execute 接缝而非改内核）、Tauri 端可承接「生效验证」UI。
- **风险提示**（沿用前文调研结论）：治理类社区插件普遍年轻、个别安全插件连 license 字段都缺失，集成任何一块都要按供应链流程过审；本仓库上游 anywhere-labs/dsh-desktop（★20,174）尚未做管理后台，此窗口存在但不等人。

## 6. 证据来源清单（2026-08-26 抓取）

1. OpenAI Help Center，《Global Admin Console》——help.openai.com/en/articles/12289294（Tenant/Access/Users/Credentials/Analytics/Billing/Usage limits/Agents/Audit logs 全部小节直读）
2. ChatGPT Learn，《Agent approvals & security》——learn.chatgpt.com/docs/agent-approvals-security（sandbox×approval 矩阵、network_proxy 域名规则、protected paths、auto_review、OTel 事件类别、Dev Container 参考实现）
3. ChatGPT Enterprise & Edu Release Notes——help.openai.com/en/articles/10128477（Codex RBAC 与 compliance API 条目，搜索摘要引用）
4. developers.openai.com《ChatGPT Work cloud security》（公网关闭时 managed destination allowlist，搜索摘要引用）
5. Anthropic 官方博客《Giving admins more visibility and control over Claude spend》——claude.com/blog/...（2026-07-02：按组/按人分析、Usage/Value 双 tab、Analytics API、模型 entitlements、75%/90% 告警）
6. Claude Code Docs，《Set up Claude Code for your organization》——code.claude.com/docs/en/admin-setup（managed settings 四级来源与合并语义、org 模型限制/默认模型/effort 限制、Cloud environments、网关审计、/status 验证）
7. anthropic.com/news/claude-code-on-team-and-enterprise（合规面板对接、数据保留管理，搜索摘要引用）
8. Cursor Docs，《Members, Roles, and Seat Types》——cursor.com/docs/account/teams/members
9. Cursor Docs，《Admin API》——cursor.com/docs/account/teams/admin-api（audit-logs 事件类型枚举、daily-usage-data 字段、filtered-usage-events、user-spend-limit、billing groups、model access、repo blocklists、Analytics/AI Code Tracking/Organization API 目录树）
10. Cursor Docs，《Privacy and Data》——cursor.com/help/security-and-privacy/privacy（Privacy Mode 团队默认开启 + dashboard 强制，搜索摘要引用）
11. Cursor Docs，《Enterprise》——cursor.com/docs/enterprise（设备侧 enforce allowed team IDs/extensions，搜索摘要引用）
12. Claude Support，《View usage analytics for Team and Enterprise plans》——support.claude.com/en/articles/12883420（搜索摘要引用）
13. 平台 Admin Analytics API 存在性——platform.claude.com/docs/en/api/admin/analytics（搜索摘要引用）
14. 本地证据：`.deepseek-harness/` 检出 grep（无企业能力）；`docs/deepseek-harness-enterprise.md` §2/§3/§6（内核权限预设、Wavect 评测、AgentKit 十模块、治理插件普查数据）

方法说明：本文是独立于《deepseek-harness-enterprise.md》各轮生态调研的管理后台专项调研，只回答「三家后台有什么、dsh 缺什么」；所有对标页均直读官方文档原文，仅标注「搜索摘要引用」的条目未全文展开（多为辅助性佐证，主结论不依赖它们）。