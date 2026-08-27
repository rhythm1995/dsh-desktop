# DeepSeek Harness 企业级落地与办公插件生态调研

> 调研文档，**不是本仓库实现规格**。本仓库只做 Tauri 桌面壳。入口：[`README.md`](README.md)。管理后台对标见 [`enterprise-admin-console.md`](enterprise-admin-console.md)。

调研日期：2026-08-26（当日三轮：第一轮盘点生态全景，第二轮全量重核硬数据 + 治理类插件逐个深挖用途，第三轮补强 AgentKit 能力全景、办公/IM/桌面插件逐项核实、客户端发行形态与目录站普查）。对象是本地检出 `.deepseek-harness/`（deepseek-ai/deepseek-harness 官方内核，MIT）以及它对应的社区插件生态：dshmarket.com、deepseek1024.com、dshfind.com 三家主要插件目录（第三轮另发现三家小型目录站，见 §0）+ GitHub。

本文只写事实与可核验数据。每一条结论都带来源；没有公开证据支撑的地方明确说「未发现/待核实」，不下似是而非的判断。优先级只有 P0 / P1 / P2，不排期。

## 更新记录

### 第三轮（2026-08-26 同日）

| 项 | 第三轮结果 |
| --- | --- |
| **上游硬数据补齐（本轮最重要）** | `anywhere-labs/dsh-desktop` **★20,174**（第二轮「★ 未取到」已补，GitHub API）；12,906 commits；最新 release v2.0.2（2026-08-21，Windows x64 NSIS + macOS Universal DMG，官网 www.dshdesktop.cn 自分发）；**git submodule 固定内核 commit `b150a55` = 本仓库 pin 完全一致**；README 带 UCloud 星图 AstraFlow 赞助商位与「独立社区项目、与深度求索无隶属/合作/授权/背书」声明 |
| AgentKit 能力全景（第二轮待补项） | 官方文档（docs.volcengine.com/86681）全目录核验：智能体运行时 / Skills 中心 / AIO Sandbox 三类沙箱工具（Code/Browser/Skills）/ 记忆库 / 知识库 / MCP 网关 / 身份与权限 IAM / 评测（接入 CozeLoop）/ 观测（运行时·工具·记忆库·MCP 监控+应用观测+数据观测）/ 日志 / 完整 OpenAPI + CLI（Local/Cloud/Hybrid 三模式）/ SDK 与 veADK 框架；「新功能发布记录」确认新增 Code Sandbox 沙箱模板 |
| 腾讯浏览器技能归属更正 | 真实仓库 = **`Tencent/BrowserSkill`**（★1,321，腾讯官方 org；此前按 owner=Tencent 猜的 `Tencent/dsh-plugin-browserskill` 是 404）。定位：「通过 CLI 与扩展让智能体操控真实登录的浏览器，无干扰地自动化任务」；dsh 插件只是该项目的发布物之一（1024Store id=`Tencent/BrowserSkill/packages/dsh-plugin-browserskill`） |
| 客户端赛道新增 | 【新】`zouyuxuan122/Deepseek-Harness-EAC` ★1,304——捆绑 Node.js 运行时 + 完整 dsh CLI 内核、一键启动、10 套内置 UI 主题、与 CLI 共享 DSH_HOME 会话/密钥、agent 与客户端分离自动更新；【新】`csthinker/deepseek-harness-desktop` ★1——Windows 自包含壳（双击 exe 开原生窗口）；linux.do 社区帖（2026-08）再次确认「官方目前没有桌面版，现在的桌面版都是第三方的」 |
| 目录站普查扩充 | 第二轮「三家目录」实际不止三家：【新】dshplugin.wiki（独立插件检索库）、dshplugin.market（按官方发布 spec 逐个验证「哪些是真 bundle / 构建失败 / 带安装时脚本」，安装可靠性向）、cordisplugin.com（插件安装指南站）；dshmarket 收录数 1,884 → **2,143**（站点 meta description 自述，同日）；imsai-sh/awesome 清单仓库描述自证绑定 deepseek1024.com（即它就是 1024Store 的目录仓库，「3,100+」为其精选口径） |
| 硬数据微调（GitHub API/npm registry，2026-08-26） | 官方内核 ★195,052；dataelement/dsh-desktop ★2,466；Minke ★519；strukto-ai/mirage **★3,566**（第二轮记录时的初发新品已冲高，Apache-2.0）；dream-num/dsh-univer-office 仓库坐实 ★129 Apache-2.0；zhuiyueya/dsh-im-gateway ★40；omdsh-dev/dsh-lark ★46 BSD-3-Clause、omdsh-dev/dsh-office ★16 Apache-2.0；ZRui-C/dsh-computer-use ★23 Apache-2.0；dsh-budget-guard npm 674/wk（较第二轮 335 翻倍）；dsh-full-remote npm 2,036/wk；dsh-plugin-desktop npm 1,174/wk（npm 口径，1024Store 自有窗口曾报 1,377）|
| 待核 | Andy8647/dsh-auto-approval 的 GitHub license 字段已检测为空（第二轮记 BSD-3-Clause）——安全类插件 license 缺失本身是集成前需复核的信号 |

### 第四轮（2026-08-26 同日，盲区补扫）

| 项 | 第四轮结果 |
| --- | --- |
| 方法 | 不再逐个核实已知项，改用 GitHub topic/search 扫描 + 1024Store API 按关键词分类查询（search/voice/agent-teams/memory/pet/vscode/mobile）+ 量子位等媒体报道交叉比对，专找前三轮未覆盖的**项目类别** |
| 总体发现 | 前三轮覆盖了治理/办公/客户端/云平台四条线，但漏掉了至少八条活跃赛道：**记忆系统、联网搜索与视觉桥、多智能体团队、工作台 IDE 化、移动远程与 HITL、崩溃恢复、语音交互、内容创作**；另有一个纯娱乐子生态（全部见新增 §7） |
| 重要更正 | `modlens` 此前按 1024Store 描述写成「模块/包透镜工具」——实际是 `liustack/modlens` **★3,665 的视觉桥插件**（粘贴图片得结构化 OCR/版面/语义 JSON 证据），定位完全不同；§4.4 已更正 |
| 最大遗漏 | 记忆系统赛道被三个大项目占据：`vectorize-io/hindsight` ★21,130（coding-agents 插件即其 DSH 集成）、`MemTensor/MemOS` ★10,985（官方声明支持 DSH、宣称省 35.24% token）、`zilliztech/memsearch` ★2,511（Markdown+Milvus，跨 Claude Code/Codex/DSH 共享记忆） |
| 待核 | zhu1090093659 名下 `dsh-aionui-panel`/`dsh-task-board` 在 GitHub 已检索不到（疑并入其 `dsh-web` ★6,074 聚合包）；vectorize-io 的 1024Store 包名 `coding-agents` 与仓库名 `hindsight` 不同源 |

### 第五轮（2026-08-26 同日，办公 Agent 平台可改造性评估）

| 项 | 第五轮结果 |
| --- | --- |
| 任务 | 回答「dsh 生态插件能否改造为豆包 / WorkBuddy / QwenWork 等办公 agent 的能力」——先查三个平台各自的扩展机制，再按插件架构分级评估可移植性 |
| 平台扩展机制 | WorkBuddy（腾讯 CodeBuddy 团队）：**兼容 OpenClaw 技能 + 内置 MCP 协议** + 开放插件安全审查；千问办公 QwenWork（阿里钉钉线，2026-08 由 QoderWork/MuleRun/悟空 三产品合并公测）：**自定义技能 + MCP 插件 + 技能市场**；豆包电脑版工作任务模式（字节）：**技能商店（200+ 技能与连接器，2026-08-21 上新）+ 自定义技能 + 连接器接 Office/飞书**，未见公开 MCP 文档 |
| 关键判断 | 三家都吃 Skill、两家吃 MCP，而 **Agent Skills（SKILL.md）是跨平台开放标准**——改造的正确姿势不是「移植插件」而是「抽出插件的工具层与工作流知识，换上 Skill/MCP 外壳」；治理与 UI 层不可移植也不需要移植（平台自建） |
| 新发现 | 腾讯在 dsh 生态的第二个插件 `dsh-weknora`（npm7d=1,071，文档→RAG/Wiki 知识管理）；DSH↔外部 harness 双向桥已成气候（dsh-codex-sync npm7d=2,356 双向同步、两个 OpenClaw 桥件、codex2dsh 等），格式互通是既成事实 |
| 详细评估 | 见新增 §8 |

### 第二轮（2026-08-26）

| 项 | 第二轮结果 |
| --- | --- |
| 硬数据重核 | 全部引用过 star 的仓库重新过 GitHub API / HTML 抓取；npm 周下载全部重拉 registry API（窗口 2026-08-18~24）；1024Store 开放 API 重拉并解析 `meta.catalogTotal` |
| 1024Store 规模 | 站方 API `catalogTotal=10,681`（meta，2026-08-25）——第一轮写 9,223 已过时 |
| 官方内核 | ★195,025（GitHub API）；最新 tag `v0.1.1-rc.2` = 本仓库 pin（docs/kernel-pin.json） |
| 治理插件深挖 | 完成 19 个既有插件逐个 README 级用途深挖 + 发现 6 个新品 + 多租户/计费分类普查 |
| 重要更正 | ① HOL Guard 由「区块链项目、待核实」更正为「AI Agent 防病毒 + 官方级 DSH 插件接入」；② 腾讯可观测插件正式名为 `tencentcloud-agentobs-sdk-dsh`；③ `irisinb` → `irisnb`；④ `ChisaAlter/dsh-usage-panel` 仓库已 404、★139 不可核实；⑤ 多租户插件「完全不存在」的结论需窄化（已有 4 个早期开源件） |
| 待补 | 「官方内核能力全景 + AgentKit/云平台」「办公/桌面/IM 插件逐项用途」「三家目录站点与客户端发行形态」三轮深挖进行中被中止，§3.1/§4.2 用途细节以第一轮 + 本轮 API 描述为准，已标注 |

## 0. 生态入口全景（调研范围）

| 入口 | 形态 | 规模（各自口径） | 数据可信度与来源 |
| --- | --- | --- | --- |
| dshmarket.com（dsh-market/dsh-market，★2,369） | 内置插件市场：分类、一键安装、更新/卸载 GUI | **2,143 个插件**（2026-08-26 站点 meta description 自述；同日第一轮首页人工核验为 1,884——收录在当日持续增长） | 收录自 awesome-dsh-plugin 精选列表；带 star + 周安装量；首页明示「进入精选列表不代表完成安全审查」 |
| deepseek1024.com（DSH 1024Store） | 排行/商店：按安装活跃度、star、release、仓库活跃度排序 | **API `catalogTotal=10,681`（2026-08-25，本轮直接读 API meta）**；单响应返回 500 条 | **开放 API `/api/v1/plugins`（robots.txt 白名单）+ `?q=` 过滤**，每插件含 installs30d/failureCount/verification/growth 字段，可程序化核验——本文安装可靠性数据全部取自该 API |
| dshfind.com | 插件市场 + 学习社区：全量索引 dsh-plugin 仓库，每日同步 star 与质量分；另有课程、Cordis 论文精读、皮肤市场（100+，带评分与人工审核） | 全量索引（其公告称插件 6,000+，2026-08 官方 Discussion #1179） | 官方仓库 deepseek-ai/deepseek-harness Discussions #1179 自我推介；zhihu 有实测文章（dshfind 评分榜第一 dsh-vision-toolkit） |
| imsai-sh/awesome-deepseek-harness-plugins | 社区精选目录 | GitHub API 描述现写「3,100+ plugins」（★186）；第一轮记录 10,566（PR 收录 + dsh-plugin topic 自动发现，2026-08-25）——**两个口径待核实**（可能是「精选商店数 vs 全量清单数」） | **第三轮澄清：该仓库描述自证绑定 deepseek1024.com**（「plugin store, marketplace and hub — 3,100+ dsh plugins with search, rankings, install commands and a free public API. … deepseek1024.com」），即它就是 1024Store 的目录仓库而非独立第四家；与 catalogTotal=10,681 的差异是「精选清单 vs 全量自动收录」的关系 |

**第三轮新发现的小型目录站**（规模远小于上述三家，仅登记存在）：

| 站点 | 形态 | 核验方式（2026-08-26） |
| --- | --- | --- |
| dshplugin.wiki | 独立插件检索库：「Discover, search, and explore the continuously updated DeepSeek Harness plugin ecosystem」 | 首页 title/description 直读 |
| dshplugin.market | 安装可靠性向验证站：按官方发布 spec 逐个检查「哪些是真 bundle、哪些构建失败、哪些带安装时脚本」（"Which DeepSeek Harness Plugins Install"） | 首页 title/description 直读；其插件详情页带「Install check: Not checked / no package.json found」等核验状态 |
| cordisplugin.com | 插件安装指南站（每插件一页安装命令 + GitHub 指引；页面 JS 渲染，静态抓取无元信息） | Google 搜索结果 + 页面直读 |

