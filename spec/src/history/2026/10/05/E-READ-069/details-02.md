---
schema: 1
id: E-READ-069
---

# E-READ-069 details 2

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
cargo test -p jj-store-cli --test native_rpc --locked browser_operation_prefix_ignores_unrelated_inventory_pressure -- --nocapture
```

**Notes**

One exact regression fails with Limit after96 unrelated files; browser-kind-before-069.log.
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
cargo test -p jj-store-cli --test native_rpc --locked browser_operation_prefix_ignores_unrelated_inventory_pressure -- --nocapture
```

**Notes**

Exact original regression passes; browser-kind-after-069.log.
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
cargo test -p jj-store-cli --test native_rpc --locked browser_ -- --nocapture
```

**Notes**

Two browser-kind regressions pass;warm operations/copies and historical roots each10 reads/zero descriptors;browser-kinds-069.log.
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
cargo test -p jj-store-jj --lib --locked query::access::kinds_tests -- --nocapture
```

**Notes**

First3 owned authority units pass;kind-authority-units-069.log.
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
cargo test -p jj-store-service --lib --locked reads::preparation::tests -- --nocapture
```

**Notes**

One exact explicit preparation regression passes;prepared-kind-units-069.log.
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
cargo test -p jj-store-jj --lib --locked query::access::kinds_tests -- --nocapture
```

**Notes**

Final4 authority units pass,including historical wrapper;kind-authority-final-069.log.
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
cargo test -p jj-store-service --lib --locked reads::preparation::tests -- --nocapture
```

**Notes**

Final exact preparation regression passes,including epoch invalidation;prepared-kind-final-069.log.
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
cargo fmt --all
```

**Notes**

Formatting after each implementation/test update;no unrelated tracked changes.
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
pnpm check:boundaries
```

**Notes**

1.38s; .local/bounded-reads/performance/boundaries-069.log
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

1.49s; .local/bounded-reads/performance/architecture-069.log
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
pnpm spec:first
```

**Notes**

2.51s; .local/bounded-reads/performance/spec-first-069.log
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
cargo fmt --all -- --check
```

**Notes**

4.0s; .local/bounded-reads/performance/rust-format-069.log
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
pnpm spec:validate
```

**Notes**

3.7s; .local/bounded-reads/performance/spec-validation-069.log
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
cargo test -p jj-store-jj --lib --locked query::
```

**Notes**

5.55s; .local/bounded-reads/performance/query-consumers-069.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

3.79s; .local/bounded-reads/performance/document-format-069.log
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
pnpm check:progress
```

**Notes**

4.29s; .local/bounded-reads/performance/progress-069.log
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

128.59s; .local/bounded-reads/performance/packages-069.log
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
cargo test -p jj-store-cli --test native_rpc --test native_lifecycle --locked
```

**Notes**

138.64s; .local/bounded-reads/performance/native-consumers-069.log
<!-- /run -->
<!-- /evidence-field -->

