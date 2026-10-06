---
schema: 1
id: E-READ-015
---

# E-READ-015

<!-- evidence-field:id markdown -->
## Id

E-READ-015
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

enclosing Jujutsu change llnzqsrq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Verified prepared local log pages with fresh source capabilities
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Persistence and wire DTOs unchanged. Library-owned prepared local log handle retains a verified JJ format6 resident and read-only extra segment handles.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/commit.rs
- crates/jj-store-jj/src/local_browser/preparation/extras.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
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
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
- crates/jj-store-jj/src/local_browser/preparation/graph.rs#resident_admission_releases_only_when_owner_drops
<!-- /evidence-field -->

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
cargo check -p jj-store-jj --all-targets --locked
```

**Notes**

Initial2type errors: extra_headers.find expects string, not byte literal. Corrected; final affected check follows.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance:: -- --nocapture
```

**Notes**

Two initial runs aborted on worker stack overflow. Heap buffer plus boxed async loader/construction removed stack pressure.
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

Final3passed: joined1-row pages without full history admission; source tree/index damage; cache-file removal; missing extras without import; fresh revocation; conflict/filtered graph reads; same-size extra mutation with restored mtime. Logs local-prepared-pages-final.log and earlier boxed/expanded/stamps runs.
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
cargo test -p jj-store-jj --lib --locked local_browser::preparation:: -- --nocapture
```

**Notes**

3passed: protobuf framing and resident charges retained until final owner release.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

Initial14passed/1failed/1ignored. Equal-timestamp siblings differed between source incremental and cold indexes. Replaced reference with exact upstream cold segment parity plus exact ordered graph rows; no sorting/normalization.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance::prepared_local_log_preserves_conflicts_and_filtered_native_edges -- --exact --nocapture
```

**Notes**

1passed; cold segment/link bytes identical to pinned upstream, with exact graph rows/edges for all, description and bookmark ancestry.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

Final15passed/1ignored; expanded because shared local graph/budget/capability paths changed. External large-repository probe not run.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected crates/direct consumers checked. Existing linker unwind-size advisory remains in test builds.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; pnpm check:boundaries; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md; pnpm check:progress
```

**Notes**

Protected canonical chapters,17package boundaries,167requirements,formatting and ledger consumers checked.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Prepared log reader remains library-only. Ordinary local graph HTTP dispatch
  still performs full preflight. Service-owned retained generation/task
  lifecycle and browser delivery remain next. Non-prepared upstream GitBackend
  calls still require implicit-import audit.
- Resident estimates use twice segment bytes plus512KiB and shallow-root
  overhead capped at256MiB/originating executor. Source extra seeks check
  stamps; coarse/non-Unix proof falls back to bounded full checksums. Broad
  platform and peak-memory qualification pending.
- Native limits, cross-profile/process quota admission, immutable generations,
  historical/prefix/copy/search/RPC and collaboration/card projections remain
  open. Linux/native2m/10krefs/deep-page/near-capacity/cold/real-RustFS
  qualification remains pending. No running service changed. Unchanged UI/native
  graph/root aggregate lanes out of scope; E-READ-005 registry/full-contract
  limitations retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified prepared local logs, then wire shared quota admission, local task/cache lifecycle and HTTP/browser delivery.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