口径提示：四家目录的筛选规则不同（精选列表 / spec 自动入库 / 全量 topic 扫描），**star 与下载量数字不可跨站直接比较**；本文硬数据以 GitHub API、npm registry API、1024Store API 为原始来源并标注日期，第三方站点数字仅作趋势参考。

## 0. 前提澄清：两个「DeepSeek Harness」同名不同物

检索「DeepSeek Harness 企业级部署」时大量内容（如 53AI 的《DeepSeek Harness 企业级部署解决方案》，2026-08-24）讲的「Harness」是指**把 DeepSeek 模型做成企业服务的工程化框架**（vLLM/SGLang + API 网关 + 鉴权/限流/观测 + 成本控制的方法论），与 deepseek-ai/dsh 这个 Agent 运行时**不是同一个东西**，只是同名。

- 本文「官方内核」= deepseek-ai/deepseek-harness（Agent 运行时，社区口径简称 dsh）。
- 涉及「同名泛化概念」的文章一律标注为【同名泛化】并只作方法论参考，不作为 dsh 的能力依据。

## 1. 结论（TLDR）

- **官方内核没有企业级能力**：本地检出全仓库 grep `enterprise / billing / admin / 多租户 / 计费` 零命中；`apps/` 只有 `cli` 和 `web`；`docs/api-gateway.md` 讲的是 Web 客户端内部 RPC，不是多租户网关。官方 README 明确标注「开发者预览，未来将出现破坏兼容性的变更」。但生态动量极大：官方仓库 **★195,052**（2026-08-26），本仓库 pin = 最新 tag `v0.1.1-rc.2` ≥ npm `@deepseek-ai/dsh@0.1.1-rc.2`（第三轮 tags.atom 复核：最新 tag 未变）。
- **企业级落地目前有两条真实路径**：① 外部云平台承载（火山引擎 AgentKit 是唯一有完整叙事与实操文章的）；② 用插件自建「治理组合包」（对账 + 可观测 + 审批 + 供应链 + 沙箱），这是工程责任，不是开箱即用产品。第二轮新增确认：**HOL Guard（★476）确为 AI Agent 防病毒产品，且提供面向 dsh 的插件（hol-guard-plugin）与插件 CI 扫描器**——「治理组合包」的供应链层有了更硬的候选。
- **计费结论窄化**：用量与成本看板仍然全是个人/小团队工具（无计费网关、无发票账单、`department` 类目 0 命中），**但多租户/多用户粒度的早期开源件已经出现**（dsh-multi-tenant、dsh-accounts（PostgreSQL + admin）、dsh-multi-user、dsh-gov（按 agent token 配额）、dsh-budget-guard（预算硬拦截）），均未发布 npm 或周下载 < 400，成熟度与审计都无从谈起。
- **办公场景：碎片插件齐全，但没有 WorkBuddy / TRAE Work 式成品**。文档（Word/Excel/PPT/PDF，另有 Univer 引擎新品 dsh-univer-office）、邮件、CalDAV 日历、macOS 桌面自动化、浏览器控制、9~20+ 通道 IM 全部有插件，文档类插件有 WPS 官方论坛的正式对比评测；但缺少统一产品层、深度微信/企微耦合和邮箱客户端（§4.2 用途细节待第三轮补强，本轮 API 描述已更新工具清单）。
- **市场情报（与本仓库直接相关，第三轮大更新）**：GitHub 上存在同名高星项目 `dataelement/dsh-desktop`（★2,466）与 `dsh-market/dsh-market`（★2,369）；**本仓库的 Electron 上游 `anywhere-labs/dsh-desktop` 已达 ★20,174**（12,906 commits），以 v2.0.2 通过官网 dshdesktop.cn 分发 Windows/macOS 安装包、带 UCloud 赞助商位，且 **git submodule 固定的内核 commit `b150a55` 与本仓库 pin 完全一致**——上游已是社区桌面发行版的事实标准。上游自己也以插件形态上架 1024Store（npm `dsh-plugin-desktop@2.0.0`，installs30d=53 / failureCount=254 / npm7d=1,174~1,377 视窗口口径，2026-08-25/26 数据）——DSH Desktop 既是发行形态又是生态内插件。**客户端赛道第三轮继续变挤**：除 dshfind「Clients」类已有的 oh-dsh、`lencx/Minke`（★519，Apache-2.0）、`FlashingChen/dsh-desktop-hub`（★53）、`ChisaAlter/Deepseek-Harness-Desktop`（★140）外，新增 `zouyuxuan122/Deepseek-Harness-EAC`（★1,304，捆绑 Node + dsh CLI 内核、一键启动、10 主题）与 `csthinker/deepseek-harness-desktop`（★1，Windows 自包含壳）；linux.do 社区帖确认「官方目前没有桌面版」。
- **插件生态规模（2026-08-26 重核）**：dshmarket 1,884 精选（首页） / 1024Store 10,681（API catalogTotal） / dshfind 全量 6,000+ / imsai awesome 3,100+（现描述）或 10,566（旧记录，待核），口径不同不可直接比较（§0）。**第四轮盲区补扫（同日）另发现八条此前未覆盖的活跃赛道——记忆系统（hindsight ★21,130 / MemOS ★10,985 / memsearch ★2,511 三巨头）、联网搜索与视觉桥、多智能体团队、工作台 IDE 化、移动远程 HITL、崩溃恢复、语音交互、内容创作，外加一个高星娱乐子生态（鲸鱼娘皮肤 ★1,716），全部见 §7。**
- **办公 agent 平台可改造性（第五轮，同日）**：WorkBuddy 兼容 OpenClaw 技能+内置 MCP、QwenWork 支持自定义技能+MCP 插件+技能市场、豆包有 200+ 技能的技能商店——三家都吃 Skill、两家吃 MCP。**dsh 办公向插件的工具层（exceljs/nodemailer/CalDAV 等纯逻辑）可改造为 Skill/MCP 包输出到这些平台；治理与 UI 层不可移植也不需要。分级清单与合规义务见 §8。**

## 2. 官方内核现状（证据）

| 项 | 证据 | 核验方式 |
| --- | --- | --- |
| 无企业级代码/文档 | `docs/`、`website/`、`packages/` grep 无命中；`apps/` 仅 cli+web | 本地检出直接 grep（2026-08-26） |
| 开发者预览 | 官方 `README.zh.md`：「目前处于开发者预览阶段，正在快速迭代。未来将出现破坏兼容性的变更」 | 本地检出直接读 |
| 架构定位「一切皆插件」 | 官方 README + docs/architecture；模型、工具、会话存储、权限、Agent 循环均可换插件 | 本地检出直接读 |
| 企业就绪度独立评测 | Wavect《DeepSeek Harness 评测：企业生产就绪度》，2026-08-16：结论「适合做范围受控的工程试点，还不是可以直接采用的生产控制平面」；sandbox 只覆盖文件系统，**不覆盖网络访问与进程可见性** | wavect.io/zh/blog/deepseek-harness-enterprise-review/ |
| 官方内置权限基线 | 默认预设含需审批的 `workspace-write` 与免审批的 `danger-full-access` 两档 | Wavect 评测 + 本地 presets |
| 体量与版本 | ★195,052；最新 tag `v0.1.1-rc.2`（tags.atom，2026-08-26 第三轮复核未变）＝本仓库 pin；npm `@deepseek-ai/dsh@0.1.1-rc.2`；仓库 slogan「Everything is a Plugin」；官方发布页 deepseek.com/harness/：「DeepSeek Harness is now available in developer preview…Everything is a plugin」，2026-08-13 上线两天即 ★9.5 万（flowtivity.ai 追踪文） | GitHub API + tags.atom + docs/kernel-pin.json |

## 3. 企业级落地方案

### 3.1 平台托管型（云厂商承载 dsh）

| 平台 | 形态 | 证据（含权威性） | 认证/硬数据 |
| --- | --- | --- | --- |
| **火山引擎 AgentKit**（唯一有完整叙事者） | 云端沙箱（microVM）运行 dsh：安全隔离、7×24 持续运行、统一身份鉴权、MCP/技能/知识库管理、多租户隔离、大模型防火墙、内容安全护栏、行为审计；沙箱约 0.9 元/小时（2vCPU/4G，按秒计费） | 火山引擎官方产品页 / 官方文档（docs 86681）；官方文章《用 AgentKit，5 分钟搭建云端安全隔离的 DeepSeek Harness》（火山引擎开发者，2026-08-23，含从创建沙箱到产物落 TOS 的完整实操）；B 站官方视频 | 官方文档「产品功能」页确认：支持多种部署形态（本地代码包/容器镜像）、企业级长期记忆、满足企业级权限与治理要求 |
| **AgentKit 能力全景**（第三轮按官方文档全目录核验，补第二轮待补项） | 平台模块共 10 块：① **智能体运行时 Runtime**——CLI 或控制台创建、版本管理、实例/模型/环境变量/IAM 角色配置、观测开关、多模态调用；② **Skills 中心**——Skill 与 Skills 空间的发布/版本/集成管理；③ **工具体系（AIO Sandbox 三类沙箱模板）**——Code Sandbox（代码生成/调试/依赖安装/脚本执行与测试验证，「新功能发布记录」确认新增）、Browser Sandbox（浏览器交互）、Skills Sandbox（内置常用+可执行 Skills 的智能体，适合复杂技能编排与企业级系统场景），工具实例带 TTL 生命周期/实时日志/异步命令；④ **记忆库**——创建/导入/检索/提取策略/私网访问；⑤ **知识库**——导入/切片详情/检索问答/私网访问；⑥ **网关**——MCP 服务与 MCP 工具集托管；⑦ **身份与权限**——IAM 策略类型、项目与标签；⑧ **评测**——接入 CozeLoop、Agent 评测；⑨ **观测**——运行时/工具/记忆库/MCP 四类基础监控 + 应用观测 + 数据观测 + 日志字段规范；⑩ **交付链**——完整 OpenAPI + CLI（init/config/build/deploy/launch/invoke/status/destroy，Local/Cloud/Hybrid 三种部署模式）+ SDK 与 veADK 应用框架 | docs.volcengine.com/86681 文档目录与新功能发布记录（2026-01 最近更新标注；2026-08-26 第三轮抓取）；官方示例仓库 `volcengine/agentkit-samples` |
| 腾讯云 | 可观测 SDK 直传 CLS：五层 Trace（Session/Agent Loop/模型流/工具生命周期） | 腾讯云官方仓库 **`TencentCloud/tencentcloud-agentobs-sdk-dsh`**（第一轮名字少了 tencentcloud- 前缀，已更正；README 描述「将 GenAI trace 数据直传腾讯云 CLS」） | ★11（2026-08-25 有 push） |
| 阿里 | OpenTelemetry GenAI Trace（Agent Turn/模型/工具/token） | 阿里系仓库 `loongsuite/dsh-plugin` + `alibaba/loongsuite-pilot` | ★17（Apache-2.0）/ ★143（Apache-2.0）（均 2026-08-25 活跃） |
| 火山（开源侧） | 官方开源项目 OpenViking 的 dsh 记忆插件：长期记忆/知识检索/技能整合为自演进上下文数据库 | `volcengine/OpenViking`（examples/dsh-memory-plugin）；OpenViking 本体 ★33,243、AGPL-3.0，「Self-evolving Context Database for AI Agents. Unify Agent Memory, Knowledge RAG and Skills」 | 1024Store：installs30d=143、npm7d=4,906（2026-08-26）；说明火山不只平台承载，也在 dsh 生态内发官方插件 |

要点：**AgentKit 是「管控面」，dsh 是「执行面」**——在 AgentKit 里跑 dsh，身份、权限、审计、计费、内容安全由平台提供。这是目前唯一有官方文档 + 实操指南的企业级承载路径。（第三轮已按官方文档补齐能力全景：对「企业管控面需要什么」可直接对照上表 10 个模块，dsh 插件生态里对应能力均只有社区早期件。）

### 3.2 私有化部署型

- **veStack（火山引擎混合云）+ AgentKit + ArkClaw**：官方文章《veStack × DeepSeek-V4：从模型到企业级 Agent，一步到位》（火山引擎 Agent 社区，2026-05-07）——全栈版（企业级智算中心、万卡纳管）/ 轻量智算版（快速自建 Agent 环境）；模型走 `vaeutil` 三步部署，Agent 平台预集成。注意：此文面向 DeepSeek-V4 模型私有化 + AgentKit 通用架构，不是 dsh 专属，但企业「数据不出域 + 跑 Agent」的路径在此。
- 【同名泛化】53AI《DeepSeek Harness 企业级部署与实践指南》（2026-08-24）属于方法论（三层抽象、token 级准入、潮汐调度、mTLS/RBAC/审计留存），核心结论可与 dsh 交叉引用但**不是 dsh 的实现**。文中两个生产案例（金融券商 16×H20 私有化：合规改造三周 > 部署两周；互联网公司多模型纳管：利用率 42%→78%、账单 -31%）可作企业级诉求清单的参考。

### 3.3 自建治理组合包（插件级，全部为社区插件）

这是「插件即企业能力」的现实形态。第二轮把每一款都读了 README，按「定位 → 能做什么用 → 数据 → 来源」逐个展开；第一轮的表格要点放开头做索引。

