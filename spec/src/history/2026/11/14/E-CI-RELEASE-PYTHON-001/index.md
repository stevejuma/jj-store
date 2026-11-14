---
schema: 1
id: E-CI-RELEASE-PYTHON-001
---

# E-CI-RELEASE-PYTHON-001

<!-- evidence-field:id markdown -->
## Id

E-CI-RELEASE-PYTHON-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/publish-release.yml
- scripts/ci-workflows.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/14/E-CI-RELEASE-PYTHON-001/index.md
- spec/src/history/2026/11/14/E-CI-RELEASE-PYTHON-001/SUMMARY.nav.md
- spec/src/history/2026/11/14/index.md
- spec/src/history/2026/11/14/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#successful main builds publish bounded source and
  rolling image tags
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
GitHub Actions run 43594283560 job identity
```

**Notes**

The latest qualified-release dispatch stopped before qualification because the
hosted Python 3.10 default did not provide `tomllib`. No archive, package,
registry, image or release publication job ran.
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
node --test --test-name-pattern='successful main builds publish bounded source and rolling image tags' scripts/ci-workflows.test.mjs
```

**Notes**

The pre-change workflow regression passed one test because it did not yet
require a Python runtime before release metadata validation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node --test --test-name-pattern='successful main builds publish bounded source and rolling image tags' scripts/ci-workflows.test.mjs
```

**Notes**

The test-only change failed because the identity job had no setup-python step,
reproducing the missing release-runtime contract locally.
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
node --test --test-name-pattern='successful main builds publish bounded source and rolling image tags' scripts/ci-workflows.test.mjs
```

**Notes**

The exact regression passed after the identity job pinned Python 3.12 before
the coordinated version check. Source, tag and publication checks are unchanged.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:release
```

**Notes**

All 89 release, workflow, artifact, package and automation tests passed.
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
actionlint .github/workflows/publish-release.yml
```

**Notes**

The repository's checksum-pinned actionlint 1.7.12 accepted the changed workflow.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm format:check
```

**Notes**

Prettier reported the two edited canonical ledgers. Both were formatted without
changing their 400-line and 399-line bounds.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm format:check
```

**Notes**

Rust formatting and repository-wide Prettier checks passed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

The first evidence draft used a prose test name and claimed pass while retaining
hosted gaps. The record was corrected to a named test reference and pending.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

The next invocation exposed the incomplete named-test reference; the exact test
identity was added without changing product behavior.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

The valid record then proved the protected workflow also maps to JST-GOV-002.
The canonical verification-scope chapter now owns that release-tooling surface.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

The gate passed two protected files against both required canonical chapters.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Progress validation passed 186 tasks and 1,044 evidence records.
<!-- /run -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Merge after the main-image repair and run qualified release v0.2.0 from the final main source.
- Hosted qualification and all downstream publication jobs remain required.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Python 3.12; Node.js 22 in the hosted identity job; release v0.2.0 pending.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub Actions YAML and coordinated Cargo TOML metadata; no persisted product
format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

This slice repairs only the qualified-release identity preflight. It pins the
parser runtime already used by the repository's shared setup action and does not
weaken immutable-source, version, qualification or publication gates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run the complete qualified-release workflow from the final protected main SHA.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now requires a pinned Python 3.12 parser runtime before coordinated release metadata validation.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress until the hosted qualified release and every downstream publication gate pass.
```
<!-- /evidence-field -->
