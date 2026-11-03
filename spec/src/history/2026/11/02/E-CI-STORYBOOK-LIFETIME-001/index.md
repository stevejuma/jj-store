---
schema: 1
id: E-CI-STORYBOOK-LIFETIME-001
---

# E-CI-STORYBOOK-LIFETIME-001

<!-- evidence-field:id markdown -->
## Id

E-CI-STORYBOOK-LIFETIME-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
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

- scripts/ci-browser.mjs
- scripts/ci.test.mjs
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/governance/24-unchanged-contract-chapters.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci.test.mjs#browser orchestration selects one suite per lane and
  forwards real shard arguments
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte --reporter=default
```

**Notes**

The exact hosted shard-3 assertion passed locally: one story file and three
tests passed in 11.95 seconds. No retry or assertion change was used.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/3 --reporter=default
```

**Notes**

The unchanged long-lived baseline passed all 72 files and 333 tests in 322.26
seconds, including AgentChatChanges. Hosted PR 37 had instead closed the shard-1
browser after 69 files and transiently missed the shard-3 assertion.
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
node --test --test-name-pattern='browser orchestration selects one suite per lane' scripts/ci.test.mjs
```

**Notes**

The initial focused orchestration regression passed. A displayed `2/3` shard
mapped to fresh `2/6` and `5/6` processes with distinct JUnit paths. Subsequent
local lifecycle evidence showed this first bound was insufficient.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node --test scripts/ci.test.mjs
```

**Notes**

Twelve of thirteen tests passed, including the changed orchestration. The
unrelated package-impact test could not spawn `cargo` because the current shell
does not expose a Cargo executable; no product or CI assertion failed.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/6 --reporter=default
```

**Notes**

The first two-process bound passed 36 files and 200 tests in 151.62 seconds.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=6/6 --reporter=default
```

**Notes**

The complementary process passed 35 files and 160 tests before Vite could not
dynamically import the final MarkdownReading story. This reproduced the
late-process lifecycle failure and rejected the initial two-segment design.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/MarkdownReading.stories.svelte --reporter=default
```

**Notes**

The story that failed to import at the end of Run 6 passed cold: one file and
one test passed in 3.85 seconds. No story or product assertion changed.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/12 --reporter=default
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=6/12 --reporter=default
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=9/12 --reporter=default
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook --shard=12/12 --reporter=default
```

**Notes**

All four replacement processes for displayed shard `3/3` passed. They covered
72 files exactly once: 3/12 passed 87 tests with two skipped in 60.11 seconds;
6/12 passed 93 in 79.02 seconds; 9/12 passed 82 in 79.86 seconds; and 12/12
passed 80 in 85.63 seconds.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

All 88 shared CI, validation, release and automation-governance tests passed.
The orchestration regression proves complete twelve-segment mapping, unique
JUnit paths and fail-fast behavior without replay.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
```

**Notes**

The first spec-first run correctly rejected the protected script change because
its JST-GOV-002 canonical governance confirmation was missing.
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
npx --yes pnpm@10.32.1 exec prettier --check .
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Repository formatting passed. Four selected validators checked 4,585 chapters
and 211 requirements; spec-first passed with two protected files and two
canonical chapters; progress checked 186 tasks and 1,010 evidence records.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete hosted stacked-PR confirmation.
- Confirm the parent clipboard repair in the hosted application browser lane.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Vitest 4.1.11; Playwright Chromium 153.0.8000.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

JUnit XML paths change from one `storybook.xml` per job to one uniquely named
report per twelve-way segment. Selection, archive, receipt and evidence formats
are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Source Storybook process lifetime and report naming only. The three visible
jobs, story inventory, assertions, retries, UI product behavior and other lanes
are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Validate formatting and policy documents, then push the updated PR 37 head and
propagate it through PR 38.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now requires twelve complete, disjoint Storybook process
  lifetimes with distinct reports inside the existing three-job source gate;
  JST-GOV-002/011 ownership and evidence gates are explicitly retained.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This bounds one UI lifecycle failure surface;
  hosted confirmation for this and the parent clipboard repair, heavy
  qualification and release gates remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