**组合包索引（数据为 2026-08-26 实测）**：成本对账 dsh-whale-report（★29，npm 328/wk）│用量看板 dsh-context（★1,029，npm 27,812/wk）│成本/预算 dsh-cost-meter（★193，npm 17,152/wk）│余额/账户控制台 deepseek-harness-control-center（★65，npm 包名 `deepseek-harness-wallet` 6,961/wk）│审批 dsh-auto-approval（★4）│保险丝 anti-virus HOL Guard（★476）+ hol-guard-plugin（★4）│执行沙箱 sandbox-micro（★3，未发布 npm）│【新】沙箱矩阵 dsh-k8e-sandbox-bundle（k8e ★480 体系插件）│【新】虚拟工作区 mirage-dsh（strukto-ai/mirage）│供应链审查 DShScan（★8）│凭据加固 dsh-credentials-keyring（★3，未发布 npm）│兼容性实证 upstream-radar（★8，npm 4,617/wk）│出口管控 dsh-proxy-routing（★1，npm 未发布）│可观测 loongsuite/dsh-plugin（★17）+ tencentcloud-agentobs-sdk-dsh（★11）│通知与运维控制台 dsh-notifier（★71，npm 2,367/wk，27 通道）│权限规则 dsh-permission-rules（★42，npm 1,141/wk）│自动评审 dsh-auto-review（★106，npm 1,302/wk）│会话治理 dsh-session-manager（★49，npm 1,326/wk）。

---

#### dsh-whale-report（SenmuuuuW）——深迹 DeepTrace：只读、确定性的 Agent 运营报告引擎

- **能做什么用**：从会话事件日志生成日报/周报/月报/年报/滚动 24h/自定义区间共 6 种报告；内置 10 条确定性 Finding 规则（深夜消耗、峰谷成本、重试风暴、缓存命中率变化、致命操作、疑似密钥、工具健康等，每条带阈值与归因）；成本口径统一（input=miss、cacheRead=hit、output 三块分列，reasoning 归入 output 拆解，fees 按官方峰谷价计算，2026-08-17 峰/谷两档 + 6h 缓存价）并对齐 Asia/Shanghai 自然日；providerBreakdown 只取 deepseek-official 便于与平台账单对账；IMPROVE 引擎 4 条确定性改进规则（工具重复失败、重试浪费、用户纠正、峰期成本）只出建议不自动执行，全程 0 额外 LLM token。**给谁用**：个人成本复盘 + 团队周报/月报分发（输出 Web 面板 / PNG / HTML / PDF / markdown）。**只读保证**：不改写 session 历史、密钥扫描只报有无不存原文、损坏日志只披露 id 与原因、API 仅本机 loopback。费用仍是估算而非账单。
- **数据**：★29，npm `dsh-whale-report` 328/wk，MIT。来源：github.com/SenmuuuuW/dsh-whale-report

#### dsh-context（bowenliang123）——上下文窗口的「X 光机」

- **能做什么用**：Context 面板 + `/context` 命令实时展示模型当前上下文构成——六色堆叠条（系统提示/工具 schema/用户消息/注入上下文/助手回复/工具结果）对照窗口容量、top-5 最贵工具 schema；逐请求趋势图（Step/Turn 粒度 × Total/Delta，标注压缩位置）；事件流（Inject/Compact/Prune/Switch/Mode 每项标生产源与 token delta）；消息列表逐条计价；Context browser 下钻任意 step 逐条内容（含 read_image 缩略图、Diff）；多模态按官方 image→token 换算估价（DSH 0.1.1+）。**能定位「哪部分吃掉了预算」、压缩前后窗口变化、注入来源**。口径复用官方 contextPressure/contextBreakdown，压缩前 step 从归档消息重建并诚实标近似。以洞察为主，无写操作（只读 + 用户偏好设置）。
- **数据**：★1,029，npm `dsh-context` 27,812/wk，Apache-2.0。来源：github.com/bowenliang123/dsh-context

#### dsh-cost-meter（Han-1413141）——最重的个人计费仪表盘

- **能做什么用**：会话费用（输入区/标题栏常驻）、当日费用、**预算图框**（额度-周期-已用 %，≥80% 预警但只提醒不阻断）、官方余额（复用 API key 只发官方域名，非官方 baseURL 拒绝查询）、自定义 Provider 余额（HTTP+extract 规则，含 NewApi 换算模板）、OpenCode Go 额度、**9 家 Coding Plan 额度**（Anthropic/Z.ai/MiniMax/Kimi/OpenRouter/SiliconFlow/CommandCode/SCNet/火山方舟，AK/SK HMAC 签名）、26 周 token 热图、按天账本（默认留 180 天、装前历史自动回放导入）、价格表手工编辑 + 官方定价页一键同步（中英双币）。峰谷时钟：UTC 01:00–04:00 / 06:00–10:00 为峰，2026-08-23 起周末全谷，切峰/谷前弹窗 + 系统通知。Plan（订阅等值记账）与 API（按量计费）双轨分离。**给谁用**：重度多账户个人用户；无多用户/账单导出能力。
- **数据**：★193，npm `dsh-cost-meter` 17,152/wk，MIT。来源：github.com/Han-1413141/dsh-cost-meter

#### deepseek-harness-control-center（feibi-mochi）——「钱包 + 控制台」常驻挂件

- **能做什么用**：官方余额 60s 刷新；本会话成本按事件时刻锁价（含峰谷）；v4 模型峰/谷环形时钟（北京时间 09:00–12:00 / 14:00–18:00 峰、周末全天谷）；官方定价页自动校验同步（结构异常拒改）；365 天本地用量账本（token 热图，不存提示词/响应）；**多账户切换**：把 key 写入 DSH credentials 接缝，下一条 LLM 调用即换计费、无需重启；凭据加密（Windows DPAPI，其他平台 owner-only AES-GCM key 文件）+ 双副本 + 废钥锁写 + 掩码显示；环境变量提供 key 时拒绝切换；低余额红点 + 桌面通知、悬浮窗拖拽缩放。
- **数据**：★65，npm 包名 `deepseek-harness-wallet` 6,961/wk，MIT。来源：github.com/feibi-mochi/deepseek-harness-control-center

#### dsh-auto-approval（Andy8647）——审批策略新增 `auto` 档：规则 + LLM 两级分类器

- **能做什么用**：在 host 侧加 `auto` 审批档，L0 确定性规则 + L1 LLM 分类器对每次工具调用做 allow/deny 二态判定，**拿不准即 deny，完全无人在环**；client 侧在权限选择器旁挂状态 chip + 最近决策表。demo 行为：文件读写与 `ls` 白名单直发、无害命令 L1 放行、danger/legacy-ask/self-kill 守卫一律 deny。**适合**希望「大部分例行动作免打扰、危险动作兜底拒绝」的单用户。
- **数据**：★4（第三轮复核：GitHub license 字段已检测为空，第二轮记 BSD-3-Clause——安全类插件 license 缺失需在集成前人工复核 LICENSE 文件）。来源：github.com/Andy8647/dsh-auto-approval

#### HOL Guard（hashgraph-online/hol-guard ★476 + hol-guard-plugin ★4）——AI Agent 的开源「防病毒」，已确认接入 dsh【重点更正】

- **能做什么用（本轮核实）**：hol-guard 是**本地优先的 AI Agent 安全层**，不是区块链项目——在工具执行前评估 shell/文件/MCP/prompt/工具结果事件，拦截密钥暴露、提示注入、危险命令、恶意包（package install / plugin / skill / MCP server / hook），输出 Block / Approval / Receipt 三件套，四个保护等级 Gentle/Balanced/Strict/Paranoid；可选 Guard Cloud 做团队策略/审批流同步。Hedera 背景只在品牌（HOL Registry 存链上，org 名 hashgraph-online）。**DSH 接入是官方插件**：hol-guard-plugin README 标题即「Codex and DeepSeek Harness plugin」，`dsh plugin --profile headless/web add github:hashgraph-online/hol-guard-plugin` 安装，执行契约 = tools/pre-execute 异步审查 + 官方一次性审批 + `ctx.tools.guard()` 单调最终拒绝边界，缺 Guard/超时/畸形响应一律 fail-closed，普通 pre-execute 监听器无法放行 Guard 的 deny；自带 DSH headless 端到端拦截测试。主仓库 plugin-scanner 支持 `--ecosystem deepseek-harness`（识别 dsh.bundle + cordis.patch.yml + Cordis apply 导出），即**插件上线前 CI 扫描器**（评分制 + SARIF + GitHub Action）。
- **第一轮结论勘误**：「HOL Guard 待核实 / 是区块链项目」均不成立，正确结论 = 本地 agent 防病毒（hol-guard CLI，pipx 安装，Python≥3.10）+ DSH 插件（hol-guard-plugin）+ dsh 插件 CI 扫描（plugin-scanner）。主 README 的「Supported AI Agents」清单列 12 个 agent（Codex/Claude Code/Copilot/Cursor/Gemini/Hermes/OpenClaw/OpenCode/Antigravity/Kimi/Grok/Pi/ZCode），DSH 走插件路径接入。
- **数据**：★476（Apache-2.0）/ ★4。来源：github.com/hashgraph-online/hol-guard、github.com/hashgraph-online/hol-guard-plugin。待核实：dsh-security-boundary.md 的完整 failure matrix 未展开。

#### sandbox-micro（omdsh-dev）——fail-closed 微虚拟机能力包

- **能做什么用**：提供 `ctx.microsandbox` provider + `microsandbox_exec`/`microsandbox_fs` 两个模型工具，在一致性微虚拟机里跑 bash 与客户文件读写、保留退出码；**绝不降级到非隔离的主机执行**。装后 provider 与工具行在 cordis.patch.yml 默认 disabled，须显式 `config.enabled: true` 才生效——装插件不会隐式暴露微 VM。
- **注意（第一轮勘误）**：**Linux 需要 /dev/kvm**（不满足不可用）；**macOS 尚未支持**；Windows 未提。SDK 钉死 microsandbox 0.6.7。npm 未发布（@deepseek-ai/dsh-sandbox-microsandbox 查无）。
- **数据**：★3。来源：github.com/omdsh-dev/sandbox-micro

#### dsh-k8e-sandbox-bundle（xiaods/k8e）——【新发现】「E2B 自托管版」沙箱矩阵

- **能做什么用**：把 k8e（CK8s 单二进制自托管沙箱基础设施，★480）接到 dsh：Sandbox Gateway（gRPC mTLS + E2B 兼容 HTTP）+ 可拔插隔离运行时（gVisor/Kata/Firecracker）+ Cilium eBPF 每会话网络策略 + <500ms 预热池 + 内容寻址快照；Claude Code/Codex/Pi/dsh 共享同一沙箱矩阵，给模型的工具面为 exec/files/PTY/快照。**场景**：企业要在自有机房给多 agent 提供统一、隔离、可审计的执行环境。
- **数据**：npm `@k8e-sandbox/dsh-k8e-sandbox-bundle` 1,923/wk（v0.3.9）；1024Store 窗口 2,255/wk。来源：github.com/xiaods/k8e

#### mirage-dsh（strukto-ai/mirage）——【新发现】文件与 shell 全部虚拟化

- **能做什么用**：用 Mirage 统一虚拟文件系统替换 dsh 的 ctx.fs 与 ctx.shell——约 50 种后端（RAM/S3/Redis/Slack/Gmail/Notion/Postgres…）挂成目录、按挂载控制读写/执行模式、命令按需路由到沙箱（monty/pyodide/quickjs 进程内；docker/e2b/daytona 远程）、注册 CLI（git/gh/slack…）。治理意义：**模型的文件与命令操作全部落在虚拟工作区而非宿主盘**。
- **数据**：v0.0.1 初发，npm `@struktoai/mirage-dsh`（1024Store npm7d=1,438；npm registry 口径 587/wk）；**GitHub ★3,566、Apache-2.0**（2026-08-26 第三轮，初发即冲高——虚拟文件系统是生态稀缺位）。来源：github.com/strukto-ai/mirage

#### DShScan（shaoshi20/dshscan）——插件供应链扫描器（静态 + 可选 LLM 语义）

- **能做什么用**：输入插件名/GitHub 仓库/本地目录/zip/npm 包，输出 JSON 报告（risk_score 0-100、safe_to_install 门槛、findings 每条带 id/severity/category/evidence/rule）。风险面覆盖通用恶码（R001 远程管道执行、R002 eval/exec 动态执行、R003 混淆、R005 凭据窃取、R006 持久化提权、R009 生命周期脚本）**与 DSH 特有攻击面**（R010 cordis.patch.yml 插件树注入、R011 client.mjs 浏览器侧恶码（外连/键盘/剪贴板）、R012 profile 篡改禁用安全行）；R008 专扫 README 提示注入；内置硬编码密文检出器；附 --benchmark 恶意/良性评估集、--serve Web 面板、JSONL 审计日志。**给谁用**：装插件前自检、dsh 目录巡检（GHA 每日批扫）。npm 未发布。
- **数据**：★8，MIT。来源：github.com/shaoshi20/dshscan

#### dsh-credentials-keyring（irisnb）——OS 原生钥匙串凭据（owner 拼写已更正：irisinb → irisnb）

