---
schema: 1
id: E-M15-006
---

# E-M15-006 details 2

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
cargo test -p jj-store-service --test native_collab --locked (before)
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
cargo fmt -p jj-store-service
```

**Notes**

Repeated after each implementation and fixture edit.
<!-- /run -->

<!-- run -->
### Run 3

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
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test native_collab --locked (initial)
```

**Notes**

Fixture used struct-update syntax on an enum variant; fixed explicitly.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test native_collab --locked (code profile)
```

**Notes**

Real service creation automatically seeds replication, correctly blocking Thread/Reaction writes until reducer 3. Fixtures now explicitly use supported WorkStore homes and a separate code rejection test preserves that gate. No reducer or authority weakening.
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
cargo test -p jj-store-service --test native_collab --locked (WorkStore home)
```

**Notes**

Seven tests, including explicit code rejection and frozen WorkStore origin retention.
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
cargo test -p jj-store-service --test native_collab --locked (conflict counts)
```

**Notes**

Eight tests; admitted immutable conflict fixture contributes no ordinary value, hidden actors contribute no count or identity.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings (initial)
```

**Notes**

Fixture field assignment after Default; replaced with explicit struct initializer.
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
cargo test -p jj-store-service --test native_collab --locked (deadline)
```

**Notes**

Nine tests; exact unknown request survives service deadline and retry without duplicate publication.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings
```
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
cargo test -p jj-store-service --test native_collab --locked (known errors preserved)
```

**Notes**

Nine exact service regressions pass. Native acceptance remains WorkStore home plus explicit replicated-code rejection.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings (final)
```
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
cargo check -p jj-store-cli -p jj-store-rpc --all-targets --locked
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```
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
pnpm check:boundaries
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate
```
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
pnpm spec:first
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
pnpm check:progress (initial)
```

**Notes**

M15.6 table retained its placeholder dash beside the new evidence ID; removed the placeholder.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-service -- --check
```
<!-- /run -->

<!-- run -->
### Run 20

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```
<!-- /run -->
<!-- /evidence-field -->

