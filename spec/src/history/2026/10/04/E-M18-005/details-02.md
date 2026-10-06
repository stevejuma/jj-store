---
schema: 1
id: E-M18-005
---

# E-M18-005 details 2

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
jj --no-pager st; read tracked/parent guidance and owning collaboration/API/UI/CLI/desktop chapters; inspect Community then Design Core Settings controls
```

**Notes**

Clean main and preview; no unrelated work changed. Narrow target: edit disabled local capture, revision-safe persistence, retained drafts and restart-selected configuration.
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
cargo check -p jj-store-server -p jj-store-http --locked (initial)
```

**Notes**

AgentCaptureConfig validation needs Access and Storage. Validate the replacement through the containing Config rather than calling a zero-argument method.
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
pnpm contracts:generate; pnpm contracts:check
```

**Notes**

Generated OpenAPI/TypeScript, 224 actual HTTP/SSE responses and 7 contract-tool tests pass. Full registered capture lane required because routes/schemas share an inventory. Existing registered capture HTTP fixture includes GET/POST settings, stale revision, actor injection and foreign Origin; body limits remain4096 except bounded settings144KiB. No runner changes.
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
cargo test -p jj-store-server --test capture_management settings:: --locked; cargo test -p jj-store-server --test capture_management --locked
```

**Notes**

2 exact new settings tests, then all16 direct capture-management regressions. Admin/current identity, same-file lock, stale/malformed/unsupported roots/version/profile rejection, regular-file policy, preservation of unrelated config and journal bytes, saved versus effective and no initialization while disabled. Full binary took92.59s; no full Rust workspace suite.
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
cargo test -p jj-store-server --lib configuration_remains_editable_while_capture_inspection_holds_the_runtime_gate --locked (first)
```

**Notes**

Test dropped its temporary Fixture through an underscore destructure, removing configuration before observation. Retain the fixture binding.
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
cargo test -p jj-store-server --lib configuration_remains_editable_while_capture_inspection_holds_the_runtime_gate --locked (final)
```

**Notes**

1 test proves settings read/save independent from occupied runtime inspection gate, retaining policy. Configuration files use their own shared advisory lock, never a worker/journal lock.
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
cargo check -p jj-store-server -p jj-store-http -p jj-store-service --locked (final); cargo fmt --all
```

**Notes**

Affected packages compile. Final server/UI backend rebuilt from explicit local binary paths.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-server -p jj-store-http -p jj-store-service --all-targets --locked -- -D warnings; same with --no-deps
```

**Notes**

Supplemental strict Clippy diagnostics remain open: unchanged jj-store-jj/src/query/language/identity.rs:108 and jj-store-service/src/collab_reads/materialization.rs:203 collapsible_if warnings. They are outside this configuration slice. No source allowances or unrelated fixes applied; focused package check/tests pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial checks)
```

**Notes**

Initial web4 errors: request bodies are generated inline, so use operation-derived Exact type. Initial catalog13 errors/1 warning: same type issue, Testing Library exact option and initial prop capture. Next catalog boundary rejects direct API fixture import; expose production presentation type through capture-types. All repaired.
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
pnpm web:check; pnpm catalog:check (final)
```

**Notes**

0 errors/0 warnings; catalog172 modules. Editing uses existing Field/Switch/Input/Textarea controls, Settings rows/action columns and shared notices.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-settings.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/production-capture-management.test.ts (first)
```

**Notes**

25 pass, one existing synchronous single-flight expectation fails because absent settings transport added an unnecessary await. Only await when a settings transport exists.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-settings.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/production-capture-management.test.ts (final)
```

**Notes**

27 tests/4 files. Disabled editing, retained drafts, external revisions, unknown response reconciliation, validated bounds, no runtime mutations while restart pending, and configuration save while a runtime observation waits. Separate commandBusy/saving state prevents ordinary observations from disabling the editor; epochs reject stale config observations.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

14 stories pass including3 new edit/stale/unknown scenarios, Settings columns, tab retention and existing capture direct consumers. Initial enabled prop warning repaired with untrack. Stories remain visual-pending.
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
pnpm web:build; pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts local-capture-settings.spec.ts
```

**Notes**

Final fresh production bundle, all5 cold direct browser tests pass. New disabled editor saves typed fields, retains drafts across refresh/tabs, shows defaults and restart state, passes scoped Axe and mobile no-overflow. Existing no-auth/capability/notification/freshness checks retained. Uses main compiled preview asset host; mocked UI transport evidence is separate from actual service persistence.
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
node --test scripts/dev-ui.test.mjs; pnpm test:dev:ui
```

**Notes**

6 preflight tests pass, including exact matching HMR fixture retention and missing retained-file refusal. Shared restart helper changed, so escalate to its existing full isolated native HMR startup lane: actual proxy writes/SSE, Svelte/TS/CSS HMR without reload, backend-only restart retains Vite PID, wrong Origin/Host rejection and owned stop all pass; temporary service cleaned and fixture retained.
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
pnpm dev:backend:build; explicit preview JJ_STORE_DEV_ROOT/JJ_STORE_DEV_PORT/JJ_STORE_DEV_BACKEND_PORT/JJ_STORE_DEV_SERVER/JJ_STORE_DEV_RUSTFS_RUNTIME pnpm dev:backend:restart; jj rebase/update-stale; CUA live editor save/restart/restore
```

**Notes**

Main executable rebuilt; only isolated preview7470/7472 restarted. Real UI saved readers4→3, restart reports desired/effective3 and no pending configuration; restore4 and restart reports desired/effective4. Fixture/installation/control-policy SHA256 hashes stay identical. First restart used stale preview helper and reset the temporary edit; snapshot main and refresh preview, then repeat successfully with fixed helper. .local/capture-settings-live.json contains proof; screenshot .local/capture-settings-live.png; preview tab retained. Main running fixture and installed services untouched.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

893 UI modules;17 package boundaries;1 ownership validator/22 chapters/167 requirements.
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
pnpm spec:first (first)
```

**Notes**

Mapped service/HTTP scope also requires compatibility.md and mutations.md. Add typed conditional config and format/publication boundaries.
<!-- /run -->
<!-- /evidence-field -->

