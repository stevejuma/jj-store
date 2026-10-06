---
schema: 1
id: E-READ-074
---

# E-READ-074 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids -- --nocapture
```

**Notes**

Baseline lacks Limits and three source APIs;13 compiler diagnostics, two unused-import warnings, no functional tests. .local/bounded-reads/performance/kind-preparation-before-074.log
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
cargo test -p jj-store-service --lib --locked native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids -- --nocapture
```

**Notes**

Implemented target: one fixture assertion failure (605 actual descriptors versus assumed602); no production fault. .local/bounded-reads/performance/kind-preparation-after-074.log
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
cargo fmt --all
```

**Notes**

.local/bounded-reads/performance/kind-preparation-format-074.log
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
cargo fmt --all
```

**Notes**

.local/bounded-reads/performance/kind-preparation-format-final-074.log
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
cargo fmt --all
```

**Notes**

.local/bounded-reads/performance/kind-preparation-format-delivery-074.log
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
cargo fmt --all
```

**Notes**

.local/bounded-reads/performance/kind-preparation-format-reuse-074.log
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/mutations.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

.local/bounded-reads/performance/kind-preparation-document-format-074.log
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
cargo test -p jj-store-service --lib --locked native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids -- --nocapture
```

**Notes**

12.11s; 1 tests passed. .local/bounded-reads/performance/kind-preparation-after-repair-074.log
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
cargo test -p jj-store-service --lib --locked native_inventory::
```

**Notes**

2.04s; 12 tests passed. .local/bounded-reads/performance/kind-preparation-units-074.log
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
cargo test -p jj-store-service --lib --locked reads::preparation::tests
```

**Notes**

0.4s; 1 tests passed. .local/bounded-reads/performance/kind-preparation-pinned-074.log
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
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

25.81s; 0 tests passed. .local/bounded-reads/performance/kind-preparation-package-074.log
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
cargo test -p jj-store-service --lib --locked prepared_kind_progress_and_ready_ids_reauthorize_after_token_revocation -- --nocapture
```

**Notes**

9.32s; 1 tests passed. .local/bounded-reads/performance/kind-preparation-revocation-074.log
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
cargo test -p jj-store-service --lib --locked native_inventory::
```

**Notes**

2.12s; 13 tests passed. .local/bounded-reads/performance/kind-preparation-units-final-074.log
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
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

3.39s; 0 tests passed. .local/bounded-reads/performance/kind-preparation-package-final-074.log
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
cargo test -p jj-store-service --lib --locked native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids -- --nocapture
```

**Notes**

9.55s; 1 tests passed. .local/bounded-reads/performance/kind-preparation-exact-delivery-074.log
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
cargo test -p jj-store-service --lib --locked native_inventory::
```

**Notes**

1.84s; 13 tests passed. .local/bounded-reads/performance/kind-preparation-units-delivery-074.log
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
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

4.7s; 0 tests passed. .local/bounded-reads/performance/kind-preparation-package-delivery-074.log
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
pnpm check:boundaries
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-boundaries-074.log
<!-- /run -->
<!-- /evidence-field -->

