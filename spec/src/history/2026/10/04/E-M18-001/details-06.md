---
schema: 1
id: E-M18-001
---

# E-M18-001 details 6

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
pnpm web:check (main 90d119e0)
```

**Notes**

Zero errors/warnings; inherited temporary Design Core validation setup
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
pnpm web:build (main 90d119e0)
```

**Notes**

Fresh built UI
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts
```

**Notes**

Three cold Capture browser scenarios including unavailable=14000; 15.2 seconds
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before unavailable-observations label assertion)
```

**Notes**

Six pass; one stale exact text assertion
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (final)
```

**Notes**

Seven interaction/accessibility scenarios; 9.88 seconds; visual-pending
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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs (final)
```

**Notes**

32 tests; desktop M17 remains distinct from capture M18
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (capture runbook before protected-doc mapping)
```

**Notes**

Unmapped docs/capture-preview.md; owning chapters and mapping added
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
pnpm spec:first (after runbook mapping)
```

**Notes**

446 protected files, 14 canonical chapters
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
pnpm spec:validate
```

**Notes**

Five validators, 22 chapters, 166 requirements
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
pnpm check:architecture
```

**Notes**

866 modules; production controller below 300 lines
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
pnpm check:ui-ownership
```

**Notes**

Repository-owned presentation
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
cargo fmt --all; pnpm format:rust:check
```

**Notes**

Workspace and embedded-runtime formatting
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
pnpm exec prettier --write affected documents/scripts/TS/Svelte/JSON excluding historical ledger
```

**Notes**

58 changed paths, followed by final three handoff documents; historical raw evidence remains unchanged
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
pnpm check:progress (before final close)
```

**Notes**

135 tasks; evidence bookkeeping only
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
pnpm dev:capture:setup; pnpm dev:capture:restart (explicit embedded RustFS executable)
```

**Notes**

Isolated native fixture .local/codex-preview; ports7470/7472; retained fixture and HMR
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
target/debug/jj workspace add secondary (disposable native trial)
```

**Notes**

Workspace created but snapshot saw existing native request concurrency; no destructive recovery
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
JJ_CONFIG=/dev/null target/debug/jj --no-pager -R .local/codex-preview/secondary workspace update-stale
```

**Notes**

Recovered disposable secondary physical native workspace
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
codex exec/resume 0.156.1 (deliberately created alpha/beta/gamma/delta, secondary, unlinked and history trials)
```

**Notes**

Two active native session identities; gamma physical file edit; automatic UI updates and retained selection/draft; unrelated trial bodies withheld; seeded sessions are not real harness qualification
<!-- /run -->
<!-- /evidence-field -->