- **能做什么用**：实现 CredentialProvider 四操作，把 dsh 默认明文 .credentials.yaml 换成 **Windows Credential Manager / macOS Keychain / Linux Secret Service**；消费者零改码（resolve 返回 {value, source:'keyring'}）；无 Secret Service 的 headless Linux 挂载后 set 直接抛错、绝不假存。
- **数据**：★3，MIT，**未发布 npm**（git 直装），底层 @napi-rs/keyring。来源：github.com/irisnb/dsh-credentials-keyring

#### upstream-radar（MicroMilo）——插件生态的「精确 release × 精确宿主」兼容性实证平台

- **能做什么用**：①生成插件的精确 IR（npm 产物字节 ↔ 源码 commit ↔ DSH 宿主 ↔ 运行时/profile ↔ 依赖 ↔ advisories）；②在**一次性、secret-free 的 GitHub VM** 里实证三个执行面（headless 安装加载、Chromium Web 启动、真实 PTY TUI 交互），**缺证据从不当 pass，disposable runner 而非模型定结果**；③DSH/插件/依赖变更或证据过期自动重测；④确认插件可归属的失败开单给维护者 + 复检闭环。已跑 100 插件 feed（87 兼容 / 9 复核 / 0 复现不兼容 / 4 未观察），历史报告 13 份。**第一轮勘误**：README 未见 OSV 漏洞端口，测的是「版本漂移、peer 范围、锁文件不一致、构建门槛、宿主依赖缺失」——定位是兼容性实证，不是安全审计或长效徽章。**给谁用**：插件开发者（可修复报告 + 复检闭环）为主，使用者（`npx upstream-radar inspect` 精确定产物）为辅。
- **数据**：★8，npm `upstream-radar` 4,617/wk，Apache-2.0。来源：github.com/MicroMilo/upstream-radar

#### dsh-proxy-routing（chenjiyan2001）——按审批门控的出站代理路由

- **能做什么用**：连接「已存在」的 HTTP(CONNECT)/SOCKS5 代理，给 bash/PowerShell 子进程（含嵌套 git/curl/npm/pnpm，前后台）按执行注入代理环境变量；per-provider LLM 路由与直连隔离；配置走官方 settings namespace（热更新）；NO_PROXY 默认排除 loopback 与 api.deepseek.com。**与 dsh 权限联动**：net_proxy_enable/disable 需人工审批——Full Access 即时生效，低权限档强制人工确认；状态查询任何档位免审批。**场景**：GitHub 克隆超时 / curl 断流 / 依赖安装 ETIMEDOUT / provider 域名不可达（docker pull 不适用）。
- **数据**：★1，MIT，npm 未发布（README 标 "After release"）。来源：github.com/chenjiyan2001/dsh-proxy-routing

#### dsh-notifier（THEWOLFWALKER）——手机端运维/审批控制台（第一轮「25+ 通道」更正为 27 通道）

- **能做什么用**：一个 notify() API，**27 个推送通道**（telegram/slack/discord/feishu/dingtalk/wecom/wecom-app/qq-bot/onebot/teams/mattermost/gchat/bark/pushover/pushdeer/chanify/ntfy/gotify/igot/wxpusher/pushplus/serverchan/qmsg/xizhi/webhook/bell/desktop）；双触发（会话 turn/end、approval/asked、agent/error 自动推 + 模型 notify 工具）+ 层级路由（timeSensitive/active/passive → 静默/优先级/艾特、分级重试）；**6 个通道进站反向控制**：手机审批（按钮/回复，静默从不批准）、远程对话（followup/inject/！前缀 steering、合并窗口）、**手机命令中心**（长任务心跳 15min、停滞告警 10min、Telegram/Feishu 卡片带 ⏹ 停干按钮 HMAC 一次性 token、/quiet）；本地 Web 管理控制台 6 页（仅 127.0.0.1 + Bearer）；**身份系统**（配对码 /pair、复合键绑定 channel:userId 防冒用、角色、空白名单引导态）。**给谁用**：挂机跑长任务 + 必须人审审批的个人/小团队管理员——把审批台搬上手机，零运行时依赖。
- **数据**：★71，npm `dsh-notifier` 2,367/wk（v0.8.6），MIT。第三轮 README 复核：「channels-27」徽章 + 「six inbound channels」原文确认 27 出 / 6 进；仓库 About 里写「8 channel adapters」是未更新的旧文案，以 README 为准。来源：github.com/THEWOLFWALKER/dsh-notifier

#### dsh-permission-rules（PerryLink）——Claude Code 风格声明式权限规则 + 进程级网络策略

- **能做什么用**：YAML 规则（默认 .dsh/rules.yaml）挂在 tools/pre-execute 瀑布上：tools 名 glob（含 mcp__*）/agents 选择器（main|subagent|preset:*）/参数键值 glob 或 regex/workspace 相对路径 glob 任意深度/when(env+platform)/network（domains/ips/ports/schemes，CIDR、端口范围）；deny→reason 回填 tool result，ask 走官方审批，allow 与 no-match 严格 next() 不短路；分层规则文件自 cwd 向上逐层合并（最近优先）；enforce:false 干跑审计；Chokidar HMR（编辑损坏保留旧规则）；每次判定写入 permissionRules/decision 审计事件（log-only 不进模型上下文）+ `/rules list|reload|decisions|test`；内置 HTTP/CONNECT 本地代理 + Codex 式网络策略（deny-all/whitelist/allow-all/auto 对应官方沙箱预设）。与 dsh-auto-review 组成「decision→asked→verdict→decided」完整闭环。
- **数据**：★42，npm `dsh-permission-rules` 1,141/wk，Apache-2.0，Node≥22.19。来源：github.com/PerryLink/dsh-permission-rules

#### dsh-auto-review（PerryLink）——第二模型自动评审审批请求，fail-closed

- **能做什么用**：审批请求触发只读 reviewer 子代理（一次性 fork，工具白名单 read/glob/grep，身份可识别再委托、maxDepth 防递委）返回结构化 {decision, reason, riskLevel}；per-tool 策略（ai/human/never）+ regex 风险规则；reviewer 崩溃/超时/schema 错 → 默认 rejected（fail-closed）；deny 理由注回被拒 tool result 让主模型学会不盲重试；熔断器（连续 3 deny 或 10 中 6 转人类）、一次性 `/auto-review approve` 覆盖；**审计链完整可重建**：approval/asked → autoReview/verdict → approval/decided，全部可从会话日志重放。默认只 AI 审 bash 与 write。
- **数据**：★106，npm `dsh-auto-review` 1,302/wk，Apache-2.0。来源：github.com/PerryLink/dsh-auto-review

#### dsh-session-manager（dream12347）——会话治理驾驶舱

- **能做什么用**：删除（回收站恢复/彻底清除，禁删「正在思考」的会话）、每会话统计（轮次/用户/助手/工具调用/活动窗口）、继续/暂停运行中会话、打开日志目录抽屉、**上下文压缩阈值 17%–90% 全局统一并持久化**（对成本治理直接有用）、工作区拖拽排序/分组、未读状态点同步官方侧边栏、新聊天 fork 继续。
- **数据**：★49，npm `dsh-session-manager` 1,326/wk。来源：github.com/dream12347/dsh-session-manager

#### 第二轮新录治理件（1024Store 发现，各 2-4 行）

| 插件（owner） | 能做什么用 | 数据（2026-08-26） |
| --- | --- | --- |
| dsh-approve-for-me（timeance） | 规则门控 + 可选无工具 LLM 复核的沙箱升级自动审批：固定高危清单 + 字面前缀规则（git status 等，复合命令逐段匹配）；拿不准回人类原生弹窗；**每次成功只授 allowed-once，绝不授永久**；默认 commandPrefixes 为空 = 装完不自动批任何东西 | ★11，npm 933/wk（v0.2.2，MIT） |
| dsh-auto-approve（Jiao-XXX） | 在 workspace-write 与 danger-full-access 之间插 `auto` 权限档：分类模型对例行升级做 approve/ask；内置确定性危险清单（rm -rf 变体/设备写/force-push/下载即执行/破坏性 SQL/关机/递归 chmod 777/fork 炸弹/Terraform 销毁/$()`<()` 混淆）**LLM 无法推翻**；超时/拿不准转人工；真实用户最新消息可作授权证据（≤2000 字符） | ★11，npm 597/wk（v0.5.1，MIT） |
| dsh-full-remote（JUANWANG-BUAA） | 鉴权反向代理（127.0.0.1:3081）让 DSH Web 经隧道/局域网远程可用：重写 Host/Origin 使 settings/credentials 等特权 API 不再 403；192-bit 访问令牌（0600 状态文件）+ 一次性邀请 + 按设备会话（登录生成独立凭据只存哈希，可重命名/吊销/IP 显示）+ 新设备首访审批；可挂 ssh/frp/ngrok/Tailscale。**场景**：手机远程确认、异地访问自己电脑上的 agent | ★24，npm 2,036/wk（v0.3.7，MIT） |
| dsh-win32（sjh9714） | Windows 原生治理：检测官方 Windows 包契约/PowerShell 7/坏 koffi 运行时并 safe-repair；无 key 的真实验收（隔离临时家目录跑物理 pwsh 验证「受限 shell 写入被拒」）；busybox-w32/Git Bash 为 --legacy 路径 | ★20，npm 3,628/wk（v0.17.0，MIT） |

#### 多租户 / 部门 / 计费分类普查（第二轮结论，回应「没有多用户配额、部门额度、计费网关、账单/发票」旧结论）

1024Store 目录（10,681 条）关键词命中：billing=24、quota=42、budget=27、invoice=4、tenant=3、**department=0**、multiuser=1、enterprise=7、team=19。最接近企业计费/多用户的条目：

| 插件 | 能做什么用 | 数据 |
| --- | --- | --- |
| dsh-multi-tenant（GuoMonth） | 租户身份、会话隔离、授权控制、租户感知 MCP 与审计（第三轮仓库描述确认五能力）——**定位最接近 SaaS 化扩展的早期件** | ★7，npm 未发布，MIT |
| dsh-accounts（kangshifu1） | PostgreSQL 后端多租户认证、admin 管理、按用户工作区隔离 | ★1（license 字段空），npm 86/wk |
| dsh-multi-user（nabin-qq273274877） | 单进程多用户工作区视图分区 | ★0，npm 149/wk |
| dsh-gov（863683348） | 策略化工具门控（allow/deny/ask）、JSONL 审计链、按 agent token 配额——自我定位「enterprise companion」 | ★1，npm 51/wk |
| dsh-budget-guard（haoku123） | 成本计量 + 预算**硬拦截（deny/warn）** + 峰谷定价 | ★2（license 字段空），npm **674/wk**（第三轮重拉，较第二轮 335 翻倍） |

**修订结论**：「部门额度（department=0）、计费网关、发票账单」依然空缺（invoice 命中的 4 款全部是发票 PDF 解析/报销/报关的个人工具，与计费无关）；但「没有多用户配额/多租户」**已不成立**——出现 4 个多用户/租户级治理件 + 1 个预算硬拦截，不过全部是 npm 未发布或周下载 <400 的早期开源件，企业级成熟度与审计尚无任何一款达标。

治理类插件普遍年轻（多为 8 月新发布、star 个位数），「进入精选列表不代表完成安全审查」（dshmarket 原文警示；插件以机器权限运行），采购或集成前需自行 review。

### 3.4 网关 / 分发 / 发行形态

