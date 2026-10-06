---
schema: 1
id: E-READ-014
---

# E-READ-014

<!-- evidence-field:id markdown -->
## Id

E-READ-014
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

enclosing Jujutsu change oqrwtyxo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Read-only bounded local commit-index construction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; redb4.3.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native persistence, public wire DTOs, JJ format6 and extra-metadata formats unchanged. Shared compact traversal readers; library-only local construction handle owns disposable output.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/budget.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/preparation/extras.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/segment.rs
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_releases_canceled_workers_and_rechecks_binding
- crates/jj-store-jj/src/local_browser/preparation/extras.rs#malformed_extra_framing_never_becomes_a_synthetic_change
<!-- /evidence-field -->

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
Source inspection: local_browser::Context::open; jj-lib0.45.1 git_backend::read_commit/import_head_commits and stacked_table::get_head
```

**Notes**

Before: ordinary local graph reads preflight complete commits and trees. New fixture confirms missing source tree still rejects that path. Upstream read_commit can implicitly import missing extras/write keep refs; table get_head may reconcile. Local preparation now avoids both methods, reads compact Git fields and seeks verified extra tables.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj --all-targets --locked
```

**Notes**

Shared-source and first local-construction checks passed; compact-reader initializer then omitted git,shallow,extras fields. Corrected constructor and next compilation passed.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance:: -- --nocapture
```

**Notes**

Initial1passed; expanded/final2passed. Exact upstream cold segment/link bytes, rewritten history/later-op pin, damaged source index/missing tree, metadata/ref/dirty-file preservation, corrupt and missing extras, commit quota, output cleanup, cancellation, current revocation and reserved interactive capacity. Final run includes byte charges for table seek records.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

14passed/1ignored: local query/metadata/reference/graph/diff/conflict/history and registration consumers. Ignored external large-repository probe was not run. Expanded for shared local Budget and constructor impact.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

24passed after shared traversal extraction, retaining native exact bytes, graph semantics, cache/quota/concurrency/cancellation/revocation. Existing linker unwind-size advisory remains.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture; cargo test -p jj-store-jj --lib --locked local_browser::preparation:: -- --nocapture
```

**Notes**

2disk quota/order tests and2extra protobuf tests passed. Malformed/truncated/overflow framing fails; legacy and repeated fields preserve pinned protobuf interpretation.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected crate and direct consumers pass.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; pnpm check:boundaries; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md; pnpm check:progress
```

**Notes**

Protected source mapped to3canonical chapters;17package boundaries,167requirements and changed formatting/ledger consumers pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Local construction is a library-only handle, with separate trusted defaults:4m
  commits,500m work,16GiB cumulative read bytes,8m metadata records and1hour
  maximum. Scratch/output require a caller-reserved allowance at most2GiB. No
  large-scale memory/latency claim follows from these limits.
- Local browser graph dispatch still uses existing full preflight. Retained
  verified generations, prepared graph/point readers, service tasks, client
  delivery and aggregate cross-profile quota ownership remain pending. Existing
  non-prepared GitBackend read paths still need the implicit-import audit
  addressed.
- Native preparation limits remain unchanged by this slice. Other
  endpoint/card/collaboration work and
  Linux/native2m/10krefs/deep-page/near-capacity/cold/real-RustFS qualification
  remain open. Running app unchanged; no new browser capability or contract
  lane. E-READ-005 registry/full-capture limitations retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit bounded local construction, then add verified prepared local graph delivery with fresh read-only source capabilities and service-owned cache/task lifecycle.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


