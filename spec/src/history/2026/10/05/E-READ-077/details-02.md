---
schema: 1
id: E-READ-077
---

# E-READ-077 details 2

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
cargo fmt --all
```

**Notes**

Run after production and fixture edits; final format check recorded separately.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/mutations.md spec/src/architecture.md spec/src/compatibility.md spec/src/spec-governance.md docs/implementation-plan.md
```

**Notes**

Canonical formatting; architecture rerun after current delivery wording correction.
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
cargo test --locked -p jj-store-cli --test native_rpc remote_copy_family_ignores_unrelated_inventory_and_body_pressure
```

**Notes**

98.08s; 1 Rust tests passed. .local/bounded-reads/performance/remote-family-after-077.log
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
pnpm check:architecture
```

**Notes**

2.28s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-architecture-077.log
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
cargo test --locked -p jj-store-cli --test native_rpc remote_copy_family_ignores_unrelated_inventory_and_body_pressure
```

**Notes**

23.85s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-before-077.log
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
pnpm check:boundaries
```

**Notes**

2.05s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-boundaries-077.log
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
cargo test --locked -p jj-store-cli --test native_rpc
```

**Notes**

49.17s; 28 Rust tests passed. .local/bounded-reads/performance/remote-family-complete-rpc-077.log
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
cargo test --locked -p jj-store-cli --test native_rpc
```

**Notes**

61.8s; 28 Rust tests passed. .local/bounded-reads/performance/remote-family-complete-rpc-final-077.log
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test native_rpc rpc_copy_family_deadline_cancels_pending_storage_and_releases_capacity
```

**Notes**

20.3s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-deadline-diagnostic-077.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md spec/src/mutations.md spec/src/architecture.md spec/src/compatibility.md spec/src/spec-governance.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

7.99s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-doc-final-077.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md spec/src/mutations.md spec/src/architecture.md spec/src/compatibility.md spec/src/spec-governance.md docs/implementation-plan.md
```

**Notes**

2.37s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-doc-format-077.log
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test native_rpc copy_family
```

**Notes**

19.45s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-focused-077.log
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
cargo test --locked -p jj-store-cli --test native_rpc copy_family
```

**Notes**

39.22s; 7 Rust tests passed. .local/bounded-reads/performance/remote-family-focused-final-077.log
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test native_rpc copy_family
```

**Notes**

58.11s; 5 Rust tests passed. .local/bounded-reads/performance/remote-family-focused-fixed-077.log
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
cargo test --locked -p jj-store-service --lib native_inventory::
```

**Notes**

36.89s; 13 Rust tests passed. .local/bounded-reads/performance/remote-family-native-inventory-077.log
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
cargo check --locked --all-targets -p jj-store-service -p jj-store-rpc -p jj-store-cli
```

**Notes**

81.95s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-packages-077.log
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
cargo check --locked --all-targets -p jj-store-service -p jj-store-rpc -p jj-store-cli
```

**Notes**

29.95s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-packages-final-077.log
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

7.54s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-progress-077.log
<!-- /run -->
<!-- /evidence-field -->

