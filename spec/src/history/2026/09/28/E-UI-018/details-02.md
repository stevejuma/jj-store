---
schema: 1
id: E-UI-018
---

# E-UI-018 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

Test-first stories initially had no repository action controller, typed command forms or in-memory mutation implementation. Preview/apply, destructive confirmation and unknown-outcome recovery all failed before implementation.
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
pnpm exec tsx --test web/tests/production-store-mutations.test.ts web/tests/store-mutation-model.test.ts
```

**Notes**

All six controller and form-to-command regressions pass, including stable request bytes, captured heads, committed refresh, unknown-outcome recovery and all supported command variants.
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
pnpm contracts:generate
```

**Notes**

The OpenAPI document and lossless TypeScript client were regenerated after distinguishing semantic protected-transition responses from credential denial.
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
cargo test --quiet --locked -p jj-store-cli --test native_lifecycle http_mutation_acceptance::http_mutations_enforce_protection_revocation_and_principal_scoped_status -- --exact; abandon_acceptance::typed_abandon_rejects_root_protected_unknown_and_invalid_batches_atomically -- --exact; reconciliation_acceptance::typed_reconciliation_resource_and_protection_failures_preserve_heads -- --exact; content_acceptance::diff_acceptance::conflict_acceptance::resolve_acceptance::typed_resolve_rebases_descendants_updates_references_and_preserves_evolution -- --exact
```

**Notes**

Four focused native cases pass. Protected semantic transitions return 409 protected_transition while revoked credentials remain access_denied, so policy rejection does not disconnect the browser session.
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
pnpm exec playwright test --config web/playwright.config.ts mutations.spec.ts -g 'repository command'
```

**Notes**

The mutation committed, but the fixed application canvas retained Bits UI body overflow and pointer locks after the dialog closed. Focused menu and dialog isolation reproduced the same blocked navigation.
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
pnpm web:build && pnpm exec playwright test --config web/playwright.config.ts mutations.spec.ts -g 'repository action menu|repository command dialog cancel|repository command preview'
```

**Notes**

All three focused built-browser regressions pass after using Design Core's supported preventScroll=false integration for the already non-scrolling shell canvas and deferring menu-to-dialog selection until close completion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

All three real-control interaction/accessibility stories pass and leave the document interactive after close. Stories remain visual-pending.
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
pnpm web:test:browser
```

**Notes**

All 17 captured-response production-browser cases pass, including four mutation workflows, route restoration, cached reads, races, SSE reset and command search.
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
pnpm web:test:real
```

**Notes**

All three cold built-browser cases pass against a fresh native repository and pinned RustFS: Tantivy search, external CLI publication with explicit SSE refresh, and one previewed/applied mutation followed by navigation to its native commit. The disposable stack was removed.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The aggregate reached the native lifecycle suite with 173/174 cases passing. One older assertion still expected generic Denied for a protected semantic preview/apply after the service introduced the distinct Protected variant.
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
cargo test --quiet --locked -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_mutations_reject_stale_intent_conflicts_readers_and_protected_previews -- --exact
```

**Notes**

The exact native regression passes after retaining Denied for reader/authentication failures and expecting Protected only for semantic commit protection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual test gate passes after the stale protected-transition assertion was corrected: 23 governance, three storage-harness, six contract, 35 production web, 34 catalog unit, 64 Storybook interaction/accessibility and the complete locked Rust workspace tests, plus cold catalog browser acceptance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical specification lanes pass, including architecture, generated contracts, production and catalog diagnostics, 64 Storybook interaction/accessibility cases, cold catalog construction, browser acceptance and the complete locked Rust workspace.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Final protected-file mapping covers 45 files across eight canonical chapters; the 46-task progress ledger, rustfmt and Prettier checks pass after recording the aggregate outcomes.
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

Community source 9d82df591548a1efc49681b744f7a5f244b8ac99; Design Core source 6f8ee4570708ffdae406e9c48457546db07ec71a inspected and consumed through registry range ^0.2.0 with the guarded local source hook; @tanstack/svelte-query 6.3.0; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x; RustFS rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The UI uses the existing strict V1 mutation preview/apply/status contract. Each request retains one 128-bit hexadecimal request ID, exact captured epoch/position/heads and one typed native command. Committed receipts alone replace the selected operation and protected query cache. Semantic protection now has the typed 409 protected_transition code; authentication denial remains 401/403 access_denied. No native object, WAL, credential, URL or persistent layout format changes. Stories remain visual-pending and visual tests were skipped at the user's request.
<!-- /evidence-field -->

