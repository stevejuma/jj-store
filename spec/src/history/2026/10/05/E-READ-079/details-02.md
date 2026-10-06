---
schema: 1
id: E-READ-079
---

# E-READ-079 details 2

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
cargo test -p jj-store-cli --test local_browser managed_code::warm_code_index_does_not_walk_commit_bodies --locked -- --exact --nocapture
```

**Notes**

.local/bounded-reads/managed/baseline.log: 0 passed; old custom preparation exceeds 16 metadata records.
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
cargo check -p jj-store-jj --locked
```

**Notes**

.local/bounded-reads/managed/check.log: initial missing PathBuf/Repo imports and View constructor argument; corrected. check-native.log additionally preserves the missing native module-path attempt.
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
cargo test -p jj-store-cli --test local_browser managed_code::warm_code_index_does_not_walk_commit_bodies --locked -- --exact --nocapture
```

**Notes**

.local/bounded-reads/managed/warm.log: 1 passed, warm construction before history-read assertions.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser managed_code --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/local.log and local-final.log: 0 passed, 2 failed, 1 ignored. Registered/canonical path mismatch and nested scoped source corrected. local-cold-debug.log preserves the separate failed cold page attempt.
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
cargo test -p jj-store-cli --test local_browser managed_code --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/local-corrected.log: 2 passed, 1 ignored. Later selected-chain copying has separate Linux evidence.
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
cargo check -p jj-store-jj --locked
```

**Notes**

.local/bounded-reads/managed/check-selected-chain.log: source preparation now copies only the nearest indexed association and its parent segment chain.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo cargo test -p jj-store-cli --test local_browser managed_code --locked -- --include-ignored --nocapture
```

**Notes**

.local/bounded-reads/managed/linux-final.log: 3 passed. Warm reuse 4.869s, 4 metadata nodes, 0 commit bodies; three 50-row pages 1.389/1.296/1.231s and 54 metadata nodes each. Source index/head/extra-head bytes unchanged.
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
cargo test -p jj-store-cli --test native_lifecycle graph_acceptance::managed_code::standard_native_index_extends_indexed_ancestor --locked -- --exact --nocapture
```

**Notes**

.local/bounded-reads/managed/native.log and native-final.log: 1 passed each. Initial 42 object reads versus 7 incremental; old pinned page retained; warm reopening at most one object read.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

.local/bounded-reads/managed/local-consumers.log: 42 passed, 3 ignored; 83.55s. Direct shared local Pin/scoped/operation guard consumers, including advanced paths.
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
cargo check -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

.local/bounded-reads/managed/check-consumers.log: affected crate and CLI/direct service/RPC/HTTP consumers compile.
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
cargo fmt -p jj-store-jj -p jj-store-cli
```

**Notes**

.local/bounded-reads/managed/Focused Rust formatting.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/local-jj.md spec/src/spec-governance.md docs/implementation-plan.md
```

**Notes**

.local/bounded-reads/managed/format-docs.log.
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
pnpm spec:first
```

**Notes**

.local/bounded-reads/managed/spec-first.log.
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
pnpm spec:validate
```

**Notes**

.local/bounded-reads/managed/spec-validate.log: 5 validators, 23 chapters, 172 requirements.
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
pnpm check:architecture
```

**Notes**

.local/bounded-reads/managed/architecture.log: 992 modules.
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
pnpm check:progress
```

**Notes**

.local/bounded-reads/managed/progress.log: 139 tasks; bookkeeping only.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

.local/bounded-reads/managed/rust-format-check.log.
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
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

.local/bounded-reads/managed/ledger-format.log: only new evidence arrays need formatter compaction.
<!-- /run -->
<!-- /evidence-field -->

