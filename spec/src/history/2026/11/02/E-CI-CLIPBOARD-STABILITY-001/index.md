---
schema: 1
id: E-CI-CLIPBOARD-STABILITY-001
---

# E-CI-CLIPBOARD-STABILITY-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CLIPBOARD-STABILITY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-API-001
- JST-API-002
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/browser/comparison-parents.spec.ts
- spec/src/current/contracts/api/09-operation-differences.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/governance/24-unchanged-contract-chapters.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/comparison-parents.spec.ts#comparison parent dropdown
  searches descriptions and sends the full recorded identity
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
npx --yes pnpm@10.32.1 dev:cargo -- build --locked -p jj-store-http --example browser_preview
```

**Notes**

The first preview build stopped before compilation because this shell did not
expose `cargo`. No source or test failed.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- build --locked -p jj-store-http --example browser_preview
```

**Notes**

The managed build completed the `browser_preview` executable in 62 seconds.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_BROWSER_PREVIEW_EXECUTABLE="$PWD/target/debug/examples/browser_preview" npx --yes pnpm@10.32.1 exec playwright test --config web/playwright.config.ts web/tests/browser/comparison-parents.spec.ts --grep "comparison parent dropdown searches descriptions and sends the full recorded identity"
npx --yes pnpm@10.32.1 web:build
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 web:build
JJ_STORE_BROWSER_PREVIEW_EXECUTABLE="$PWD/target/debug/examples/browser_preview" npx --yes pnpm@10.32.1 exec playwright test --config web/playwright.config.ts web/tests/browser/comparison-parents.spec.ts --grep "comparison parent dropdown searches descriptions and sends the full recorded identity"
```

**Notes**

The first start exposed a missing production browser root. The two build
commands generated that root but returned nonzero during unrelated docs staging:
first for missing Cargo, then for missing `uv`. The next start exposed missing
`.scratch/web-responses.json`. No Playwright assertion ran in these four
prerequisite failures.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node scripts/browser-fixtures.mjs
```

**Notes**

The native HTTP/SSE and mutation browser fixture capture completed.
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
JJ_STORE_BROWSER_PREVIEW_EXECUTABLE="$PWD/target/debug/examples/browser_preview" npx --yes pnpm@10.32.1 exec playwright test --config web/playwright.config.ts web/tests/browser/comparison-parents.spec.ts --grep "comparison parent dropdown searches descriptions and sends the full recorded identity"
```

**Notes**

The exact cold browser regression passed once in 2.5 seconds and the Playwright
run completed in 3.0 seconds. The assertion polls the exact full operation ID
from the browser clipboard; no retry is configured.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte --reporter=default
```

**Notes**

The source Storybook passed one file and nine tests in 6.09 seconds, including
the exact clipboard write and transient copied-state coverage.
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
npx --yes pnpm@10.32.1 exec prettier --check .
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Repository formatting passed. Four selected validators checked 4,583 chapters
and 211 requirements; spec-first passed with one protected file and three
canonical chapters; progress checked 186 tasks and 1,009 evidence records.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Confirm the repaired application browser lane on the hosted stacked PR.
- Qualify the independent Storybook browser-lifetime repair and remaining gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Vitest 4.1.11; Playwright Chromium 153.0.8000.0;
Rust 1.99.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No application, clipboard, fixture, report, receipt or archive format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One cold Playwright assertion and its direct source Storybook contract. Product
code, copied feedback, operation selection, API requests and CI retries are
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Validate formatting, specification ownership and progress; then rebase the
upper CI slices and push the complete stack.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now assigns the durable clipboard effect to cold browser
  acceptance and retains transient copied feedback in source Storybook; product
  behavior is unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Hosted confirmation, Storybook lifecycle,
  heavy qualification and release gates remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
