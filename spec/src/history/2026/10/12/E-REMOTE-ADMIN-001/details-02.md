---
schema: 1
id: E-REMOTE-ADMIN-001
---

# E-REMOTE-ADMIN-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; read parent/root/UI AGENTS, owning M27 and security/API/UI chapters; inspect maintained Community and released Design Core
```

**Notes**

Started above lssvruoo/fbabaf94 with no unrelated working-copy changes. This enclosing change owns the implementation. Published dependencies and four development repositories retained.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo test --locked -p jj-store-server --lib agent_administration::tests -- --nocapture
```

**Notes**

1 passed. Exact independent permission, conditional revision, restart indication and unrelated configuration preservation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo test --locked -p jj-store-http --lib remote_agents::tests
```

**Notes**

1 passed. Authenticated agent API admission regression.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo test --locked -p jj-store-service --test authorization
```

**Notes**

10 passed, including independent ManageAgents authorization and original revocation/publication regression coverage.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo test --locked -p jj-store-server --lib connections::transport::tests
```

**Notes**

1 passed. Fixed gateway route/method admission retains local-management denial.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo check --locked -p jj-store-http -p jj-store-server -p jj-store-cli; pnpm dev:backend:build
```

**Notes**

Focused Rust checks passed. Builds passed; earlier runs included Cargo lock queues. Exact package scope, not a workspace Rust suite.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/server-agent-administration.test.ts
```

**Notes**

9 passed. Draft/base retention, explicit rebase, no replay recovery, principal-set ordering, stale-owner rejection, separate connection drafts, hidden polling and switching guards.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerAgentAdministration.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

32 passed across all three direct consumer story families with interaction/Axe. Administration-only final rerun: 6 passed after mode-help/default correction. Stories remain visual-pending.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/global-settings-history.test.ts web/tests/vite/directory.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/server-agent-administration.test.ts
```

**Notes**

29 passed. Cold failure proved authenticated reload redirected Agents to Projects. Correction retains Agents/installation Threads after repository discovery; explicit regression added. Related Settings, directory and bootstrap owners retained.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/server-connections.test.ts; pnpm check:ui-ownership; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

6 connection tests passed; UI ownership and 17 package boundaries passed; architecture passed after meaningful principal-collection extraction brought the new controller under 300 counted lines.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```

**Notes**

Final web/catalog checks: zero errors and warnings. An intermediate catalog check found two profile prop typing errors; the dialog now carries the same optional profile as its onboarding consumer.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

Generated schemas and TypeScript agree; 300 actual HTTP/SSE responses conform. Eight contract helper tests and contract TypeScript checks passed. New bounded operator/native onboarding behavior is also exercised by the two-instance acceptance.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:generate; pnpm docs:review services/configuration/cli/http/application/contracts with topic-specific reviewed explanations; PATH="$PATH:/Users/stevejuma/.local/bin" pnpm docs:check
```

**Notes**

Generated CLI/API references refreshed and source/page reviews updated. Docs check passes: 9 diagrams in both themes, 141-file closure, valid links/anchors. Initial stale reference/review failures required generation/review; missing ambient uv was resolved using its existing user installation.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vite build --config web/vite.config.ts; node scripts/web-manifest.mjs; PATH="$PATH:/Users/stevejuma/.local/bin" pnpm web:build
```

**Notes**

Cold production bundle/manifest and full manual staging passed. No visual baseline changes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/remote-agent-administration-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs desktop/resources/jj-store-rustfs --acp desktop/resources/jj-acp --web web/dist --privacy-cache .local/dev-demo/local/agent-execution/privacy-cache --tooling-bin .local/dev-demo/local/agent-execution/tooling/ffdb4d8db47046d0a6e3a350490fb435/bin
```

**Notes**

Passed on two disposable local macOS real-RustFS installations and real installed Codex. Cold operator UI checks enrollment, home/code Write grants, sharing save, independent roles, drafts, mobile, Axe, reload, Back, CSP and no persisted tokens. Provider edits only remote-acceptance.txt in an admitted isolated workspace; dirty source stays exact, captured state publishes, restart returns the original completed prompt with one started event, identity revocation denies access. Temporary roots/processes clean up.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Cold harness iterations and focused UI regression checks
```

**Notes**

Before corrections: save hit a Svelte proxy clone error; principal pattern interpolation was invalid; production profile normalization excluded native repositories; onboarding drafts remounted; nested Select scrolling mispositioned the menu; identity landmarks duplicated; capability discovery could select the wrong tab. Corrected each with controller/story/cold checks. Harness initially omitted enrollment/grant submission and was repaired; provider ask mode was unavailable, so the fixture and new Codex default use its advertised read-only ID. No prompt was sent in that failed preparation run.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl read-only live main Add server geometry and screenshot before/after scoped CSS
```

**Notes**

Live main before: header trailing/body-top padding 20px + 8px. Initial compact 8px + 0px clipped the outer 1px card ring at the scroll viewport edge. Final: header top 16px, trailing 4px, body top 4px. Full outline verified in the running app; header is 8px shorter than the initial compact version. Description remains in Connection details. Final fresh connection interaction/Axe run: all 12 stories pass, including viewport clearance, mobile, keyboard, busy state and original actions.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:backend:restart; read-only /v1/repositories with the existing protocol/Origin/Fetch-Metadata admission headers
```

**Notes**

Main backend restarted with Vite retained. All four existing repository names and state remain. An initial headerless inventory probe correctly received 403; proper admitted read passed. No reset or migration.
<!-- /run -->
<!-- /evidence-field -->

