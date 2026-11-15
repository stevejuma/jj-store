---
schema: 1
id: E-VERSION-PR-APP-AUTH-001
---

# E-VERSION-PR-APP-AUTH-001

<!-- evidence-field:id markdown -->
## Id

E-VERSION-PR-APP-AUTH-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/release.yml
- scripts/validation/lanes.json
- scripts/version-pr-auth.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/32-version-pr-authentication.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/verification/public-distribution.md
- spec/src/history/2026/11/15/SUMMARY.nav.md
- spec/src/history/2026/11/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/version-pr-auth.test.mjs#publisher and dispatch authority remain
  separate from the PR App
- scripts/release.test.mjs#publication order resolves internal edges and rejects
  cycles
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
tests: 3
hasNote: false
noteEncoding: markdown

```

```shell
node --test scripts/version-pr-auth.test.mjs
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
tests: 93
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native TMPDIR=/home/steve/.cache/lapis-maintenance-validation pnpm test:release
```
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm exec spec-validator validate; pnpm spec:first; pnpm check:progress
```

**Notes**

All canonical validation, fresh evidence coverage, mapped source-first and progress checks passed. M30 stays In progress.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:architecture; pnpm spec:build; pinned actionlint .github/workflows/release.yml
```

**Notes**

Source budgets, canonical mdBook and changed workflow lint passed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm evidence:new -- --id E-VERSION-PR-APP-AUTH-001 --task M30 --requirement JST-ARCH-005 --requirement JST-GOV-011
```

**Notes**

Global navigation regeneration rejects unchanged 399-line E-UI-CATALOG-HELP-001 when appending navigation. Reproduced from unchanged main via the same index writer. Restored all unrelated generated edits; existing scoped helpers updated only the new topic/evidence indexes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.19.0; pnpm 10.32.1; pinned Changesets action v1.9.0 and CLI v2 retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub Actions YAML, Node test modules, validation-routing JSON and canonical Markdown.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Canonical-main repository-scoped dependency App PR writes; ephemeral checkout credentials; existing workflow-token publisher and full-check dispatch authority retained. No runtime, native, signature, container, protection or package-version change.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now scopes version PR writes to the dependency App; every
  existing immutable-source, signing and publication gate remains. JST-GOV-011
  evidence/source ownership remains unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Focused source tooling does not establish full
  native/hosted/container/release qualification.

```
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Qualify the next eligible hosted App-created candidate without manufacturing a
  version bump.
- Native/platform, complete release and container acceptance remain separate
  existing gates.
- Global evidence navigation regeneration still fails on unchanged 399-line
  E-UI-CATALOG-HELP-001; only new owning navigation indexes were refreshed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the protected source PR, retain the unchanged release gates, and qualify the next eligible App-created version candidate.
<!-- /evidence-field -->

