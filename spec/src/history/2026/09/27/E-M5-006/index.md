---
schema: 1
id: E-M5-006
---

# E-M5-006

<!-- evidence-field:id markdown -->
## Id

E-M5-006
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

atomic-native-conflict-terms
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/conflicts.rs
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/resolve.rs
- crates/jj-store-jj/src/query/content/conflicts.rs
- crates/jj-store-service/src/commands.rs
- docs/evidence/m5-resolve-terms.md
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_native_terms_are_atomic_preserve_other_conflicts_and_retry_receipts
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rejects_the_whole_stale_or_invalid_batch_before_staging
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_accepts_multisided_base_absent_and_directory_terms
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rebases_descendants_updates_references_and_preserves_evolution
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs#http_resolve_validates_atomic_fingerprints_permissions_and_selected_receipts
- crates/jj-store-jj/src/query/content/conflicts.rs#conflict_fingerprint_v1_matches_independent_canonical_vector
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_report_extension_preserves_legacy_v1_canonical_receipt_bytes
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked
```

**Notes**

Initial acceptance fails: resolve_conflicts is not a recognized typed command.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked
```

**Notes**

Original native variant/partial resolution/retry test passes. Removed one unused-import warning.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked
```

**Notes**

Expanded fixtures: 3 pass, 1 fixture fails codec validation because unlabeled five-term conflict supplied only one empty label. Corrected fixture to five empty labels, preserving native arity.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle resolve --locked
```

**Notes**

All 5 focused native/HTTP fixtures pass, including term variants, stale/invalid no-write batches, descendants, references, policy, revocation and durable retry.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked
```

**Notes**

Final four native fixtures pass after explicitly testing an out-of-range term index independent of duplicate-path validation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle resolve --locked
```

**Notes**

All five fixtures pass after exporting owned resolution types through the service; fixture constructs selections through that public boundary.
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

Normal checks pass, including pinned rustfmt, Clippy, boundaries/progress/spec validation, 40 actual HTTP/SSE response schemas, six schema tests and zero Svelte diagnostics. Repeated after adding service-owned DTO exports.
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

291 Rust tests pass, including 136 native lifecycle cases; six external tests explicitly ignored. Also 16 governance, three harness, six schema and eight browser client tests pass.
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

Production asset and locked Rust workspace builds pass, including final service export integration. Browser runtime bundle unchanged.
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

All 13 lanes pass, including final Rust workspace tests, eight cold browser interaction/accessibility tests, rustfmt, contracts, mdBook and spec-first.
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

Pinned rustfmt and Prettier pass.
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
jj --no-pager diff --name-only -- web
```

**Notes**

Only generated TypeScript API declarations change. No UI behavior, feature or visual changes.
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
ps -p 54647,54802 -o pid,ppid,etime,stat,command
```

**Notes**

Existing isolated RustFS harness and Docker volume child still running at the previously recorded wait. No restart or duplicate harness launched; no new deployment result claimed.
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

Adds resolve_conflicts typed variant with bounded paths, fingerprint, select_term/delete actions and stale_conflict HTTP code/capability. No native codec, WAL, StoredResult V1 or Prepared format changes; earlier receipts reencode unchanged. Only generated TypeScript changes under web; UI stays Deferred M4.3. Enclosing Jujutsu change is the revision reference. Authorized uploaded file replacements remain M5.3; real RustFS/TLS SSE qualification awaits the stopped shared Docker engine.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


