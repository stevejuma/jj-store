---
schema: 1
id: E-M21-008
---

# E-M21-008 details 3

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
pnpm docs:review local "Private WC baseline reads exact named-workspace operation/view with raw file/link digests and JJ captured ignore precedence. No latest opening, source index repair, physical snapshot, wire route or executor is added; active checkout and global/exclude capture remain unqualified."
```

**Notes**

working-baseline-docs-local-review.log: reviewed owning source/manual coverage.
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
pnpm docs:review services "Private physical comparison retains tracked ignored changes and all untracked file/link obstacles, validates captured gitignore bytes and supports cancellation. It does not establish active Unstaged, current checkout/sparse state, transport or execution."
```

**Notes**

working-baseline-docs-service-review.log: reviewed owning source/manual coverage.
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
pnpm docs:check
```

**Notes**

working-baseline-docs.log: 139 files, nine rendered Mermaid diagrams, strict manual build and links/anchors pass.
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
pnpm spec:first
```

**Notes**

working-baseline-spec-first.log: 13 protected files and six canonical chapters pass before ledger append.
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
pnpm spec:validate
```

**Notes**

working-baseline-spec.log: five validators, 23 chapters and 173 requirements pass before ledger append.
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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md
```

**Notes**

working-baseline-format-docs.log; additional UI/plan, verification and coverage targeted formatting logged separately. milestones.md never formatted.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/ui.md spec/src/verification.md
```

**Notes**

working-baseline-format-final-docs.log: coverage review JSON and the two changed verification rows needed targeted formatting.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

working-baseline-format-verification.log: only two verification rows changed, no historical ledger formatting.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

working-baseline-format-coverage.log.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/ui.md spec/src/verification.md
```

**Notes**

working-baseline-format-docs-repaired.log: all selected documents pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

working-baseline-progress.log: five new evidence test paths lacked required function anchors. New evidence repaired; historical objects unchanged.
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
node --test scripts/progress-mapping.test.mjs
```

**Notes**

working-baseline-progress-tests.log: five historical task mapping regressions pass.
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
pnpm spec:first
```

**Notes**

working-baseline-spec-first-final.log: 13 protected files/seven chapters, including ledger.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

working-baseline-spec-final.log: five validators, 23 chapters, 173 requirements including ledger.
<!-- /run -->

<!-- run -->
### Run 15

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

working-baseline-progress-repaired.log: 142 tasks pass after exact new test anchors; all 489 parent evidence objects remain byte-identical.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "M21.2 and Stage 2 remain incomplete: current physical WC operation/tree and
  sparse-state proof, bounded global/repository-exclude policy capture, scan
  transport, cold registered-folder/browser acceptance and active-combination
  Unstaged assignment remain. Whole-tree ignored directories still count toward
  physical scan limits."
- "Strict lint qualification remains incomplete: four unchanged JJ and two
  unchanged service diagnostics, reproduced and not suppressed or waived.
  Focused tests/compile do not qualify those failed lanes."
- Recoverable exact preparation/publication/checkout, durable combinations and
  merge/WIP retention, file/hunk and graph commands, guarded undo, linked-native
  lifecycle and full cold acceptance remain approved implementation work.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Capture and verify current physical WC/sparse/ignore-policy witnesses, integrate bounded read transport and real-folder UI evidence, then qualify durable prepared execution before writable controls.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

UI type/Storybook/browser lanes (no UI implementation or runtime/bundle change), generated wire contracts (no new route), root Rust/UI aggregates, package publication, installed-service changes, checkout execution, screenshots/baselines and fresh live visual review. Storybook listener remains 6057/PID 36264.
<!-- /evidence-field -->

