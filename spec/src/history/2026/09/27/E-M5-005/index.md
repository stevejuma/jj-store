---
schema: 1
id: E-M5-005
---

# E-M5-005

<!-- evidence-field:id markdown -->
## Id

E-M5-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-typed-abandon
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/abandon.rs
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-jj/src/commands/abandon.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- docs/evidence/m5-abandon.md
- docs/licenses/NOTICE.md
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

- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_preserves_native_descendant_bookmark_workspace_and_history_semantics
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_adjacent_targets_and_hidden_noop_keep_complete_native_history
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_merge_retains_ordered_parents_and_native_conflicted_bookmark_terms
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_rejects_root_protected_unknown_and_invalid_batches_atomically
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_report_extension_preserves_legacy_v1_canonical_receipt_bytes
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_abandon_previews_native_effects_and_reports_hidden_noop_without_publication
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked
```

**Notes**

Initial compile failure: absent Abandon variant and two report fields, plus fixture used nonexistent MergedTree::names. Repaired fixture to native entries iterator and implemented command/report.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked
```

**Notes**

Original four-option fixture passed; removed one unused-import warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked
```

**Notes**

Expanded evolution fixture needed Arc&lt;CommittedRead&gt; for the graph method; corrected capability ownership without changing assertions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked
```

**Notes**

All five expanded native fixtures passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_abandon --locked
```

**Notes**

Five final native fixtures pass with binary/mode/copy/symlink/submodule value preservation added.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_abandon --locked
```

**Notes**

HTTP preview/apply/status/retry and no-publication hidden no-op pass.
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

All normal lanes pass, including rustfmt, workspace all-target Clippy, specification/progress/boundaries, 37 actual HTTP/SSE response schemas, six schema tests, generated TypeScript and zero Svelte diagnostics.
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

286 Rust tests including 131 native lifecycle tests pass; six external tests explicitly ignored. Also 16 governance, three storage-harness, six schema and eight browser client tests pass.
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

Production assets and locked Rust workspace build pass. Browser runtime bundle unchanged.
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

All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and workspace/contract lanes.
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
pnpm check:progress
```

**Notes**

32 tasks validate.
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

Mapped protected-file/canonical-chapter gate passes.
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
jj --no-pager diff --name-only -- web
```

**Notes**

Only the generated TypeScript API declaration changes; no UI feature, behavior or visual changes.
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

Adds abandon typed command and capability, optional default-empty/empty-skipped abandoned and skipped_hidden report lists, and HTTP no_changes code. StoredResult V1 and existing report canonical bytes remain compatible; native codecs/WAL formats unchanged. No browser behavior/visual changes; generated TypeScript only. Enclosing Jujutsu change is the revision reference. ResolveConflicts remains M5.3; real RustFS/TLS SSE qualification awaits stopped shared Docker; M4.3 stays Deferred. M5.2 native abandon gate is complete; M5 remains In progress for conflict resolution.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


