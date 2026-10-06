---
schema: 1
id: E-UI-047
---

# E-UI-047 details 3

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
node .scratch/review-activity-main-state.mjs before
```

**Notes**

Three projects plus WorkStore pages, code snapshot and configuration fingerprints captured.
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
node .scratch/review-activity-main-state.mjs after
```

**Notes**

Exact native state and configuration equality after restart; no reset, seed or repository writes.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/project-polish.spec.ts web/tests/real/projects-activity.spec.ts
```

**Notes**

Existing Activity/review test passes; new linked-project count ran before async rows arrived. Added wait for first project before counting. .scratch/project-polish-live.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/project-polish.spec.ts
```

**Notes**

Cold page/reload: five shared document tabs, inline code, indicator; repository tabs and linked-project detail; Activity-first order, searchable native filter, pinned commit Back/reload, honest empty state and axe. .scratch/project-polish-live-final.log.
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

Protected scope/canonical chapters passed. .scratch/project-polish-spec-first.log.
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

4 validators / 21 chapters / 131 requirements. .scratch/project-polish-spec-validate.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/native-activity.test.ts web/tests/vite/native-activity-filter.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

Final 56 direct tests including both added repository/scope regressions. .scratch/project-polish-direct-units-final.log.
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
pnpm catalog:check
```

**Notes**

Final after repaired safe-rendering assertions: zero errors/warnings and 98 modules. .scratch/project-polish-catalog-complete.log.
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
pnpm spec:first
```

**Notes**

Final 28 protected files / 3 canonical chapters. .scratch/project-polish-spec-first-final.log.
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
pnpm spec:validate
```

**Notes**

Final four validators / 21 chapters / 131 requirements. .scratch/project-polish-spec-validate-final.log.
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
pnpm check:progress
```

**Notes**

103 tasks, evidence bookkeeping. .scratch/project-polish-progress.log.
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
python3 - <<'PY'
import subprocess
files=subprocess.check_output(['jj','--no-pager','diff','--name-only'],text=True).splitlines()
subprocess.run(['pnpm','exec','prettier','--check',*files],check=True)
PY
```

**Notes**

All 32 changed files formatted. Targeted Prettier writes and follow-ups logged in .scratch/project-polish-{format,format-followup,preview-format,live-format,live-format-final,scope-format,ledger-format,ledger-format-final}.log. .scratch/project-polish-format-check.log.
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

One UI/controller integration surface with direct Markdown, repository-project, activity, route/history and bootstrap consumers. Focused units, direct stories/accessibility, type/architecture/spec checks and cold owned-demo browser acceptance. Root aggregates, full Storybook and unrelated Rust test suites are out of scope under targeted policy. CLI rebuild is part of the explicitly authorized dev restart, not Rust acceptance. Settings workspace untouched.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No Rust, API schema, WAL, manifest or persistent model change. Native V1 snapshots, IDs and comment semantics unchanged. Browser-only activity enum all/agent/unassisted is validated and retained in Projects routes. Metadata classification never invents associations or reads session bodies. Public Community Markdown safe policy omits executable HTML while Source retains bytes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; existing pinned Community and Design Core public exports, Svelte and Chromium.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused regressions. M11.4 shared-release qualification and visual baseline approval remain separate.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baseline approval remains pending.
- Shared release and independent deployment qualification remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/project-polish-*.log and native-state captures; web/test-results-real/project-polish-repository--39f01-filters-retain-native-scope/{repository-markdown,project-activity-filter}.png inspected. Fresh Storybook/Vitest processes and freshly built served bundle; no baselines changed.
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/community-projects/RepositoryDocuments.stories.svelte#All
  document tabs use the shared Markdown preview
- catalog/components/store-work/ProjectsActivityControls.stories.svelte#Hub
  activity filters recorded native context
- catalog/components/store-work/ProjectsActivityControls.stories.svelte#Project
  activity starts before tasks and filters linked repositories
- catalog/components/store-work/ProjectsActivityControls.stories.svelte#Repository
  projects retain repository tabs and reuse Community rows
<!-- /evidence-field -->

