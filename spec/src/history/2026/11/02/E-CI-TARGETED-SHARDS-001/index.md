---
schema: 1
id: E-CI-TARGETED-SHARDS-001
---

# E-CI-TARGETED-SHARDS-001

<!-- evidence-field:id markdown -->
## Id

E-CI-TARGETED-SHARDS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
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

- .github/workflows/ci.yml
- scripts/ci-packages.mjs
- scripts/ci-evidence.mjs
- scripts/ci-evidence.test.mjs
- scripts/ci-inventory.test.mjs
- scripts/ci-workflows.test.mjs
- docs/reference/ci-validation-plan-status-2026-11-02.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-evidence.test.mjs#targeted backend gate binds both shards to the
  selected heavy packages
- scripts/ci-inventory.test.mjs#targeted backend inventory uses two complete
  disjoint shards
- scripts/ci-workflows.test.mjs#standard checks never schedule heavy
  qualification and preserve the protected gate
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
gh stack merge 38 --yes --merge
```

**Notes**

GitHub atomically merged PRs 35, 37 and 38 into `main` at `b751c03`. Their
required checks passed before the merge.
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
gh run download 41530852380 -n qualification-results-targeted-build-1 -n qualification-results-targeted-1-1 -n qualification-results-targeted-2-1 -D /tmp/jj-store-timing.6iRZj6
jq -c . /tmp/jj-store-timing.6iRZj6/qualification-results-targeted-1-1/.release/ci-results/commands.jsonl
jq -c . /tmp/jj-store-timing.6iRZj6/qualification-results-targeted-2-1/.release/ci-results/commands.jsonl
```

**Notes**

The exact hosted stack-tip artifacts report 887.948 and 1126.407 command
seconds, including 881.32 and 1116.83 seconds of heavy-profile test execution.
The shards held 626 and 635 passing tests respectively. JUnit totals were
845.548 and 1080.389 test-seconds; shard 2 contained one 85.7-second test.
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
gh workflow run ci-heavy.yml --ref main -f ref=b751c03beebb5bc49059726619495411622bafb6
```

**Notes**

Integrated full qualification was dispatched as GitHub run 41537333075 against
the exact atomic merge SHA. Its eventual result remains separate evidence.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci-evidence.test.mjs scripts/ci-inventory.test.mjs scripts/ci-workflows.test.mjs
```

**Notes**

All 18 focused workflow, inventory and aggregate-evidence regressions passed.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

All 88 shared CI, validation, release and automation-governance regressions
passed.
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
/tmp/jj-store-actionlint.KVppJF/actionlint -shellcheck= -pyflakes=
npx --yes pnpm@10.32.1 exec prettier --check .
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Workflow lint, repository formatting, selected specification validators,
spec-first enforcement and progress-ledger validation passed.
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
gh run download 41538302895 -n qualification-results-targeted-1-1 -n qualification-results-targeted-2-1 -n qualification-results-targeted-3-1 -n qualification-results-targeted-4-1 -D /tmp/jj-store-pr39-timing.tpKs1u
for shard in 1 2 3 4; do jq -s -c --argjson shard "$shard" '{shard:$shard,command_seconds:.[0].seconds,exit_code:.[0].exit_code}' "/tmp/jj-store-pr39-timing.tpKs1u/qualification-results-targeted-$shard-1/.release/ci-results/commands.jsonl"; done
gh api repos/stevejuma/jj-store/actions/runs/41538302895/jobs?per_page=100
```

**Notes**

All four reports passed the same 1,261 tests in partitions of 327, 321, 299 and
314. Command times were 710.868, 542.950, 824.422 and 464.688 seconds. Against
the two-shard baseline, the maximum command fell 26.8%, but aggregate command
time rose 26.2%, affected-job runner execution rose from 2,345 to 3,278 seconds
(39.8%), and complete PR wall time rose from 2,017 to 2,042 seconds. The
production cohort therefore retains two shards.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; Nextest 0.9.148;
GitHub CLI stack extension 0.3.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The hosted trial produced four partitions and reports `targeted-1` through
`targeted-4`; production retains two partitions and `targeted-1` through
`targeted-2`. Inventory format 1, source/digest/package identity, heavy JUnit
profile and archive format are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Affected-backend partition measurement and exact aggregate evidence only.
Selection, retained partition count, test identities, heavy-profile execution,
retries, product behavior and complete standard/heavy qualification are
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the measured no-change decision, then evaluate alternatives that reduce the
slow tail without increasing complete PR wall time or runner consumption.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 retains two complete disjoint affected-backend reports. The
  rejected four-partition trial changed neither coverage nor evidence authority.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Integrated full qualification and release gates
  remain outstanding; the rejected trial establishes no milestone promotion.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
