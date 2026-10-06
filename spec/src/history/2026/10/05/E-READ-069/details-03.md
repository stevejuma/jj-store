---
schema: 1
id: E-READ-069
---

# E-READ-069 details 3

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
cargo test -p jj-store-jj --lib --locked query::access::kinds_tests::upstream_authorities_use_kind_lookup_and_recheck_query_budget -- --exact
```

**Notes**

Strengthened exact unit reproduces lost resource cause:budget.failure None instead of Some(Limit);kind-error-before-069.log.
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
cargo test -p jj-store-jj --lib --locked query::access::kinds_tests::upstream_authorities_use_kind_lookup_and_recheck_query_budget -- --exact
```

**Notes**

Exact strengthened unit passes after recording provider causes in the owned query budget;Limit and Denied survive upstream wrappers.kind-error-after-069.log.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

Changed canonical and implementation documents formatted;repeated after attaching actual results.
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
pnpm check:boundaries
```

**Notes**

1.61s; .local/bounded-reads/performance/boundaries-069-final.log
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
pnpm check:architecture
```

**Notes**

1.73s; .local/bounded-reads/performance/architecture-069-final.log
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
pnpm spec:first
```

**Notes**

2.76s; .local/bounded-reads/performance/spec-first-069-final.log
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
cargo fmt --all -- --check
```

**Notes**

5.54s; .local/bounded-reads/performance/rust-format-069-final.log
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
pnpm spec:validate
```

**Notes**

4.42s; .local/bounded-reads/performance/spec-validation-069-final.log
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
cargo test -p jj-store-jj --lib --locked query::
```

**Notes**

7.14s; .local/bounded-reads/performance/query-consumers-069-final.log
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
pnpm check:progress
```

**Notes**

5.95s; .local/bounded-reads/performance/progress-069-final.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

5.61s; .local/bounded-reads/performance/document-format-069-final.log
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

22.74s; .local/bounded-reads/performance/packages-069-final.log
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
cargo test -p jj-store-cli --test native_rpc --test native_lifecycle --locked
```

**Notes**

125.3s; .local/bounded-reads/performance/native-consumers-069-final.log
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked reads::preparation::tests -- --nocapture
```

**Notes**

Strengthened two-operation fixture reproduces inherited one-object initiating page capacity on explicit task;prepared-bound-before-069.log.
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
cargo test -p jj-store-service --lib --locked reads::preparation::tests -- --nocapture
```

**Notes**

Exact strengthened task passes with independent caller/session result admission;one-object page remains limited.Task caller limit1,expiry,epoch and forbidden full inventory remain enforced.prepared-bound-after-069.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::index_acceptance
```

**Notes**

6 directly affected native preparation/cursor/policy tests pass;225 unrelated tests filtered.preparation-consumers-069.log.
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
pnpm check:boundaries
```

**Notes**

2.4s; .local/bounded-reads/performance/boundaries-069-delivery.log
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
pnpm check:architecture
```

**Notes**

2.63s; .local/bounded-reads/performance/architecture-069-delivery.log
<!-- /run -->
<!-- /evidence-field -->

