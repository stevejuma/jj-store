---
schema: 1
id: E-UI-093
---

# E-UI-093

<!-- evidence-field:id markdown -->
## Id

E-UI-093
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

history-query-metadata-wait-disposal-regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qyrqmyzz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

After verified main41a24e81, exact new disposal-during-metadata test fails1/6: disposing suppresses metadata application but the awaiting run resumes and starts a graph read. Command: pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query.test.ts. /tmp/history-metadata-dispose-before.log.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Same locked Svelte5/CodeMirror6/frontend toolchain as E-UI-092; qualified Design Core04b1ec1ba7e7 remains unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No contract/schema/backend/persistence changes. Query execution checks its invalidation sequence again after awaiting identity metadata; navigation, disposal and edits cannot resurrect a waiting submission.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/history-query.svelte.ts
- web/tests/vite/history-query.test.ts
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/api.md
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query.test.ts (after)
```

**Notes**

Exact6cases pass after sequence guard; before failed1. /tmp/history-metadata-dispose-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

55files/315tests. Shared query invalidation is used by multiple controller scenarios, justifying the complete controller lane after the new guard. /tmp/history-app-final-controller.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm spec:first; pnpm spec:validate; changed-file formatting
```

**Notes**

No errors/warnings; architecture and canonical gates pass. Private UI-only follow-up does not require Rust, new contracts or source Design Core reruns. /tmp/history-{web-race-check,catalog-race-check,architecture-final,race-spec-first,race-spec-validate,race-format}.log. Initial spec-first correctly required the mapped API chapter; explicit client-await cancellation contract added before final pass.
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
pnpm web:build; pnpm exec playwright test --config .scratch/history-query.playwright.config.ts web/tests/browser/history-query.spec.ts; pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

Fresh production UI/host2cold cases pass;4interaction/accessibility stories pass. Strict CSP, identity default, multiline highlighting and retained invalid drafts remain qualified. /tmp/history-{web-final-build,cold-browser-final,final-stories}.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Only UI query metadata-await cancellation is complete. E-UI-092 retains the separate registry release gate; Design Core visuals remain excluded. Main Vite picks up this source guard through HMR; backend63916 and fixture unchanged. Disposable static9370host stopped; original trunk() query restored without errors.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified UI follow-up in main. Existing source package release and wider M13/visual gates remain separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


