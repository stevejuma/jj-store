---
schema: 1
id: E-M15-005
---

# E-M15-005 details 2

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
cargo test -p jj-store-wal --test collab_commands --locked (before)
```

**Notes**

Target absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-collab -p jj-store-wal -p jj-store-service
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 83
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-wal --all-targets --locked (initial)
```

**Notes**

Ambiguous glob imports selected domain Error/Result; explicit domain imports repair the cascade.
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
cargo check -p jj-store-wal --all-targets --locked
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test collab_commands --locked (initial)
```

**Notes**

Fixture function shadowing and native-copy signature; repaired without production changes.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-wal
```
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
cargo test -p jj-store-wal --test collab_commands --locked
```

**Notes**

Initial seven native command cases; additional disjoint-actor race and staged commit rejection added afterward.
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
cargo test -p jj-store-wal --test collab_commands --locked (final)
```

**Notes**

Eight native provider tests, including source Thread/review races, disjoint actor CAS retry, exact stale state, receipt reauthorization, unknown outcomes/reopen, selected code membership, optional session linking and retained custom images/copy.
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
cargo test -p jj-collab -p jj-store-wal --locked
```

**Notes**

106 tests pass across shared domain and complete WAL suites; memory provider qualification, not real RustFS or cold processes.
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
cargo test -p jj-store-service --test local_collab --locked
```

**Notes**

Eight real filesystem companion regressions pass after extracting shared typed variants.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings
```
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
pnpm check:boundaries
```

**Notes**

16 package boundaries.
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

4 validators, 21 chapters, 140 requirements.
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

7 protected files and 5 canonical chapters.
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

111 tasks.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/milestones.md
```
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
cargo check -p jj-store-cli -p jj-store-rpc --all-targets --locked
```

**Notes**

Direct transport and CLI consumers compile; dependent HTTP/server crates compile as well.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-collab -p jj-store-wal -p jj-store-service -- --check
```
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
pnpm exec prettier --write spec/src/milestones.md
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/milestones.md
```
<!-- /run -->
<!-- /evidence-field -->

