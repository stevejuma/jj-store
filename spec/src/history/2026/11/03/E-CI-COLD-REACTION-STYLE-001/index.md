---
schema: 1
id: E-CI-COLD-REACTION-STYLE-001
---

# E-CI-COLD-REACTION-STYLE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-REACTION-STYLE-001
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
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41634440310 --json status,conclusion,headSha,jobs,url
gh run download 41634440310 --name qualification-results-cold-cold-3-1 --dir /tmp/jj-store-cold4-41634440310.Me5JAx
```

**Notes**

Complete qualification at merged SHA `a7b9760` passed cold shards 1/4 through
3/4 and exposed a 2.187255859375 CSS-pixel row-edge delta in shard 4/4. The
exact computed primitive padding, width and transform matched. Independent
standard lanes also recorded a transient catalog `Failed to fetch` page error
and a workflow-sidebar navigation readiness miss.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=task-reactions.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The first focused alternative still compared fractional bounds relative to each
primitive row. The 0.0888671875 CSS-pixel difference exceeded the one-decimal
closeness threshold, confirming that no bounding-box comparison belongs in the
acceptance contract.
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
npx --yes pnpm@10.32.1 test:storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

The exact source Storybook file passed all nine stories in 13.98 seconds; the
task reaction action-surface story passed in 703 milliseconds using only exact
computed primitive style comparisons.
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
PATH="$PWD/.local/cargo/bin:/tmp/jj-store-uv.5u5xwl:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_REAL_TEST=task-reactions.spec.ts npx --yes pnpm@10.32.1 web:test:real
```

**Notes**

The exact cold file passed in 8.9 seconds after 210.516 seconds of fresh fixture
preparation. Embedded resources measured 8.054-second cold and 7.889-second
restart startup with 421,072 KiB resident memory.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; Playwright
1.63.0; uv 0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, route codec, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Task-comment leading-icon acceptance in the source story and exact cold browser
test. Production menu composition and behavior are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the focused repair, then rerun complete integrated qualification. Recheck
the two independent standard-lane timing failures before changing their tests.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: Source-story and cold-browser checks still verify that Design Core menu
  primitives align Copy and Edit leading icons without consumer CSS. Exact row
  padding and gap plus icon margin, width and transform replace all fractional
  geometry comparisons.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.4 and M30 remain In progress. The exact reaction acceptance passes,
  while shared-package, visual, standard timing and complete integrated release
  gates stay independent and open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
