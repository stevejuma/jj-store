---
schema: 1
id: E-CI-CHANGESETS-PIN-002
---

# E-CI-CHANGESETS-PIN-002

<!-- evidence-field:id markdown -->
## Id

E-CI-CHANGESETS-PIN-002
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

- .github/workflows/release.yml
- scripts/ci-workflows.test.mjs
- spec/src/history/2026/11/14/E-CI-CHANGESETS-PIN-002/index.md
- spec/src/history/2026/11/14/E-CI-CHANGESETS-PIN-002/SUMMARY.nav.md
- spec/src/history/2026/11/14/index.md
- spec/src/history/2026/11/14/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#Changesets CLI v2 keeps its compatible release action
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
GitHub Actions run 43620801397 job version
```

**Notes**

Changesets action v2.1.2 rejected the CLI-v2 inputs and then stopped because
Changesets CLI v2 is unsupported. No version pull request or artifact was
published.
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
node --test --test-name-pattern='Changesets CLI v2 keeps its compatible release action' scripts/ci-workflows.test.mjs
```

**Notes**

The new focused regression failed on the incompatible action SHA before the
workflow pin changed.
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
node --test --test-name-pattern='Changesets CLI v2 keeps its compatible release action' scripts/ci-workflows.test.mjs
```

**Notes**

The exact regression passed after restoring the v1.9.0 action SHA and retaining
all five CLI-v2 input names.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:release
```

**Notes**

All 90 release, workflow, artifact, package and automation tests passed.
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
actionlint 1.7.12 .github/workflows/release.yml
```

**Notes**

The repository's checksum-pinned actionlint release accepted the restored
workflow.
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
pnpm exec prettier --check .github/workflows/release.yml scripts/ci-workflows.test.mjs spec/src/history/2026/11/14/E-CI-CHANGESETS-PIN-002/index.md spec/src/history/2026/11/14/index.md spec/src/history/2026/11/14/SUMMARY.nav.md
```

**Notes**

All changed YAML, JavaScript and Markdown files passed Prettier.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

The progress ledger accepted 186 tasks and 1,045 evidence records before this
record was finalized.
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

Changesets action v1.9.0; @changesets/cli 2.31.1; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No package, Cargo, archive, container or tag format changes. The action pin and
its workflow regression are the only executable changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Restore the canonical CLI-v2-compatible action after an automated grouped
Actions update replaced it with an incompatible major version.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the pin, confirm that Changesets opens or updates the version pull
request, and retain qualified publication as a separate gate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 and the owning public-distribution chapter already require
  Changesets action v1.9.0 with CLI v2. This restores that declared contract.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Version orchestration is restored, while main
  image, complete qualification and coordinated publication remain separate.
```
<!-- /evidence-field -->
