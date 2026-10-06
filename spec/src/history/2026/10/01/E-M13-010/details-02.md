---
schema: 1
id: E-M13-010
---

# E-M13-010 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_projector
```

**Notes**

Before extraction: two E0432 absent projector diagnostics; five draft fixture shadow/type errors corrected to full immutable thread snapshot preconditions.
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
python3 /tmp/jj-store-extract-review-projector.py
```

**Notes**

Extracted command projection, complete review-owned closure and cross-object audits; left all native external verification and publication wrappers.
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
cargo fmt -p jj-store-wal
```

**Notes**

Owning crate formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_projector
```

**Notes**

One forged fixture failed shape validation before cross-object audit; corrected it to a shape-valid but false event tail.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_projector
```

**Notes**

Two independent-provider regressions pass; exact native root/result byte parity, old snapshots, stale/resolved rules and false event tail rejection.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_projector --test review_provider --test review_catalog --test review_commands --test review_relations --test native_copy
```

**Notes**

17 pass across native review/provider/projector and cold native-copy restoration.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-replication --locked --test review_convergence --test collaboration --test bootstrap
```

**Notes**

10 pass; graph/workflow reduction and transitive root-closure verification are direct consumers of this extraction.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test review_threads
```

**Notes**

Three pass including selected object detail reads, body-byte limits and subscription freshness.
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
cargo fmt -p jj-store-wal
```

**Notes**

Owning crate formatting repeated after final comments and negative test.
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
cargo fmt -p jj-store-wal -- --check
```

**Notes**

Final Rust formatting.
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
pnpm exec prettier --write spec/src/publication.md spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Canonical and ledger formatting.
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
pnpm spec:first
```

**Notes**

19 protected files; owning chapters and ledger included.
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

Four validators; 21 chapters and 129 requirements.
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

15 package boundaries.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test review_commands
```

**Notes**

Two parser contract regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-wal --lib --test review_projector --test review_provider --test review_catalog --test review_commands --test review_relations --test native_copy --locked -- -D warnings
```

**Notes**

One collapsible_if lint; collapsed repository identity guard without changing behavior.
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
cargo fmt -p jj-store-wal
```

**Notes**

Formatting after lint correction.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-wal --lib --test review_projector --test review_provider --test review_catalog --test review_commands --test review_relations --test native_copy --locked -- -D warnings
```

**Notes**

Focused owning-package lint clean.
<!-- /run -->
<!-- /evidence-field -->

