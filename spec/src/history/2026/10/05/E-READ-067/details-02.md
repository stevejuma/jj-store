---
schema: 1
id: E-READ-067
---

# E-READ-067 details 2

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
cargo test -p jj-store-cli --test native_rpc --locked inventory::in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions -- --exact --nocapture
```

**Notes**

63.55s; .local/bounded-reads/performance/kind-in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions-before-067.log
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions -- --exact --nocapture
```

**Notes**

43.64s; .local/bounded-reads/performance/kind-rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions-before-067.log
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions -- --exact --nocapture
```

**Notes**

71.86s; .local/bounded-reads/performance/kind-in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions-after-067.log
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions -- --exact --nocapture
```

**Notes**

15.25s; .local/bounded-reads/performance/kind-rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions-after-067.log
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
cargo test -p jj-store-service --lib --locked native_inventory::cache::tests -- --nocapture
```

**Notes**

38.08s; .local/bounded-reads/performance/kind-cache-067.log
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
cargo test -p jj-store-rpc --lib --locked inventory::tests -- --nocapture
```

**Notes**

43.93s; .local/bounded-reads/performance/kind-wire-067.log
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::complete_kind_inventory_discards_nonretained_baselines_and_rejects_corrupt_additions -- --exact --nocapture
```

**Notes**

29.03s; .local/bounded-reads/performance/kind-fallback-verification-067.log
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
cargo test -p jj-store-cli --test native_rpc --locked inventory::rpc_inventory_returns_sorted_copy_ids_and_excludes_staged_objects -- --exact --nocapture
```

**Notes**

13.56s; .local/bounded-reads/performance/kind-copy-verification-067.log
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

15.53s; .local/bounded-reads/performance/kind-rpc-consumers-verification-067.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

87.74s; .local/bounded-reads/performance/kind-native-consumers-verification-067.log
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
cargo test -p jj-store-service --lib --locked native_inventory::cache::tests
```

**Notes**

6.86s; .local/bounded-reads/performance/kind-units-final-verification-067.log
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
cargo check -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

37.87s; .local/bounded-reads/performance/kind-packages-verification-067.log
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
cargo fmt --all -- --check
```

**Notes**

4.42s; .local/bounded-reads/performance/kind-rust-format-067.log
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
pnpm check:boundaries
```

**Notes**

2.02s; .local/bounded-reads/performance/kind-boundaries-067.log
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
pnpm check:architecture
```

**Notes**

2.18s; .local/bounded-reads/performance/kind-architecture-067.log
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
pnpm spec:first
```

**Notes**

2.59s; .local/bounded-reads/performance/kind-spec-first-067.log
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
pnpm spec:validate
```

**Notes**

3.13s; .local/bounded-reads/performance/kind-spec-validation-067.log
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
pnpm check:progress
```

**Notes**

4.83s; .local/bounded-reads/performance/kind-progress-067.log
<!-- /run -->
<!-- /evidence-field -->

