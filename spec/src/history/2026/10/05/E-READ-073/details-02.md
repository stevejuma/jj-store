---
schema: 1
id: E-READ-073
---

# E-READ-073 details 2

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
cargo test -p jj-store-cli --test native_rpc --locked native_copy_family_ignores_unrelated_inventory_and_body_pressure -- --nocapture
```

**Notes**

Original regression; one functional resource-limit failure. .local/bounded-reads/performance/copy-family-before-073.log
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
cargo check -p jj-store-jj -p jj-store-service --all-targets --locked
```

**Notes**

Two distinct compiler failures; no functional tests ran. .local/bounded-reads/performance/copy-family-compile-073.log
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
cargo check -p jj-store-jj -p jj-store-service --all-targets --locked
```

**Notes**

Both compilation repairs passed. .local/bounded-reads/performance/copy-family-compile-repair-073.log
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
cargo test -p jj-store-cli --test native_rpc --locked native_copy_family_ignores_unrelated_inventory_and_body_pressure -- --nocapture
```

**Notes**

Original exact target repaired. .local/bounded-reads/performance/copy-family-after-073.log
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
cargo test -p jj-store-cli --test native_rpc --locked native_copy_family_ignores_unrelated_inventory_and_body_pressure -- --nocapture
```

**Notes**

Exact five body reads on cold and warm backend calls. .local/bounded-reads/performance/copy-family-bodies-073.log
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_rpc --locked pinned_copy_families_reuse_additions_without_loading_bodies -- --nocapture
```

**Notes**

One fixture accounting failure: three requests applied to the existing single-request read allowance. .local/bounded-reads/performance/copy-family-pins-073.log
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
cargo fmt --all
```

**Notes**

Formatting passed; two earlier same-command formatting runs also passed. .local/bounded-reads/performance/copy-family-format-073.log
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
cargo test -p jj-store-cli --test native_rpc --locked pinned_copy_families_reuse_additions_without_loading_bodies -- --nocapture
```

**Notes**

26.72s; 1 tests passed. .local/bounded-reads/performance/copy-family-pins-repair-073.log
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
cargo test -p jj-store-jj --test native_backend --locked families:: -- --nocapture
```

**Notes**

12.52s; 3 tests passed. .local/bounded-reads/performance/copy-family-stage-exact-073.log
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
cargo test -p jj-store-jj --lib --locked copy_family -- --nocapture
```

**Notes**

14.15s; 2 tests passed. .local/bounded-reads/performance/copy-family-authority-exact-073.log
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
cargo test -p jj-store-jj --test copy_index --locked
```

**Notes**

1.79s; 7 tests passed. .local/bounded-reads/performance/copy-family-index-073.log
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
cargo test -p jj-store-service --lib --locked native_inventory::families::cache::tests
```

**Notes**

29.34s; 3 tests passed. .local/bounded-reads/performance/copy-family-cache-073.log
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
cargo test -p jj-store-service --lib --locked reads::preparation::tests
```

**Notes**

0.6s; 1 tests passed. .local/bounded-reads/performance/copy-family-preparation-073.log
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
cargo test -p jj-store-cli --test native_rpc --locked native_copy_family_ignores_unrelated_inventory_and_body_pressure -- --nocapture
```

**Notes**

37.19s; 1 tests passed. .local/bounded-reads/performance/copy-family-final-exact-073.log
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
cargo test -p jj-store-jj --lib --locked copy_family -- --nocapture
```

**Notes**

5.7s; 2 tests passed. .local/bounded-reads/performance/copy-family-final-authority-073.log
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
cargo test -p jj-store-jj --lib --locked query::
```

**Notes**

3.91s; 66 tests passed. .local/bounded-reads/performance/copy-family-final-query-073.log
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
cargo test -p jj-store-jj --test native_backend --locked
```

**Notes**

6.61s; 15 tests passed. .local/bounded-reads/performance/copy-family-final-backend-073.log
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
cargo test -p jj-store-service --lib --locked native_inventory::
```

**Notes**

9.83s; 6 tests passed. .local/bounded-reads/performance/copy-family-final-cache-073.log
<!-- /run -->
<!-- /evidence-field -->

