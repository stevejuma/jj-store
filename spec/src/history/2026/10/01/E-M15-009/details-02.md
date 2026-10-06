---
schema: 1
id: E-M15-009
---

# E-M15-009 details 2

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
cargo test -p jj-store-rpc --test collab --locked (before)
```

**Notes**

Target absent.
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
cargo fmt -p jj-store-rpc -p jj-store-service
```

**Notes**

Repeated after production/test edits.
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
cargo check -p jj-store-rpc --all-targets --locked
```

**Notes**

Build regenerates protobuf/prost methods and type-checks every handler.
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
cargo test -p jj-store-rpc --test collab --locked
```

**Notes**

Two actual Unix transport regressions; /tmp/jj-collab-rpc-test.log.
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
cargo test -p jj-store-rpc --test collab --lib --locked (initial codec fixture)
```

**Notes**

WalPosition is a tuple struct, not a new() constructor; corrected fixture.
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
cargo test -p jj-store-rpc --test collab --lib --locked (final)
```

**Notes**

Two transport and three library tests, including one new binary response identity/metadata/byte-carrier regression.
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
cargo test -p jj-store-rpc --locked
```

**Notes**

Complete relevant RPC suite: 16 tests plus zero doctests; generated protobuf service and existing routes pass.
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
cargo test -p jj-store-service --test native_collab --test collab_pagination --locked
```

**Notes**

17 direct native command/read/pagination service regressions pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-rpc -p jj-store-service --all-targets --locked -- -D warnings (initial)
```

**Notes**

Two constant chunks_exact diagnostics in PNG fixtures; replaced with as_chunks::&lt;2&gt;().0.iter().
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
cargo check -p jj-store-cli --all-targets --locked
```

**Notes**

Direct CLI consumer compiles against generated RPC methods.
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
cargo clippy -p jj-store-rpc -p jj-store-service --all-targets --locked -- -D warnings (final)
```

**Notes**

Strict lint passes; no policy suppressions.
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
cargo test -p jj-store-rpc --test collab --lib --locked (final PNG fixtures)
```

**Notes**

Both transport tests and all three library tests pass after the test-only lint repair.
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
cargo fmt -p jj-store-rpc -p jj-store-service -- --check
```

**Notes**

Focused Rust formatting passes.
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
pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Canonical chapters formatted.
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

Nine protected files; seven canonical chapters.
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

