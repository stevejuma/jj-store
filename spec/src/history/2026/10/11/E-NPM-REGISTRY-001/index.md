---
schema: 1
id: E-NPM-REGISTRY-001
---

# E-NPM-REGISTRY-001

<!-- evidence-field:id markdown -->
## Id

E-NPM-REGISTRY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-ARCH-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- Installed Design Core, Mira and Spec Validator package origin/version checks
- web/tests/managed-agent.test.ts#inbox repository links name the repository and
  round trip exact workspace and operation
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
outcome: passed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --lockfile-only --ignore-pnpmfile --registry=https://registry.npmjs.org
```

**Notes**

Registry lockfile refreshed; Design Core 0.3.1, Mira 0.0.3, Spec Validator 0.1.4.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
outcome: passed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --frozen-lockfile --ignore-pnpmfile --force --registry=https://registry.npmjs.org
```

**Notes**

Requested foundations are installed from npm; the active local foundation hook was removed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
outcome: passed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

0 errors and 0 warnings.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
outcome: passed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

0 errors and 0 warnings; 384 modules satisfy catalog boundaries.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
outcome: failed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

153 passed, 1 failed. managed-agent.test.ts:130 passes a session fixture without the worktree array read by sessionItems; no production source or test fixture changed in this dependency slice.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
outcome: passed
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

961 passed and 1 skipped.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

partial
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The existing managed-agent fixture failure blocks full functional
  qualification.
- Full Storybook, static-browser and web build qualification are not complete;
  no push or release occurs in this slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Design Core 0.3.1; Mira 0.0.3; Spec Validator 0.1.4; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Portable package.json dependency ranges and registry pnpm-lock.yaml.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace the local Design Core and Mira tarball installation with the released npm packages. Existing unrelated history edits remain in the preceding Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete consumer qualification and repair or explicitly accept the independently identified fixture failure before pushing.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-012 and JST-ARCH-001 retain public foundation/source ownership
  boundaries; only dependency delivery changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M25 remains In progress. Platform, real-provider and visual
  qualification gates are unchanged.

```
<!-- /evidence-field -->
<!-- evidence-relations -->
## Resolution and supersession

The original result remains **partial**. Milestone gates select their own evidence.

- [E-LEDGER-CLEANUP-001](../../12/E-LEDGER-CLEANUP-001/index.md) supersedes this entry. Corrects malformed metadata for validation only. Original partial result, failed runs and remaining qualification gaps are retained; milestone gates still use original evidence.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages


