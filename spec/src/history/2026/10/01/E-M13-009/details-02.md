---
schema: 1
id: E-M13-009
---

# E-M13-009 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_catalog
```

**Notes**

Before extraction: five native catalog regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_provider
```

**Notes**

Added interface regression before implementation: E0432 unresolved owned interfaces.
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

Repeated after engine extraction and repository mismatch assertion.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_provider --test review_catalog
```

**Notes**

Initial provider and five catalog regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_provider --test review_catalog --test review_commands --test review_relations
```

**Notes**

14 pass; foreign repository provider rejected before read/stage; native publication boundary and audits preserved.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test review_threads
```

**Notes**

Three direct service review regressions pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test review_commands
```

**Notes**

Shared target had stale jj-agent artifacts from sibling checkout; actual local sources contain portable types.
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
touch crates/jj-agent/src/lib.rs
```

**Notes**

Mtime-only rebuild request; no content change.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test review_commands
```

**Notes**

Next stale dependency was jj-store-model; no source defect or workaround.
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
touch crates/*/src/lib.rs
```

**Notes**

Mtime-only invalidation of all local crate entry points; registry dependency cache retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-wal --lib --test review_provider --test review_catalog --test review_commands --test review_relations --locked -- -D warnings
```

**Notes**

Focused owning-package lint.
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
cargo fmt -p jj-store-wal -- --check
```

**Notes**

Rust formatting.
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
pnpm exec prettier --write spec/src/publication.md spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed canonical and evidence documents formatted.
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
pnpm spec:first
```

**Notes**

Seven protected files with owning canonical chapters.
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
pnpm spec:validate
```

**Notes**

Four validators; 21 chapters and 129 requirements.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test review_commands
```

**Notes**

Two CLI contract regressions pass after fresh compilation of all local crates.
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

Final evidence bookkeeping.
<!-- /run -->
<!-- /evidence-field -->