| 项目 | 硬数据 | 说明 |
| --- | --- | --- |
| `HarnessRouter/harnessrouter` | ★593（Apache-2.0，2026-08-25 活跃） | 自托管容器：把 Codex/Claude Code/Hermes/Pi/DSH 统一暴露为本机 UHP API + 控制台（网关层）。第三轮补全定位：实现开放标准 **Unified Harness Protocol (UHP)**——会话、流式、文件、取消、失败处理统一接口，「Your keys, your infrastructure」，即企业把多 agent 接入自有网关时的协议层候选 |
| **`anywhere-labs/dsh-desktop`（本仓库直系上游）** | **★20,174**；12,906 commits（master）；MIT；最新 release **v2.0.2**（2026-08-21） | 【第三轮重点核实】产品形态：把 dsh 的本地 Web UI、Host 服务和插件系统集成进原生桌面应用，**git submodule 固定内核 commit `b150a55` 原样运行**（= 本仓库 pin），自己提供窗口、托盘、终端、更新器与工作配置。发行：官网 www.dshdesktop.cn 分发 Windows x64 NSIS 安装包 + macOS Universal DMG，一键下载开箱即用。商业化信号：README 带 UCloud 星图 AstraFlow 赞助商位 + Discord 社区。合规姿态：明示「独立社区开源项目，与深度求索不存在隶属、合作、授权或背书关系」。仓库内含 `dsh-plugin-desktop` / `dsh-community-market` / `dsh-community-fabric` 子目录，同时以 npm `dsh-plugin-desktop@2.0.0` 上架 1024Store（installs30d=53 / failureCount=254，2026-08-25）。「桌面本身也是插件」+ 独立官网 + 赞助 = 既有产品叙事又有运营实体 |
| `dataelement/dsh-desktop` | ★2,466（MIT） | **与本仓库同赛道的桌面发行版**：DSHDesktop——把本地 Harness web 体验打包为桌面应用，自动拉起本地 Harness 实例并管理随机端口，免装 Node.js。产品定位前必须对照 |
| `zouyuxuan122/Deepseek-Harness-EAC` | ★1,304【第三轮新发现】 | EAC「揽尽万象」（Embracing All Creation）：捆绑 Node.js 运行时 + 完整 dsh CLI 内核的桌面端，一键启动、10 套内置 UI 主题；与 CLI 共享 DSH_HOME 会话与 API Key，桌面用独立 web-desktop profile 互不干扰，agent 与客户端分离自动更新 |
| `dsh-market/dsh-market` | ★2,369 | 插件市场（dshmarket.com 同项目，收录 2,143 个）；与本仓库生态位直接相关 |
| `lencx/Minke` | ★519（Apache-2.0） | 同赛道桌面壳（dshfind「Clients」类收录） |
| `ChisaAlter/Deepseek-Harness-Desktop` | ★140 | 同赛道桌面壳，主打主题/背景图个性化配置（其旗下 dsh-usage-panel 已 404，见 §5） |
| `FlashingChen/dsh-desktop-hub` | ★53（Electron + TypeScript） | DSH Desktop Hub：多 Tab 管理 Harness/Plugin/MCP/Skills 的桌面控制台，双击即用 |
| `csthinker/deepseek-harness-desktop` | ★1【第三轮新发现】 | Windows 自包含壳：双击 exe 在原生窗口打开 DSH Web UI，无需 npx 命令 |
| `dsh-web`（Tom6814） | — | Docker 一键部署完整 Web 界面 + 工作区 + 插件市场（自托管形态） |

> 口径注：linux.do 社区帖（2026-08）在用户询问「desktop 版是官方出的吗」时得到确认——**官方目前没有桌面版，现有桌面版全部为第三方**；上游 README 的免责声明与此一致。

### 3.5 安装可靠性观察（1024Store 独有数据，2026-08-26 重核）

1024Store 的开放 API 公开每个插件的安装计数与失败计数（`failureCount` 为全历史累计、语义未公开），这是其他目录没有的运维级信号。本轮重核头部插件原始值（与第一轮一致）：

| 插件 | installs30d | failureCount（全史） | verified |
| --- | --- | --- | --- |
| volcengine OpenViking dsh-memory-plugin | 143 | 344 | 全部安装方式 verified |
| dsh-agent-teams | 27 | 10 | npm/github 双源 verified |
| dsh-cost-meter | 21 | 15 | verified |
| dsh-context | 20 | 9 | 含 unverified 方式 |
| dsh-im（@xmanrui） | 18 | 10 | verified |
| **anywhere-labs dsh-plugin-desktop（本仓库上游）** | 53 | 254 | verified（npm 双检） |

结论只做事实陈述：**头部插件的失败计数与成功安装同量级**，说明「一键安装」在小样本下并不总是成功；企业分发或内网镜像时应把首次安装失败作为预期内流程，而不是当异常。不做跨插件成功率比较（分母口径未公开）。本仓库若沿用上游的插件化分发，**上游自己 254/53 的失败比就是第一手参照物**。

## 4. 办公场景插件体系（对照 WorkBuddy / TRAE Work）

### 4.1 行业背景（第三方权威数据）

- WorkBuddy（腾讯）：2026 年 3 月发布，6 月 PC 端月访问 2,097 万，易观《2026 Q2 中国办公智能体平台市场洞察报告》17 款桌面办公智能体中排名第一（超过第二、三名之和）；计费为「积分+会员」；聚合 DeepSeek/混元/GLM/Kimi 多模型；微信生态分发。（来源：36氪/定焦《腾讯、阿里、字节，大战 AI 办公》，2026-08-06，转引易观报告数据）
- TRAE Work（字节）：从 TRAE IDE 升级做「编码+办公」工作智能体，业内评价「做得一般、定位尴尬」，字节方向转向飞书+豆包深度整合。（来源：同上 36氪文章访谈原话）
- 市场规模：2025 年中国 AI 智能体市场 804 亿元、同比 +123.2%，预计 2030 年 6,968 亿元（艾媒《2026 年中国 AI 办公智能体产业发展白皮书》，经 36氪引用）。
- 形态差异：WorkBuddy 类 = 面向非技术岗的**闭环办公工作台**（文档/表格/PPT/邮件/桌面自动化，深度绑微信/企微）；dsh = 开源 Agent 底座，办公能力靠插件拼。业内已有声音认为 dsh 开源会冲击 WorkBuddy 类产品（AIGC 连线，2026；需注意是观点不是数据）。

### 4.2 dsh 侧能力拼图（逐项带插件与来源）

> 第二轮状态：本表 star/npm 已全部重核（2026-08-26），工具清单按 GitHub API 仓库描述更新；WPS/jdon 评语沿用第一轮核验。**第三轮（同日）已把本表所有有 GitHub 仓库的条目逐一过 API 复核并补齐 star/license，腾讯 browserskill 归属更正见下。**

| 办公能力 | 插件/来源 | 硬数据 | 能做什么用（含新检定） |
| --- | --- | --- | --- |
| 文档/表格/PPT/PDF 生成编辑 | `didclawapp-ai/DSH-Office` | ★5，MIT | **WPS 官方论坛对比评测**（bbs.wps.cn/topic/95386，2026-08-21）：基于 zagens-office 引擎，4 工具（schema/write/edit/read）契约驱动；仓库描述「PPTX / DOCX / XLSX / PDF read/write/edit」 |
| 同上（工具最全） | `@huiliyi37/dsh-office` | npm 1,141/wk；Apache-2.0；需 DSH ≥ 0.1.0-rc.5 | WPS 官方论坛评测：从天枢终端编码 Agent 移植，16 工具（XLSX×5 含公式重算+公式审计 6 类静态检测 / PDF×4 创建合并拆分 / PPTX×3 / DOCX×2）；npm 分发，无公开仓库 |
| 同上（Excel 精准） | `jiazekang/dsh-office`（npm `dsh-office`） | ★0，npm 1,242/wk，MIT | WPS 官方论坛评测：基于 exceljs；仓库描述「可以读写办公文件，支持拖动办公文件到对话框」（拖入附件栏） |
| 【新】Univer 引擎办公套件 | `dsh-univer-office`（dream-num） | **★129，Apache-2.0**（第三轮 GitHub API 坐实仓库存在且活跃 08-25）；1024Store npm7d=5,116 | 基于 Univer 引擎：在 dsh 内**预览、创建、编辑**电子表格/文档/演示文稿（仓库描述 "Preview, create, edit office spreadsheets, docs & slides inside DeepSeek Harness. Power by Univer"）——四款文档插件中唯一带「预览」与可视化引擎的；dream-num 即 Univer 官方团队，属上游厂商亲自下场 |
| 【新】办公三件套 | `dsh-office`（omdsh-dev） | ★16，Apache-2.0；1024Store npm7d=1,179 | 「办公三件套」：生成/读取/编辑 xlsx/PDF/pptx 工具（该 npm 名与 jiazekang 的 `dsh-office` 同名，安装时需按 owner 区分） |
| 同上（第四款） | `dsh-office-tools`（vibeinging） | npm 1,548/wk；**GitHub 仓库本轮已 404**（仅 npm 包仍存活，1024Store 条目 verified 存疑） | 1024Store 描述：创建与编辑 Office 文档的工具集；未包含在 WPS 论坛评测内。**替代同名仓库**：`kw78/dsh-office-tools`（★9）「面向模型的 Word/Excel/PPT 工具」 |
| 表格对话式处理 | `dsh-excel-chat`（hccccc01333） | ★6，npm 679/wk | jdon 生态文章点名；仓库描述：talk to Excel——create/edit/repair/verify spreadsheets（对话创建/修改/修复/验证） |
| 邮件 | `dsh-email`（STARDUSTLC666） | ★9，npm 440/wk，MIT | 读取/发送邮件：email_list/read/search/send/folders/attachment 六工具；**预设已从 7 家扩到 8 家（+iCloud）**：QQ/163/126/新浪/阿里/Gmail/Outlook/iCloud；多账号、附件收发、Web 设置页、纯 Node 全平台（jdon 生态文章点名） |
| 日历 | `dsh-calendar`（STARDUSTLC666） | ★3，npm 349/wk，MIT | 操作 CalDAV 日历：calendar_list/create/update/delete/search 五工具，支持 Google/iCloud/Nextcloud/自定义端点，**RRULE 重复事件自动展开**、插件级代理、配置缺失不崩启动（第三轮仓库描述补全）（jdon 生态文章点名） |
| IM 网关（飞书/微信/钉钉/企微/QQ/Slack/Telegram/Discord/WhatsApp） | `dsh-im`（xmanrui） | ★848，npm `@xmanrui/dsh-im` 10,826/wk，MIT | 扫码或机器人凭据接入 9 类 IM；多机器人、会话绑定、远程审批（dshmarket + awesome 清单 + 1024Store verified） |
| 【新】聚合 IM 网关 | `dsh-im-gateway`（zhuiyueya） | **★40**，MIT（第三轮坐实）；1024Store npm7d=960 | 把 dsh agent 接入微信、飞书等 20+ 聊天平台的聚合网关（与 dsh-im 的 9 通道互补） |
| 飞书深度桥 | `dsh-lark-bot`（PlutoKeating） | ★34，AGPL-3.0，npm 4,952/wk | 飞书扫码即用：流式卡片、项目工作区、并行任务、多角色 Agent、跨会话通知、**对话内模型/密钥管理**、安全网守护（dsh 崩溃后飞书侧继续应答） |
| 【新】飞书 IM 通道 | `dsh-lark`（omdsh-dev） | **★46，BSD-3-Clause**（第三轮坐实）；1024Store npm7d=1,521 | dsh 的 Lark/飞书 IM bot 通道（与 lark-bot 的深桥定位不同，属纯通道） |
| 通知与远程审批 | `dsh-notifier`（THEWOLFWALKER） | ★71，npm 2,367/wk | 27 通道 + 手机反向审批/命令中心（详见 §3.3 条目） |
| macOS 桌面自动化 | `dsh-computer-use`（Anionex） | ★31，MIT，npm 1,119/wk | Accessibility-first 的 macOS 电脑控制：新鲜观测、过期状态拒绝、作用域权限与安全输入（jdon 生态文章点名「把电脑操作扩展到 macOS」） |
| 【新】文本优先桌面/浏览器控制 | `dsh-computer-use`（ZRui-C，同名不同作者） | **★23，Apache-2.0**（第三轮坐实）；1024Store npm7d=876 | 文本优先的浏览器控制 + macOS 后台控制：**后台操作 Chromium 与 macOS、不抢前台、不移动真实指针**（仓库描述原文），与 Anionex 版功能有重叠，安装按 owner 区分 |
| 浏览器控制 | `dsh-browser-control`（kyo615） | ★6，MIT，npm 179/wk | Playwright 控制真实可见 Chrome，逐动作现场回放，约 80 个工具（jdon 生态文章点名） |
| 【新：腾讯官方浏览器技能】 | `dsh-plugin-browserskill`（Tencent） | 1024Store：installs30d=24、npm7d=1,920 | 【第三轮归属更正】真实仓库 = **`Tencent/BrowserSkill`（★1,321，腾讯官方 org）**，此前猜测的 `Tencent/dsh-plugin-browserskill` 为 404。本体定位：「通过 CLI 与扩展让智能体操控真实登录的浏览器，无干扰地自动化任务」——dsh 插件只是该项目的发布物之一（1024Store id=`Tencent/BrowserSkill/packages/dsh-plugin-browserskill`）。与腾讯云 agentobs 是否同系仍未核实 |
| 数据库 | `dsh-data-agent`（omdsh-dev） | ★167，MIT | 连接数据库写 SQL、对话式数据分析出可执行商业洞察（npm 未发布/无 scope 包，git 直装） |
| 定时/任务看板 | 官方 `schedule` 包 + 社区看板 | — | 官方内置调度能力；`zhu1090093659/dsh-task-board` GitHub 仓库本轮 404（1024Store 条目仍保留 verified 记录）；现存替代：`xuanlanwuta/dsh-task-board`（★3）、`etonny668/dsh-task-board`（★2）、`SLin-code/dsh-task-notice-board`（★3）；社区看板支持 cron（Host 侧执行，关浏览器也生效） |
| 技能体系 | 官方 Skills + `dsh-skill-picker`（a735624258） + `lcthe/dsh-skills-hub` | dsh-skill-picker：1024Store npm7d=883；skills-hub ★2 | dsh-skill-picker 是「WorkBuddy 同款选择 skill 功能」（1024Store 描述原文）；dsh-skills-hub：集中式技能管理——浏览/启停/从 Codex、Claude Code 等导入（本轮定位到 `lcthe/dsh-skills-hub` ★2；另见 `sulfide2085/dsh-skill-manager` ★9、`cheshireez/dsh-skill-hub` ★7） |
| 外部 agent 调度 | `dsh-agent-conductor`（MJorgin） | ★7，MIT | 会话内派发任务给 11 个外部 agent CLI（Codex/Claude Code/TraeCode/OpenCode/Gemini/Cursor/Kimi/Qwen/Copilot/**WorkBuddy**/Grok） |
| 办公监控面板（自用） | `dsh-office`（Fayelin12）★4 / `dsh-office-plugin`（geguanming）★2 | 各 ≤4 | Fayelin12 版：6 列「像素办公室」仪表盘——工作区/会话/token/子代理 + Agent Mail/飞书消息/会议/逐字稿/日志一屏总览；geguanming 版：用 pixi.js 把多 agent 会话实时渲染成办公室场景（趣味向） |

