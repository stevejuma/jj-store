---
schema: 1
id: E-CI-CHANGESETS-ACTION-001
---

# E-CI-CHANGESETS-ACTION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CHANGESETS-ACTION-001
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

2026-11-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/release.yml
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/06/E-CI-CHANGESETS-ACTION-001/index.md
- spec/src/history/2026/11/06/SUMMARY.nav.md
- spec/src/history/2026/11/06/index.md
- spec/src/history/2026/11/SUMMARY.nav.md
- spec/src/history/2026/11/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
gh workflow run release.yml --repo stevejuma/jj-store --ref main
```

**Notes**

Run 42289876318 failed in one minute. Changesets action v2.1.2
(`ae32849d5ba541f9ae29e40e22a623bc13562f51`) rejected the `version` and
`publish` inputs and does not support Changesets CLI v2. No Version Packages
pull request was opened.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
GitHub Actions run 42290359166 on 42e1597b2a04
```

**Notes**

Specification and release policy failed at the spec-first gate because
`.github/workflows/release.yml` changed without the owning public-distribution
chapter. The preflight gate failed because that required check failed. Progress
evidence passed before the spec-first failure.
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
node --test scripts/ci-workflows.test.mjs
npx --yes pnpm@10.32.1 exec prettier --check .github/workflows/release.yml spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/progress/M30/index.md spec/src/history/2026/11/06/E-CI-CHANGESETS-ACTION-001/index.md spec/src/history/2026/11/06/index.md spec/src/history/2026/11/06/SUMMARY.nav.md spec/src/history/2026/11/index.md spec/src/history/2026/11/SUMMARY.nav.md
npx --yes pnpm@10.32.1 exec spec-validator validate --only governance,summary,fileLimits,verification --json
npx --yes pnpm@10.32.1 exec spec-validator first --base main --head @ --json
JJ_STORE_VALIDATION_RUNTIME=native npx --yes pnpm@10.32.1 check:progress
```

**Notes**

The workflow regression, formatting, document validators, spec-first from main,
and the progress ledger pass after the chapter records action v1.9.0
(`a45c4d594aa4e2c509dc14a9f2b3b67ba3780d0d`).
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

Changesets action v1.9.0; @changesets/cli ^2.31.1; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No package, Cargo, archive, container or release-tag format changes. The
orchestration action pin is the only workflow change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Restore Changesets release orchestration for CLI v2. Publication remains
disabled until initial qualification is ready. This does not open a version
pull request or publish artifacts.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the pin, then rerun Changesets orchestration so it can open the Version
Packages pull request. Publication still waits for that merge and qualified
`publish-release.yml`.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 still withholds automatic publication until initial
  qualification is ready. The same-change chapter records that orchestration
  uses Changesets action v1.9.0, which accepts the existing CLI v2 commands.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. The action pin restores version orchestration
  only. Cargo, native bundles, Pages and GHCR publication gates stay open.
```
<!-- /evidence-field -->
