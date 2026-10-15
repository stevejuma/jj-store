---
schema: 1
id: E-CI-CACHE-TRIAL-001
---

# E-CI-CACHE-TRIAL-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CACHE-TRIAL-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Bounded compilation-cache trial scaffolding, enclosing Jujutsu change ymxozwno. Existing rust-cache baseline was corrected in urkpvmsr. No normal checks adopt sccache without three matched warm runs meeting the specified threshold. Shared CI tooling warrants the complete CI/release/governance/spec regression suites and workflow/document checks.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/ci-cache-build.yml
- .github/workflows/ci-cache-trial.yml
- .github/workflows/ci.yml
- docs/releases.md
- scripts/ci-cache-compare.mjs
- scripts/ci-cache-sample.mjs
- scripts/ci-cache.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-CACHE-TRIAL-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-CACHE-TRIAL-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-cache.test.mjs#cache adoption requires at least ten percent median
  complete-job improvement
- scripts/ci-cache.test.mjs#cache comparisons reject missing samples cold caches
  changed inventories sources and failed tests
- scripts/ci-cache.test.mjs#cache trial is opt-in branch-only bounded and uses
  isolated read-only measured caches
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
node --test scripts/ci-cache.test.mjs
```

**Notes**

Three regressions pass: threshold boundaries, invalid experiments, dispatch/cache scope and pinned tooling.
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
pnpm release:check
```

**Notes**

38 CI/release/automation regressions passed; all 20 Cargo package containment and coordinated metadata checks passed.
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
actionlint
```

**Notes**

All standard, manual, publication and nested comparison workflows passed.
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
pnpm check:architecture
```

**Notes**

Shared script boundaries and source limits passed.
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
pnpm test:governance
```

**Notes**

71 tests passed.
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
pnpm test:spec
```

**Notes**

28 tests passed.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3870 chapters and 198 requirements passed.
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
pnpm check:progress
```

**Notes**

155 tasks and 812 records passed.
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

7 protected files and 2 canonical topics passed.
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
pnpm exec prettier --check .
```

**Notes**

Complete formatter gate passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Push scaffold after the affected/cache slice hosted outcome; dispatch
  branch-only trial and collect six warm samples plus both backend test
  outcomes.
- Record the cache adoption/removal decision from complete build-job medians.
- Full manual qualification awaits default-branch workflow availability;
  existing heavy failure remains unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99.0; Nextest 0.9.148; sccache v0.18.0; mozilla-actions/sccache-action fc920bf0ec8de6ee65d409111f7ec508035751ba; existing pinned Swatinem/rust-cache; Ubuntu 22.04; Node 22.23.3; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Explicit cache_trial dispatch adds no normal or scheduled work. Source-specific branch caches isolate the warmup; measured dependency and compiler caches are restore-only. Samples preserve packages, test identities, partitions, build duration and compiler statistics; decision uses complete build-job wall time including setup, cache and artifacts.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run the matched hosted trial, verify PR cache restoration, and record a fresh measured decision. Do not adopt sccache from scaffold tests alone.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now defines the bounded experiment and at-least-10-percent
  adoption threshold. Routine merge/release and all heavy gates remain
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; no publication, merge or qualification gap is
  promoted.

```
<!-- /evidence-field -->

