---
schema: 1
id: E-READ-005
---

# E-READ-005 details 3

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
/Users/stevejuma/code/lapismd/jj-store/target/debug/deps/capture_management-8eb3eacdec15b02e --exact api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free --nocapture
```

**Notes**

Original-workspace existing binary independently reproduced same unchanged 120-second capture timeout from E-READ-004; final teardown 193.11s. Capture owner remains unqualified; no deadline or sanitizer changes.
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
pnpm exec playwright test --config .local/bounded-reads/playwright.config.ts web/tests/browser/workspaces.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

Final fresh preview process: 19 passed, including accessible workspace pagination and missing-name recovery. Temporary isolated example removed after completion; existing services preserved.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress
```

**Notes**

Added owning mutation precondition requirement and updated two renamed historical test pointers without rewriting historical evidence.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check (changed TS/Svelte/canonical chapters excluding milestones); pnpm check:architecture
```

**Notes**

New mutation prose needed formatter; formatting expanded native HTTP source to 303 counted lines. Extracted the complete reference request helper into http-reference-reads.ts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-reference-reader.test.ts; pnpm web:check; pnpm web:build; pnpm check:architecture
```

**Notes**

Final extraction direct consumers: 18 passed; web 0 errors/warnings; fresh built UI; 930 production modules pass.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; pnpm check:ui-ownership; cargo fmt --all -- --check; pnpm exec prettier --check (changed TS/Svelte/canonical chapters excluding milestones)
```

**Notes**

Spec-first, 136-task ledger, 167 requirements, ownership and focused formatting pass. Historical ledger formatting preserved. Full unrelated aggregate suites remain out of scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Historical lookup/graph preparation, actual-work admission, immutable cache
  segments/quotas, operation/copy/search/RPC and collaboration/card projections
  remain in subsequent implementation slices.
- Large-scale Linux/native 2m/near-capacity/cold/cache-loss/real-RustFS gates
  and full contract capture remain pending; no milestone or full-plan completion
  claimed. Registry Design Core release qualification and visual approval remain
  pending. Unrelated root aggregates out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified reference/startup slice, then implement bounded on-demand history/index preparation and remaining audited reads.
<!-- /evidence-field -->

