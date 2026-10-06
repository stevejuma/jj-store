---
schema: 1
id: E-M5-004
---

# E-M5-004

<!-- evidence-field:id markdown -->
## Id

E-M5-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

explicit-native-reconciliation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/reconcile.rs
- crates/jj-store-service/src/commands.rs
- crates/jj-store-service/src/mutations.rs
- docs/evidence/m5-reconciliation.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_merges_native_heads_rebases_descendants_and_replays_receipt
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_requires_exact_heads_and_explicit_current_base
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_recursive_ancestors_and_conflicted_bookmarks_match_upstream
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_resource_and_protection_failures_preserve_heads
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_reconciliation_preserves_get_heads_and_publishes_only_explicit_apply
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation_acceptance:: --locked
```

**Notes**

Before implementation: Command::ReconcileOperations missing; acceptance did not compile.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation_acceptance:: --locked
```

**Notes**

Original two native reconciliation acceptance fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation --locked
```

**Notes**

Five expanded fixtures pass: native rewrites/retry, strict context, recursive ancestors/conflicted bookmarks, protection/resource rejection and explicit HTTP application.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation --locked
```

**Notes**

All five fixtures pass after extending recursive-ancestor acceptance to three current heads.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

Owning-package Clippy passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All normal lanes pass: rustfmt, all-target Clippy, progress/boundaries/specification, 34 actual HTTP/SSE schema responses, six schema tests, TypeScript and zero Svelte diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

280 Rust tests including 125 native lifecycle tests pass; six external tests explicitly ignored. Also 16 governance, three storage-harness, six schema and eight browser client tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production assets and locked Rust workspace build pass; browser runtime bundle unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and existing workspace/contract lanes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Pinned rustfmt and Prettier pass.
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
pnpm check:progress
```

**Notes**

32 tasks validate.
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
pnpm spec:first
```

**Notes**

13 protected files mapped to six canonical chapters pass.
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds reconcile_operations with explicit current base to typed Command and generated HTTP contracts/capabilities. Reports, StoredResult V1, other canonical command digests, native codecs and WAL formats are unchanged. HeadsNormalized is the existing semantic kind. No browser behavior or visual changes; generated TypeScript only. Enclosing Jujutsu change is the revision reference. Abandon and ResolveConflicts remain M5 gates. Real RustFS/TLS qualification remains blocked by stopped shared Docker; M4.3 browser work stays Deferred. M5.1 basic command, preview/apply/status, receipt and reconciliation gate is complete; M5 remains In progress for abandon and conflict resolution.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


