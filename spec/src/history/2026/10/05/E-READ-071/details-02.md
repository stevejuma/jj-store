---
schema: 1
id: E-READ-071
---

# E-READ-071 details 2

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
jj rebase -r @ -o zspxprxy
```

**Notes**

Clean main working change advanced onto E-READ-070, retaining E-READ-069 ancestry. No conflicts or source replacements.
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
jj describe -m "Land and verify pinned native kinds and bounded inventory projections"
```

**Notes**

Main integration evidence uses enclosing change xvyloluq.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-cli --test native_rpc --locked browser_operation_prefix_ignores_unrelated_inventory_pressure
```

**Notes**

90.13s; .local/bounded-reads/performance/landing-browser-exact-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-wal --test publication --locked inventory_projections_preserve_pins_copy_edges_and_manifest_only_visibility
```

**Notes**

13.16s; .local/bounded-reads/performance/landing-projection-exact-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-jj --lib --locked query::
```

**Notes**

24.22s; .local/bounded-reads/performance/landing-query-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-wal --lib --locked
```

**Notes**

22.39s; .local/bounded-reads/performance/landing-wal-library-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-wal --test publication --locked
```

**Notes**

1.62s; .local/bounded-reads/performance/landing-wal-publication-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-service --lib --locked reads::preparation::tests
```

**Notes**

33.25s; .local/bounded-reads/performance/landing-preparation-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-service --lib --locked native_inventory::cache::tests
```

**Notes**

0.73s; .local/bounded-reads/performance/landing-cache-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-rpc --lib --locked inventory::tests
```

**Notes**

34.90s; .local/bounded-reads/performance/landing-wire-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-cli --test native_rpc --locked
```

**Notes**

3.37s; .local/bounded-reads/performance/landing-rpc-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

75.76s; .local/bounded-reads/performance/landing-native-071.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target cargo check -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

51.47s; .local/bounded-reads/performance/landing-packages-071.log
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
cargo fmt --all -- --check
```

**Notes**

3.16s; .local/bounded-reads/performance/landing-rust-format-071.log
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

0.40s; .local/bounded-reads/performance/landing-boundaries-071.log
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
pnpm check:architecture
```

**Notes**

0.61s; .local/bounded-reads/performance/landing-architecture-071.log
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
pnpm spec:first
```

**Notes**

0.92s; .local/bounded-reads/performance/landing-spec-first-071.log
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
pnpm spec:validate
```

**Notes**

1.71s; .local/bounded-reads/performance/landing-spec-validation-071.log
<!-- /run -->
<!-- /evidence-field -->

