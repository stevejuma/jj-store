---
schema: 1
id: E-READ-076
---

# E-READ-076 details 2

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
pnpm exec prettier --write spec/src/architecture.md spec/src/spec-governance.md docs/implementation-plan.md
```

**Notes**

Main landing document formatting; output retained in this chat's main integration tool result.
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
pnpm check:architecture
```

**Notes**

2.57s; 0 Rust tests passed. .local/bounded-reads/performance/main-architecture-076.log
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
pnpm check:boundaries
```

**Notes**

2.3s; 0 Rust tests passed. .local/bounded-reads/performance/main-boundaries-076.log
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
pnpm test:contracts
```

**Notes**

4.3s; 0 Rust tests passed. .local/bounded-reads/performance/main-contract-types-076.log
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
cargo test --locked -p jj-store-jj --test copy_index
```

**Notes**

27.73s; 7 Rust tests passed. .local/bounded-reads/performance/main-copy-primitive-076.log
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
cargo test --locked -p jj-store-jj --lib query::
```

**Notes**

25.08s; 66 Rust tests passed. .local/bounded-reads/performance/main-copy-query-076.log
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
pnpm exec prettier --check spec/src/architecture.md spec/src/spec-governance.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

2.31s; 0 Rust tests passed. .local/bounded-reads/performance/main-doc-check-076.log
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
pnpm exec prettier --check spec/src/architecture.md spec/src/spec-governance.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

2.32s; 0 Rust tests passed. .local/bounded-reads/performance/main-doc-check-final-076.log
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
cargo test --locked -p jj-store-http --lib
```

**Notes**

73.14s; 22 Rust tests passed. .local/bounded-reads/performance/main-http-contract-076.log
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
cargo test --locked -p jj-store-jj --test native_backend
```

**Notes**

4.28s; 15 Rust tests passed. .local/bounded-reads/performance/main-native-backend-076.log
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
cargo test --locked -p jj-store-service --lib native_inventory::
```

**Notes**

41.52s; 13 Rust tests passed. .local/bounded-reads/performance/main-native-inventory-076.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle
```

**Notes**

104.77s; 235 Rust tests passed. .local/bounded-reads/performance/main-native-lifecycle-076.log
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
cargo test --locked -p jj-store-cli --test native_rpc
```

**Notes**

148.72s; 22 Rust tests passed. .local/bounded-reads/performance/main-native-rpc-076.log
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
cargo check --locked --all-targets -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli -p jj-store-server
```

**Notes**

65.59s; 0 Rust tests passed. .local/bounded-reads/performance/main-packages-076.log
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
cargo test --locked -p jj-store-service --lib reads::preparation::tests
```

**Notes**

0.88s; 1 Rust tests passed. .local/bounded-reads/performance/main-preparation-pin-076.log
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

6.33s; 0 Rust tests passed. .local/bounded-reads/performance/main-progress-076.log
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
pnpm check:progress
```

**Notes**

5.98s; 0 Rust tests passed. .local/bounded-reads/performance/main-progress-final-076.log
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
pnpm format:rust:check
```

**Notes**

2.75s; 0 Rust tests passed. .local/bounded-reads/performance/main-rust-format-076.log
<!-- /run -->
<!-- /evidence-field -->