### 4.3 与 WorkBuddy / TRAE Work 的差距（事实性陈述）

对照「能直接对比产品」的清单看，dsh 差在：

1. **没有统一产品层**：以上能力分散在几十个插件里，无「一个桌面工作台」成品；安装、配置、权限审查是使用者自己的责任。
2. **缺邮件客户端级集成**：`dsh-email` 可发可读（六工具），但没有收件箱视图/邮件流工作流。
3. **IM 是网关不是深度耦合**：dsh-im/dsh-im-gateway 是协议桥接；WorkBuddy 的微信/企微是「手机指挥电脑 + 微信发文件给同事」的闭环分发，这个生态红利 dsh 没有对应物。
4. **桌面自动化仅有 macOS 社区插件**（`dsh-computer-use` ★31、`dsh-mobile-gui-agent` ★9、`dsh-adb` ★2、`dsh-hdc-bridge`），成熟度低、无官方维护，与 WorkBuddy 的桌面自动化不可同日而语。
5. **面向用户的交付对象不同**：WorkBuddy 交付「干完活」给白领；dsh 交付「可控的 Agent 执行引擎」给开发者/平台团队——这不是同层产品，用它做 WorkBuddy 平替需要自建全部产品层。

结论：**dsh 具备拼成办公场景的底座（文档/表格/PPT/邮件/日历/IM/定时/技能/桌面控制全有插件），但没有 WorkBuddy/TRAE Work 那样的办公智能体成品**；若本仓库要在办公方向做产品，dsh-office 系列（WPS 官方评测覆盖的三款 + 新品 Univer 款）+ dsh-im/dsh-lark-bot + 官方 schedule 是现成底座，产品层与闭环要自己造。

### 4.4 生态头部新面孔（第二轮按 1024Store 目录新录，2026-08-25 数据）

第一轮未覆盖、本轮在 1024Store 头部看到的新插件（仅列有验证数据者，定位取自其 API 描述原文）：

| 插件（owner/项目） | npm7d | 能做什么用 |
| --- | --- | --- |
| DSH-better-sidebar（omdsh-dev） | 78,284 | Web UI 侧边栏增强（1024Store 周下载第一梯队） |
| dsh-vision-router（ysr666） | 30,009 | 视觉/多模态路由 |
| dsh-vision-toolkit（Anionex） | 22,932 | 视觉工具包（dshfind 评分榜第一，zhihu 实测文） |
| dsh-TUI（ccch1mneyyy） | 25,277 | 终端 TUI 客户端 |
| modlens（liustack） | 75,686 | 【第四轮更正】**视觉桥插件**（★3,665，MIT）：粘贴图片得结构化 OCR/版面/语义 JSON 证据——此前按 1024Store 描述误录为「模块/包透镜工具」；同作者的 modsearch（★278）为联网搜索桥，详见 §7.2 |
| coding-agents（vectorize-io） | 17,324 | 批量编码 agent 编排 |
| deepseek-harness-acp（openma-ai） | 6,519 | ACP profile 插件 + 独立 stdio server：把完整 DSH 暴露给支持 ACP 的客户端 |
| dsh-import-agents（Chang-Tong） | 715 | 导入 pi/opencode/codex/claude-code 的会话与聊天历史（迁移工具，与本仓库形态有关） |
| dsh-strata（jsdvjx） | 6,214 | 会话分层浏览（transcript scroller） |
| dsh-ai4scholar（literaf） | —（部分收录） | 38 个学术工具：Semantic Scholar 等科研检索/写作链 |
| dsh-docs（Sqhao-O） | 842 | 本地文档智能：解析 PDF/Office、本地 RAG |
| dsh-oomol（oomol-lab） | 634 | OOMOL 连接器：发现并执行已连接 app 的能力 |
| dsh-imagegen / dsh-image-gen | 2,911 / 1,602 | 多 provider 图像生成 |

## 5. 插件硬数据总表（2026-08-26 第二轮全量重核）

| 插件 | GitHub | ★ | npm 周下载 | 评测/认证来源 |
| --- | --- | --- | --- | --- |
| dsh-context | bowenliang123 | 1,029 | 27,812 | dshmarket Most installed；Apache-2.0 |
| dsh-cost-meter | Han-1413141 | 193 | 17,152 | dshmarket |
| dsh-agent-teams | NanmiCoder | 1,006 | 19,057（dshmarket）/ 21,210（1024Store） | dshmarket |
| dsh-im | xmanrui | 848 | `@xmanrui/dsh-im` 10,826 | dshmarket + awesome 清单 + 1024Store verified |
| dsh-lark-bot | PlutoKeating | 34 | 4,952 | dshmarket；AGPL-3.0 |
| dsh-notifier | THEWOLFWALKER | 71 | 2,367 | dshmarket + awesome 清单；**27 通道** |
| dsh-whale-report | SenmuuuuW | 29 | 328 | awesome 清单；MIT |
| dsh-office（jiazekang） | jiazekang | 0 | 1,242 | **WPS 官方论坛** |
| @huiliyi37/dsh-office | 无公开仓库 | — | 1,141 | **WPS 官方论坛**，Apache-2.0 |
| DSH-Office（didclawapp） | didclawapp-ai | 5 | —（GitHub 安装） | **WPS 官方论坛** |
| dsh-office-tools（vibeinging） | **仓库 404** | 旧值 632 不可核实 | 1,548（npm 仍存活） | 1024Store；未过 WPS 评测；同名替代 kw78 ★9 |
| dsh-univer-office | dream-num | **129** | 5,116（1024Store 窗口） | 1024Store；第三轮 GitHub API 坐实（Apache-2.0）；Univer 官方团队出品 |
| dsh-computer-use | Anionex | 31 | 1,119 | jdon 生态文章；MIT |
| dsh-excel-chat | hccccc01333 | 6 | 679 | jdon 生态文章 |
| dsh-email / dsh-calendar | STARDUSTLC666 | 9 / 3 | 440 / 349 | jdon 生态文章；MIT |
| dsh-data-agent | omdsh-dev | 167 | —（npm 未发布） | jdon 生态文章 |
| dsh-agent-conductor | MJorgin | 7 | — | jdon 生态文章 |
| dsh-usage-plugin | feiyang-dev | 35 | **1,936（真实包名 `@feiyang666/dsh-usage-plugin`）** | 1024Store（旧口径 1,882）；CSV/JSON/PNG 导出 |
| TokenLedger | zh667 | 152 | **1,181（真实包名 `dsh-tokenledger`；查 `tokenledger` 只得 24 是占位无关包）** | 1024Store（旧口径 1,242） |
| dsh-usage-panel | **ChisaAlter 仓库已 404** | **★139 不可核实，删除** | npm `dsh-usage-panel` 238（**已被 AlfredChaos/dsh-usage-panel 接管**，全新实现：半年热力图/按模型堆叠柱/导出） | 1024Store 条目描述错位（描述的是其桌面壳 Deepseek-Harness-Desktop），数据存疑 |
| dsh-usage-stats | Ychris12138 | 122 | `@ychris12138/dsh-usage-stats`（1024Store 1,638）；无 scope 的 `dsh-usage-stats` 1,956 是另一作者 lanlandeli 的同名包 | 1024Store；余额/订阅配额/token 热力图 |
| dsh-permission-rules | PerryLink | **42** | 1,141 | 1024Store；Apache-2.0 |
| dsh-auto-review | PerryLink | **106** | 1,302 | 1024Store；Apache-2.0 |
| dsh-session-manager | dream12347 | 49 | 1,326 | 1024Store |
| dsh-memory-plugin（火山官方） | volcengine/OpenViking | 33,243（OpenViking 本体，AGPL-3.0） | 4,906 | 1024Store；installs30d=143 |
| HOL Guard | hashgraph-online | 476（+hol-guard-plugin 4） | —（pipx 分发） | **本轮更正：确为 DSH 插件（hol-guard-plugin）+ 插件 CI 扫描** |
| upstream-radar | MicroMilo | 8 | 4,617 | awesome 清单；Apache-2.0；兼容性实证平台 |
| control-center | feibi-mochi | 65 | 6,961（npm 名 `deepseek-harness-wallet`） | dshmarket |

口径说明：npm 周下载 = npm registry `downloads/point/last-week`（2026-08-18~24）；1024Store 的 `npmDownloads7d` 为其自有窗口（08-17~23），两处已分别标注；dshmarket 的安装量口径含 GitHub 拉取，不可与 npm 直接比较；**scoped 包名易错**——本轮再次踩中并修正：`dsh-usage-plugin` 真实包名 `@feiyang666/dsh-usage-plugin`、`TokenLedger` 真实包名 `dsh-tokenledger`、`dsh-usage-stats` 无 scope 名属于另一作者、`dsh-office`/`dsh-computer-use` 各有两个作者同名。star 为 GitHub API/HTML 当日值。许可证：仅少数插件明确标注（Apache-2.0：dsh-context、@huiliyi37/dsh-office、dsh-permission-rules、dsh-auto-review、upstream-radar、HarnessRouter、Minke、loongsuite；AGPL-3.0：OpenViking、dsh-lark-bot；其余以 MIT 居多但未逐一证实），集成前需逐个核对（dshmarket 已对此给出安全警示）。

## 6. 给本仓库的参考（按优先级，不排期）

- **P0（做产品定位前必看，第三轮后紧迫度上升）**：上游 `anywhere-labs/dsh-desktop` 已是 **★20,174、12,906 commits** 的社区头部项目：独立官网分发安装包、UCloud 赞助、「桌面本身也是插件」的既定叙事，且 submodule 固定的内核 commit 与本仓库 pin 一致——它事实上就是「Electron 版 DSH Desktop」的标准答案，社区用户问「官方桌面版」时被指向的就是这类第三方壳（linux.do）。同赛道还有 `dataelement/dsh-desktop`（★2,466）、EAC（★1,304，捆绑内核一键启动+10 主题）、Minke（★519）、dsh-market（★2,369，市场侧）、hub（★53）等。**本仓库 Tauri 改写必须回答：相对上游的差异点是什么（性能/体积/原生窗口体验/安全边界），以及叙事是继承、修正还是另起**——仅「把 Electron 换成 Tauri」在 ★2 万面前不构成独立产品理由。
- **P0（若有企业客户）**：企业「管控面」不自己造，优先评估 AgentKit（唯一有完整文档+实操的企业承载；第三轮已核验其 10 大模块能力全景，见 §3.1）；自建则按 §3.3 组合包（推荐起点：dsh-permission-rules + dsh-auto-review 审批链、HOL Guard 插件 + plugin-scanner/DShScan 供应链、dsh-whale-report 对账、sandbox-micro/k8e 沙箱）且接受其安全附带责任（治理插件普遍年轻，star 个位数到十位数；本轮还发现个别安全插件 license 字段缺失需人工复核）。火山在 dsh 生态还有官方插件动作（OpenViking 记忆插件 ★33k 项目背书），腾讯以 BrowserSkill 本体（★1,321）下场浏览器控制，说明大厂在持续投入。
- **P1（办公方向）**：底座用 dsh-office 系列（WPS 评测三件套 + Univer 款——第三轮已坐实 `dream-num/dsh-univer-office` ★129 且为 Univer 官方团队出品，可信度高于第二轮的「仅 1024Store 描述」）+ dsh-im/dsh-lark-bot + 官方 schedule；产品层（收件箱、日历视图、IM 深度协作）没有现成插件，需自研——工作量即门槛。注意 `dsh-office-tools`（vibeinging）仓库已 404，四款候选须重新评估；腾讯 BrowserSkill 本体 ★1,321 的出现说明平台方也在下场做办公/浏览器能力。
- **P1（计费方向）**：市场没有企业计费成品，但需求信号真实存在（TokenLedger ★152、dsh-usage-* 多款、1024Store installs30d 可作需求代理指标）；`dsh-usage-plugin` 是唯一支持 CSV/JSON/PNG 导出的，离「账单」仍差一步。多租户/配额类已出现 4 个早期开源件（§3.3 普查），都是蓝海信号但全新建造。
- **P2（质量与分发）**：1024Store 开放 API 提供 `verified`/`failureCount` 字段，可作插件的程序化 pre-flight 校验（§3.5）；头部插件失败计数与成功安装同量级，分发端要把首次安装失败当预期流程；`upstream-radar` 的「精确产物 + 一次性 VM 实证」模式可作本仓库插件的 CI 兼容性防线。
- **P1/P2（盲区赛道取舍）**：第四轮补扫出的八条赛道对本仓库的含义见 §7.11——要点：工作台基座协议（dock 的 ctx.workbench）与「离线救砖模式」（dsh-undo-savepoint）是桌面壳最值得吸收的两个思路；记忆/搜索/视觉做接入层不自研。

