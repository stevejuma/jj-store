---
schema: 1
id: E-READ-020
---

# E-READ-020

<!-- evidence-field:id markdown -->
## Id

E-READ-020
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

enclosing Jujutsu change ypuuqqxs
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared resident allocation leases and predecode index framing admission
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native/local persisted formats unchanged. Shared process-local1GiB resident-index estimate allowance replaces independent256MiBexecutor allowances. Exact JJ commit6/changed-path0 framing is admission only.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/memory/framing.rs
- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/mod.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-jj/src/local_browser/preparation/graph.rs#resident_admission_releases_only_when_owner_drops
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/memory.rs#shared_resident_admission_lives_until_the_last_reader_exits
- crates/jj-store-jj/src/query/memory/framing.rs#rejects_allocating_parent_headers_cycles_and_impossible_lengths
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
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
Source inspection: query/cache/resident.rs, query/index.rs and local_browser/preparation/graph.rs
```

**Notes**

Before: native retention weight was checked after upstream decode; local had a separate per-executor counter. Upstream parent-name length allocates before file-framing validation. Shared tests exercise final-owner release and malformed allocation framing.
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
cargo test -p jj-store-jj --lib --locked query::memory:: -- --nocapture
```

**Notes**

First compile found old local Memory unit test after moving admission. Replaced it with shared pool/last-reader coverage; corrected pinned changed-path format0 before final run.
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
cargo test -p jj-store-jj --lib --locked query::memory:: -- --nocapture
```

**Notes**

2passed. Aggregate competing holders, final-reader release, overflow, oversized parent name, cycle, impossible table lengths and excessive commit count covered before decoder entry.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

27passed/7.78s. Complete affected graph module justified by changed shared cache/index admission; native corruption, saved generations, historical scopes, quotas/cancellation/revocation retained.
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

21passed/2ignored/24.51s. Complete local reader/preparation surface consumes shared admission; library and HTTP/task regressions retained. External Linux probes ignored in this suite.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Affected packages/direct consumers checked; existing linker advisory retained.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/local-jj.md spec/src/cli.md; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check
```

**Notes**

10protected files,167requirements,17package boundaries and focused formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The shared allowance accounts conservative immutable index estimates, not
  total process RSS, source Git/redb caches or inline builder peaks. Existing
  native inline/additional-operation fallback retains ordinary query bounds and
  charges final files before retention; fully pre-growth bounded construction
  and immutable generations remain open. No global LRU across native/local
  managers yet; active capacity exhaustion fails explicitly.
- Framing admission supplements checksum verification. Semantic graph/table
  validation remains pinned JJ behavior; malformed cache receipts/links must
  continue failing closed. Large native index measurements and allocation
  pressure/race/crash/platform qualification pending.
- E-READ-018 Linux full preparation remains running on its original binary; no
  new pass claimed. Remaining local endpoints, history/prefix/copy/search/RPC,
  collaboration projections/cards and
  Linux/native2m/10krefs/deep-page/near-capacity/real-RustFS gates remain
  pending. Original dev services unchanged; inherited registry/full-contract
  gaps retained. UI, contracts and root aggregates out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit resident admission; continue bounded inline/immutable generations, local endpoint conversion and cancelable browser preparation while completing scale probes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


