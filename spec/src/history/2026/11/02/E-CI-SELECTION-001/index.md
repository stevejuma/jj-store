---
schema: 1
id: E-CI-SELECTION-001
---

# E-CI-SELECTION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-SELECTION-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-007
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
- scripts/ci-build.mjs
- scripts/ci-evidence.mjs
- scripts/ci-evidence.test.mjs
- scripts/ci-inventory.mjs
- scripts/ci-inventory.test.mjs
- scripts/ci-packages.mjs
- scripts/ci-plan.mjs
- scripts/ci-required.mjs
- scripts/ci-selection.test.mjs
- scripts/ci-tests.mjs
- scripts/ci-workflows.test.mjs
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs
- spec/src/current/contracts/cli/37-conversation-metadata.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-selection.test.mjs#shadow domains separate Help catalog contracts
  and targeted backend
- scripts/ci-inventory.test.mjs#targeted backend inventory uses two complete
  disjoint shards
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_stale_generation_and_unknown_publication_recover_without_new_inference
- scripts/ci-evidence.test.mjs#targeted backend gate binds both shards to the
  selected heavy packages
- scripts/ci-workflows.test.mjs#aggregates reject missing, failed, canceled and
  unexpectedly skipped prerequisites
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
node --test scripts/ci-selection.test.mjs scripts/ci-inventory.test.mjs scripts/ci-evidence.test.mjs scripts/ci-workflows.test.mjs
```

**Notes**

Focused planner, archive inventory, execution evidence and workflow validation:
31 tests passed. The process sandbox blocked existing Cargo/Git child fixtures
after successful execution, so the unchanged tests were rerun outside that
process sandbox.
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
node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

Complete CI, validation, release-policy and automation regression set: 87 tests
passed, including versioned domain planning, partial targeted package admission,
two-shard inventories and exact execution evidence.
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
/tmp/actionlint-check/actionlint -shellcheck= -pyflakes=
```

**Notes**

Checksum-verified pinned actionlint 1.7.12 accepted the updated workflow schema
and expressions. Shellcheck and Pyflakes were unavailable and were not run.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
```

**Notes**

All four validators passed across 4580 chapters and 211 requirements.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @; node scripts/check-progress.mjs
```

**Notes**

Spec-first mapped 13 protected files to four canonical chapters. Progress
bookkeeping passed with 186 tasks and 1007 evidence records; M30 remains In
progress.
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
cargo nextest run -p jj-store --test agent_execution --profile heavy -E 'test(=metadata_tests::recovery::metadata_stale_generation_and_unknown_publication_recover_without_new_inference)'
```

**Notes**

The exact hosted shard-2 failure was reproduced locally. The corrected
retained-or-stale source assertion passed three times, including two concurrent
heavy-profile repetitions.
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
gh pr checks 35 --repo stevejuma/jj-store
```

**Notes**

The final-stack hosted run passed selection, the standard and affected
producers, all four standard Rust shards, both affected-backend shards and their
evidence gates at source `b6bfd2edf35f`. Existing UI workflow failures remain
outside the new selection and targeted-backend lanes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Representative shadow observations for source UI, browser, Help, catalog and
  contract changes.
- A complete fresh canonical-main scheduled run and a separate reviewed
  promotion before any shadow UI decision can skip a required lane.
- Measured PR latency after the stacked workflow runs on hosted infrastructure.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.19.0; Rust 1.99.0; JJ 0.45.1; actionlint 1.7.12; retained Nextest archive inventory format 1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Selection schema version 1 in shadow mode. Existing archive inventory format 1
and source/run/attempt/package/digest/JUnit evidence bindings are retained for
the new targeted cohort.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Versioned shadow domain planning and blocking affected-package coverage for
`jj-store`, `jj-store-server` and `jj-store-capture`. Existing broad standard
Rust and UI selection and complete heavy qualification remain required.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Collect representative shadow observations, complete a fresh canonical-main
scheduled run, and measure cold/warm latency before considering any narrower UI
selection.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now records a versioned shadow selection plan and requires
  exact blocking evidence when affected backend packages are selected. Existing
  broad PR and complete qualification obligations remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local tooling regressions do not establish the
  hosted targeted run, representative shadow observations, latency improvement
  or complete release qualification.
```
<!-- /evidence-field -->
