---
schema: 1
id: E-M15-003
---

# E-M15-003 details 2

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
cargo test -p jj-collab --test target_resolution --locked (before)
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
cargo test -p jj-collab --lib
```

**Notes**

First resolver compile; no lib tests.
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
cargo test -p jj-collab --test target_resolution --locked
```

**Notes**

9 focused regressions.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab --all-targets --locked -- -D warnings
```
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
cargo test -p jj-store-wal --test collab_persistence --locked (before)
```

**Notes**

9 retained native tests.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal --all-targets --locked -- -D warnings (first adapter compile)
```

**Notes**

E0282/E0283 ambiguous closure Result type; corrected explicit error type. Initial native target compile reported the same diagnostics.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test collab_persistence --locked (first adapter compile)
```

**Notes**

Same closure type diagnostics; corrected.
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
cargo test -p jj-store-wal --test collab_persistence --locked (after annotation)
```

**Notes**

11 tests.
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
cargo test -p jj-collab --locked (initial suite)
```

**Notes**

22 tests.
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
cargo test -p jj-store-wal --test collab_persistence --locked -- target_resolver_uses_repository_bound_agent_ids_from_pinned_root (before correction)
```

**Notes**

Resolver initially compared payload hash to existing repository envelope identity. Exact regression exposed mismatch; removed duplicate identity codec and retained owning reader authority.
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
cargo test -p jj-store-wal --test collab_persistence --locked -- target_resolver_uses_repository_bound_agent_ids_from_pinned_root
```

**Notes**

Exact regression after correction.
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
cargo test -p jj-store-wal --test collab_persistence --locked
```

**Notes**

13 native memory-store tests including real repository-bound agent/review IDs, moderation, staged exclusions and old/new pins.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal --all-targets --locked -- -D warnings
```
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
cargo test -p jj-collab --locked
```

**Notes**

22 final domain/resolver tests.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-collab -p jj-store-wal
```
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

4 validators, 21 chapters, 140 requirements.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --all-targets --locked
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Cargo.lock maps to architecture.md. Added exact target-reader ownership boundary.
<!-- /run -->

<!-- run -->
### Run 19

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

111 tasks.
<!-- /run -->
<!-- /evidence-field -->

