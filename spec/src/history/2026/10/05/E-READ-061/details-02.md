---
schema: 1
id: E-READ-061
---

# E-READ-061 details 2

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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (before matching-ID routing)
```

**Notes**

Legacy RPC and native authority fail under32-node admission with96 unrelated retained files;native-inventory-before-061.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (after matching-ID routing)
```

**Notes**

Six inventory cases pass;native-inventory-after-061.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (strengthened native result fixture)
```

**Notes**

Six cases pass;real second operation and exact result exhaustion;native-inventory-native-exact-061.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --lib --locked publication::reader::tests::inventory_pages_continue_before_byte_admission_without_losing_descriptors -- --exact --nocapture (before)
```

**Notes**

Three valid near-ceiling descriptors fail Storage(Limit);native-inventory-byte-before-061.log.
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
cargo test -p jj-store-wal --lib --locked publication::reader::tests::inventory_pages_continue_before_byte_admission_without_losing_descriptors -- --exact --nocapture (after)
```

**Notes**

Early continuation reconstructs exact descriptor order;native-inventory-byte-after-061.log.
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
cargo test -p jj-store-wal --lib --locked publication::reader::tests -- --nocapture
```

**Notes**

Five reader cases pass including byte/work early boundaries,deep seek and retained roots;native-inventory-wal-exact-061.log.
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
cargo test -p jj-store-wal --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 9.95s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.62s; test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.85s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s; test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s; test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s; test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.33s; test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 34.45s; .local/bounded-reads/performance/native-inventory-wal-061.log
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
cargo test -p jj-store-rpc --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 34.96s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.43s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 48.73s; .local/bounded-reads/performance/native-inventory-rpc-061.log
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
cargo test -p jj-store-cli --test native_rpc --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 22.67s; test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.69s; 25.66s; .local/bounded-reads/performance/native-inventory-native-rpc-061.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 24.10s; test result: ok. 228 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 70.23s; 96.39s; .local/bounded-reads/performance/native-inventory-native-lifecycle-061.log
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
cargo check -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 16.78s; 16.89s; .local/bounded-reads/performance/native-inventory-packages-061.log
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
cargo fmt -p jj-store-wal -p jj-store-service -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

; 1.18s; .local/bounded-reads/performance/native-inventory-rust-format-061.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 1.54s; .local/bounded-reads/performance/native-inventory-boundaries-061.log
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

web architecture ok (988 modules); 1.39s; .local/bounded-reads/performance/native-inventory-architecture-061.log
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_rpc --locked inventory::inventory_deadlines_reject_ready_io_that_finishes_after_the_budget -- --exact --nocapture (before split)
```

**Notes**

Ready page I/O escapes1ns admitted timeout;native-inventory-deadline-before-061.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_rpc --locked inventory::inventory_deadlines_ -- --nocapture (before)
```

**Notes**

Independent page and complete-result paths both escape ready-I/O deadline;native-inventory-deadlines-before-061.log.
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
cargo test -p jj-store-cli --test native_rpc --locked inventory:: -- --nocapture (final)
```

**Notes**

Eight exact inventory cases pass including both deadlines;native-inventory-final-exact-061.log.
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
cargo test -p jj-store-rpc --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 27.73s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.54s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.43s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.62s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s; test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s; 41.62s; .local/bounded-reads/performance/native-inventory-rpc-final-061.log
<!-- /run -->
<!-- /evidence-field -->

