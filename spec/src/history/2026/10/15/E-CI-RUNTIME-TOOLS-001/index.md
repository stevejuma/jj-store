---
schema: 1
id: E-CI-RUNTIME-TOOLS-001
---

# E-CI-RUNTIME-TOOLS-001

<!-- evidence-field:id markdown -->
## Id

E-CI-RUNTIME-TOOLS-001
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

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kszvxsyy
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair runtime-only protocol compiler provisioning after the first parallel hosted run. Preserve structured RustFS counts and source reports, and accurately distinguish compilation-bearing standalone suites. Shared CI/reporting changes run complete release/orchestration regressions, architecture, actionlint and document gates. Local product Rust/browser suites are out of scope; hosted full qualification remains required.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/actions/qualification-report/action.yml
- .github/workflows/ci-qualification-build.yml
- docs/releases.md
- scripts/ci-artifacts.test.mjs
- scripts/ci-report.mjs
- scripts/ci-workflows.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-RUNTIME-TOOLS-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-RUNTIME-TOOLS-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#embedded runtime installs protoc before
  compiling its Pulsar dependency
- scripts/ci-artifacts.test.mjs#timing reports include RustFS counts and
  preserve startup failures
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
gh run view 38725300550 --repo stevejuma/jj-store --json jobs
```

**Notes**

At source 2d0d9dbde9d2a0013d37940646ce5517fe1f51dd, standard required passed. Seven independent heavy jobs started. External RustFS passed eight probes. Embedded runtime failed to compile pulsar 6.9.0: protoc missing. Embedded acceptance skipped; unrelated jobs continue. Final results will be recorded separately.
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
gh api repos/stevejuma/jj-store/actions/jobs/116394361414/logs --allow-escape-sequences
```

**Notes**

Preserved runtime build diagnostics. The failed producer still uploaded qualification-results-producer-runtime-1.
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
gh run download 38725300550 --repo stevejuma/jj-store --name qualification-results-storage-external-1
```

**Notes**

External fixture logs preserve eight pass rows, but timing.json omitted their counts. Added the existing structured RustFS report to timing counts and artifact uploads.
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
docker info
```

**Notes**

Local Docker socket unavailable. No daemon was started or restarted. Linux apt installation and runtime compilation require hosted confirmation.
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
node --test scripts/ci-artifacts.test.mjs
```

**Notes**

Seven focused artifact/reporting tests passed.
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
pnpm test:release
```

**Notes**

47 workflow, provenance, reporting and release-caller tests passed.
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
pnpm check:architecture && actionlint -oneline
```

**Notes**

1918 modules checked; actionlint clean.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check scripts/ci-report.mjs scripts/ci-artifacts.test.mjs .github/actions/qualification-report/action.yml docs/releases.md
```
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
pnpm check:progress
```

**Notes**

155 tasks / 816 evidence records.
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
pnpm spec:first
```

**Notes**

Reporting changes also map to the governance chapter; added an explicit unchanged-governance confirmation.
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
pnpm progress:history
```

**Notes**

Refreshed M30 backlinks and restored unrelated generated navigation.
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
pnpm check:progress && pnpm spec:first && pnpm check:ui-ownership && pnpm spec:validate --only governance,summary,fileLimits,verification && actionlint -oneline
```

**Notes**

155 tasks / 816 evidence records; 6 protected files / 2 canonical chapters; 3886 chapters / 198 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted retry must verify runtime compilation with protoc and all dependent
  suites.
- Final baseline and six standalone measurements remain pending.
- Existing Kingfisher identity failure remains unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v24.10.0, pnpm 10.32.1, actionlint 1.7.12; hosted Rust 1.99.0, Ubuntu 22.04. Runtime apt installs the distro protobuf-compiler and logs its version.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Artifact/timing formats remain version 1. Timing results now include additive rustfs counts and cases; the existing schema-1 RustFS report is retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run document gates, push with immediate protection restoration, retry complete qualification and collect final baseline/standalone results.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 qualification now explicitly provisions the embedded
  dependency protocol compiler and retains structured real-storage results. Test
  inventory, product contracts and complete release gates are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local checks do not qualify missing runtime
  builds or failing product tests.

```
<!-- /evidence-field -->

