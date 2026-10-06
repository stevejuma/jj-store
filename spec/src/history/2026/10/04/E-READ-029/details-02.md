---
schema: 1
id: E-READ-029
---

# E-READ-029 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repository-visibility.test.ts (before)
```

**Notes**

Exact regression observes100metadata directory calls for100discovered repositories before any visible-card lease, rather than0. visible-cards-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-repositories.test.ts (after)
```

**Notes**

9pass/663ms. Discovery with0metadata requests, global two-read admission including ignored cancellation, late-result suppression, visible-only refresh, hidden stale reload, deduplicated/ref-counted observation, reset isolation and unchanged error/auth/pin behavior. visible-cards-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/visible-repository.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/projects-hub.test.ts
```

**Notes**

41direct consumer tests pass/5.22s. Includes local review/work transport, Projects restoration, viewport action entry/update/exit/unmount and cancellation during shared local snapshot observation with no subsequent collaboration/graph work. visible-cards-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/visible-repository.test.ts (final)
```

**Notes**

13pass/723ms after final failure cleanup aborts leftover sibling metadata transports while retaining the error episode. Re-runs complete changed viewport/controller subset; direct local/hub consumers were unchanged since their41-test pass. visible-cards-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

9stories pass/24.69s through a fresh browser/Vite process. Directory and linked cards, list/grid/narrow/read-only interactions, link/add focus/dialog behavior and scoped failed metadata/dismissal recovery. Accessibility/runtime diagnostic guards remain active. visible-cards-stories.log. Visual baselines remain pending.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Web/catalog0errors/0warnings,190catalog modules; ownership1validator/23chapters/172requirements; architecture971modules. web-check-029-final.log/catalog-check-029.log/ownership-029.log/architecture-029.log. UI-only slice runs no Rust lanes.
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
pnpm spec:first (initial)
```

**Notes**

Protected controller/source map additionally requires api.md; recorded independent directory-read/cancellation/partial-activity boundary there without changing transport schemas. spec-first-029.log.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

13protected files/3canonical chapters;5validators/23chapters/172requirements. spec-first-029-after.log/spec-029.log.
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
pnpm exec prettier --write <changed controller/UI/test/canonical/plan paths>; pnpm exec prettier --check <same paths>
```

**Notes**

Exact changed paths in this evidence files list excluding the historical ledger. format-029.log/format-tests-029.log/format-cancellation-029.log/format-final-029.log/format-check-029.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte -t "Failed repository details retain successful metadata"
```

**Notes**

Story title filter selects0targets/6skipped; not acceptance evidence. Corrected to the complete directly affected directory story family. visible-cards-error-story-final.log.
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
pnpm check:architecture (final cancellation addition)
```

**Notes**

http-local-store-data-source.ts reaches301counted lines. Extracted the existing exact repository/profile/metadata-pin validator to local-review-snapshot.ts and retained its calls. No budget exemption. architecture-029-final.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial E-READ-029 entry)
```

**Notes**

Seven test references lacked required exact selectors; repaired all new-entry references without altering historical evidence. progress-029.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (named-reference repair)
```

**Notes**

Story declarations are not test definitions supported by the ledger checker. Removed three Storybook references from tests; story files and actual browser commands remain recorded. progress-029-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/visible-repository.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/projects-hub.test.ts
```

**Notes**

42 tests/5 files pass after final cancellation and validator extraction; visible-cards-consumers-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

6 stories pass through fresh browser/Vite/40.53s; visible-cards-stories-final.log. Corrects the zero-target filter probe. Visual baselines remain pending.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final extracted validator checks: web/catalog0errors/0warnings;190catalog modules,972production modules; ownership1validator/23chapters/172requirements. Logs *-029-committed.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (after validator extraction)
```

**Notes**

New helper additionally maps to architecture.md. Recorded viewport/controller and unchanged pin-validation boundary there; spec-first-029-committed.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Final15protected files/5chapters;5validators/23chapters/172requirements and138task bookkeeping pass. Logs spec-first-029-complete.log/spec-029-complete.log/progress-029-complete.log. No milestone promotion.
<!-- /run -->
<!-- /evidence-field -->

