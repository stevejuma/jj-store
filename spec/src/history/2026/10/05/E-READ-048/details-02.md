---
schema: 1
id: E-READ-048
---

# E-READ-048 details 2

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
cargo test -p jj-store-jj --lib --locked cold_cache_admission_does_not_charge_full_file_hashing_under_namespace -- --nocapture (before)
```

**Notes**

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.34s cold-proof-before-048.log.
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
cargo test -p jj-store-jj --lib --locked query::cache::proof_tests -- --nocapture (after)
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out; finished in 1.59s cold-proof-after-048.log.
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
cargo test -p jj-store-jj --lib --locked query::cache -- --nocapture (initial)
```

**Notes**

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 1.81s cold-proof-cache-units-048.log.
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
cargo test -p jj-store-jj --lib --locked query::cache -- --nocapture (admission-preservation adjustment)
```

**Notes**

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 1.61s cold-proof-cache-units-final-048.log.
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
cargo test -p jj-store-jj --lib --locked query::cache -- --nocapture (failed-proof state preservation)
```

**Notes**

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 1.72s cold-proof-cache-units-complete-048.log.
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
cargo test -p jj-store-jj --lib --locked cold_cache_admission_does_not_charge_full_file_hashing_under_namespace -- --nocapture (exact after)
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 54 filtered out; finished in 0.55s cold-proof-exact-final-048.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked (initial)
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 59.01s cold-proof-package-check-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cache_acceptance -- --nocapture (initial)
```

**Notes**

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 204 filtered out; finished in 6.06s cold-proof-cache-consumers-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_projection_recovery -- --nocapture (immutable-profile correction)
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 2.58s cold-proof-disk-recovery-final-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cache_acceptance -- --nocapture
```

**Notes**

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 204 filtered out; finished in 6.24s; 33.55s command. cold-proof-cache-final-048.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked disk_projection_recovery -- --nocapture
```

**Notes**

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 220 filtered out; finished in 2.09s; 21.08s command. cold-proof-disk-recovery-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cache_acceptance -- --nocapture
```

**Notes**

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 204 filtered out; finished in 6.38s; 45.1s command. cold-proof-cache-complete-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_projection_recovery -- --nocapture
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 2.38s; 25.42s command. cold-proof-disk-recovery-complete-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_ -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 217 filtered out; finished in 1.49s; 19.86s command. cold-proof-graph-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_acceptance:: -- --nocapture
```

**Notes**

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 216 filtered out; finished in 1.95s; 28.28s command. cold-proof-preparation-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_ -- --nocapture
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 0.95s; 17.79s command. cold-proof-disk-preparation-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filtered_references_seek_pinned_text_before_selected_target_reads -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 28.17s; 51.98s command. cold-proof-filter-048.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 1.12s; 16.66s command. cold-proof-http-048.log.
<!-- /run -->
<!-- /evidence-field -->

