---
schema: 1
id: E-CHAT-WORKSPACE-009
---

# E-CHAT-WORKSPACE-009 details 3

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
cua_repl live Changes → Since last commit
```

**Notes**

Verified Files toolbar/count0/view controls remain with No changes and empty-icon SVG in the body. Missing-base comparison also retains the toolbar without reporting a false empty success. No prompt or publication dispatched.
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
pnpm evidence:new -- --id E-CHAT-WORKSPACE-009 --task M24 --task M13.3 --requirement JST-UI-001 --requirement JST-UI-015 --requirement JST-UI-018 --requirement JST-API-001 --requirement JST-API-006 --requirement JST-LOCAL-004 --requirement JST-LOCAL-005
```

**Notes**

Created fresh record and reviewed M24; M13.3 source publication qualification unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/WorkspaceFilesPerformance.stories.svelte
```

**Notes**

Final cold rerun after the native locator repair:13/13 interaction/accessibility stories pass; workspace-perf-stories-verified.log. Existing ResizeObserver warning remains. Focused repaired skeleton story:1 pass/9 skipped, workspace-perf-skeleton-native.log.
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
pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Final story types and owned-source boundary verified again after the locator repair; workspace-perf-catalog-verified.log and workspace-perf-ownership-verified.log.
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
pnpm exec prettier --write <changed Svelte/TypeScript/Markdown paths>; cargo fmt --all -- --check
```

**Notes**

Changed source/docs formatted; final Rust formatting clean. workspace-perf-format-complete.log and workspace-perf-rustfmt-final.log. Initial Rust/Prettier formatting differences were corrected, no formatting policy relaxed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial ledger verification reported the newly generated M13.3/M24 history files missing from the evidence file list; refreshed the evidence list after history generation. workspace-perf-progress-check.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Initial gate required the CLI, JJ integration, compatibility and mutation chapters for changed Rust boundaries. Added explicit unchanged-contract confirmations and additive binding projection compatibility, then reran. workspace-perf-spec-first.log.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final checks:146 tasks/707 evidence records;32 protected files and7 canonical chapters; all7 validators pass. Logs workspace-perf-progress-final2, spec-first2 and spec-validate. Generated unrelated M25/Bootstrap navigation was restored; a prior regeneration caused two ledger coverage failures before that restoration.
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
cua_repl final live rail inspection after backend restart
```

**Notes**

Empty Changes still shows count0, toolbar and icon; last browser warn/error log list empty. Closed only agent-created profiling tabs and preserved the user tab.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ResizeObserver loop completed with undelivered notifications is still
  reproduced during existing responsive stories. Visual baselines remain
  unapproved.
- Six existing architecture size violations remain; no limit changes or
  full-suite pass claimed.
- M24 remains In progress. M13.3 publication/retained-source/GC and native
  installed-harness/cold production-bundle gates are not closed by this bounded
  local inspection slice.
- Root aggregate lanes and unrelated Rust/UI suites are out of scope under
  targeted verification. No packages published; main workspace and existing demo
  data preserved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; pnpm10.32.1; Vitest4.1.11; Svelte5.57.1; macOS ARM64. Enclosing Jujutsu change is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional workspace_name in code binding projections, generated OpenAPI/TypeScript updated. Inspection v1, source selectors, capture/publication formats and event envelope unchanged. Cache is session-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cross-layer workspace browsing: static binding menu, bounded observation caches and event invalidation; representative skeleton and Empty within retained listing; remove duplicate JJ verification and repeated inner companion admission. Read-only scoped workspace watchers, cleanup and late-response fencing retain authority boundaries. Exact integration regression also repairs swallowed non-prompt worker frames.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete remaining cold native/installed adapter, retained-source and resize qualification independently.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "UI001/015/018 and API001/LOCAL004/005 now specify static binding data,
  clean memory caching and scoped physical freshness. API006 worker delivery is
  explicitly unchanged: prompt queuing must still forward other controls. No
  source/publication capability expanded."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M13.3 remain In progress. Local read optimization is not
  portable review publication qualification; native cold flows, retained-source
  and visual gates remain open.

```
<!-- /evidence-field -->

