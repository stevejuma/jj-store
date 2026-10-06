---
schema: 1
id: E-READ-023
---

# E-READ-023

<!-- evidence-field:id markdown -->
## Id

E-READ-023
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tprtkznw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Operation-bound local visibility, reverse annotations and prepared commit/evolution delivery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64; shared process-wide resident estimate policy from E-READ-020.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Public DTOs, source persistence and task contracts unchanged. Internal canonical selection digest and resident visibility/annotation projections are disposable and exact-operation bound.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/prepared_details.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/dispatch.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation/visibility.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-service/src/local_index_tasks.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_details_preserve_hidden_rewrites_without_history_preflight
- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
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
cargo test -p jj-store-cli --test local_browser --locked prepared_local_details_preserve_hidden_rewrites_without_history_preflight -- --nocapture
```

**Notes**

Before: prepared evolution rejected as Invalid despite a ready generation; prepared-details-before.log. First implementation run failed because four operation visits were insufficient for JJ lookahead and the underlying limit became Unavailable; retained explicit error mapping and admitted eight visits, still below the complete history. prepared-details-after.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_local_details_preserve_hidden_rewrites_without_history_preflight -- --nocapture
```

**Notes**

1pass/8.43s, then strengthened current denial/generation and later-existing-Git-object assertions1pass/10.79s. prepared-details-final.log/prepared-details-auth.log. Selected/current/hidden/root details match upstream; source index corruption/removal does not prevent resident reads.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks -- --nocapture
```

**Notes**

First fixture compilation caught incorrect RefName/RemoteName reference conversions (3errors). Next fixture used a bare divergent symbol, which upstream correctly rejects; switched to explicit /0,/1,/2 offsets. No assertion was weakened. prepared-details-references.log/prepared-details-references-final.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_local_ -- --nocapture
```

**Notes**

3pass/35.81s; after selection-digest/reread checks3pass/17.20s. prepared-details-exact.log/prepared-details-sealed-exact.log. Ten-thousand shared-target bookmarks, conflict add/remove flags, remote ordering, divergent visibility and hidden offsets match identical pinned cold JJ index ordering. Valid-view substitution fails Corrupt.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

24pass/2explicit-Linux-probes-ignored/47.17s; final complete affected local-reader suite24pass/2ignored/30.81s after bounded digest hashing. Shared context, index visibility and HTTP dispatch affect multiple local-reader consumers, justifying full local binary. Real isolated HTTP startup qualifies cold commit/evolution409 recovery and ready responses. prepared-details-suite.log/prepared-details-sealed-suite.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --all-targets --locked; cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Earlier affected package checks and final sealed all-target check pass (19.03s). prepared-details-check.log/prepared-details-check-final.log/prepared-details-sealed-check.log.
<!-- /run -->

<!-- run -->
### Run 7

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

Mapped mutations.md missing after operation-store change; added explicit read-only publication boundary. prepared-details-first.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

11protected source files/5chapters,167requirements,17package boundaries and932web modules. Repeated owning spec checks after final policy changes. prepared-details-*-first/spec/boundaries/architecture.log.
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
cargo fmt --all; cargo fmt --all -- --check; pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md
```

**Notes**

Focused formatting and final Rust formatting pass; prepared-details-format*.log/prepared-details-*-rustfmt.log. Ledger appended without reformatting historical entries. An attempted jj diff --check is unsupported; manual diff inspection used instead, with Rust/spec formatters providing checks.
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
pnpm check:progress
```

**Notes**

Same-change evidence/task bookkeeping passes; prepared-details-progress.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Prepared visibility uses a conservatively admitted96bytes/indexed commit
  B-tree estimate plus exact potential reverse-label/head charges, including
  conflict name replication. This contributes to the existing1GiB resident
  allowance and is not an isolated process-RSS proof. Native equivalent reuse
  and shared global eviction remain open.
- Local change-reference windows, operation/prefix lookup/comparisons,
  historical scoped membership reuse, search/copy/native RPC and deep seek
  continuation remain open. Evolution preserves upstream lookahead/batch
  behavior under explicit operation/predecessor admission; deeper
  operation-predecessor indexing is separate work.
- Latest Linux preparation/isolated RSS/release latency, native2m,
  native10krefs, near-capacity collaboration, restart/revocation/publication
  races and real-RustFS qualification remain pending. Earlier Linux full
  preparation correctness evidence remains E-READ-022, not qualification of this
  new visibility policy.
- Collaboration projections and lightweight visible repository cards remain
  pending. UI/wire-generation/root aggregates/visual baselines out of scope for
  this Rust-only wire-preserving slice. Original dev services and Linux source
  untouched; inherited registry/full-contract gaps retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified local history slice, then implement prepared change-reference and historical/operation lookup and continue native visibility, collaboration/card and large-scale qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


