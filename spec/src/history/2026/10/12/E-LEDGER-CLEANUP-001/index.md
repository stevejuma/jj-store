---
schema: 1
id: E-LEDGER-CLEANUP-001
---

# E-LEDGER-CLEANUP-001

<!-- evidence-field:id markdown -->
## Id

E-LEDGER-CLEANUP-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
- M25
- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-004
- JST-GOV-010
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/spec/evidence-corrections.mjs
- scripts/spec/evidence-corrections.test.mjs
- scripts/spec/evidence-signature.mjs
- scripts/spec/progress-validation.mjs
- scripts/spec/repaginate-evidence.mjs
- scripts/spec/repaginate-evidence.test.mjs
- scripts/published-foundations.test.mjs
- spec/history-relations.yml
- spec/src/current/governance/02-requirements.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/governance/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- .pnpmfile.cjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/spec/evidence-corrections.test.mjs#appended_metadata_correction_preserves_original_payload_and_outcome
- scripts/spec/evidence-corrections.test.mjs#metadata_projection_never_supplies_a_passing_completion_gate
- scripts/spec/repaginate-evidence.test.mjs#repagination_is_bounded_lossless_and_repeatable_for_committed_evidence
- scripts/published-foundations.test.mjs#released_foundations_remain_registry_owned_without_active_overrides
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

Inherited E-NPM-REGISTRY-001 metadata: retired .pnpmfile.cjs, prose masquerading as a test/file, legacy run outcome vocabulary and invalid overall result.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/spec/evidence-corrections.test.mjs
```

**Notes**

New exact regressions fail before correction support is implemented.
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
node scripts/spec/repaginate-evidence.mjs spec/src/history/2026/10/11/E-PROJECT-ROWS-001
```

**Notes**

Parsed payload equality retained; split the oversized representation without truncation or outcome changes.
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
node --test scripts/spec/evidence-corrections.test.mjs scripts/spec/repaginate-evidence.test.mjs scripts/published-foundations.test.mjs
```

**Notes**

8 pass. Strict prior-value checks, invalid references, failed outcomes, milestone non-promotion, bounded lossless repeatability and registry-only installed foundation checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:corrections yaml -->
## Corrections

```yaml
- target: E-NPM-REGISTRY-001
  reason: Correct only malformed metadata. The original partial result, failed web
    test and exact commands/notes remain in immutable history. The prose
    package-origin observation is not an executable named test; current registry
    checks are recorded separately here.
  fields:
    tests:
      before:
        - Installed Design Core, Mira and Spec Validator package origin/version
          checks
        - web/tests/managed-agent.test.ts#inbox repository links name the
          repository and round trip exact workspace and operation
      after:
        - web/tests/managed-agent.test.ts#inbox repository links name the
          repository and round trip exact workspace and operation
    runs:
      before:
        - outcome: passed
          command: pnpm install --lockfile-only --ignore-pnpmfile
            --registry=https://registry.npmjs.org
          note: Registry lockfile refreshed; Design Core 0.3.1, Mira 0.0.3, Spec Validator
            0.1.4.
        - outcome: passed
          command: pnpm install --frozen-lockfile --ignore-pnpmfile --force
            --registry=https://registry.npmjs.org
          note: Requested foundations are installed from npm; the active local foundation
            hook was removed.
        - outcome: passed
          command: pnpm web:check
          note: 0 errors and 0 warnings.
        - outcome: passed
          command: pnpm catalog:check
          note: 0 errors and 0 warnings; 384 modules satisfy catalog boundaries.
        - outcome: failed
          command: pnpm web:test
          note: 153 passed, 1 failed. managed-agent.test.ts:130 passes a session fixture
            without the worktree array read by sessionItems; no production
            source or test fixture changed in this dependency slice.
        - outcome: passed
          command: pnpm catalog:test
          note: 961 passed and 1 skipped.
      after:
        - outcome: passed
          command: pnpm install --lockfile-only --ignore-pnpmfile
            --registry=https://registry.npmjs.org
          note: Registry lockfile refreshed; Design Core 0.3.1, Mira 0.0.3, Spec Validator
            0.1.4.
          result: pass
          failures: 0
        - outcome: passed
          command: pnpm install --frozen-lockfile --ignore-pnpmfile --force
            --registry=https://registry.npmjs.org
          note: Requested foundations are installed from npm; the active local foundation
            hook was removed.
          result: pass
          failures: 0
        - outcome: passed
          command: pnpm web:check
          note: 0 errors and 0 warnings.
          result: pass
          failures: 0
        - outcome: passed
          command: pnpm catalog:check
          note: 0 errors and 0 warnings; 384 modules satisfy catalog boundaries.
          result: pass
          failures: 0
        - outcome: failed
          command: pnpm web:test
          note: 153 passed, 1 failed. managed-agent.test.ts:130 passes a session fixture
            without the worktree array read by sessionItems; no production
            source or test fixture changed in this dependency slice.
          result: fail
          failures: 1
        - outcome: passed
          command: pnpm catalog:test
          note: 961 passed and 1 skipped.
          result: pass
          failures: 0
    result:
      before: partial
      after: pending

```
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

Node 26.9.0; Jujutsu 0.45.1; released Design Core 0.3.1, Mira 0.0.3, Spec Validator 0.1.4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Evidence schema 1 with additive corrections metadata. Original archives, payloads, gate selections and outcomes remain exact.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-authorized inherited ledger cleanup. Strict validation-only metadata projection; no correction can create historical success or qualify a product gate. A removed-files declaration records the already-retired active npm override. Source limits and UI integration are qualified separately.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain original evidence and apply ordinary validation to all projected metadata.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-GOV-004/010/011 explicitly govern appended metadata repair and
  lossless pagination. Completion, archive and immutable-payload authority is
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Bootstrap, M25 and M27 gate selections and original outcomes remain
  unchanged. This validates bookkeeping, not platform/provider product
  qualification.

```
<!-- /evidence-field -->


<!-- evidence-relations -->
## Resolution and supersession

The original result remains **pass**. Milestone gates select their own evidence.

- This entry supersedes [E-NPM-REGISTRY-001](../../11/E-NPM-REGISTRY-001/index.md). Corrects malformed metadata for validation only. Original partial result, failed runs and remaining qualification gaps are retained; milestone gates still use original evidence.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages


