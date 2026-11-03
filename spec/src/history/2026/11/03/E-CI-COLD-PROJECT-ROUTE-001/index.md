---
schema: 1
id: E-CI-COLD-PROJECT-ROUTE-001
---

# E-CI-COLD-PROJECT-ROUTE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-PROJECT-ROUTE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-WORK-005
- JST-UI-015
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/real/project-polish.spec.ts
- spec/src/current/contracts/ui/20-repository-linked-projects-documents-and-activity-filters.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/project-polish.spec.ts#repository documents linked projects and
  Activity filters retain native scope
<!-- /evidence-field -->

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
gh run view 41537333075 --json conclusion,headSha,url,jobs
unzip -p /tmp/jj-store-cold-failures.RsC1dN/qualification-results-cold-cold-2-1/web/test-results-real-3-4/project-polish-repository--39f01-filters-retain-native-scope/trace.zip 1-trace.trace
```

**Notes**

Integrated qualification at `b751c03` failed to find the repository document
preview. The trace proved `main` was selected on Branches, then the intentional
page-selection reset entered Overview on `conflict/demo`; the test depended on
cross-page state the route contract does not retain.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=project-polish.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The first focused correction entered `/files?bookmark=main` directly. Cold
admission correctly rejected a repository route without `at_op` and returned to
the Projects hub, confirming that operation pinning must remain explicit.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=project-polish.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold file passed in 21.3 seconds after 212.901 seconds of fresh
fixture preparation. Embedded resources measured 7.629-second cold and
8.179-second restart startup with 418,520 KiB resident memory.
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
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Changed-file formatting, selected specification validation, spec-first
enforcement and progress-ledger checks passed.
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

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; Playwright 1.63.0; uv
0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, route codec, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The cold repository-document acceptance route only. Production navigation,
reference selection, document rendering and project behavior are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Repair the independent reaction-menu geometry failure, then rerun complete
integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The same test still verifies all repository documents, the operation pin,
  linked Projects, Activity scope, Back/reload behavior and accessibility. It now
  enters Overview through the supported explicit operation/bookmark route instead
  of relying on a Branches selection across a page change.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.4 and M30 remain In progress. The exact cold project/document gate
  passes, while reaction geometry, visual/shared-package gates and complete
  release qualification remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
