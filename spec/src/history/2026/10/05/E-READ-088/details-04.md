---
schema: 1
id: E-READ-088
---

# E-READ-088 details 4

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
pnpm check:ui-ownership
```

**Notes**

ownership-final.log: new extracted attribution component lacked its local provenance entry; entry copied from its owning directory rows and revalidated.
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
cargo test -p jj-store-cli --test local_browser managed_code:: --locked
```

**Notes**

history-local.log: Eight pass, one opt-in smoke ignored. Quick and deeper modes, old paths beyond64, merges, shared wide scan and source byte parity.
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
JJ_STORE_FAST_PATH_CAPTURE=$PWD/.local/fast-path-metadata/local-contracts.json cargo test -p jj-store-cli --test local_browser managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state --locked
```

**Notes**

anchor-final.log: One passes. Quick mixed provenance, signed pagination, direct metadata seek and continuation, raw-anchor plus cursor rejection, source byte parity.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound --locked
```

**Notes**

history-native.log: One passes. Quick integrations/Snapshot and deeper original changes; authoritative write fault remains unconsumed.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --locked
```

**Notes**

check-history-final.log: Affected package checks pass.
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
pnpm dev:backend:build
```

**Notes**

backend-history-build.log: Quick/history executable built; existing linker warning.
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
pnpm dev:backend:build
```

**Notes**

backend-final-build.log: Final direct-seek cursor fix rebuilt.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store pnpm dev:backend:restart
```

**Notes**

backend-history-restart.log: Backend replaced with fixture and Vite preserved; transient reload during restart was retried after readiness.
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
pnpm web:check
```

**Notes**

web-check-complete.log: Final UI zero errors/warnings.
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
pnpm check:architecture
```

**Notes**

architecture-complete.log: 1001 modules pass.
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
pnpm spec:validate
```

**Notes**

spec-final-fixed.log: Five validators,23 chapters,172 requirements pass after provenance entry.
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
pnpm check:ui-ownership
```

**Notes**

ownership-complete.log: Production ownership passes.
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
node .local/fast-path-metadata/format-ledger.mjs
```

**Notes**

ledger-format.log: Only newest E-READ-088 formatted; all530 previous entries preserved. Initial formatting guard rejected an indentation mismatch before writing; corrected.
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
pnpm check:progress
```

**Notes**

progress-history.log: 139 tasks; no milestone promotion.
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
cargo fmt --all -- --check
```

**Notes**

rust-format-final.log: Rust source formatted.
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
xargs pnpm exec prettier --check < .local/fast-path-metadata/changed-format-files.txt
```

**Notes**

format-final.log: Changed UI/contracts/specification/provenance files checked.
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
node --input-type=module responseValidator over local-contracts.json and native-contracts.json
```

**Notes**

contract-captures-final.log: Five local and11 native real HTTP response samples validate against generated schema.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/fast-path-metadata/linux-benchmark.py
```

**Notes**

linux-timings.log/json:17 HTTP200 responses captured. Eighteenth request produced an HTTP error with non-JSON body; parser failed. Performance target missed; no cache/index expansion. Each quick page makes one names and one metadata request; metadata work root314779,arch312816,block320037;67 metadata nodes, zero predicate bytes/lines. Process-first timing with retained OS caches, not an OS-cold benchmark.
<!-- /run -->
<!-- /evidence-field -->

