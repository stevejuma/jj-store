---
schema: 1
id: E-READ-078
---

# E-READ-078 details 3

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
pnpm check:progress
```

**Notes**

4.38s; 0 tests passed. .local/bounded-reads/performance/kind-browser-progress-corrected-078.log
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
pnpm check:progress
```

**Notes**

5.54s; 0 tests passed. .local/bounded-reads/performance/kind-browser-progress-final-078.log
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
pnpm exec prettier --write web/src/lib/controller/contracts.ts web/src/lib/controller/store-settings-types.ts catalog/fixtures/KindPreparationFixture.svelte
```

**Notes**

1.36s; 0 tests passed. .local/bounded-reads/performance/kind-browser-repair-format-078.log
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
pnpm spec:first
```

**Notes**

1.82s; 0 tests passed. .local/bounded-reads/performance/kind-browser-spec-first-078.log
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
pnpm spec:first
```

**Notes**

1.86s; 0 tests passed. .local/bounded-reads/performance/kind-browser-spec-first-final-078.log
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
pnpm spec:validate
```

**Notes**

2.61s; 0 tests passed. .local/bounded-reads/performance/kind-browser-spec-validation-078.log
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
pnpm spec:validate
```

**Notes**

2.88s; 0 tests passed. .local/bounded-reads/performance/kind-browser-spec-validation-final-078.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreKindPreparation.stories.svelte
```

**Notes**

6.68s; 0 tests passed. .local/bounded-reads/performance/kind-browser-stories-078.log
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreKindPreparation.stories.svelte
```

**Notes**

9.81s; 3 tests passed. .local/bounded-reads/performance/kind-browser-stories-corrected-078.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreKindPreparation.stories.svelte
```

**Notes**

12.07s; 4 tests passed. .local/bounded-reads/performance/kind-browser-stories-final-078.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreKindPreparation.stories.svelte
```

**Notes**

13.55s; 0 tests passed. .local/bounded-reads/performance/kind-browser-stories-refreshed-078.log
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
pnpm web:check
```

**Notes**

12.58s; 0 tests passed. .local/bounded-reads/performance/kind-browser-web-check-078.log
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
pnpm web:check
```

**Notes**

14.54s; 0 tests passed. .local/bounded-reads/performance/kind-browser-web-check-final-078.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- "Before state: the exact native kind controller regression fails because
  settings.prepareKindIndex is absent. It passes after delivery; both new files
  then pass17 tests. The complete direct
  preparation/settings/HTTP/local/automatic-refresh impact surface passes72
  tests in seven files. Existing graph cancellation regression already proves
  explicit retry, no automatic cancel replay, continued bounded observation and
  timer release."
- Native operation and copy preparation sends an exact full operation and kind
  over the already implemented task routes; status and bodyless cancellation
  encode the retained task ID and share its abort signal. Task/reset/transport
  errors never invoke broader inventory or graph work. Controls remain
  native-only and gated by the existing index-task capability and optional typed
  source methods.
- One shared owner admits only one graph/kind task and one request at a time.
  Validate the original task ID, operation, repository, kind, version and
  snapshot, six finite safe nonnegative counters, monotonicity and ready
  completeness before restoring the view. Late responses after navigation are
  suppressed and pending HTTP observation is aborted. Server task ownership and
  expiry stay independent.
- Kind tasks use bounded exponential status backoff even with healthy SSE
  because descriptor windows deliberately emit no events. Exact settlement hints
  wake observation. Graph preparation retains zero periodic reads with healthy
  SSE. Failed cancellation restores the observable running state and remains
  explicitly retryable; do not represent loading or retry as confirmed recovery.
- Use repository-owned Settings command rows and released public Design Core
  Buttons. Keep progress and exact unconfirmed cancellation contextual;
  transient preparation failures use the shared notification service. Local
  profile hides native kind controls. Production remains independent of catalog
  fixtures. No new Community-derived source or provenance change.
- "The first cold story process cannot start: the installed local Design Core
  package snapshot lacks panel-toggle.tokens.css now referenced by its source
  styles. Frozen reinstall fails with the existing pnpmfileChecksum mismatch.
  Offline no-frozen reinstall refreshes the existing local validation dependency
  without downloads; restore tracked pnpm-lock.yaml byte-for-byte. Registry-only
  acceptance remains deferred. The next cold run exposes a wrong new fixture
  stylesheet path; correct it to existing StoreSettings.css. The next run has
  one failed and three passed stories because the new completion assertion
  observes before the500ms task-status read; use findByText for the same exact
  terminal text without changing production timing or assertions. Final fresh
  story process passes all four stories with blocking axe enabled. Preserve
  every startup/failing log."
- Architecture check catches contracts.ts at304 counted lines against300. Move
  the Settings state contract into dedicated store-settings-types.ts and
  re-export its unchanged public name; no baseline relaxation. Final
  architecture check and both web/catalog checks pass with zero diagnostics.
  Initial and final results are recorded separately.
- The first progress check rejects eight test file references without named
  fragments. Replace them with exact path#test-name references; story evidence
  stays in changed files and actual runs. Canonical requirements precede
  implementation. Preserve all518 historical evidence entries verbatim and
  JSON-identically; entry revision is the enclosing Jujutsu change. Focused
  correctness does not establish Linux-scale, RSS, latency, full browser or
  RustFS acceptance.
- User requests pause after committing this slice to discuss implementation.
  Main remains clean on E-READ-076; E-READ-077 and078 landing, main combined
  verification and retained-workspace continuation are pending. Keep the
  temporary workspace open. No installed binary replacement, user
  fixture/reset/migration, Linux checkout change, publication/push or
  visual-baseline change.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Land E-READ-077 and078 into main and verify the focused combined sources there
  when the user resumes. Retain this workspace.
- Remote kind preparation delivery and native application mutation capability
  providers remain; service commands Access still uses complete inventory and
  needs selected kind/family forwarding. Indexed get_copy_records DAG range/path
  queries and larger copy-relation chunk/disk/resumable adjacency construction
  remain. Kind copies preparation currently warms IDs, not relation adjacency.
- General filtered-log continuation preserving indirect/missing edges,
  uncached/ambiguous evolution, indexed code/local application search and
  oversized comparison/change grouping/extreme-filter candidates remain.
  Reassess their priority against the original large-repository browsing flows
  before resuming broad delivery.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification,
  Help/generated-reference/source-review refresh and inherited
  installation-reference drift stay deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified UI slice, then pause at the user's explicit request. Discuss scope and implementation before landing or starting another slice.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

