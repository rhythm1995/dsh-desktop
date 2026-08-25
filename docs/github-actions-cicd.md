# GitHub Actions CI/CD 方案

两条工作流，都在 `.github/workflows/`：

| 工作流 | 文件 | 职责 | 触发 |
| --- | --- | --- | --- |
| CI | `ci.yml` | 质量门禁：类型检查、单测、Rust 编译与测试 | push 到 `main`、所有 PR |
| CD | `release.yml` | 打 macOS 生产包并发布 GitHub Release | 推 `v*` tag、手动 `workflow_dispatch` |

## CI：`ci.yml`

两个 job，互相独立、并行跑：

| Job | Runner | 步骤 |
| --- | --- | --- |
| `host` | `ubuntu-latest` | `pnpm install --frozen-lockfile` → `pnpm build:host` → `pnpm typecheck` → `pnpm test` |
| `shell` | `macos-latest` | `pnpm install --frozen-lockfile` → `pnpm build:host` → `cargo check --all-targets` → `cargo test`（工作目录 `src-tauri/`，带 `Swatinem/rust-cache`） |

- Host 是纯 Node/TypeScript，跑在更便宜更快的 ubuntu runner 上；Rust 壳的产物平台是 macOS，`shell` job 用 `macos-latest` 与发布保持一致。
- 两个 job 都要先 `pnpm build:host`：`host/tests/host-entry-launch.spec.ts` 需要编译产物 `host/dist/main.js`；`tauri-build` 又会校验 `tauri.conf.json` 里的 `bundle.resources`（`../host/dist`）必须存在，否则 `cargo check` 直接失败。
- Node 版本固定 `22`（`package.json` 的 `engines` 要求 `^22.19.0 || >=24.0.0`）；pnpm 版本由 `pnpm/action-setup` 从 `package.json` 的 `packageManager` 字段读取。
- `pnpm test` 里 `official-host.spec.ts` 的集成用例依赖 git 忽略的上游 `dsh-plugin-desktop` 检出，CI 上没有该检出时自动跳过（`it.skipIf`），不算失败。
- 生产语义在 CI 自动生效：`GITHUB_ACTIONS=true` 时 `write-build-profile.mjs` 生成 `COMPILED_DEV_TOOLS=false`（见 `docs/devtools-listen.md`）。

## CD：`release.yml`

触发：推 `v*` tag 或手动 dispatch。单个 `macos` job：

1. `pnpm install --frozen-lockfile`
2. `npx tauri build --bundles app,dmg`，环境变量 `DSH_DEV_TOOLS=0`、`DSH_PROFILE=production`、`CSC_IDENTITY_AUTO_DISCOVERY=false`（不做 macOS 签名/公证）
3. `.app` 是目录，先压成 `DSH.Desktop.app.tar.gz`
4. 推 `v*` tag 触发时，`softprops/action-gh-release` 把 `dmg/*.dmg` 与 `DSH.Desktop.app.tar.gz` 上传到该 tag 对应的 GitHub Release（不存在时自动创建）；`workflow_dispatch` 手动触发只打包并上传 artifact，不发布（`if: startsWith(github.ref, 'refs/tags/')`）。注意 globs 只圈顶层产物，别写 `**/*`——那会把 `.app`/`host/dist` 里的每个文件都当成资产上传

`tauri.conf.json` 的 `bundle.targets` 是 `["dmg", "app"]`，命令行 `--bundles app,dmg` 与之一致；`beforeBuildCommand`（`npm run build:host`）会在 Tauri 打包前自动跑，无需单独步骤。

## 需要的权限 / Secrets

- 仓库 **Settings → Actions → General → Workflow permissions** 需要 *Read and write permissions*（`release.yml` 要创建 Release 并上传产物）。`release.yml` 自身已声明 `permissions: contents: write`。
- 不需要任何自定义 secret。未来要做 Apple 签名/公证时再加 `APPLE_CERTIFICATE`、`APPLE_CERTIFICATE_PASSWORD`、`APPLE_ID` 等，并同步改 `tauri.conf.json` 的 `signingIdentity`。

## 发布流程

```bash
git tag v0.1.1
git push origin v0.1.1   # 触发 release.yml，产物出现在 GitHub Release
```

日常开发只需把 PR 合并到 `main`，CI 会拦住类型错误与回归。

## 本地验证（与 CI 等价）

```bash
pnpm install --frozen-lockfile
pnpm build:host
pnpm typecheck
pnpm test
cd src-tauri && cargo check --all-targets && cargo test
```

工作流本身可以用 [act](https://github.com/nektos/act) 在本地 Docker 里预演：

```bash
act push -W .github/workflows/ci.yml -P ubuntu-latest=catthehacker/ubuntu:act-latest
```

注意：act 会把**当前工作区**（含未提交的改动）拷进容器，而不是只构建已提交代码。本地有未完成的改动时，先在干净克隆里跑：

```bash
git clone -b <branch> . /tmp/dsh-ci-test && cd /tmp/dsh-ci-test
act push -W .github/workflows/ci.yml -P ubuntu-latest=catthehacker/ubuntu:act-latest
```

## 优先级

| 事项 | 优先级 |
| --- | --- |
| CI 门禁（typecheck + vitest + cargo） | P0 |
| CD 打包并发布 GitHub Release（macOS） | P0 |
| Windows / Linux 打包 | P1 |
| Apple 签名与公证 | P1 |
| 自动更新（tauri-plugin-updater + latest.json） | P2 |
