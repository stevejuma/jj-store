---
schema: 1
id: E-UI-089
---

# E-UI-089 details 3

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
pnpm spec:first --base 1b2dbc2f --head <actual merge commit>; pnpm spec:first --base 36c92787 --head <actual merge commit>; pnpm spec:validate; pnpm check:progress; pnpm check:architecture
```

**Notes**

Both explicit parent ranges pass:96protected files/10chapters and376protected files/13chapters.5validators/21chapters/147requirements;120tasks;808modules. Final ledger/type/generated changes trigger the same gates before commit.
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
cargo build -p jj-store-cli --bin jj-store --locked; cargo build -p jj-store-cli --bins --locked
```

**Notes**

Integrated backend built7m53s; both development CLI bins built2m37s including build-lock wait. Explicit main target/debug paths only; installed CLI unchanged.
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
pnpm catalog:test --maxWorkers=1 --no-file-parallelism (after immutable flags)
```

**Notes**

293/293 across52files in55.81s. Later added production HTTP reader test qualifies in the final complete rerun.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/work-reactions.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/disclosure-layout.test.ts (final added regressions)
```

**Notes**

15/15 across3files, including independent production QueryClient creation/teardown, exact reaction transport invalidation and retained unrelated reactions. JSON media-type fixture corrected without relaxing Client admission.
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
pnpm web:check; pnpm catalog:check (qualified merged types)
```

**Notes**

Both0errors/0warnings;153catalog modules. Shared review snapshots retain their actual native/local identities, with no synthetic epochs or casts.
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
trash -v node_modules/.cache/storybook/10.5.7; pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte
```

**Notes**

Owned generated optimizer cache recoverably cleared; fresh process6/6stories42.59s. Together with other8passing files:59qualified stories across9files. No source code/cache alias workaround, test timeout changes or visual comparisons. Full final transcript/tool-comment consumer coverage follows.
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
pnpm catalog:test --maxWorkers=1 --no-file-parallelism (complete final UI units); pnpm web:build (final)
```

**Notes**

294/294 across52files, including the new production repository sidebar transport regression. Final production bundle45.95s; minifier :global and large-chunk warnings retained as diagnostics, not hidden. Web node-test lane70/70 passed earlier.
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
python3 parent evidence and main dependency/runtime metadata audit
```

**Notes**

All295 prior evidence identities retained; main262entries exact, Appearance differs only in shared E-M15-031 runs/remaining/next due main’s newer evidence. All unique Appearance entries remain exact. No temporary references in main node_modules .modules.yaml/.pnpm/lock.yaml or runtime.json. An initial over-strict equality assertion detected the expected shared update; this identity-aware audit verifies it.
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
pnpm test:storybook catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

22/22 interaction/accessibility tests across4additional agent families in114.07s. Together with the9qualified direct families:81stories across13files. No Design Core visual comparisons or baseline updates.
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
pnpm contracts:check (regenerated final merged contract)
```

**Notes**

Generated OpenAPI and TypeScript match combined Rust contracts;147actual HTTP/SSE responses conform. Contract node tests7/7 and consumer contract type check pass. Exact capture fixtures include local/native browsing, management, mutations, search, collaboration and WorkStore.
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
Codex browser fresh main-app tab: pinned repository Files, Agents, recorded Codex session and Focus sheet; inspect browser error logs
```

**Notes**

Original files and3agent sessions present from retained fixture. Recorded response shows Update the parser, Expand3tool calls and agent-session comment footer. Focus sheet preserves listing; browser error log empty. User tab untouched; temporary acceptance tab35 will be closed after final checks.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 subprocess argument-array changed-file prettier check
```

**Notes**

All source/generated files formatted; final evidence update requires reformatting spec/src/milestones.md. Final documented formatting and ledger gates follow.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/local-browser-acceptance.mjs (initial integrated cold filesystem profile)
```

**Notes**

1/5 passed. Failures: old ungrouped-tool locator, retired Saved status (with genuine Snapshot superseded race), old review reply-composer label, and physical Save test following fixture registration removal. Fresh recorded fixture and server isolated from the persistent app; code/registration lifecycle test passed. First fixture was automatically cleaned. No assertion/timeouts weakened.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/local-browser.test.ts (fresh-capture regression before fix)
```

**Notes**

13/15 passed; both newest-first and oldest-first refresh completions reproduced Snapshot superseded. Existing ordinary-read cancellation and metadata pin checks retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/local-browser.test.ts web/tests/vite/production-work.test.ts web/tests/vite/automatic-refresh.test.ts
```

**Notes**

24/24 across3files. Concurrent fresh requests receive the newest accepted pin; original ordinary-read supersession, independent metadata pinning, unknown intent recovery and coalesced automatic refresh remain qualified.
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
pnpm catalog:test --maxWorkers=1 --no-file-parallelism (local adapter final consumers); pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm web:build
```

**Notes**

296/296 across52files. Both type checks0errors/0warnings, catalog153modules and architecture808modules; rebuilt production bundle6.61s. Existing chunk-size diagnostics retained. Rust/public schemas unchanged by local adapter correction; qualified147-response contract evidence still applies.
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
Inspect lsof cwd and exact command for4temporary Design Core http.server previews; send SIGTERM to43790,45057,46017,46681; close owned Codex browser tab35
```

**Notes**

Only obsolete previews rooted in design-core-jj-store-appearance stopped. Unrelated source/consumer processes untouched. User browser tab preserved; verification tab closed. Proof /tmp/jj-appearance-land-preview-retirement.json.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7478/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained isolated fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts (retained fixture reruns)
```

**Notes**

First retained run3pass/2fail: task-list refresh and folder-removal leakage into physical Save. Restore removed registration in finally using a fresh request ID. Next run4pass/1fail in48.9s: task still absent while Create task dialog open. Trace confirms committed command and refreshed work/page data but unchanged work/browse listing. These servers retained older built assets and cannot qualify the later notification-order fix.
<!-- /run -->
<!-- /evidence-field -->

