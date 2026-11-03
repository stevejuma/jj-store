---
schema: 1
id: E-CI-COLD-REACTION-PRIMITIVE-001
---

# E-CI-COLD-REACTION-PRIMITIVE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-COLD-REACTION-PRIMITIVE-001
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
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41604607037 --json status,conclusion,headSha,jobs,url
```

**Notes**

Complete qualification at merged SHA `a9c95dc` passed standard, build, backend,
contract, storage, native, asset and profile-restart gates. The reaction check
measured a 1.656005859375 CSS-pixel absolute SVG delta against its fixed 1.25
bound, alongside three independent cold response-body races.
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

The exact source Storybook file passed all nine stories in 14.81 seconds; the
task action-surface interaction passed in 810 milliseconds.
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

The exact cold file passed in 9.0 seconds after 222.764 seconds of fresh fixture
preparation. Embedded resources measured 8.248-second cold and 8.504-second
restart startup with 421,236 KiB resident memory.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; Playwright
1.63.0; uv 0.9.28.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, route codec, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Task-comment leading-icon acceptance geometry in the source story and exact cold
browser test. Production menu composition and behavior are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the two focused cold repairs, then rerun complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The source-story and cold-browser checks still verify that Design Core
  menu primitives align Copy and Edit leading icons without consumer CSS. The
  primitive's leading padding, icon width and transform are now compared
  directly, with a two-pixel row-edge guard for material indentation.
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
