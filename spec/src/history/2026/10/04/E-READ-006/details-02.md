---
schema: 1
id: E-READ-006
---

# E-READ-006 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked operation_pages_do_not_admit_or_build_a_commit_index -- --nocapture
```

**Notes**

Before: small Op Log page rejected by IndexBudget despite needing no commits.
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
cargo test -p jj-store-cli --test local_browser --locked operation_pages_ignore_corrupt_commit_indexes -- --nocapture
```

**Notes**

Before: operation read silently repaired source index. Strengthened byte-identical index assertion exposed this; initial GraphNodes test typo corrected before meaningful baseline.
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
cargo test -p jj-store-cli --test native_lifecycle --locked operation_pages_do_not_admit_or_build_a_commit_index -- --nocapture
```

**Notes**

1 passed; max_index_commits=1 and max_inline=0, cache directory remains empty.
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

10 passed, 1 opt-in ignored. Final regression corrupts all views and index files; Op Log succeeds without repairing them.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Initial run 17 passed/1 failed: old cache-lock test assumed Op Log used index. Changed request to actual indexed all() query and retained lock/cancellation assertions.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Final 18 graph tests passed.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

Initial 3 passed/1 failed: old preparation test expected Op Log Reused cache. Changed to indexed all() and retained cache reuse assertion.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

Final 4 passed.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance::worker_acceptance:: --test-threads=2
```

**Notes**

3 passed.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 pagination_acceptance:: --test-threads=2
```

**Notes**

4 passed.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

All affected Rust targets passed. Removed unused import warning.
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
pnpm contracts:generate; pnpm test:contracts; pnpm web:check
```

**Notes**

Generated additive enum; 7 contract tests plus generated TS; web 0 errors/warnings.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo target/debug/deps/local_browser-893d8c4057fcb950 --exact bounded_reads::registered_large_repository_metadata_smoke --ignored --nocapture
```

**Notes**

Final Linux: Op Log 1.296s, 9 metadata records, 5,238,753 charged bytes, 33 work. One bookmark and ten root rows also pass; scoped .clang-format 24,291 bytes. Whole probe 17.78s under concurrent checks; timings are observations, not isolated benchmarks. Earlier Op Log probe 0.421s included unnecessary view read, subsequently removed.
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
JJ_STORE_BROWSING_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/operations-native-capture.json target/debug/deps/native_lifecycle-e6652c34f7ed2d93 --exact http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured; JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/operations-local-capture.json target/debug/deps/local_browser-893d8c4057fcb950 --exact filesystem_runtime_registers_and_reads_without_native_storage; node --input-type=module (responseValidator on both captures)
```

**Notes**

Both real fixture captures pass; all 43 HTTP/SSE responses validated against current schemas.
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
pnpm spec:first; cargo fmt --all -- --check; pnpm exec prettier --check (changed canonical chapters excluding milestones)
```

**Notes**

Spec-first 13 protected files/6 chapters; Rust and focused document formatting pass. Final pnpm check:progress, pnpm spec:first and pnpm spec:validate also pass: 136 tasks, 13 protected files/7 chapters, 167 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ID-prefix operation selectors retain complete admission. Historical
  commit/file lookup, commit graphs, bounded preparation, cache quotas,
  copies/search/RPC and collaboration projections remain unfinished.
- Full contract capture baseline timeout, registry-only Design Core gate,
  large-scale native 2m, deep-page/cache-loss/corruption/cold/real-RustFS gates
  remain pending. Running app unchanged; no milestone or full-plan completion
  claimed. Unrelated root aggregates and unchanged UI browser tests out of
  scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Validate ledger and commit operation-only reads, then continue bounded history preparation and remaining audited endpoints.
<!-- /evidence-field -->

