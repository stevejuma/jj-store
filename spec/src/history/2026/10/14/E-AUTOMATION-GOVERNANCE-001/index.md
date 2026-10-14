---
schema: 1
id: E-AUTOMATION-GOVERNANCE-001
---

# E-AUTOMATION-GOVERNANCE-001

<!-- evidence-field:id markdown -->
## Id

E-AUTOMATION-GOVERNANCE-001
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
- JST-GOV-004
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sqrqmnrn
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/README.md
- .github/workflows/ci.yml
- docs/releases.md
- package.json
- scripts/automation-governance.test.mjs
- scripts/changeset-check.mjs
- scripts/release-evidence.mjs
- scripts/release-version.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/01-specification-governance.md
- spec/src/current/governance/02-requirements.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/current/verification/gov.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- scripts/release-evidence.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/automation-governance.test.mjs#bot PRs skip only change-specific
  governance
- scripts/automation-governance.test.mjs#bot PRs retain static spec and planned
  test lanes
- scripts/automation-governance.test.mjs#version synchronization leaves spec and
  evidence unchanged
- scripts/automation-governance.test.mjs#release intent still enforces notes on
  ordinary PRs
- scripts/automation-governance.test.mjs#Changesets selects versions without
  writing spec evidence
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/automation-governance.test.mjs
```

**Notes**

Before implementation, one of four tests passed; failures established missing conditional gates, automatic spec-evidence generation, and release-intent rejection of generated PRs.
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
node --test scripts/automation-governance.test.mjs
```

**Notes**

All five regression tests passed, including the added real Changesets CLI version selection in an isolated Git fixture. The fixture disables only the external GitHub changelog provider; private-package version policy uses the repository configuration.
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
pnpm release:check
```

**Notes**

Ten release/CI tests passed and all 20 Cargo package static metadata/contained-input checks passed.
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
pnpm test:governance
```

**Notes**

All 68 governance tests passed.
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
pnpm test:spec
```

**Notes**

All 26 specification tooling tests passed.
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
actionlint
```

**Notes**

Workflow syntax and GitHub expression checks passed.
<!-- /run -->

<!-- run -->
### Run 7

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

The inherited release docs file had no spec-first mapping. Added an exact owning public-distribution mapping; no protected-path ignore was introduced.
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

155 tasks and 799 evidence records validated with retained original payloads.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Four static validators passed: 3815 chapters and 198 requirements.
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
pnpm spec:first
```

**Notes**

Nine protected files and four owning canonical chapters passed after the exact release-doc mapping repair.
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
pnpm exec prettier --write .changeset/README.md .github/workflows/ci.yml docs/releases.md package.json scripts/automation-governance.test.mjs scripts/changeset-check.mjs scripts/release-version.mjs spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/governance/01-specification-governance.md spec/src/current/governance/02-requirements.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M30/index.md spec/src/current/verification/gov.md
```

**Notes**

Formatted only changed implementation and canonical docs. Historical evidence is generated by the evidence writer and retained without global navigation regeneration.
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
pnpm exec prettier --write spec-validator.config.mjs docs/releases.md
```

**Notes**

Formatted the mapping repair and clarified that Dependabot extends the lapis-notes Changesets precedent.
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
pnpm exec prettier --check .changeset/README.md .github/workflows/ci.yml docs/releases.md package.json scripts/automation-governance.test.mjs scripts/changeset-check.mjs scripts/release-version.mjs spec-validator.config.mjs spec/src/current/contracts/architecture/23-public-distribution.md spec/src/current/governance/01-specification-governance.md spec/src/current/governance/02-requirements.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M30/index.md spec/src/current/verification/gov.md
```

**Notes**

All changed format-checked files passed.
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

Changesets CLI 2.31.1; Node 24.10.0 local tests; pnpm 10.32.1; Rust code is unchanged in this slice.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence, API, native identity or model format changes. Only CI conditions and automated versioning side effects change.
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [Automation governance evidence](details-02.md)
