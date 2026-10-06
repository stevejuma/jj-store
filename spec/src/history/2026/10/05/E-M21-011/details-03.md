---
schema: 1
id: E-M21-011
---

# E-M21-011 details 3

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
pnpm docs:review application "Stacks now owns uncached current checkout observations separately from historical reads. Exact pins, complete bounds, lossless integers and cancellation are validated; observations do not populate complete Unstaged or enable commands."
```

**Notes**

working-client-docs-application-review.log: owning source/manual review acknowledged; portable coverage file subsequently formatted.
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
pnpm docs:check
```

**Notes**

working-client-docs-check.log: nine Mermaid diagrams and strict manual build, 139-file link/anchor closure pass.
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
pnpm exec prettier --write web/src/lib/controller/working-tree-types.ts web/src/lib/controller/working-tree-validation.ts web/src/lib/controller/http-working-tree.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/http-local-store-data-source.ts web/src/lib/controller/stack-workspace/current-working-tree.ts web/src/lib/controller/stack-workspace/physical-observation.svelte.ts web/src/lib/controller/stack-workspace/production-adapter.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/tests/vite/working-tree-http.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/production-stacks-application.test.ts catalog/fixtures/stack-workspace/production-http.ts web/tests/real/stacks-browser.spec.ts scripts/stacks-browser-acceptance.mjs spec/src/ui.md spec/src/api.md
```

**Notes**

working-client-format.log: explicit changed files formatted; historical ledger excluded.
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
pnpm exec prettier --write web/src/lib/controller/http-working-tree.ts web/src/lib/controller/working-tree-read.ts web/src/lib/controller/stack-workspace/current-working-tree.ts web/tests/vite/working-tree-http.test.ts docs/implementation-plan.md docs/site/guides/browsing.md spec/src/architecture.md spec/src/local-jj.md spec/src/spec-governance.md spec/src/documentation.md
```

**Notes**

working-client-format-final.log: explicit changed files formatted; historical ledger excluded.
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
pnpm exec prettier --write web/tests/vite/stack-working-tree.test.ts
```

**Notes**

working-client-format-test-final.log: explicit changed files formatted; historical ledger excluded.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

working-client-format-verification.log: explicit changed files formatted; historical ledger excluded.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

Formatted coverage after application review; only its owning reason/fingerprint changed.
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
pnpm spec:first
```

**Notes**

working-client-spec-first.log: all 20 protected files map to seven updated canonical chapters.
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
pnpm spec:validate
```

**Notes**

working-client-spec-validate.log: five validators, 23 chapters and 173 requirements pass.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

working-client-progress.log: evidence used an unsupported conditional-test reference and an unanchored story path. Corrected only the ledger references; actual conditional capture test and both story lanes remain unchanged and passed, with exact commands retained.
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
pnpm exec prettier --check catalog/fixtures/stack-workspace/production-http.ts docs/implementation-plan.md docs/maintenance/coverage.json docs/site/guides/browsing.md scripts/stacks-browser-acceptance.mjs spec/src/api.md spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/http-local-store-data-source.ts web/src/lib/controller/http-working-tree.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/current-working-tree.ts web/src/lib/controller/stack-workspace/physical-observation.svelte.ts web/src/lib/controller/stack-workspace/production-adapter.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/working-tree-read.ts web/src/lib/controller/working-tree-types.ts web/src/lib/controller/working-tree-validation.ts web/tests/real/stacks-browser.spec.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/stack-working-tree.test.ts web/tests/vite/working-tree-http.test.ts
```

**Notes**

working-client-format-check.log: every changed source/test/doc passes; ledger objects intentionally not formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

working-client-progress-repaired.log: evidence result used free prose instead of the required pass/pending/blocked/fail value. Moved prose to summary and retained result=pass; no milestone promoted.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

working-client-progress-final.log: pass correctly requires no remaining milestone gaps. Restored pending for the still-incomplete M21.2 feature; successful slice qualification remains in actual runs and summary, without weakening the checker.
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
pnpm check:progress
```

**Notes**

working-client-progress-qualified.log: 142 tasks validate after exact test-reference/result bookkeeping repairs; M21.2 remains In progress and evidence result stays pending for remaining feature gates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "M21.2 and Stage 2 remain incomplete for complete active-combination Unstaged
  assignment and watcher/lifecycle qualification. Current disk delta is not all
  unfinished work: durable combination merge/WIP state is required. No
  workspace, applied membership, staged token or command capability is
  fabricated."
- Continue Stage 3 recoverable exact unpublished execution,
  journal-before-publication, foreground WC coordination, guarded checkout,
  durable status and restart/fault/retry acceptance. Stages 4–7 remain approved
  work, including named combinations, exact file then hunk transfers, graph
  actions/guarded undo and linked-native full lifecycle.
- Six unchanged JJ/service strict lint diagnostics remain unresolved and
  unwaived as recorded in E-M21-010. This UI/client slice does not rerun or
  claim aggregate Rust qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement and qualify the durable request journal and retained unpublished provider preparation, then guarded code publication/physical checkout before enabling combinations or writable UI.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No new visual review, screenshot comparisons/baseline updates, package publication, source-reference dependencies, installed services, reset/migration, Rust code/build/test changes or unrelated UI/Rust aggregate lanes. Storybook remains 6057; source-only HMR needs no dependency/startup restart.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Verified current physical-read client and independent Stacks observation with cold real dirty-folder browser evidence; M21.2 remains In progress.
<!-- /evidence-field -->