## 7. 盲区补扫：前三轮未覆盖的项目（第四轮，2026-08-26）

> 背景：GitHub topic `dsh-plugin` 下公开仓库在 2026-08-15 已超 700 个（量子位统计口径），本文前三轮只覆盖了治理/办公/客户端/云平台四条线。本轮按类别补扫，只录有 GitHub 仓库或 1024Store 数据佐证者。

### 7.1 记忆系统（最大遗漏——已有大厂级玩家）

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `vectorize-io/hindsight`（1024Store 包名 coding-agents） | **★21,130**；插件 npm7d=18,072、installs30d=50 | 「可学习的 Agent 记忆」：自动召回与沉淀的长期项目记忆、知识页、深度反思、按仓库隔离的记忆库——即给 Agent 装一个会自己总结经验教训的大脑。同 org 还有 self-driving-agents ★2,731 与 agent-memory-benchmark |
| `MemTensor/MemOS`（插件 memos-local-plugin） | **★10,985**，Apache-2.0；npm7d=2,318 | 「LLM 记忆操作系统」：持久化自进化记忆 + 混合检索 + 跨任务技能复用，官方声明支持 DSH 并宣称节省 35.24% token——学术系团队下场，定位是记忆基础设施 |
| `zilliztech/memsearch` | ★2,511，MIT；npm7d=630 | Milvus 官方出品：以 Markdown + Milvus 为底的统一记忆层，**跨 Claude Code/Codex/DSH 共享同一份记忆**；自动捕获、步骤前上下文注入、搜索召回、审阅面板——「换工具不丢记忆」是其差异化 |
| `csyangwen/dsh-memory-evolve` | ★247 | 纯插件实现的跨会话长期记忆：五轨记忆（项目约定/架构决策/踩坑/进度等）、Git 分支感知、回合内自我审查、技能自我进化与管理器、四轨待办、COI 调度、会话广播与搜索、提示词管理器；零核心修改随装随用。量子位报道点名的代表作品 |
| LMA 分层记忆架构（szx-a） | ★2，概念期 | 把「记忆体」做成可挂载/卸载的容器协议，让 memory-evolve、EchoCore 等能力插件挂载其上——试图终结记忆插件各自为政；早期但思路值得跟踪 |

### 7.2 联网搜索 / 视觉桥（纯文本模型的外挂感官）

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `liustack/modsearch` | ★278，MIT；npm7d=12,070 | 免费免注册免 API key 的联网搜索桥：问网页或 X，返回结构化 JSON 证据（搜索/抓取/引用），给不能联网的模型补上实时信息——下载量第一梯队的刚需件 |
| `liustack/modlens` | **★3,665**，MIT；npm7d=75,686（§4.4 旧口径） | 【更正】不是「模块透镜」而是**视觉桥**：粘贴图片即得结构化 JSON 证据（OCR、版面、实体、语义），为 DeepSeek/GLM 等纯文本模型外挂视觉；自称「第一个 DSH 视觉插件」 |
| anysearch-dsh（anysearch-team） | npm7d=2,001 | 常规网络搜索 + 高级搜索工具集，modsearch 之外的另一选择 |

### 7.3 多智能体协作

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `NanmiCoder/dsh-agent-teams` | **★1,020**，MIT；npm7d=21,243、installs30d=29（1024Store 双源 verified） | 会话内一句话拉起多 Agent 团队：当前 Agent 升任 Captain（队长），自动拆任务、设依赖、子 Agent 间可直接互发消息，Web 右上角实时围观谁在干活/谁空闲（量子位报道）。生态头部插件中除 better-sidebar 外下载量最高者，但本文此前只在总表出现过一次，未展开 |

### 7.4 工作台 IDE 化（对桌面端产品形态最有参考价值的一组）

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `AKS1st/dock` + dock-files/dock-editor/dock-git | ★2（新发布） | **VSCode 风格工作台基座**：活动栏/侧边栏/编辑器区/面板/状态栏五区布局外壳 + `ctx.workbench` 开放注册表——任何插件都能往基座里注册面板，「功能插件即插即用」。这是插件间 UI 组合协议的雏形，Tauri 桌面端若做可扩展工作台应直接对接该注册表而非另造 |
| DSH-better-sidebar（omdsh-dev 发行；GitHub 镜像 zhu1090093659 名下） | npm7d=78,284（全生态第一） | 侧边栏塞进文件管理/代码编辑/真终端/Git 面板/后台任务/子 Agent——「少切窗口」的迷你 IDE 化（量子位专题介绍） |
| `FSMargoo/dsh-at-file` | ★474 | Codex 式 @file：输入框里搜工作区文件、把路径附进 Prompt——高频刚需小件 |
| `ZhangFengshun/dsh-remote-ssh` | ★7，MIT；npm7d=4,753 | VSCode Remote-SSH 式远程开发：SSH 连远端工作区 + 文件浏览器 + 集成终端 |
| DSH_VsCodeMode / dsh-review / dsh-statusbar / dsh-vscode-theme | npm7d=2,198 / — / — / — | VS Code 形态模仿系列：网页界面切 VS Code 模式；diff 逐行批准拒绝（在 VS Code 里审 dsh 改动）；Token/CPU/内存状态栏；加载 .vsix 主题 |

### 7.5 移动 / 远程 / Human-in-the-loop

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `agentrq/agentrq` | **★1,089** | 自托管「真人介入」任务管理器：从手机/网页/桌面实时控制自己的 agent 军团，设计为配合自有 Claude 订阅与任意 harness 通吃；配套 ACP Gateway（★11）、Codex Gateway、Claude/Gemini 扩展——**跨 harness 任务指挥台**，与本仓库 host 架构同层 |
| `saya-ch/dsh-mobile` | ★144，Apache-2.0；npm7d=4,378 | DSH 移动端适配与安全访问：局域网/远程连接 + Android App + 手机浏览器认证访问 |
| dsh-aionui-panel 系列（zhu1090093659） | installs30d=34（疑已并入 dsh-web ★6,074） | 任务看板 + Git 图谱 + 右侧面板 + 移动远程 UI + 宠物 + 实时 Token 统计 + 皮肤中心的聚合 Web 插件包（「创意工坊」分发模式） |

### 7.6 可靠性与崩溃恢复（桌面壳应借鉴）

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `lire1131/dsh-undo-savepoint` | ★127 | DSH 崩溃救援：配置与插件代码变更一键 undo、secret-safe 快照（密钥不入快照）、一键 SAFE MODE、**DSH 起不来也能用的离线 CLI/GUI**——桌面发行版内置「救砖模式」的直接参照物 |
| Renzic-Stone/DSH-EasyRewrite | ★78 | Web 内最无感的消息撤回/重编辑，原版体验兼容性强 |
| Rianico/dsh-better-edit | ★18 | hash 锚定的读/编辑/撤销最后一次编辑工具链，省 token 降成本 |
| humblebanana/dsh-record-replay 等 | ★11 | macOS 桌面操作「演示一遍→沉淀为 agent skill」（RPA 式示教）；另有 VCR 式确定性录制回放 subagent 的 dsh-subagent-cassette |

### 7.7 语音交互（全新空白类别，本轮新发现）

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `leaveimagination/dsh-qwen-voice` | ★2（新发布） | Qwen Audio Agent 作实时语音引擎：语音控制 + 多会话任务派发 + DSH 插件界面 + ACP 桥接（官方 Discussion #1038 自荐） |
| `WizisCool/dsh-ears` | ★11，MIT；npm7d=945 | 多 ASR 后端语音输入 + 经 dsh 自己的 LLM 路由润色后再入 Prompt |
| dsh-speak（Alan2Z）/ dsh-voice-input-plugin / dsh-omi-voice | ★7 / npm7d=235 / 豆包 TTS | 语音播报方案 / 输入+实时转录+流式朗读回复 / 豆包 TTS 朗读——输入、输出、双向三段都有人做，尚无整合成品 |

### 7.8 内容创作与其他值得登记的项目

| 项目 | 硬数据 | 能做什么用 |
| --- | --- | --- |
| `zenstory-ai/oh-story-dsh` | ★180 | 小说写作与短剧制作插件（Oh Story + Drama Skills 技能集）——内容创作类别此前完全空白 |
| `hust-open-atom-club/oh-dsh` | ★279 | 开源原子俱乐部出品：「一套 DSH runtime，Desktop、Web 与 TUI 三种开发体验」——社团背景的多形态发行版，客户端赛道又一对照物 |
| `zhu1090093659/deepseek-pp` | ★1,682 | DeepSeek 网页版浏览器扩展：MCP 工具/记忆/Skills/自动化/联网搜索/对话导出的 AI 工作区——网页侧伴侣产品 |
| zhu1090093659/spec_driven_develop / minister | ★975 / ★142 | spec 驱动开发工作流（架构先行/任务分解/Issue-PR 跟踪）；把 Claude Code 和 Codex 塞进飞书当团队同事 |
| Claude Code 迁移件：dsh-plugin-claude-bridge / dsh-claude-move | 量子位报道 | 搬运 CLAUDE.md/Skills/配置乃至整体旧会话进 DSH、可续聊——「赛博大迁徙」工具对（与 §4.4 的 dsh-import-agents 同类） |

### 7.9 娱乐向子生态（文化现象，简记）

皮肤/桌宠/小游戏已自成一类且星数惊人：`Small-tailqwq/dsh-deep-whale` **鲸鱼娘皮肤系列 ★1,716**（超过绝大多数实用插件）、whale-girl QQ 宠物形态桌宠（npm7d=717）、dsh-whale-pet（晓伊神经网络语音互动桌宠）、lhh010/dsh-minigames ★27（18 款离线小游戏「摸鱼神器」）、Moeblack/deepseek-manners ★15（每条消息后自动说「谢谢你，鲸鱼大人」）、dsh-ads（2005 年中文互联网风格假广告 UI）。**含义：个性化/情感化是社区真实需求，桌面端主题与陪伴能力有数据背书。**

### 7.10 目录与聚合站补充（§0 之后再 +4）

| 项目 | 硬数据 | 说明 |
| --- | --- | --- |
| like-study1/Oh-My-DSH | ★72 | 社区聚合目录：自动同步 dsh-plugin topic，每 4 小时自动维护 |
| Alex-Yanggg/awesome-DSH-plugin | ★83，CC0-1.0 | 精选清单（生产力/功能扩展/调试/自定义开发四类） |
| Dominic789654/awesome-deepseek-harness | ★198 | 分类含 **MCP servers 与 orchestrators 独立类目**——印证 MCP 编排是独立赛道（2026-08-26 仍在更新） |
| jiji262/awesome-deepseek-harness | ★13 | 中英双语精选（插件/桌面客户端/市场/教程） |
| dsh-find-plugin（awesome-dsh-plugin 出品）/ dsh1024（imsai-sh 出品） | npm7d=10,577 / 3,063 | 目录站反向做成插件：会话内直接搜索目录并返回安装命令——分发入口本身插件化 |

### 7.11 盲区补扫对本仓库的含义（按优先级，不排期）

- **P1（产品形态）**：dock 的 `ctx.workbench` 注册表 = 插件间 UI 组合协议的事实雏形；Tauri 桌面端做可扩展工作台时应评估对接该协议。better-sidebar npm7d=78,284 说明「IDE 化侧边栏」是最大单一需求。
- **P1（可靠性）**：dsh-undo-savepoint 的「离线 CLI/GUI 救砖 + secret-safe 快照」模式值得桌面壳原生实现——这是 Electron 上游没有的能力，可作 Tauri 版差异点。
- **P2（集成而非自研）**：记忆赛道已被 hindsight/MemOS/memsearch 三个大项目锁定，桌面端做记忆应做「接入层」不做引擎；modsearch/modlens 证明「外挂感官桥」类需求由独立插件满足得很好。
- **P2（个性化）**：deep-whale 皮肤 ★1,716 > 大多数实用插件的星数——桌面端主题/个性化能力有明确社区需求信号。

## 8. DSH 生态插件向办公 Agent 平台的可改造性评估（第五轮，2026-08-26）

> 问题：这些 dsh 插件，能不能改造成豆包 / WorkBuddy / QwenWork 等办公 agent 的能力？结论先行：**大部分办公向插件的核心逻辑可以改造，但改造单位不是「插件」而是「工具层 + 工作流知识」**——目标平台不吃 Cordis 插件包，吃的是 Skill（SKILL.md 开放标准）和 MCP server。

### 8.1 三个目标平台的扩展机制（决定改造落点）

