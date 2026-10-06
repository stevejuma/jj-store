---
schema: 1
id: E-READ-049
---

# E-READ-049 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts -t 'local_code_summary_uses_a_finite_pin_and_forwards_cancellation|local_review_activity_uses_explicit_pin_and_rejects_foreign_results'
```

**Notes**

Exact failing boundary reproduced:2 failed,18 skipped. Assertions aborted an already completed shared transport after caller lease release, contrary to current observer ownership.
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
cargo test -p jj-store-http --lib --locked contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies -- --exact --nocapture
```

**Notes**

Exact inventory regression passes after correcting double-counted arithmetic against the generated route union.119 total,42 local,77 nonlocal; every parent route retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts
```

**Notes**

22 tests passed. Both summary/activity paths now prove two in-flight consumers share one finite pinned request, first abort retains transport and final abort cancels it; aborted callers make no new requests. Foreign-pin validation and completed caller isolation remain asserted.
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
pnpm catalog:check
```

**Notes**

svelte-check found 0 errors and 0 warnings; catalog boundaries ok (190 modules)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts
```

**Notes**

22 tests pass with literal named entry points for progress mapping; active sharing assertions remain unchanged.
<!-- /run -->

<!-- run -->
### Run 6

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

Default spec-first cannot resolve @- for a two-parent Jujutsu merge. No policy changes or files excluded; rerun using its supported explicit-file mode over the union of actual Jujutsu diffs against both parents.
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
python3 .local/bounded-reads/integration/spec-first-049.py
```

**Notes**

Spec-first gate passed: 257 protected file(s), 15 canonical chapter(s).
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial integration ledger had one unnamed project test and two dynamically named cancellation tests unavailable to the static test-reference mapper (3 diagnostics). Replaced these with exact existing/literal named references; no regression assertions removed.
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
pnpm check:progress
```

**Notes**

progress ok (139 tasks; evidence bookkeeping only)
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
pnpm spec:validate
```

**Notes**

Specification validated: 5 validators, 23 chapters, 172 requirements.
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
pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/spec-governance.md web/src/lib/controller/http-work-data-source.ts web/tests/vite/local-browser.test.ts
```

**Notes**

Exit 0; see doc-format-049.log.
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
pnpm contracts:check
```

**Notes**

Generated contracts match;267 actual HTTP/SSE responses conform.7 contract-tool tests pass and contract TypeScript check succeeds. Complete registered response inventory checked once for this combined landing.
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
Independent JSON record equality against both saved parent ledgers; generated route keys compared against complete parent union; jj --no-pager st and conflict-marker search
```

**Notes**

472 main and482 feature historical evidence rows preserved exactly;490 unique combined evidence records. Generated routes119 total (42 local/77 nonlocal), complete parent union preserved. No unresolved conflict markers or unrelated main edits. Temporary workspace remains clean and retained.
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

progress ok (139 tasks; evidence bookkeeping only) Final combined source; log progress-final-049.log.
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
python3 .local/bounded-reads/integration/spec-first-049.py
```

**Notes**

Spec-first gate passed: 257 protected file(s), 15 canonical chapter(s). Final combined source; log spec-first-final-049.log.
<!-- /run -->

<!-- run -->
### Run 16

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

Specification validated: 5 validators, 23 chapters, 172 requirements. Final combined source; log spec-final-049.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

svelte-check found 0 errors and 0 warnings; catalog boundaries ok (190 modules) Final combined source; log catalog-final-types-049.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

docs/implementation-plan.md 129ms; spec/src/milestones.md 1524ms Final combined source; log final-doc-format-049.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

