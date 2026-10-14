---
schema: 1
id: E-CUSTOM-INDEX-LINT-001
---

# E-CUSTOM-INDEX-LINT-001 details 2

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
pnpm dev:cargo -- clippy -p jj-collab --lib --locked -- -D warnings
```

**Notes**

Original workspace reproduces 4 jj-collab lint diagnostics.
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
pnpm dev:cargo -- clippy -p jj-store-jj --no-deps --lib --locked -- -D warnings
```

**Notes**

Original workspace reproduces 22 native query/local lint diagnostics.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-service --no-deps --lib --locked -- -D warnings
```

**Notes**

Original workspace reproduces 6 service lint diagnostics.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc -p jj-store-server --all-targets --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Initial strict wide check stopped at existing collaboration warnings.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-store-service --no-deps --all-targets --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Owned/direct target checks exposed the existing service backlog.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Intermediate array/ID typing errors; corrected.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Intermediate boxed-payload conversion errors; corrected.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Existing HTTP event guards fail strict linting; cleaned up equivalently.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-rpc --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Existing server execution/process backlog: 31 diagnostics; equivalent guard changes and typed inputs added.
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
pnpm dev:cargo -- clippy -p jj-store-server --lib --no-deps --fix --allow-dirty --allow-no-vcs --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Machine-applicable equivalent guards fixed; remaining six warnings still required manual typed-input/enum cleanup. This step alone is not strict acceptance.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-rpc --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

After boxing local reader, the native reader became the large variant; both reader payloads are now boxed.
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
pnpm dev:cargo -- clippy -p jj-store-read-index -p jj-collab -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-rpc --lib --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

All eight affected production libraries pass -D warnings, with no new suppressions.
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
ps -p 91448,91453 -o pid,ppid,stat,command
kill -INT 91453
ps -ax -o pid,ppid,stat,command
```

**Notes**

Verified ownership, stopped the active serial server suite at the user request, and confirmed the runner and its custom-index test processes exited.
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
jj rebase -r nxyuuzpy -o xqllnwon
```

**Notes**

Integrated onto main parent xqllnwon/15906b72. Only the day history navigation conflicted; rebuilt the two day navigation files preserving all main and indexing evidence entries. No production source conflict.
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
pnpm spec:validate --only fileLimits
```

**Notes**

At the verification pause: 154 tasks, 797 evidence records pass bookkeeping; authored file limits pass for 3,805 chapters. This is documentation validation, not resumed Rust or full qualification.
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
jj --no-pager diff --from 6c9eb07d --to @ -- Cargo.toml Cargo.lock crates
jj --no-pager diff --from xqllnwon --to @ -- web catalog desktop
```

**Notes**

Rebase retained the pre-integration Rust source tree exactly and changed no main UI/catalog/desktop files. Main application documentation coverage is equal to the main parent.
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
pnpm docs:review configuration
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/current/progress/M29/index.md
```

**Notes**

Refreshed only configuration source coverage and formatted changed bookkeeping documents. Full docs build deferred.
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
pnpm dev:cargo -- test -p jj-store-jj --lib --test local_stacks --test local_evidence --test native_codec --test compatibility --locked --target-dir .local/build-targets/custom-index
```

**Notes**

native-tests: 132 passed, 1 failed, 3 ignored across 4 binaries. Parallel local-stack Busy; exact case and complete serial lane subsequently pass.
<!-- /run -->
<!-- /evidence-field -->

