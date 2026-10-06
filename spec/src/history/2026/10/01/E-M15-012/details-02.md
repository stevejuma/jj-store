---
schema: 1
id: E-M15-012
---

# E-M15-012 details 2

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
cargo test -p jj-store-wal --test bound_capture --locked (before)
```

**Notes**

New exact target absent; /tmp/jj-bound-capture-before.log.
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
cargo check -p jj-store-wal --all-targets --locked
```

**Notes**

Affected provider package and targets compile; /tmp/jj-bound-capture-check.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test bound_capture --locked (initial)
```

**Notes**

Ambiguous ThreadStatusV1 and two nonexistent ThreadLifecycleActionV1 uses; qualified domain status and used ThreadActionV1; /tmp/jj-bound-capture-test.log.
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
cargo test -p jj-store-wal --test bound_capture --locked (fixed)
```

**Notes**

Four atomic provider regressions; /tmp/jj-bound-capture-test-fixed.log.
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
cargo test -p jj-store-rpc --lib --locked
```

**Notes**

Four codec regressions; /tmp/jj-bound-capture-codec.log.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --locked (initial)
```

**Notes**

Linking ran out of disk space; no CLI assertion ran. Removed only this isolated workspace target/debug/incremental (17 GiB generated cache), then used CARGO_INCREMENTAL=0; /tmp/jj-bound-capture-spool.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --locked
```

**Notes**

Four spool and four drain tests; /tmp/jj-bound-capture-spool-fixed.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-wal --test bound_capture --test agent_catalog --test collab_commands --test collab_persistence --locked
```

**Notes**

Four bound tests, three Agent catalog, thirteen collaboration command and eight persistence regressions; /tmp/jj-bound-capture-wal-consumers.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --locked
```

**Notes**

All 17 relevant package tests pass, zero doctests; /tmp/jj-bound-capture-rpc-suite.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-collab --locked
```

**Notes**

23 shared domain tests pass; /tmp/jj-bound-capture-domain.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-wal --test bound_capture --locked
```

**Notes**

Five regressions including deterministic CAS races: disjoint Thread creation preserved, changed primary assignment selects no Agent edit; /tmp/jj-bound-capture-races.log.
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
cargo fmt -p jj-collab -p jj-store-wal -p jj-store-rpc -p jj-store-cli
```

**Notes**

Focused Rust formatting, repeated after regressions.
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
CARGO_INCREMENTAL=0 cargo clippy -p jj-collab -p jj-store-wal -p jj-store-rpc -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Strict shared-domain/provider/RPC/CLI lint passes; /tmp/jj-bound-capture-clippy.log.
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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/publication.md spec/src/milestones.md
```

**Notes**

Canonical chapters and evidence ledger formatted.
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
pnpm spec:first
```

**Notes**

12 protected files; five canonical chapters.
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

Four validators; 21 chapters; 140 requirements.
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

111 tasks; evidence bookkeeping only.
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

16 package boundaries.
<!-- /run -->
<!-- /evidence-field -->

