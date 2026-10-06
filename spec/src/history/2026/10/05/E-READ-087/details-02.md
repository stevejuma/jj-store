---
schema: 1
id: E-READ-087
---

# E-READ-087 details 2

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
cargo test -p jj-store-cli --test local_browser indexed_path_history_resumes_beyond_sixty_four_commits --locked
```

**Notes**

before.log: one exact product regression; metadata beyond 64 ancestors never resolves.
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
cargo test -p jj-store-cli --test local_browser indexed_path_history_resumes_beyond_sixty_four_commits --locked
```

**Notes**

local-final.log and cooperative-final.log: JJ ancestry continuation, optional changed-path extension, cached reuse and synchronous/cooperative-poll regression pass; source byte parity retained.
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
cargo test -p jj-store-cli --test local_browser source_changed_path_index_is_copied_without_repair_or_history_tree_reads --locked
```

**Notes**

source-copy-final.log: upstream changed-path segments copied with no source repair/tree scans.
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
cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_resumes_and_preserves_source_state --locked
```

**Notes**

http-path-final.log: real HTTP optional attribution resumes; names-only query remains independent; source state preserved.
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
cargo test -p jj-store-cli --test local_browser managed_code:: --locked
```

**Notes**

local-managed-reader-final.log: 4 passed and one explicit large-repository fixture ignored before scratch merge addition.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_acceptance:: --locked
```

**Notes**

native-final.log and native-metadata-runtime-final.log: bounded native metadata continuation and cached reuse pass.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_acceptance:: --locked
```

**Notes**

native-merge-files.log: clean merge synthesizes content successfully; fixture wrongly assumed Right edit wins equal-time native branch ordering. Inspecting actual native JJ topological traversal requires Left edit; assertion remains exact and rejects merge attribution.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_acceptance:: --locked
```

**Notes**

native-merge-corrected.log: exact four-row attribution, continuation, cached reuse and untouched injected authoritative-write fault pass.
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
cargo test -p jj-store-cli --test local_browser indexed_merge_history_resolves_parent_contents_without_source_writes --locked
```

**Notes**

local-merge-files.log: clean merged-parent contents yield original branch edit; complete source .jj/repo byte parity holds.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

local-suite-final.log: 51 passed/1 failed/3 ignored. Fixed 3-second automatic activation check observed index_rebuild_required under concurrent pressure; exact rerun local-pressure-rerun.log passed.
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
cargo test -p jj-store-cli --test local_browser managed_local_http_activates_without_subscribers_resumes_and_captures_tasks --locked
```

**Notes**

local-pressure-rerun.log: exact activation failure rerun passed without weakening assertions.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=4
```

**Notes**

local-suite-bounded-concurrency.log: 52 passed/3 ignored before scratch addition.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

native-suite-final.log: 238 passed/1 failed; independent upstream deep-anchor expected graph query reached Limit under pressure. Exact native-pressure-rerun.log passed.
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
cargo test -p jj-store-cli --test native_lifecycle prepared_all_log_pages_seek_deep_anchors_under_fixed_work --locked
```

**Notes**

native-pressure-rerun.log: exact deep-anchor rerun passed without assertion changes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

merge-files-check.log: scratch limit usize/u64 compile mismatches; fixed before corrected package check.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
cargo check (affected JJ/service packages; check-initial.log, check-service.log, check-service-corrected.log)
```

**Notes**

Initial integration failures: omitted Context.code_index (1), missing prost dependency for checked source index link decoding (8), moved request in service wrapper (1). Corrected before check-service-final.log; later final exact package command passes.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

merge-files-check-corrected.log and packages-frozen-final.log: all affected package checks pass after scratch and allocation fixes.
<!-- /run -->

<!-- run -->
### Run 18

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

contracts-generate.log: additive Pending and NotFound native/local reasons generated; existing routes unchanged.
<!-- /run -->
<!-- /evidence-field -->

