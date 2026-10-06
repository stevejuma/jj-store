---
schema: 1
id: E-READ-075
---

# E-READ-075 details 3

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

7.49s; 0 tests passed. .local/bounded-reads/performance/kind-task-format-deadline-075.log
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
cargo fmt --all
```

**Notes**

7.66s; 0 tests passed. .local/bounded-reads/performance/kind-task-format-final-075.log
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

2.95s; 0 tests passed. .local/bounded-reads/performance/kind-task-format-http-075.log
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

4.72s; 0 tests passed. .local/bounded-reads/performance/kind-task-format-repaired-075.log
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

5.82s; 0 tests passed. .local/bounded-reads/performance/kind-task-format-tests-075.log
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
pnpm contracts:generate
```

**Notes**

222.42s; 0 tests passed. .local/bounded-reads/performance/kind-task-generate-075.log
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-http --lib
```

**Notes**

1.53s; 20 tests passed. .local/bounded-reads/performance/kind-task-http-tests-075.log
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
cargo test --locked -p jj-store-http --lib
```

**Notes**

38.25s; 22 tests passed. .local/bounded-reads/performance/kind-task-http-tests-final-075.log
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
cargo test --locked -p jj-store-service --lib native_inventory::
```

**Notes**

22.2s; 13 tests passed. .local/bounded-reads/performance/kind-task-inventory-tests-075.log
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::kind_task_acceptance
```

**Notes**

26.48s; 0 tests passed. .local/bounded-reads/performance/kind-task-lifecycle-075.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::kind_task_acceptance
```

**Notes**

143.09s; 0 tests passed. .local/bounded-reads/performance/kind-task-lifecycle-repair-075.log
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

189.93s; 235 tests passed. .local/bounded-reads/performance/kind-task-native-lifecycle-075.log
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
cargo check --locked --all-targets -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli -p jj-store-server
```

**Notes**

59.21s; 0 tests passed. .local/bounded-reads/performance/kind-task-package-check-075.log
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

8.58s; 0 tests passed. .local/bounded-reads/performance/kind-task-progress-075.log
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
pnpm check:progress
```

**Notes**

4.61s; 0 tests passed. .local/bounded-reads/performance/kind-task-progress-final-075.log
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
pnpm spec:validate
```

**Notes**

2.31s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-final-075.log
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
pnpm spec:first
```

**Notes**

3.48s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-first-075.log
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
pnpm spec:first
```

**Notes**

4.95s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-first-final-075.log
<!-- /run -->
<!-- /evidence-field -->

