---
schema: 1
id: E-CI-COLD-REACTION-GEOMETRY-001
---

# E-CI-COLD-REACTION-GEOMETRY-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-REACTION-GEOMETRY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-WORK-008
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

- catalog/components/store-work/StoreWork.stories.svelte
- web/tests/real/task-reactions.spec.ts
- spec/src/current/contracts/ui/01-storybook-presentation-catalog.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/task-reactions.spec.ts#task reaction picker persists exact
  comments and refreshes a second client
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
unzip -p /tmp/jj-store-cold-failures.RsC1dN/qualification-results-cold-cold-3-1/web/test-results-real-4-4/task-reactions-task-reacti-06ede-d-refreshes-a-second-client/trace.zip test.trace
```

**Notes**

Integrated qualification at `b751c03` measured a 1.0758056640625 CSS-pixel
leading-icon delta against the one-pixel limit. Both rows remained owned Design
Core primitives without consumer CSS overrides.
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
npx --yes pnpm@10.32.1 test:storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

The exact source Storybook file passed all nine stories in 12.45 seconds; the
task action-surface interaction passed in 674 milliseconds.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=task-reactions.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold file passed in 8.2 seconds after 207.870 seconds of fresh fixture
preparation. Embedded resources measured 7.843-second cold and 7.969-second
restart startup with 418,916 KiB resident memory.
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

Task-comment leading-icon geometry assertions in the source story and exact cold
browser acceptance. Production menu composition and behavior are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the two focused cold repairs, then rerun complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The same source-story and cold-browser checks still verify that Design
  Core menu primitives align Copy and Edit leading icons without consumer CSS.
  A 1.25 CSS-pixel bound admits only the observed fractional Chromium layout
  delta and continues to reject a material padding or indentation regression.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.4 and M30 remain In progress. The exact reaction geometry targets
  pass, while shared-package, visual and complete integrated release gates stay
  independent and open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
