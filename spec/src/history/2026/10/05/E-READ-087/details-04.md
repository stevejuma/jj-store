---
schema: 1
id: E-READ-087
---

# E-READ-087 details 4

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
cargo test -p jj-store-cli --test local_browser --locked (full binary; concurrency flags not retained)
```

**Notes**

local-suite-frozen-final.log: 53 passed/0 failed/3 explicitly ignored after scratch/allocation fixes; before the final private work admission and new wide-directory regression.
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
cargo test -p jj-store-cli --test local_browser managed_code:: --locked -- --test-threads=2
```

**Notes**

local-wide-window-final.log: 5 passed/0 failed/1 ignored after combined candidate/path admission and synchronous runtime boundary.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_acceptance:: --locked
```

**Notes**

native-wide-window-final.log: one native continuation/merge/no-authority-write regression passes after combined admission.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

packages-wide-window-final.log: affected packages pass after combined admission/runtime changes.
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
python3 -u .local/path-history-fix/linux-wide-after.py
```

**Notes**

linux-wide-after.log/json: real fixed Linux operation/commit through 7470 proxy, limit 50, 12 finite batches each for block/root/arch. All 36 responses are 200 and below the 30-second transport deadline; block remains 50 Pending at 12 batches, root and arch progress. This predates final union filtering and does not prove complete row coverage.
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
cargo test -p jj-store-cli --test local_browser wide_path_history_skips_unrelated_commits_once_per_window --locked
```

**Notes**

wide-window-prefilter-before.log and wide-window-prefilter-before-same-directory.log: 50-row fixtures pass before union filtering, including the 20,000-work ceiling, continuation and full source byte parity. This is a passing boundary fixture, not a reproduced unit failure.
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
cargo test -p jj-store-cli --test local_browser managed_code:: --locked -- --test-threads=2
```

**Notes**

local-prefilter-final.log: final source, 6 passed/0 failed/1 ignored; continuation, 50-row shared filtering, clean merges, source byte parity, index copying and warm graph reuse.
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

native-prefilter-final.log: final-source exact native merge attribution/continuation/cache/no-authority-write regression passes.
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
cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_resumes_and_preserves_source_state --locked
```

**Notes**

http-prefilter-final.log: final-source HTTP continuation, names-only independence and source state preservation pass.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

packages-prefilter-final.log: affected package checks pass on final production source.
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
cargo fmt --all -- --check
```

**Notes**

rust-format-prefilter-final.log: final Rust source format passes.
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

spec-first-prefilter-final.log: 33 protected files and 8 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

canonical-prefilter-final.log: five validators, 23 chapters and 172 requirements pass.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

preview-prefilter-restart.log: valid final-source backend running at 7472 behind preserved 7470 Vite proxy/fixture.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
python3 -u .local/path-history-fix/linux-prefilter-after.py
```

**Notes**

linux-prefilter-after.log: first cold metadata read returned 200 Pending; following block/root/arch reads returned resource_limit 413 during graph preparation. Explicit existing code-index prepare endpoint recovered to Ready; subsequent finite same-selection acceptance is recorded separately. No source/fixture reset.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

contracts-frozen-final.log: generated OpenAPI/TypeScript match; 283 actual HTTP/SSE responses conform, seven contract tests and TypeScript contract compilation pass. Final private Rust admission/filter changes are additionally covered by final managed/native/HTTP targets.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-eviction-before.log: 8 passed/1 failed; a pending result after cache eviction removed an already resolved description. Exact new regression reproduces the defect.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-eviction-final.log: final UI source, nine tests pass including cache eviction, partial completion, Busy backoff, failure retention, cancellation and filename-first behavior.
<!-- /run -->
<!-- /evidence-field -->

