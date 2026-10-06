---
schema: 1
id: E-M15-004
---

# E-M15-004 details 2

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
cargo test -p jj-store-service --test local_collab --locked (before)
```

**Notes**

Target absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-collab --test domain --locked (first projector compile)
```

**Notes**

Incorrect catalog helper name; replaced with existing bounded catalog_update.
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
cargo check -p jj-store-service --all-targets
```

**Notes**

First compile had two unused-import warnings, removed before strict Clippy.
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
cargo clippy -p jj-collab -p jj-store-service --all-targets --locked -- -D warnings (initial)
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
cargo test -p jj-store-service --test local_collab
```

**Notes**

Initial 5 filesystem regressions.
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
cargo test -p jj-store-service --test local_collab --locked (session coverage)
```

**Notes**

6 filesystem regressions, then repeated after bounded queries.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test local_collab --locked (retained-blob fixture added)
```

**Notes**

New fixture immediately dropped its temporary directory through an underscore destructure. Kept the TempDir binding; no product change for this failure.
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
cargo test -p jj-store-service --test local_collab --locked -- selected_descriptor_cannot_read_unretained_staged_blobs
```

**Notes**

Exact repaired fixture proves staged unretained bytes cannot be read through a selected descriptor.
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
cargo test -p jj-collab --locked
```

**Notes**

22 retained model/resolver tests before final projector regression.
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
cargo test -p jj-store-service --test local_collab --test local_agent --test local_agent_pages --test local_review --locked
```

**Notes**

21 focused tests: 7 Agent, 2 Agent pages, 7 new collaboration and 5 Review. Existing consumers selected because source capsule helper visibility and new closure composition share their readers.
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
cargo test -p jj-store-wal --test collab_persistence --locked
```

**Notes**

13 retained native memory-store tests.
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
cargo test -p jj-collab --test domain --locked
```

**Notes**

14 domain tests including shared projector conflict/explicit-resolution history.
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
cargo test -p jj-store-service --test local_collab --locked (final)
```

**Notes**

8 real filesystem tests including library paging, retired-definition rejection, Git GC, reopen/replay, exact revisions, hidden reviews, optional sessions, tool outputs, staged-blob rejection and ref-lock unknown outcome.
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
cargo fmt -p jj-collab -p jj-store-service
```
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

16 packages; allowed direct service-to-owned-domain dependency.
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
cargo clippy -p jj-collab -p jj-store-service --all-targets --locked -- -D warnings (final)
```
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
cargo check -p jj-store-cli -p jj-store-rpc --all-targets --locked
```

**Notes**

Direct Rust consumer compilation, including their dependencies; no HTTP/RPC schema registration changed.
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
cargo fmt -p jj-collab -p jj-store-service -- --check
```
<!-- /run -->
<!-- /evidence-field -->

