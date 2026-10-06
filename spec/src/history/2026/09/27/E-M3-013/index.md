---
schema: 1
id: E-M3-013
---

# E-M3-013

<!-- evidence-field:id markdown -->
## Id

E-M3-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.2
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

upstream-executable-semantic-parity
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/tests/executable_native.rs
- docs/cli/capabilities.md
- spec/src/cli.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_rebase_squash_split_abandon_and_undo_match_upstream
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_multiway_conflict_snapshot_resolution_matches_upstream
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_divergence_conflicted_bookmarks_and_operation_reconciliation
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_binary_symlink_executable_and_copied_files_match_upstream
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_ssh_sign_resign_and_unsign_preserve_verification
- crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_git_transport_and_gc_fail_without_publication
<!-- /evidence-field -->

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

Before new fixtures: zero matching cases. This baseline does not establish semantic acceptance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

Initial three cases: one pass, two fixture failures. Graph comparison included Git-only @git refs; abandon assertions assumed a selected revision that the revset did not match.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

After local-bookmark comparison: multiway conflict passes; two selectors still resolve to no revisions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

Quoted description selectors still match nothing. Pinned 0.45.1 uses glob matching against full multiline descriptions, so selectors must use subject or an explicit substring pattern.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

Expanded six cases: three pass; the two selector mistakes remain and chmod incorrectly used POSIX -x. Upstream requires n/normal.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic
```

**Notes**

All six scenarios pass after using first-line subject selectors and pinned chmod syntax. Comparisons include ordered parent descriptions/counts, local bookmarks, conflicts, file content, evolution and protected rejection. SSH signatures verify before and after re-signing.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Executable suite: 11 passed / 1 failed / 3 ignored. Old rebase capability assertion still expected unqualified after direct parity acceptance.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Executable suite: 10 passed / 2 failed / 3 ignored. Same old capability assertion plus ephemeral-port rebind collision during in-process service restart. Fixture now retains a socket reservation across service instances; standalone-process tests retain cold TCP acceptance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native
```

**Notes**

All 12 normal executable cases pass, with 3 OS-credential cases explicitly ignored; includes all six semantic cases and both repaired fixtures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

135 Rust cases pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including rustfmt, Rust tests, spec-first and mdBook.
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
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Standalone shared TLS acceptance and release platform qualification remain.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; current Unix host and disposable SSH Ed25519 keys.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence or protocol format changes. The JSON capability matrix now advertises commands with direct semantic acceptance. Reference-only Git tracking annotations are excluded from native/local bookmark comparisons.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


