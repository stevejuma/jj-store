---
schema: 1
id: E-AGENT-RESUME-001
---

# E-AGENT-RESUME-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search -- "JST-COLLAB-034"
```

**Notes**

Forwarded -- is unsupported.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search "JST-COLLAB-034"
```

**Notes**

QMD native ABI 147 mismatch; used rg and opened canonical owning topics.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern="version 3 alone" scripts/dev-agent-runtime.test.mjs (before)
```

**Notes**

Startup accepted a v3 helper that rejects prepared context.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts -t "publication refresh retains" (before)
```

**Notes**

Visible events emptied during publication recapture.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_execution conversation_failures::resume_without_an_acknowledged_turn_creates_context_session_without_loading --locked -- --exact (before)
```

**Notes**

Resume failed loading the no-start-ack adapter session; 94.60s.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
Initial focused unit reruns during implementation
```

**Notes**

A misplaced sameSelection declaration caused one failing rerun; fixed. Initial reactive-effect waits never ran in the SSR unit environment (nine timeouts). Replaced them with explicit app-owned state notifications, used by production and tests.
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
node --test scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs
```

**Notes**

12 tests; shared launcher consumers included.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/inbox-request-lifecycle.test.ts web/tests/vite/agent-runtime-preparation.test.ts
```

**Notes**

27 tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-*.test.ts web/tests/vite/inbox-request-lifecycle.test.ts web/tests/vite/local-inbox-threads.test.ts
```

**Notes**

22 files, 112 tests; broadened for shared agent/status observation and history lifecycle.
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
cargo test -p jj-store-cli --test agent_execution conversation_failures:: --locked
```

**Notes**

4 tests, 124.74s. Rejection preserves unrelated sessions; no-start session opens context; Stop fencing and exact receipts remain covered.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-server --lib --locked
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-server --lib --locked -- -D warnings
```

**Notes**

Fails in unchanged jj-collab: three large-enum and two collapsible-if diagnostics. No source warning exemption added.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab --lib --locked -- -D warnings
```

**Notes**

Same diagnostics reproduced directly in unchanged source package.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors, 0 warnings; 373-module boundary check.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

139 pass; timestamp presentation fixture, missing repository_bindings fixture and two Node .svelte dependency-loader failures.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
Read-only parent 5439cc97 snapshot: pnpm exec tsx --test <snapshot>/web/tests/{agent-execution,managed-agent,production-store-mutations,production-store-search}.test.ts
```

**Notes**

Same four failures reproduced with 8 passing tests. Snapshot is ignored and uses unchanged released node_modules; default workspace is not moved.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentExecution,AgentSessionState,AgentChatRuntime}.stories.svelte (initial)
```

**Notes**

22 pass; obsolete ended-session story expected manual resume and disabled Send. Strengthened it to verify automatic Resume then exactly one prompt.
<!-- /run -->
<!-- /evidence-field -->

