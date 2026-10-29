---
schema: 1
id: E-INJECTABLE-STORES-001
---

# E-INJECTABLE-STORES-001 details 2

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
pnpm dev:disk
```

**Notes**

Initial free 31.3 GiB, target 28.0 GiB. Managed Cargo pre/postflight subsequently cleaned eligible idle outputs; live embedded storage outputs remained protected.
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
pnpm dev:cargo -- test -p jj-store-service --test local_collab local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc --locked -- --exact
```

**Notes**

Before: 1 passed. After cutover: 1 passed. The later complete local_collab family also passed all 16 tests.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-core --lib
```

**Notes**

Initial extraction omitted review_command_commit; all four missing-method errors were corrected by moving the existing helper.
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
pnpm dev:cargo -- test -p jj-store-core --locked
```

**Notes**

Initial four core tests passed; after adding the canonical record test the completed core suite passed five tests and zero doctests.
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
pnpm dev:cargo -- test -p jj-store-core --locked
```

**Notes**

A subsequent manifest edit required a lockfile refresh; the locked run refused it. The following offline check refreshed the intended workspace lockfile.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-wal --offline
```

**Notes**

Native error boundaries and migrated helper visibility needed adaptation; no test assertion or persisted format was loosened.
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
pnpm dev:cargo -- check -p jj-store-wal --locked
```

**Notes**

Native publication adapter compiled against the extracted engines. Four obsolete helper warnings were subsequently removed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 70
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- check -p jj-store-service --locked
```

**Notes**

Initial service migration exposed old trait result types and error conversions. An offline short-format follow-up reduced this to five errors, then an exact regression build found one remaining error; all were corrected at typed boundaries.
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
pnpm dev:cargo -- check -p jj-store-service --offline --message-format=short
```

**Notes**

Remaining local provider I/O signatures and missing-record mapping were adapted to core errors.
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
pnpm dev:cargo -- test -p jj-store-service --test local_collab local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc --locked -- --exact
```

**Notes**

One collaboration error conversion used a nonexistent From implementation; replaced it with the existing map_wal boundary.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-wal --locked --message-format=short
```

**Notes**

First test compilation exposed migrated provider fixture signatures and expected error namespaces. Second attempt exposed six more fixture errors. Assertions retained their exact stale/corrupt/intent-conflict expectations.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-wal --locked --message-format=short
```

**Notes**

Adapted the remaining test provider signatures and preserved existing semantic assertions.
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
pnpm dev:cargo -- test -p jj-store-wal --locked --message-format=short
```

**Notes**

138 passed across the complete WAL suite; no failures or ignored tests. This broader lane is required because shared readers, projection and canonical control-record codecs serve independent domain consumers.
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
pnpm dev:cargo -- test -p jj-store-service --lib --test local_agent --test local_work --test local_review --test local_collab --test native_collab --test review_threads --test work_store --locked --message-format=short
```

**Notes**

138 passed: service unit 61, local_agent 21, local_collab 16, local_review 8, local_work 11, native_collab 10, review_threads 6, work_store 5. This is the affected service surface, not the complete service or Rust workspace suite.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-wal -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Found preexisting collapsible_if in untouched S3 uncertainty bookkeeping. Collapsed the equivalent guard under JST-OBJ-001 without changing behavior.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-wal -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Remaining service test fixtures used the previous trait error type. Adapted those signatures; the next attempt found two obsolete imports, which were removed.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-wal -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Removed obsolete wal aliases in the adapted review-activity test fixtures.
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
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-wal -p jj-store-service --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

All targets passed for the three selected crates and their dependency checks.
<!-- /run -->
<!-- /evidence-field -->

