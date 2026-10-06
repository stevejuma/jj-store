---
schema: 1
id: E-M5-008
---

# E-M5-008

<!-- evidence-field:id markdown -->
## Id

E-M5-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

complete-conflict-reports
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/report.rs
- crates/jj-store-service/src/commands.rs
- crates/jj-store-service/src/mutations.rs
- docs/evidence/m5-conflict-reports.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- web/src/lib/api/generated.ts
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rebases_descendants_updates_references_and_preserves_evolution
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_accepts_multisided_base_absent_and_directory_terms
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_report_limits_reject_without_publication_or_truncation
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_http_stages_verified_binary_without_publication_then_resolves
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_atomic_mixed_resolution_retains_copy_history_and_other_conflicts
- crates/jj-store-service/src/mutations.rs#conflict_report_decodes_legacy_selected_v1_receipt_without_changing_bytes
- crates/jj-store-cli/tests/native_rpc.rs#rpc_guarded_publication_preserves_receipts_siblings_and_protection
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve_rebases_descendants_updates_references_and_preserves_evolution --locked
```

**Notes**

Regression confirms resolution preview omits exact target and remaining-conflict report.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve_rebases_descendants_updates_references_and_preserves_evolution --locked
```

**Notes**

Exact original regression passes; target and changed descendant report paths match native trees. Removed one unused import warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service conflict_report --locked
```

**Notes**

Frozen legacy V1 result field order reencodes byte-for-byte and passes selected receipt integrity decoder with optional report absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked
```

**Notes**

Five native resolution fixtures pass, including the original regression, explicit conflict-free results and output-budget rejection without publication.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

Six upload fixtures pass with explicit HTTP/native remaining-conflict reports and exact retained report replay after checkpoint/restart and staging loss.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate
```

**Notes**

OpenAPI and generated TypeScript declarations regenerated from owned Rust DTOs; browser source unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Rustfmt, all-target Clippy, boundaries/progress, spec validation, 43 actual HTTP/SSE contract samples, schema checks and zero Svelte diagnostics pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

301 Rust tests pass, zero failures; six opt-in external tests ignored. Governance (16), harness (3), contract (6) and browser unit (8) tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Full locked Rust workspace and existing browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including eight cold browser interaction/accessibility tests, all Rust tests, 43 actual HTTP/SSE schema samples, rustfmt, mdBook and spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Pinned rustfmt and repository-wide Prettier pass.
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
pnpm spec:validate && pnpm spec:first && pnpm check:progress && pnpm spec:build
```

**Notes**

After the final M5 gate audit, all 58 requirements and 32 tasks validate; mapped spec-first and mdBook pass. Deferred UI and outstanding deployment gates remain unchanged.
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

Adds optional absent-skipped ConflictResolutionReport to Prepared and generated HTTP/TypeScript contracts. Existing V1 receipt canonical bytes and native/WAL/RPC versions unchanged. Enclosing Jujutsu change is the revision reference. UI stays Deferred M4.3; RustFS/TLS deployment gate remains separate. E-M5-007 recorded the previously missing report; this audit closes that gap. M5 backend gates are complete independently of deferred M4.3 and outstanding external deployment qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


