---
schema: 1
id: E-M17-077
---

# E-M17-077 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_publication --locked
```

**Notes**

Before: two new regressions could not compile; five diagnostics for missing inspect_snapshot_publication. Observed before the interruption.
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
cargo test -p jj-store-capture --test snapshot_publication --locked
```

**Notes**

New test fixture initially imported private request/publication helpers (two E0603 diagnostics); corrected it to inspect the exact Prepared intent.
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
cargo test -p jj-store-capture --test snapshot_publication --test reader_publication --locked
```

**Notes**

Same fixture compilation error before correction; no product test result from this run.
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
cargo test -p jj-store-capture --test snapshot_publication --test reader_publication --locked
```

**Notes**

After fixture correction: 21 JSONL and nine snapshot tests pass; observed output before interruption.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_publication --locked
```

**Notes**

Both original native regressions pass; observed before interruption.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture -p jj-store-server --test snapshot_publication --test reader_publication --test opencode_publication --locked
```

**Notes**

Fresh revalidation after interrupted tool handles and temporary logs disappeared: all 32 top-level tests pass. /tmp/jj-store-publication-exact.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --locked reader_publication::catalog::tests
```

**Notes**

Three units pass, including 4096 snapshot holes, exact capacity refusal and unchanged original JSONL encoding. /tmp/jj-store-publication-catalog.log.
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
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Strict affected packages and all targets pass. /tmp/jj-store-publication-clippy.log.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
cargo fmt -p jj-store-capture
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
pnpm check:architecture
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
pnpm check:boundaries
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
rustc --version; jj --version; uname -sm
```

**Notes**

Affected Rust/specification surface, dependency/module budgets or current versions verified; inspection also used jj status/diff, scoped rg and process/log polling.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture -p jj-store-server --test reader_progress --test codex_recovery --test codex_resume --test codex_transcript --test codex_import --test opencode_checkpoint --test opencode_ingress --test opencode_snapshot --test capture_management --test capture_recovery --test capture_source --test capture_reanchor --test opencode_lifecycle --test opencode_source --test opencode_supervisor --locked
```

**Notes**

All 101 selected top-level direct-consumer tests pass. Recovery child fixtures remain separately invoked. /tmp/jj-store-publication-consumers.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management
```

**Notes**

49 pass, one existing multi-step import test exceeds its ten-second publication deadline under parallel test/consumer load. No assertion or timeout changed. macOS linker also reports __eh_frame exceeds compact unwind offsets. /tmp/jj-store-publication-management.log.
<!-- /run -->
<!-- /evidence-field -->

