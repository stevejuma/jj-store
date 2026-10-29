---
schema: 1
id: E-FORMAT-COVERAGE-001
---

# E-FORMAT-COVERAGE-001

<!-- evidence-field:id markdown -->
## Id

E-FORMAT-COVERAGE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/history/2026/10/29/E-FORMAT-COVERAGE-001/index.md
- spec/src/history/2026/10/29/E-FORMAT-COVERAGE-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
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
gh run view 40806005760 --repo stevejuma/jj-store --log-failed
```

**Notes**

Formatting failed because docs/maintenance/coverage.json from docs:review was not Prettier-formatted.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

Rewrote coverage.json with Prettier; prettier --check passes for that file.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Progress evidence gate accepts this leaf.
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

pnpm 10.32.1; prettier 3.9.x.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Coverage fingerprint JSON whitespace only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Satisfy the PR 20 Formatting lane after docs:review rewrote coverage.json without Prettier layout.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push and confirm Formatting plus required on PR 20.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-DOC-002 coverage review content is unchanged. Only Prettier
  whitespace in docs/maintenance/coverage.json was normalized.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 unchanged. Formatting repair only.

```
<!-- /evidence-field -->