| 平台 | 归属与状态 | 扩展机制 | 改造落点 | 来源 |
| --- | --- | --- | --- | --- |
| **WorkBuddy** | 腾讯 CodeBuddy 团队；2026-03-09 上线，桌面/主流 IM/小程序全平台，积分+会员计费 | **兼容 OpenClaw 技能（即 Agent Skills 开放标准）+ 内置 MCP 协议**；开放插件有「运行环境/操作行为/开放插件」三层安全审查 | OpenClaw 格式 Skill 直接兼容；工具型能力走 MCP | workbuddy.cn；证券时报（2026-08）；AIHub/AI品台；新华网（2026-04-03） |
| **千问办公 QwenWork** | 阿里钉钉业务线；2026-08 公测（QoderWork + MuleRun + 悟空 三合一），业内首款同时覆盖桌面 Agent / 云端 Agent / 企业协同 Agent | **自定义技能 + MCP 插件 + 技能市场**（技能可保存上传共享）；内置技能体系（qwenwork-guidance 等） | SKILL.md 技能 + MCP 双通道都有官方位 | help.aliyun.com/zh/qwenwork/skills；东方财富（2026-08-03）；AITOP100；七牛云资讯 |
| **豆包电脑版（工作任务模式）** | 字节跳动；专业版订阅制（连续包月 68 元起，2026-06-24 上线）；2026-08-21 上新「技能·连接器·工作伙伴」 | **技能商店（已上架 200+ 标准化技能与连接器）+ 用户自定义技能**（工作步骤/模板固化为可复用流程）+ 连接器打通 Office/飞书等办公软件；支持操作本地电脑/浏览器、定时任务、虚拟桌面 | 自有「技能+连接器」体系（未见公开 MCP 文档，待核）；Skill 概念同源，格式需适配 | doubao.com/work、doubao.com/download；财新（2026-06-24）；21财经/财联社（2026-08-21） |

共同点：三家都把「Skill（技能）」作为第一扩展单位。Agent Skills 规范源自 Claude Code 并作为开放标准发布——同一份 SKILL.md 可在 Claude Code/Codex/Copilot/Cursor/OpenClaw 等十余个 agent 使用（MIT 科技评论中文版访谈、OpenClaw 官方文档），**WorkBuddy 官方兼容的就是这套标准**。豆包的「自定义技能」是自有实现，概念同源但格式需单独适配。

### 8.2 DSH 插件的三层结构：哪一层能搬

一个典型 dsh 插件由三层组成：

1. **工具层**：TypeScript 函数调用通用 npm 库（exceljs/pdf-lib/pptxgenjs/nodemailer/CalDAV client…），DSH 特定部分只是 tool 注册契约（名称/schema/handler）。→ **可搬运**：包成 MCP server（DSH 官方本就有 dsh-mcp-client，生态另有 dsh-mcp-panel npm7d=1,694 做运行时管理，说明工具↔MCP 双向通道现成）或写成 SKILL.md + scripts。
2. **运行时接缝层**：挂在 Cordis ctx/tools/pre-execute 瀑布、官方审批 API、会话事件流、credentials 接缝上（治理类/计费类插件的全部价值所在）。→ **不可移植**：目标平台不暴露等价拦截点；且治理本来就是平台内建职责（WorkBuddy 三层安全审查、QwenWork 企业协同管控、豆包虚拟桌面隔离）。
3. **客户端 UI 层**：client.mjs 注入 DSH Web UI（侧边栏/面板/TUI/桌宠）。→ **零可移植也无需移植**：目标平台有自己的 UI。

### 8.3 按可移植性分级的项目清单

| 级别 | 项目 | 改造成本与形态 |
| --- | --- | --- |
| **A（近直接可用）** | @huiliyi37/dsh-office（16 工具）、jiazekang/dsh-office、kw78/dsh-office-tools、omdsh-dev/dsh-office、dsh-email、dsh-calendar、modsearch、modlens、excel-chat、dsh-formatforge（30+ 格式转结构化） | 纯逻辑无状态，核心是 exceljs/pdf-lib/nodemailer/CalDAV/搜索与 VLM API 封装；抽出 handler 包 MCP server 即可在 WorkBuddy/QwenWork 用；再写一份 SKILL.md 教工作流即可上豆包。注意 AGPL/MIT/Apache 逐个核对（§5 口径说明） |
| **B（无需改造，直接接入）** | hindsight（★21,130）、MemOS（★10,985）、memsearch（★2,511）、OpenViking（★33,248）、WeKnora（腾讯） | 本来就是独立记忆/RAG 系统，自带多 harness 集成层；给目标平台写个连接器即可，不存在「从 dsh 移植」问题 |
| **C（中等成本，需重包装）** | dsh-im / dsh-notifier 的通道适配器（27 通道）、dsh-data-agent（数据库 SQL 分析）、dsh-record-replay（macOS 示教→skill）、dsh-video-understand | IM/通知适配器本质是 webhook/bot API 封装 → 可包成 MCP 工具；但注意与目标平台的母体 IM 冗余（WorkBuddy 长在微信/企微、QwenWork 长在钉钉、豆包长在飞书），桥接方向应换成各平台的母体生态。record-replay 的输出物恰好就是 skill，属「反向输出」 |
| **D（不可移植/不值得）** | permission-rules、auto-review、auto-approval、approve-for-me、HOL Guard 插件、budget-guard、cost-meter、control-center、whale-report、session-manager、better-sidebar、dock、TUI、皮肤/桌宠全家 | 治理审批类依赖 tools/pre-execute 与官方审批接缝（平台自建安全层后无此需求）；计费类依赖 dsh 会话事件流与用户自持 API key（三家都是订阅制，用户不碰 key）；UI 类是 dsh Web 专属 |
| **E（方向重选）** | dsh-lark-bot（飞书深桥）、dsh-lark | 对 WorkBuddy（微信/企微生态）、QwenWork（钉钉生态）无意义；IM 深度桥应按目标平台母体重选协议栈 |

### 8.4 已有互通先例（这条路是既成事实，不是设想）

- **Skills 标准跨平台**：OpenClaw Skills 遵循 Agent Skills 开放标准，ClawHub 商店 13,000+ 技能；WorkBuddy 官方宣布兼容 OpenClaw 技能。
- **DSH ↔ Codex 双向**：`dsh-codex-sync`（npm7d=2,356，技能/会话/工作区/MCP 镜像双向同步）、`codex2dsh`（会话转换）、claude-bridge/claude-move/import-agents（Claude Code 迁移件）。
- **DSH ↔ OpenClaw 双向**：`dsh-openclaw-persona`（npm7d=242，Persona 五件套搬进 DSH）、`dsh-openclaw-enterprise-team`（npm7d=325，OpenClaw 企业团队桥接）。
- **反向借鉴已发生**：`dsh-skill-picker` 的 1024Store 描述原文就是「WorkBuddy 同款选择 skill 功能」；`dsh-agent-conductor` 已能在会话内派发任务给 WorkBuddy CLI 等 11 个外部 agent——**两个方向的通路都已有人踩通**。

### 8.5 合规与风险（改造前必查）

1. **许可证**：A/B 级项目以 MIT/Apache 居多可直接改造；**AGPL 项目（dsh-lark-bot、OpenViking、oh-story-dsh 待核）对闭源商用产品有传染风险**，集成前逐个核对 LICENSE（本轮已发现个别插件 license 字段缺失，见更新记录第三轮）。
2. **供应链自检反哺**：改造前用 dsh 生态自己的工具过一遍被抽代码：DShScan（R010-R012 dsh 特有攻击面规则）+ HOL Guard plugin-scanner（`--ecosystem deepseek-harness`）——这是 dsh 生态工具对改造流程的直接反哺。
3. **品牌与上游声明**：插件名 `dsh-` 前缀与 DeepSeek 相关表述需在改造产物中清理；多数插件为个人作品（star 个位数到千位数），无 SLA、作者随时弃坑（已有仓库 404 先例：vibeinging office-tools、zhu1090093659 task-board/aionui-panel）——**建议 fork 锁定后再动工**。
4. **平台审查**：WorkBuddy 开放插件要过三层安全审查；上架豆包技能商店/QwenWork 技能市场各有平台规范——改造产物的分发要走对方流程，不是自由市场。

### 8.6 对本仓库的含义（按优先级，不排期）

- **P1**：若本仓库未来要做「能力输出」，最现实的形态是把 Tauri 桌面壳沉淀的窗口/托盘/更新/救砖经验写成 **SKILL.md 技能包**（跨 WorkBuddy/QwenWork/Claude Code 通用），而非做平台专属集成。
- **P2**：§7.11 的 dock `ctx.workbench` 协议观察同样适用于此处——办公 agent 平台的「连接器」概念与 dsh 的「工具注册」正在趋同，MCP 是最大公约数。
- **P2**：本仓库不做办公 agent 产品本身（§4.3 已论证产品层差距），但「把 dsh 生态 A 级插件改造为多平台 Skill/MCP 包」是一个低成本的生态卡位选项——前提是接受 §8.5 全部合规义务。

## 9. 证据来源清单

- deepseek-ai/deepseek-harness 本地检出（README.zh.md、docs/）——2026-08-26；GitHub API ★195,025（第一二轮值，第三轮复核为 **★195,052**，最新 tag v0.1.1-rc.2 未变）；docs/kernel-pin.json（pin=b150a551…）
- dshmarket.com（首页 Most installed、全部插件分类页）——2026-08-26（第一轮）
- **deepseek1024.com（DSH 1024Store）：开放 API `/api/v1/plugins`（meta.catalogTotal=10,681；`?q=` 过滤）、installs30d/failureCount/verification/growth——2026-08-26 第二轮重拉**
- **dshfind.com（市场+学习社区）；官方仓库 Discussion #1179 自荐（2026-08）；zhihu《dshfind 评分榜插件实测》**
- libukai/awesome-deepseek-harness（GitHub 页面）——2026-08-26；imsai-sh/awesome-deepseek-harness-plugins（★186，描述 3,100+ / 旧记录 10,566，待核）
- Wavect《DeepSeek Harness 评测：企业生产就绪度》——2026-08-16
- WPS 官方论坛《介绍三款 dsh-office 插件》（bbs.wps.cn/topic/95386）——2026-08-21
- 火山引擎官方文档/文章：AgentKit 文档（docs 86681）、知乎实操（2026-08-23）、veStack×V4（2026-05-07）
- 36氪/定焦《腾讯、阿里、字节，大战 AI 办公》——2026-08-06（转引易观 Q2 报告、艾媒白皮书）
- jdon《DSH Market：DeepSeek Harness 插件市场》深度生态文章——2026-08-16
- 【同名泛化参考】53AI《DeepSeek Harness 企业级部署与实践指南》——2026-08-24
- **GitHub API / HTML 抓取 / npm registry API / 1024Store API 实时核验——2026-08-26（第二轮）；治理类 19 款插件 + 6 个新品逐个 README 深挖（README 直读与 raw.githubusercontent.com）**
- **第三轮核验（2026-08-26 同日）：GitHub API（认证通道，30+ 仓库逐个拉取 full_name/★/license/pushed_at/description）；GitHub HTML 抓取（Tencent/BrowserSkill ★1,321、Deepseek-Harness-EAC ★1,304）；npm registry `downloads/point/last-week`（23 个包重拉）；deepseek1024.com API meta 重读（catalogTotal=10,681 未变）；火山引擎官方文档 docs.volcengine.com/86681 全目录与新功能发布记录（AgentKit 十大模块）；deepseek.com/harness/ 官方发布页；上游 anywhere-labs/dsh-desktop README/release/submodule 直读（master 分支）；dsh-notifier README 徽章复核（27 channels）；dshmarket/dshplugin.wiki/dshplugin.market/cordisplugin.com 首页元信息直读；linux.do/t/topic/2780908（官方无桌面版的社区确认）**
- **第四轮盲区补扫（2026-08-26 同日）：量子位《DeepSeek Harness插件一夜燃爆GitHub》（qbitai.com/2026/08/473597.html，2026-08-15，700+ dsh-plugin 仓库口径与娱乐生态报道）；GitHub API 认证搜索（topic/search 双路，20+ 新仓库核验：hindsight ★21,130、MemOS ★10,985、memsearch ★2,511、agentrq ★1,089、dsh-web ★6,074、deep-whale ★1,716 等）；1024Store API 关键词分类查询（search/voice/agent-teams/memory/pet/vscode/mobile 七组）；官方 Discussion #1038（DSH Qwen Voice 自荐）**
- **第五轮办公平台可改造性评估（2026-08-26 同日）：WorkBuddy 官网 workbuddy.cn 与 codebuddy.cn/work；证券时报《消息称腾讯正秘密开发微信AI智能体》（含「WorkBuddy 兼容 OpenClaw 技能」）；新华网《从跟风"养虾"到理性选择》（2026-04-03，开放插件三层安全审查）；阿里云官方帮助文档 help.aliyun.com/zh/qwenwork/skills（千问办公技能体系）；东方财富（2026-08-03，三产品合并公测）；财新（2026-06-24，豆包专业版订阅制）；21财经/财联社/观点网（2026-08-21，豆包技能商店 200+ 技能与连接器上新）；doubao.com/work；OpenClaw 官方文档 docs.openclaw.ai（Skills=SKILL.md 标准）、MIT 科技评论中文版访谈（Skills 规范作为开放标准发布）、ClawHub 商店报道；1024Store API 查询（mcp/skill/openclaw/convert 四组：dsh-codex-sync、openclaw 双桥、dsh-weknora 等）**